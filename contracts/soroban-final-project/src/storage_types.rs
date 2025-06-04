use soroban_sdk::{contracttype, Address, String};

pub(crate) const DAY_IN_LEDGERS: u32 = 17280;
pub(crate) const INSTANCE_BUMP_AMOUNT: u32 = 7 * DAY_IN_LEDGERS;
pub(crate) const INSTANCE_LIFETIME_THRESHOLD: u32 = INSTANCE_BUMP_AMOUNT - DAY_IN_LEDGERS;
pub(crate) const BALANCE_BUMP_AMOUNT: u32 = 30 * DAY_IN_LEDGERS;
pub(crate) const BALANCE_LIFETIME_THRESHOLD: u32 = BALANCE_BUMP_AMOUNT - DAY_IN_LEDGERS;


#[derive(Clone, Debug, PartialEq, Eq, Copy)]
#[contracttype]
pub enum ComplianceStatus {
    None,
    KycPending,
    AmlFlagged,
    Sanctioned,
    Other,
}

#[derive(Clone, Debug, PartialEq, Eq)]
#[contracttype]
pub struct FreezeInfo {
    pub expiration_ledger: u32,
    pub reason: String,
    pub compliance_tag: ComplianceStatus,
}

#[derive(Clone)]
#[contracttype]
pub struct AllowanceDataKey {
    pub from: Address,
    pub spender: Address,
}

#[contracttype]
pub struct AllowanceValue {
    pub amount: i128,
    pub expiration_ledger: u32,
}

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Allowance(AllowanceDataKey),
    Balance(Address),
    Admin,
    AccountFreezeInfo(Address),
}