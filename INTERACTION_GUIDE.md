## Testnet Deployment & Interaction Guide

This guide details the steps to compile, deploy, and interact with the FreezeGuardToken contract on the Stellar Testnet.

**Prerequisites:**
*   Soroban CLI installed.
*   At least one funded Testnet account configured in your Soroban CLI (e.g., named `alice`).

### 1. Compiling the Contract
```bash
stellar contract build
```
This command compiles the Rust smart contract into a WebAssembly (WASM) file, typically located at `target/wasm32-unknown-unknown/release/soroban_final_project.wasm`.

### 2. Deploying the Contract to Testnet
Deploy the compiled WASM file to the Testnet. Ensure your `alice` identity is funded.
```bash
stellar contract deploy \
  --wasm target/wasm32-unknown-unknown/release/soroban_final_project.wasm \
  --source alice \
  --network testnet
```
- **Contract ID:** Note the Contract ID returned by this command. For this guide, we'll use the example ID: `CAO7HFVLQ3KWIO5XW6MWRU3LJRDBPEEHYAPTIXSUC6XF67GPBMD2WI7P`. Replace this with your actual Contract ID in the following commands.
- **Example Deployment Transaction:** You can view the deployed contract on Stellar Expert (replace with your contract's link): [https://stellar.expert/explorer/testnet/contract/CAO7HFVLQ3KWIO5XW6MWRU3LJRDBPEEHYAPTIXSUC6XF67GPBMD2WI7P](https://stellar.expert/explorer/testnet/contract/CAO7HFVLQ3KWIO5XW6MWRU3LJRDBPEEHYAPTIXSUC6XF67GPBMD2WI7P)

### 3. Initializing the Contract
Initialize the contract with an administrator, decimal places, name, and symbol.
- **Admin Account (`alice`) Public Key:** `GCCE3U7AYGHR33DGHKZ6KCCUCAUOYVOPTHPLN7UI2U2FNM53H6ZC2YIB` (Verify with `stellar keys address alice`).
```bash
stellar contract invoke \
  --id CAO7HFVLQ3KWIO5XW6MWRU3LJRDBPEEHYAPTIXSUC6XF67GPBMD2WI7P \
  --source alice \
  --network testnet \
  -- \
  initialize \
  --admin GCCE3U7AYGHR33DGHKZ6KCCUCAUOYVOPTHPLN7UI2U2FNM53H6ZC2YIB \
  --decimal 7 \
  --name "MyFinalToken" \
  --symbol "MFT"
```
- **Successful `initialize` Transaction:** [https://stellar.expert/explorer/testnet/tx/38cffa3ec0801f5158df979996037cc42444509f5b4016c8b74d29789ecd4bb8](https://stellar.expert/explorer/testnet/tx/38cffa3ec0801f5158df979996037cc42444509f5b4016c8b74d29789ecd4bb8)

### 4. Reading Token Metadata
These are read-only operations.
- **Get Decimals (`decimals`):** Expected output: `7`
  ```bash
  stellar contract invoke --id CAO7HFVLQ3KWIO5XW6MWRU3LJRDBPEEHYAPTIXSUC6XF67GPBMD2WI7P --network testnet -- decimals
  ```
- **Get Token Name (`name`):** Expected output: `"MyFinalToken"`
  ```bash
  stellar contract invoke --id CAO7HFVLQ3KWIO5XW6MWRU3LJRDBPEEHYAPTIXSUC6XF67GPBMD2WI7P --network testnet -- name
  ```
- **Get Token Symbol (`symbol`):** Expected output: `"MFT"`
  ```bash
  stellar contract invoke --id CAO7HFVLQ3KWIO5XW6MWRU3LJRDBPEEHYAPTIXSUC6XF67GPBMD2WI7P --network testnet -- symbol
  ```

### 5. Token Management Operations

#### 5.1. Minting Tokens (`mint`)
- **Mint to User Account (User-1):** Admin (`alice`) mints 100 MFT (1,000,000,000 base units) to `GDSD3EWDONOYUJI2G6H3UFD6CTO2JCBAVYKCQF6U6DQWVGQB7F63AHK5` (User-1).
  ```bash
  stellar contract invoke \
    --id CAO7HFVLQ3KWIO5XW6MWRU3LJRDBPEEHYAPTIXSUC6XF67GPBMD2WI7P \
    --source alice \
    --network testnet -- \
    mint \
    --to GDSD3EWDONOYUJI2G6H3UFD6CTO2JCBAVYKCQF6U6DQWVGQB7F63AHK5 \
    --amount 1000000000
  ```
  - Transaction: [https://stellar.expert/explorer/testnet/tx/46d21820b0e10b8bf1205d25666068cb4de574f18520d1c8ba3f94c7d51719b3](https://stellar.expert/explorer/testnet/tx/46d21820b0e10b8bf1205d25666068cb4de574f18520d1c8ba3f94c7d51719b3)
  - Verify User-1 Balance: Expected output `"1000000000"`
    ```bash
    stellar contract invoke --id CAO7HFVLQ3KWIO5XW6MWRU3LJRDBPEEHYAPTIXSUC6XF67GPBMD2WI7P --network testnet -- balance --id GDSD3EWDONOYUJI2G6H3UFD6CTO2JCBAVYKCQF6U6DQWVGQB7F63AHK5
    ```

- **Mint to Admin Account (`alice`):** Admin (`alice`) mints 50 MFT (500,000,000 base units) to their own account.
  ```bash
  stellar contract invoke \
    --id CAO7HFVLQ3KWIO5XW6MWRU3LJRDBPEEHYAPTIXSUC6XF67GPBMD2WI7P \
    --source alice \
    --network testnet -- \
    mint \
    --to GCCE3U7AYGHR33DGHKZ6KCCUCAUOYVOPTHPLN7UI2U2FNM53H6ZC2YIB \
    --amount 500000000
  ```
  - Transaction: [https://stellar.expert/explorer/testnet/tx/8acc684051947c24b04b0f9cf7f600f3ecfca8da4eca35def56c0f6b6daa0b58](https://stellar.expert/explorer/testnet/tx/8acc684051947c24b04b0f9cf7f600f3ecfca8da4eca35def56c0f6b6daa0b58)
  - Verify Admin Balance: Expected output `"500000000"`
    ```bash
    stellar contract invoke --id CAO7HFVLQ3KWIO5XW6MWRU3LJRDBPEEHYAPTIXSUC6XF67GPBMD2WI7P --network testnet -- balance --id GCCE3U7AYGHR33DGHKZ6KCCUCAUOYVOPTHPLN7UI2U2FNM53H6ZC2YIB
    ```

#### 5.2. Transferring Tokens (`transfer`)
- Admin (`alice`) transfers 20 MFT (200,000,000 base units) to User-1 (`GDSD3EWDONOYUJI2G6H3UFD6CTO2JCBAVYKCQF6U6DQWVGQB7F63AHK5`).
  ```bash
  stellar contract invoke \
    --id CAO7HFVLQ3KWIO5XW6MWRU3LJRDBPEEHYAPTIXSUC6XF67GPBMD2WI7P \
    --source alice \
    --network testnet -- \
    transfer \
    --from GCCE3U7AYGHR33DGHKZ6KCCUCAUOYVOPTHPLN7UI2U2FNM53H6ZC2YIB \
    --to GDSD3EWDONOYUJI2G6H3UFD6CTO2JCBAVYKCQF6U6DQWVGQB7F63AHK5 \
    --amount 200000000
  ```
  - Transaction: [https://stellar.expert/explorer/testnet/tx/0053583f01050ebd19c6006819e85da19b8e663358d07592d36132416ec69a53](https://stellar.expert/explorer/testnet/tx/0053583f01050ebd19c6006819e85da19b8e663358d07592d36132416ec69a53)
  - Post-Transfer Balances:
    - Admin (`alice`): Expected `"300000000"` (30 MFT)
    - User-1 (`GDSD3EWDONOYUJI2G6H3UFD6CTO2JCBAVYKCQF6U6DQWVGQB7F63AHK5`): Expected `"1200000000"` (120 MFT)

### 6. Allowance Management (Spending Permissions)

This section covers how one account (`alice`) can authorize another account (`spender`) to spend its tokens (`approve`), and how this authorization is used (`transfer_from`).

#### 6.1. Creating a Spender Account
To properly test `transfer_from`, a separate spender account is needed. Let's name it `artemis_spender`.
- **Generate and Fund `artemis_spender`:**
  ```bash
  stellar keys generate artemis_spender --network testnet
  ```
  - **`artemis_spender` Public Key:** `GDEBNAUJB6UQLB7EHWO4XLOZNODXBJS33KQSPXYIS2AUIWQC3GA7NRQB` (Verify with `stellar keys address artemis_spender`).

#### 6.2. Approving Spending Allowance (`approve`)
- `alice` grants `artemis_spender` permission to spend 5 MFT (50,000,000 base units) of her tokens.
- The `expiration_ledger` specifies the last ledger for which the allowance is valid. Calculate this by adding an offset (e.g., `17280` for ~1 day) to the current `core_latest_ledger` (obtained via `curl https://horizon-testnet.stellar.org`).
  - Example `expiration_ledger`: `1412000 (current ledger) + 17280 = 1429280`
  ```bash
  stellar contract invoke \
    --id CAO7HFVLQ3KWIO5XW6MWRU3LJRDBPEEHYAPTIXSUC6XF67GPBMD2WI7P \
    --source alice \
    --network testnet -- \
    approve \
    --from GCCE3U7AYGHR33DGHKZ6KCCUCAUOYVOPTHPLN7UI2U2FNM53H6ZC2YIB \
    --spender GDEBNAUJB6UQLB7EHWO4XLOZNODXBJS33KQSPXYIS2AUIWQC3GA7NRQB \
    --amount 50000000 \
    --expiration_ledger 1429280 
  ```
  - Transaction: [https://stellar.expert/explorer/testnet/tx/a5a5d41fa68593f949d7558ee016ff53cb24bc2132fafd6ba79e589ae4c4b0b7](https://stellar.expert/explorer/testnet/tx/a5a5d41fa68593f949d7558ee016ff53cb24bc2132fafd6ba79e589ae4c4b0b7)

#### 6.3. Checking Allowance (`allowance`)
- Verify the allowance granted by `alice` to `artemis_spender`.
  ```bash
  stellar contract invoke \
    --id CAO7HFVLQ3KWIO5XW6MWRU3LJRDBPEEHYAPTIXSUC6XF67GPBMD2WI7P \
    --network testnet -- \
    allowance \
    --from GCCE3U7AYGHR33DGHKZ6KCCUCAUOYVOPTHPLN7UI2U2FNM53H6ZC2YIB \
    --spender GDEBNAUJB6UQLB7EHWO4XLOZNODXBJS33KQSPXYIS2AUIWQC3GA7NRQB
  ```
  - Expected Output: `"50000000"`

#### 6.4. Transferring Tokens on Behalf of Another Account (`transfer_from`)
- `artemis_spender` transfers 2 MFT (20,000,000 base units) from `alice`'s account to User-1 (`GDSD3EWDONOYUJI2G6H3UFD6CTO2JCBAVYKCQF6U6DQWVGQB7F63AHK5`). This action is signed by `artemis_spender`.
  ```bash
  stellar contract invoke \
    --id CAO7HFVLQ3KWIO5XW6MWRU3LJRDBPEEHYAPTIXSUC6XF67GPBMD2WI7P \
    --source artemis_spender \
    --network testnet -- \
    transfer_from \
    --spender GDEBNAUJB6UQLB7EHWO4XLOZNODXBJS33KQSPXYIS2AUIWQC3GA7NRQB \
    --from GCCE3U7AYGHR33DGHKZ6KCCUCAUOYVOPTHPLN7UI2U2FNM53H6ZC2YIB \
    --to GDSD3EWDONOYUJI2G6H3UFD6CTO2JCBAVYKCQF6U6DQWVGQB7F63AHK5 \
    --amount 20000000
  ```
  - Transaction: [https://stellar.expert/explorer/testnet/tx/7e30541e8f1edd0c8acf168776faf89575bc8a89f08749f405d4c781f01605a9](https://stellar.expert/explorer/testnet/tx/7e30541e8f1edd0c8acf168776faf89575bc8a89f08749f405d4c781f01605a9)

#### 6.5. Post-`transfer_from` Verifications
- **`alice`'s Balance:** Expected `"280000000"` (28 MFT)
- **User-1's (`GDSD3EWDONOYUJI2G6H3UFD6CTO2JCBAVYKCQF6U6DQWVGQB7F63AHK5`) Balance:** Expected `"1220000000"` (122 MFT)
- **Remaining Allowance (`alice` to `artemis_spender`):** Expected `"30000000"` (3 MFT)

### 7. Token Burning

Burning tokens permanently removes them from circulation.

#### 7.1. Burning Tokens from Own Account (`burn`)
- `alice` burns 10 MFT (100,000,000 base units) from her own balance (previously 28 MFT).
  ```bash
  stellar contract invoke \
    --id CAO7HFVLQ3KWIO5XW6MWRU3LJRDBPEEHYAPTIXSUC6XF67GPBMD2WI7P \
    --source alice \
    --network testnet -- \
    burn \
    --from GCCE3U7AYGHR33DGHKZ6KCCUCAUOYVOPTHPLN7UI2U2FNM53H6ZC2YIB \
    --amount 100000000
  ```
  - Transaction: [https://stellar.expert/explorer/testnet/tx/bb3fac3be735a7636c6f9d755b449c53ecd69b7f6d85a66fddb2a85a62e1c427](https://stellar.expert/explorer/testnet/tx/bb3fac3be735a7636c6f9d755b449c53ecd69b7f6d85a66fddb2a85a62e1c427)
  - Verify `alice`'s Balance: Expected `"180000000"` (18 MFT)

#### 7.2. Burning Tokens from Another Account with Allowance (`burn_from`)
- `artemis_spender` burns 1 MFT (10,000,000 base units) from `alice`'s account, using the existing allowance.
  - Previous `allowance` (`alice` to `artemis_spender`): 3 MFT.
  - Previous `alice` balance: 18 MFT.
  ```bash
  stellar contract invoke \
    --id CAO7HFVLQ3KWIO5XW6MWRU3LJRDBPEEHYAPTIXSUC6XF67GPBMD2WI7P \
    --source artemis_spender \
    --network testnet -- \
    burn_from \
    --spender GDEBNAUJB6UQLB7EHWO4XLOZNODXBJS33KQSPXYIS2AUIWQC3GA7NRQB \
    --from GCCE3U7AYGHR33DGHKZ6KCCUCAUOYVOPTHPLN7UI2U2FNM53H6ZC2YIB \
    --amount 10000000
  ```
  - Transaction: [https://stellar.expert/explorer/testnet/tx/a124dcb693f8ca9892e1f22bb0ba0e9aac827c3d97f2e783c85ccabd7ae36085](https://stellar.expert/explorer/testnet/tx/a124dcb693f8ca9892e1f22bb0ba0e9aac827c3d97f2e783c85ccabd7ae36085)
  - Post-`burn_from` Verifications:
    - `alice`'s Balance: Expected `"170000000"` (17 MFT)
    - Remaining Allowance (`alice` to `artemis_spender`): Expected `"20000000"` (2 MFT)

### 8. Admin Management

#### 8.1. Transferring Admin Rights (`set_admin`)
- Current admin `alice` transfers admin rights to `artemis_spender`.
  ```bash
  stellar contract invoke \
    --id CAO7HFVLQ3KWIO5XW6MWRU3LJRDBPEEHYAPTIXSUC6XF67GPBMD2WI7P \
    --source alice \
    --network testnet -- \
    set_admin \
    --new_admin GDEBNAUJB6UQLB7EHWO4XLOZNODXBJS33KQSPXYIS2AUIWQC3GA7NRQB
  ```
  - Transaction: [https://stellar.expert/explorer/testnet/tx/52d125ead356073f79efa6715de5a8851955b5f02bf95596826bef726dab4b3a](https://stellar.expert/explorer/testnet/tx/52d125ead356073f79efa6715de5a8851955b5f02bf95596826bef726dab4b3a)

#### 8.2. Verifying Admin Change
- **New Admin (`artemis_spender`) Mints Tokens:**
  ```bash
  stellar contract invoke \
    --id CAO7HFVLQ3KWIO5XW6MWRU3LJRDBPEEHYAPTIXSUC6XF67GPBMD2WI7P \
    --source artemis_spender \
    --network testnet -- \
    mint \
    --to GDEBNAUJB6UQLB7EHWO4XLOZNODXBJS33KQSPXYIS2AUIWQC3GA7NRQB \
    --amount 10000000 
  ```
  - Result: **SUCCESSFUL**. Transaction: [https://stellar.expert/explorer/testnet/tx/348e7307b0e2ae42c602b0ac37b988d2e33b90661c0d3e794c510e30f6c74e3c](https://stellar.expert/explorer/testnet/tx/348e7307b0e2ae42c602b0ac37b988d2e33b90661c0d3e794c510e30f6c74e3c)
- **Former Admin (`alice`) Attempts to Mint Tokens:**
  ```bash
  stellar contract invoke \
    --id CAO7HFVLQ3KWIO5XW6MWRU3LJRDBPEEHYAPTIXSUC6XF67GPBMD2WI7P \
    --source alice \
    --network testnet -- \
    mint \
    --to GCCE3U7AYGHR33DGHKZ6KCCUCAUOYVOPTHPLN7UI2U2FNM53H6ZC2YIB \
    --amount 10000000
  ```
  - Result: **FAILED** (Error indicates lack of authorization).

### 9. Account Freezing and Unfreezing Management

The contract admin can freeze accounts, restricting certain operations.

#### 9.1. Freezing an Account (`freeze_account`)
- Admin (`artemis_spender`) freezes `alice`'s account for 1000 ledgers with a reason and compliance tag.
  ```bash
  stellar contract invoke \
    --id CAO7HFVLQ3KWIO5XW6MWRU3LJRDBPEEHYAPTIXSUC6XF67GPBMD2WI7P \
    --source artemis_spender \
    --network testnet -- \
    freeze_account \
    --account_to_freeze GCCE3U7AYGHR33DGHKZ6KCCUCAUOYVOPTHPLN7UI2U2FNM53H6ZC2YIB \
    --duration_ledgers 1000 \
    --reason "Inceleme Amacli" \
    --compliance_tag AmlFlagged
  ```
  - Transaction: [https://stellar.expert/explorer/testnet/tx/bfb90743dc1a7ae086630ea86374ece9403d760bfc266a7d278f41b555ac1f54](https://stellar.expert/explorer/testnet/tx/bfb90743dc1a7ae086630ea86374ece9403d760bfc266a7d278f41b555ac1f54)

#### 9.2. Verifying Freeze Status and Details
- **Check if Frozen (`is_frozen` for `alice`):** Expected output: `true`
- **Get Freeze Details (`get_account_freeze_details` for `alice`):** Expected: `{"compliance_tag":"AmlFlagged","expiration_ledger":1412694,"reason":"Inceleme Amacli"}`
- **Get Compliance Tag (`get_compliance_tag_for_account` for `alice`):** Expected: `AmlFlagged`

#### 9.3. Testing Operation Restriction on Frozen Account
- Frozen account `alice` attempts to transfer 1 MFT.
  ```bash
  stellar contract invoke \
    --id CAO7HFVLQ3KWIO5XW6MWRU3LJRDBPEEHYAPTIXSUC6XF67GPBMD2WI7P \
    --source alice \
    --network testnet -- \
    transfer \
    --from GCCE3U7AYGHR33DGHKZ6KCCUCAUOYVOPTHPLN7UI2U2FNM53H6ZC2YIB \
    --to GDEBNAUJB6UQLB7EHWO4XLOZNODXBJS33KQSPXYIS2AUIWQC3GA7NRQB \
    --amount 10000000
  ```
  - Result: **FAILED** (Error: `HostError: Error(WasmVm, InvalidAction)`).

#### 9.4. Unfreezing an Account (`unfreeze_account`)
- Admin (`artemis_spender`) unfreezes `alice`'s account.
  ```bash
  stellar contract invoke \
    --id CAO7HFVLQ3KWIO5XW6MWRU3LJRDBPEEHYAPTIXSUC6XF67GPBMD2WI7P \
    --source artemis_spender \
    --network testnet -- \
    unfreeze_account \
    --account_to_unfreeze GCCE3U7AYGHR33DGHKZ6KCCUCAUOYVOPTHPLN7UI2U2FNM53H6ZC2YIB
  ```
  - Transaction: [https://stellar.expert/explorer/testnet/tx/d47c147aa03ae2bc8e92c7cfd50a383e676d244e99dc58b6cb5cf54465f2e3be](https://stellar.expert/explorer/testnet/tx/d47c147aa03ae2bc8e92c7cfd50a383e676d244e99dc58b6cb5cf54465f2e3be)

#### 9.5. Verifying Unfreeze Status and Functionality
- **Check if Frozen (`is_frozen` for `alice`):** Expected output: `false`
- **Unfrozen Account Attempts Transfer:** `alice` (now unfrozen) transfers 1 MFT to `artemis_spender`.
  - `alice`'s previous balance: 17 MFT.
  ```bash
  stellar contract invoke \
    --id CAO7HFVLQ3KWIO5XW6MWRU3LJRDBPEEHYAPTIXSUC6XF67GPBMD2WI7P \
    --source alice \
    --network testnet -- \
    transfer \
    --from GCCE3U7AYGHR33DGHKZ6KCCUCAUOYVOPTHPLN7UI2U2FNM53H6ZC2YIB \
    --to GDEBNAUJB6UQLB7EHWO4XLOZNODXBJS33KQSPXYIS2AUIWQC3GA7NRQB \
    --amount 10000000
  ```
  - Result: **SUCCESSFUL**. Transaction: [https://stellar.expert/explorer/testnet/tx/c7dea5781f6068cafaef2f4aa5600c921305ce4eeef4037eb229254d120be954](https://stellar.expert/explorer/testnet/tx/c7dea5781f6068cafaef2f4aa5600c921305ce4eeef4037eb229254d120be954)
  - `alice`'s new balance will be 16 MFT.
    ```bash
    stellar contract invoke --id CAO7HFVLQ3KWIO5XW6MWRU3LJRDBPEEHYAPTIXSUC6XF67GPBMD2WI7P --network testnet -- balance --id GCCE3U7AYGHR33DGHKZ6KCCUCAUOYVOPTHPLN7UI2U2FNM53H6ZC2YIB 
    ```
    - Expected output: `"160000000"`