use crate::storage_types::{
    DataKey, FreezeInfo, ComplianceStatus, INSTANCE_BUMP_AMOUNT, INSTANCE_LIFETIME_THRESHOLD,
};
use soroban_sdk::{
    contract, contractimpl, symbol_short,
    Address, Env, IntoVal, String, Symbol, Val, Vec,
};
use soroban_sdk::token::Interface as StandardTokenInterface;


fn check_nonnegative_amount(amount: i128) {
    if amount < 0 {
        panic!("negative amount is not allowed: {}", amount)
    }
}

fn get_active_freeze_info(e: &Env, account: &Address) -> Option<FreezeInfo> {
    let key = DataKey::AccountFreezeInfo(account.clone());
    if let Some(info) = e.storage().instance().get::<_, FreezeInfo>(&key) {
        if info.expiration_ledger == 0 {
            // Süresiz dondurma, her zaman aktif
            return Some(info);
        } else {
            // Süreli dondurma
            if e.ledger().sequence() < info.expiration_ledger {
                return Some(info);
            } else {
                return None;
            }
        }
    }
    None
}

fn is_account_effectively_frozen(e: &Env, account: &Address) -> bool {
    get_active_freeze_info(e, account).is_some()
}

fn emit_freeze_event(
    e: &Env,
    event_type_symbol: Symbol,
    admin: Address,
    account: Address,
    freeze_info: Option<FreezeInfo>,
) {
    let mut topics: Vec<Val> = Vec::new(e);
    topics.push_back(event_type_symbol.into_val(e));
    topics.push_back(symbol_short!("admin").into_val(e));
    topics.push_back(symbol_short!("account").into_val(e));

    let mut data: Vec<Val> = Vec::new(e);
    data.push_back(admin.into_val(e));
    data.push_back(account.into_val(e));

    if let Some(info) = freeze_info {
        if info.expiration_ledger != 0 {
            topics.push_back(symbol_short!("expires").into_val(e));
            data.push_back(info.expiration_ledger.into_val(e));
        }
        topics.push_back(symbol_short!("reason").into_val(e));
        data.push_back(info.reason.clone().into_val(e));

        if info.compliance_tag != ComplianceStatus::None {
            topics.push_back(Symbol::new(e, "compliance").into_val(e));
            data.push_back(info.compliance_tag.into_val(e));
        }
    }
    e.events().publish(topics, data);
}

#[contract]
pub struct Token;

#[contractimpl]
impl Token {
    pub fn initialize(e: Env, admin: Address, decimal: u32, name: String, symbol: String) {
        if crate::admin::has_administrator(&e) {
            panic!("already initialized");
        }
        crate::admin::write_administrator(&e, &admin);
        if decimal > u8::MAX.into() {
            panic!("Decimal must fit in a u8");
        }
        crate::metadata::write_metadata(
            &e,
            soroban_token_sdk::metadata::TokenMetadata { decimal, name, symbol },
        );
        e.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
    }

    pub fn mint(e: Env, to: Address, amount: i128) {
        check_nonnegative_amount(amount);
        let admin = crate::admin::read_administrator(&e);
        admin.require_auth();
        e.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
        crate::balance::receive_balance(&e, to.clone(), amount);
        e.events().publish(
            (Symbol::new(&e, "mint"), admin.clone(), to.clone()),
            amount,
        );
    }

    pub fn set_admin(e: Env, new_admin: Address) {
        let admin = crate::admin::read_administrator(&e);
        admin.require_auth();
        e.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
        crate::admin::write_administrator(&e, &new_admin.clone());
        e.events().publish(
            (Symbol::new(&e, "set_admin"), admin, new_admin),
            Val::VOID,
        );
    }

    pub fn freeze_account(
        e: Env,
        account_to_freeze: Address,
        duration_ledgers: u32,
        reason: String,
        compliance_tag: ComplianceStatus,
    ) {
        let admin = crate::admin::read_administrator(&e);
        admin.require_auth();
        e.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);

        let expiration_ledger = if duration_ledgers == 0 {
            0
        } else {
            e.ledger()
                .sequence()
                .checked_add(duration_ledgers)
                .expect("Ledger overflow for freeze duration")
        };

        let freeze_info_to_store = FreezeInfo {
            expiration_ledger,
            reason: reason.clone(),
            compliance_tag,
        };

        let key = DataKey::AccountFreezeInfo(account_to_freeze.clone());
        e.storage().instance().set(&key, &freeze_info_to_store);
        

