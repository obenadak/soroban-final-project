#![cfg(test)]
extern crate std;

use crate::{
    storage_types::ComplianceStatus,
    TokenClient,
};
use soroban_sdk::{
    testutils::{
        Address as _, Ledger, LedgerInfo,
        MockAuth, MockAuthInvoke,
    },
    Address, Env, IntoVal, String, Symbol, Val, BytesN,
};

static WASM_BYTES: &[u8] = include_bytes!(
    "../../../target/wasm32v1-none/release/soroban_final_project.wasm"
);


fn create_token_and_init<'a>(e: &Env, admin: &Address) -> TokenClient<'a> {
    let deployer_address = Address::generate(e);
    let wasm_hash = e.deployer().upload_contract_wasm(WASM_BYTES);
    let salt = BytesN::from_array(e, &[0; 32]);
    let token_id = e.deployer()
        .with_address(deployer_address, salt.clone())
        .deploy_v2(wasm_hash.clone(), ());

    let token = TokenClient::new(e, &token_id);
    token.initialize(
        admin,
        &7,
        &String::from_str(e, "Test Token"),
        &String::from_str(e, "TST"),
    );
    token
}

fn jump_ledgers(e: &Env, ledgers_to_jump: u32) {
    let mut current_ledger_info = e.ledger().get();
    let _original_sequence = current_ledger_info.sequence_number;
    current_ledger_info.sequence_number += ledgers_to_jump;
    current_ledger_info.timestamp += ledgers_to_jump as u64 * 5;

    e.ledger().set(LedgerInfo {
        sequence_number: current_ledger_info.sequence_number,
        protocol_version: current_ledger_info.protocol_version,
        timestamp: current_ledger_info.timestamp,
        network_id: current_ledger_info.network_id,
        base_reserve: current_ledger_info.base_reserve,
        min_temp_entry_ttl: current_ledger_info.min_temp_entry_ttl,
        min_persistent_entry_ttl: current_ledger_info.min_persistent_entry_ttl,
        max_entry_ttl: current_ledger_info.max_entry_ttl,
    });
}

#[test]
fn test_initialize_and_metadata_check() {
    let e = Env::default();
    e.mock_all_auths();
    let admin = Address::generate(&e);
    let token_name = String::from_str(&e, "MegaToken");
    let token_symbol = String::from_str(&e, "MGT");
    let token_decimals = 10u32;

    let deployer_address = Address::generate(&e);
    let wasm_hash = e.deployer().upload_contract_wasm(WASM_BYTES);
    let salt = BytesN::from_array(&e, &[1; 32]);
    let token_id = e.deployer()
        .with_address(deployer_address, salt.clone())
        .deploy_v2(wasm_hash.clone(), ());

    let token = TokenClient::new(&e, &token_id);
    token.initialize(&admin, &token_decimals, &token_name, &token_symbol);

    assert_eq!(token.decimals(), token_decimals);
    assert_eq!(token.name(), token_name);
    assert_eq!(token.symbol(), token_symbol);
}

#[test]
fn test_mint_and_balance_check() {
    let e = Env::default();
    e.mock_all_auths();
    let admin = Address::generate(&e);
    let user1 = Address::generate(&e);
    let token = create_token_and_init(&e, &admin);

    assert_eq!(token.balance(&user1), 0);
    token.mint(&user1, &1000);
    assert_eq!(token.balance(&user1), 1000);
}

#[test]
fn test_approve_allowance_transfer_from() {
    let e = Env::default();
    e.mock_all_auths();
    let admin = Address::generate(&e);
    let owner = Address::generate(&e);
    let spender = Address::generate(&e);
    let recipient = Address::generate(&e);
    let token = create_token_and_init(&e, &admin);

    token.mint(&owner, &1000);
    let current_ledger = e.ledger().sequence();
    let expiration_ledger = current_ledger + 100;

    token.approve(&owner, &spender, &500, &expiration_ledger);
    assert_eq!(token.allowance(&owner, &spender), 500);

    token.transfer_from(&spender, &owner, &recipient, &300);
    assert_eq!(token.balance(&owner), 700);
    assert_eq!(token.balance(&recipient), 300);
    assert_eq!(token.allowance(&owner, &spender), 200);
}