        emit_freeze_event(
            &e,
            Symbol::new(&e, "freeze_acc"),
            admin,
            account_to_freeze,
            Some(freeze_info_to_store),
        );
    }

    pub fn unfreeze_account(e: Env, account_to_unfreeze: Address) {
        let admin = crate::admin::read_administrator(&e);
        admin.require_auth();
        e.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
        let key = DataKey::AccountFreezeInfo(account_to_unfreeze.clone());

        if e.storage().instance().has(&key) {
            e.storage().instance().remove(&key);
            emit_freeze_event(
                &e,
                Symbol::new(&e, "unfrz_acc"),
                admin,
                account_to_unfreeze,
                None,
            );
        }
    }

    pub fn get_account_freeze_details(e: Env, account: Address) -> Option<FreezeInfo> {
        e.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
        get_active_freeze_info(&e, &account)
    }

    pub fn is_frozen(e: Env, account: Address) -> bool {
        e.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
        is_account_effectively_frozen(&e, &account)
    }

    pub fn get_compliance_tag_for_account(e: Env, account: Address) -> ComplianceStatus {
        e.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
        if let Some(info) = get_active_freeze_info(&e, &account) {
            info.compliance_tag
        } else {
            ComplianceStatus::None
        }
    }
}

#[contractimpl]
impl StandardTokenInterface for Token {
    fn allowance(e: Env, from: Address, spender: Address) -> i128 {
        e.storage().instance().extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
        crate::allowance::read_allowance(&e, from, spender).amount
    }

    fn approve(e: Env, from: Address, spender: Address, amount: i128, expiration_ledger: u32) {
        from.require_auth();
        check_nonnegative_amount(amount);
        e.storage().instance().extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
        if is_account_effectively_frozen(&e, &from) {
            panic!("Hesap dondurulmuş ve approve işlemi yapılamaz");
        }
        crate::allowance::write_allowance(&e, from.clone(), spender.clone(), amount, expiration_ledger);
        e.events().publish(
            (Symbol::new(&e, "approve"), from, spender, expiration_ledger),
            amount,
        );
    }

    fn balance(e: Env, id: Address) -> i128 {
        e.storage().instance().extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
        crate::balance::read_balance(&e, id)
    }

    fn transfer(e: Env, from: Address, to: Address, amount: i128) {
        from.require_auth();
        check_nonnegative_amount(amount);
        e.storage().instance().extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
        if is_account_effectively_frozen(&e, &from) {
            panic!("Hesap dondurulmuş ve token transfer edilemez");
        }
        crate::balance::spend_balance(&e, from.clone(), amount);
        crate::balance::receive_balance(&e, to.clone(), amount);
        e.events().publish(
            (Symbol::new(&e, "transfer"), from, to),
            amount,
        );
    }

    fn transfer_from(e: Env, spender: Address, from: Address, to: Address, amount: i128) {
        spender.require_auth();
        check_nonnegative_amount(amount);
        e.storage().instance().extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
        if is_account_effectively_frozen(&e, &from) {
            panic!("Hesap (from) dondurulmuş ve token transfer edilemez");
        }
        crate::allowance::spend_allowance(&e, from.clone(), spender.clone(), amount);
        crate::balance::spend_balance(&e, from.clone(), amount);
        crate::balance::receive_balance(&e, to.clone(), amount);
        e.events().publish(
            (Symbol::new(&e, "transfer"), from, to),
            amount,
        );
    }

    fn burn(e: Env, from: Address, amount: i128) {
        from.require_auth();
        check_nonnegative_amount(amount);
        e.storage().instance().extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
        if is_account_effectively_frozen(&e, &from) {
            panic!("Hesap dondurulmuş ve token yakılamaz");
        }
        crate::balance::spend_balance(&e, from.clone(), amount);
        e.events().publish(
            (Symbol::new(&e, "burn"), from),
            amount,
        );
    }

    fn burn_from(e: Env, spender: Address, from: Address, amount: i128) {
        spender.require_auth();
        check_nonnegative_amount(amount);
        e.storage().instance().extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
        if is_account_effectively_frozen(&e, &from) {
            panic!("Hesap (from) dondurulmuş ve token yakılamaz");
        }
        crate::allowance::spend_allowance(&e, from.clone(), spender, amount);
        crate::balance::spend_balance(&e, from.clone(), amount);
        e.events().publish(
            (Symbol::new(&e, "burn"), from),
            amount,
        );
    }

    fn decimals(e: Env) -> u32 {
        crate::metadata::read_decimal(&e)
    }
    fn name(e: Env) -> String {
        crate::metadata::read_name(&e)
    }
    fn symbol(e: Env) -> String {
        crate::metadata::read_symbol(&e)
    }
}