#[test]
fn test_timed_freeze_with_reason_and_compliance() {
    let e = Env::default();
    e.mock_all_auths();

    let admin = Address::generate(&e);
    let user_a = Address::generate(&e);
    let user_b = Address::generate(&e);
    let token = create_token_and_init(&e, &admin);

    token.mint(&user_a, &5000);

    let freeze_duration: u32 = 10;
    let freeze_reason = String::from_str(&e, "Şüpheli Aktivite İncelemesi");
    let compliance_details = ComplianceStatus::AmlFlagged;

    let ledger_before_freeze = e.ledger().sequence();

    token.freeze_account(
        &user_a,
        &freeze_duration,
        &freeze_reason,
        &compliance_details,
    );

    assert!(token.is_frozen(&user_a));
    let stored_freeze_info = token.get_account_freeze_details(&user_a).expect("Dondurma bilgisi bulunmalı.");
    let expected_expiration = ledger_before_freeze + freeze_duration;
    assert_eq!(stored_freeze_info.expiration_ledger, expected_expiration);
    assert_eq!(stored_freeze_info.reason, freeze_reason);
    assert_eq!(stored_freeze_info.compliance_tag, compliance_details);
    assert_eq!(token.get_compliance_tag_for_account(&user_a), compliance_details);


    let transfer_result = e.try_invoke_contract::<Val, Val>(
        &token.address,
        &Symbol::new(&e, "transfer"),
        (user_a.clone(), user_b.clone(), 100_i128).into_val(&e),
    );
    assert!(transfer_result.is_err());
    assert_eq!(token.balance(&user_a), 5000);

    jump_ledgers(&e, freeze_duration - 1);
    assert!(token.is_frozen(&user_a));

    jump_ledgers(&e, 1);
    let current_ledger_after_final_jump = e.ledger().get().sequence_number;
    let is_frozen_after_expiry = token.is_frozen(&user_a);
    assert!(!is_frozen_after_expiry, "Account should be unfrozen. Current: {}, Stored Expiry: {}", current_ledger_after_final_jump, stored_freeze_info.expiration_ledger);
    assert!(token.get_account_freeze_details(&user_a).is_none());
    assert_eq!(token.get_compliance_tag_for_account(&user_a), ComplianceStatus::None);

    token.transfer(&user_a, &user_b, &1000);
    assert_eq!(token.balance(&user_a), 4000);
    assert_eq!(token.balance(&user_b), 1000);
}

#[test]
fn test_indefinite_freeze_and_manual_unfreeze() {
    let e = Env::default();
    e.mock_all_auths();
    let admin = Address::generate(&e);
    let user_c = Address::generate(&e);
    let token = create_token_and_init(&e, &admin);
    token.mint(&user_c, &500);

    let reason = String::from_str(&e, "Kalıcı Uyumluluk İhlali");
    token.freeze_account(&user_c, &0, &reason.clone(), &ComplianceStatus::Sanctioned);

    assert!(token.is_frozen(&user_c));
    let info = token.get_account_freeze_details(&user_c).unwrap();
    assert_eq!(info.expiration_ledger, 0);
    assert_eq!(info.reason, reason);
    assert_eq!(info.compliance_tag, ComplianceStatus::Sanctioned);

    jump_ledgers(&e, 10000);
    assert!(token.is_frozen(&user_c));

    token.unfreeze_account(&user_c);
    assert!(!token.is_frozen(&user_c));
    assert!(token.get_account_freeze_details(&user_c).is_none());
    assert_eq!(token.get_compliance_tag_for_account(&user_c), ComplianceStatus::None);
}

#[test]
#[should_panic]
fn test_approve_on_frozen_account_should_panic() {
    let e = Env::default();
    e.mock_all_auths();
    let admin = Address::generate(&e);
    let user_frozen = Address::generate(&e);
    let spender = Address::generate(&e);
    let token = create_token_and_init(&e, &admin);

    token.mint(&user_frozen, &1000);
    token.freeze_account(
        &user_frozen,
        &0,
        &String::from_str(&e, "approve_test_freeze"),
        &ComplianceStatus::KycPending,
    );

    let current_ledger = e.ledger().sequence();
    token.approve(&user_frozen, &spender, &100, &(current_ledger + 100));
}

#[test]
#[should_panic]
fn test_transfer_on_frozen_account_should_panic() {
    let e = Env::default();
    e.mock_all_auths();
    let admin = Address::generate(&e);
    let user_frozen = Address::generate(&e);
    let recipient = Address::generate(&e);
    let token = create_token_and_init(&e, &admin);

    token.mint(&user_frozen, &1000);
    token.freeze_account(
        &user_frozen,
        &10,
        &String::from_str(&e, "transfer_test_freeze"),
        &ComplianceStatus::None,
    );
    token.transfer(&user_frozen, &recipient, &100);
}

#[test]
#[should_panic]
fn test_transfer_from_on_frozen_account_panics() {
    let e = Env::default();
    e.mock_all_auths();
    let admin = Address::generate(&e);
    let owner_frozen = Address::generate(&e);
    let spender = Address::generate(&e);
    let recipient = Address::generate(&e);
    let token = create_token_and_init(&e, &admin);

    token.mint(&owner_frozen, &1000);
    let current_ledger = e.ledger().sequence();
    token.approve(&owner_frozen, &spender, &500, &(current_ledger + 100));
    token.freeze_account(&owner_frozen, &0, &String::from_str(&e, "donduruldu"), &ComplianceStatus::Other);

    token.transfer_from(&spender, &owner_frozen, &recipient, &100);
}

#[test]
#[should_panic]
fn test_burn_on_frozen_account_panics() {
    let e = Env::default();
    e.mock_all_auths();
    let admin = Address::generate(&e);
    let user1 = Address::generate(&e);
    let token = create_token_and_init(&e, &admin);

    token.mint(&user1, &1000);
    token.freeze_account(&user1, &0, &String::from_str(&e, "donduruldu"), &ComplianceStatus::None);
    token.burn(&user1, &100);
}

#[test]
#[should_panic]
fn test_burn_from_on_frozen_account_panics() {
    let e = Env::default();
    e.mock_all_auths();
    let admin = Address::generate(&e);
    let owner_frozen = Address::generate(&e);
    let spender = Address::generate(&e);
    let token = create_token_and_init(&e, &admin);

    token.mint(&owner_frozen, &1000);
    let current_ledger = e.ledger().sequence();
    token.approve(&owner_frozen, &spender, &500, &(current_ledger + 100));
    token.freeze_account(&owner_frozen, &0, &String::from_str(&e, "donduruldu"), &ComplianceStatus::Other);

    token.burn_from(&spender, &owner_frozen, &100);
}


#[test]
#[should_panic]
fn test_non_admin_cannot_freeze_account() {
    let e = Env::default();

    let real_admin = Address::generate(&e);
    let _non_admin_user = Address::generate(&e);
    let account_to_be_frozen = Address::generate(&e);

    let deployer_address = Address::generate(&e);
    let wasm_hash = e.deployer().upload_contract_wasm(WASM_BYTES);
    let salt = BytesN::from_array(&e, &[2; 32]);
    let token_id = e.deployer()
        .with_address(deployer_address, salt.clone())
        .deploy_v2(wasm_hash.clone(), ());

    let token_for_init = TokenClient::new(&e, &token_id);

    e.mock_auths(&[MockAuth {
        address: &real_admin,
        invoke: &MockAuthInvoke {
            contract: &token_id,
            fn_name: "initialize",
            args: (
                real_admin.clone(),
                7u32,
                String::from_str(&e, "AuthTest"),
                String::from_str(&e, "AUT"),
            )
                .into_val(&e),
            sub_invokes: &[],
        },
    }]);

    token_for_init.initialize(
        &real_admin,
        &7,
        &String::from_str(&e, "AuthTest"),
        &String::from_str(&e, "AUT"),
    );
    e.set_auths(&[]);

    let client_as_non_admin = TokenClient::new(&e, &token_id);

    client_as_non_admin.freeze_account(
        &account_to_be_frozen,
        &0,
        &String::from_str(&e, "Yetkisiz Deneme"),
        &ComplianceStatus::Other,
    );
}

#[test]
#[should_panic]
fn test_initialize_already_initialized_panics() {
    let e = Env::default();
    e.mock_all_auths();
    let admin = Address::generate(&e);
    let token = create_token_and_init(&e, &admin);

    token.initialize(
        &admin,
        &10,
        &String::from_str(&e, "Ikinci Token"),
        &String::from_str(&e, "IKT"),
    );
}

#[test]
#[should_panic]
fn test_decimal_over_max_panics() {
    let e = Env::default();
    e.mock_all_auths();
    let admin = Address::generate(&e);

    let deployer_address = Address::generate(&e);
    let wasm_hash = e.deployer().upload_contract_wasm(WASM_BYTES);
    let salt = BytesN::from_array(&e, &[3; 32]);
    let token_id = e.deployer()
        .with_address(deployer_address, salt.clone())
        .deploy_v2(wasm_hash.clone(), ());

    let token = TokenClient::new(&e, &token_id);

    token.initialize(
        &admin,
        &(u32::from(u8::MAX) + 1),
        &String::from_str(&e, "Decimal Test"),
        &String::from_str(&e, "DCT"),
    );
}


#[test]
#[should_panic(expected = "InvalidAction")]
fn test_freeze_duration_overflow_panics() {
    let e = Env::default();
    e.mock_all_auths();
    let admin = Address::generate(&e);
    let user_a = Address::generate(&e);
    let token = create_token_and_init(&e, &admin);

    jump_ledgers(&e, 1);
    assert_eq!(e.ledger().get().sequence_number, 1, "Ledger should be 1 before overflow test");

    token.freeze_account(
        &user_a,
        &u32::MAX,
        &String::from_str(&e, "Taşma Testi"),
        &ComplianceStatus::None,
    );
}