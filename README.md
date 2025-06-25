# FreezeGuardToken
<img src="image.jpg" alt="FreezeGuardToken Logo" width="300"/>

## Overview
**FreezeGuardToken** is a smart contract built on the **Soroban Blockchain**, enhancing standard token behavior with advanced account freezing capabilities. It enables administrators to freeze accounts, either temporarily or permanently, assign reasons for the freeze, and tag them with compliance statuses such as `KycPending`, `AmlFlagged`, or `Sanctioned`. Accounts that are frozen are restricted from transferring, burning, or spending tokens.

Inspired by real-world financial compliance scenarios (like AML/KYC), this project provides a robust tool for secure and compliant asset management within decentralized systems.

---

## Vision Statement
FreezeGuardToken aims to establish a secure and compliant token standard that seamlessly integrates into regulated financial environments. By incorporating account freezing mechanics and compliance tagging, it offers institutions the necessary flexibility and control to manage risks and uphold transparency. This project envisions a future where blockchain technology is trusted and widely adopted, not only for its decentralized nature but also for its responsible and principled design.

---

## Software Development Plan

### 1. Core Token Functionality & Admin Control
- **Functions:** `initialize`, `mint`, `set_admin`, `approve`, `transfer`, `transfer_from`, `burn`, `burn_from`, `balance`, `name`, `symbol`, `decimals`.
- Administrator authorization for critical operations.

### 2. Advanced Account Freezing System
- **Functions:**
  - `freeze_account(account, duration_ledgers, reason, compliance_tag)`
  - `unfreeze_account(account)`
- **`FreezeInfo` Struct:** Stores details such as freeze expiration ledger, reason, and compliance tag.
- Internal storage mapping account addresses to their respective freeze statuses.

### 3. Query Functions for Freeze Status
- `is_frozen(account) -> bool`
- `get_account_freeze_details(account) -> Option<FreezeInfo>`
- `get_compliance_tag_for_account(account) -> ComplianceStatus`

### 4. Logic Integration for Frozen Accounts
- Modify `approve`, `transfer`, `transfer_from`, `burn`, and `burn_from` functions to restrict actions for frozen accounts.

### 5. Testing & Deployment
- All core functionalities of the FreezeGuardToken contract are thoroughly tested to ensure security and reliability. The test suite covers:
  - Standard token operations (mint, transfer, burn, approve).
  - Account freezing and unfreezing mechanics (including duration, reason, and compliance tagging).
  - Compliance status checks and freeze status queries.
  - Logic for timed automatic unfreezing.

---

## About Me
I'm **Oben**, a final-year Computer Engineering student at **Mersin University**. My primary interests include **Artificial Intelligence (AI)**, **Web Development**, and the **Internet of Things (IoT)**. With FreezeGuardToken, I aimed to channel my passion for blockchain technology into a practical application that addresses real-world compliance and security challenges.

---

---

## Testnet Deployment & Interaction Guide

Contract ID: **CAUC3T4B6KWTD6M6FFPYH2QDCLAO36MXYXFGFAS2TRHKP2BE32VCJDIV**
- Stellar Expert link: [https://stellar.expert/explorer/testnet/contract/CAUC3T4B6KWTD6M6FFPYH2QDCLAO36MXYXFGFAS2TRHKP2BE32VCJDIV](https://stellar.expert/explorer/testnet/contract/CAUC3T4B6KWTD6M6FFPYH2QDCLAO36MXYXFGFAS2TRHKP2BE32VCJDIV)

- **Successful `initialize` Transaction:** [https://stellar.expert/explorer/testnet/tx/dede6af651e2218a231f1aaaa28a9cb12a00c5447a3d9ac28396ba36d2747a51](https://stellar.expert/explorer/testnet/tx/dede6af651e2218a231f1aaaa28a9cb12a00c5447a3d9ac28396ba36d2747a51)

- **Successful `mint` Transaction:** [https://stellar.expert/explorer/testnet/tx/f2416b5a9ba1e91f9642b929d8aa8856e3611a450a0fcdeecd034a7df0f17d49](https://stellar.expert/explorer/testnet/tx/f2416b5a9ba1e91f9642b929d8aa8856e3611a450a0fcdeecd034a7df0f17d49)

- **Successful `mint` Transaction:** [https://stellar.expert/explorer/testnet/tx/c2d118f425c67532945a9506eab60ba112f92f2e5cfda442aae4bdb802c72d97](https://stellar.expert/explorer/testnet/tx/c2d118f425c67532945a9506eab60ba112f92f2e5cfda442aae4bdb802c72d97)

- **Successful `transfer` Transaction:** [https://stellar.expert/explorer/testnet/tx/0d1bf533188a915221fcfee73ef1b27a47062d9f9bb3bd41a8104f8bbc1f9fa4](https://stellar.expert/explorer/testnet/tx/0d1bf533188a915221fcfee73ef1b27a47062d9f9bb3bd41a8104f8bbc1f9fa4)

- **Successful `approve` Transaction:** [https://stellar.expert/explorer/testnet/tx/fd7dd88a5e8bac029b16ffe818645b927b978853568ea9f2111476e2d2bb8873](https://stellar.expert/explorer/testnet/tx/fd7dd88a5e8bac029b16ffe818645b927b978853568ea9f2111476e2d2bb8873)

- **Successful `transfer_from` Transaction:** [https://stellar.expert/explorer/testnet/tx/b2a185107515738e5d969c3f8193c92314a2171cf9a4cd60d314fd2d000227de](https://stellar.expert/explorer/testnet/tx/b2a185107515738e5d969c3f8193c92314a2171cf9a4cd60d314fd2d000227de)

- **Successful `burn` Transaction:** [https://stellar.expert/explorer/testnet/tx/0562c9fb414587131f75afc2f220b4ced1aecfb2d116e936021c949769f457e8](https://stellar.expert/explorer/testnet/tx/0562c9fb414587131f75afc2f220b4ced1aecfb2d116e936021c949769f457e8)

- **Successful `burn_from` Transaction:** [https://stellar.expert/explorer/testnet/tx/8704e1fd136a968fe117947c9c33cb4ed84bc90a1a0e914f42a059b07fafbb90](https://stellar.expert/explorer/testnet/tx/8704e1fd136a968fe117947c9c33cb4ed84bc90a1a0e914f42a059b07fafbb90)

- **Successful `set_admin` Transaction:** [https://stellar.expert/explorer/testnet/tx/0262753c434ed6a806af2568da9f8f92a10d141e55ef4d8c57e8363d76d52577](https://stellar.expert/explorer/testnet/tx/0262753c434ed6a806af2568da9f8f92a10d141e55ef4d8c57e8363d76d52577)

- **Successful `freeze_account` Transaction:** [https://stellar.expert/explorer/testnet/tx/ad0c7be8b0f96c94dba20c306a8e4e67741b77a7bd404835a9c027b4c5f774c0](https://stellar.expert/explorer/testnet/tx/ad0c7be8b0f96c94dba20c306a8e4e67741b77a7bd404835a9c027b4c5f774c0)

- **Successful `unfreeze_account` Transaction:** [https://stellar.expert/explorer/testnet/tx/20c9212b626fe1c56890e7030335d7f5f697daad534169f90efa980177e5e8e9](https://stellar.expert/explorer/testnet/tx/20c9212b626fe1c56890e7030335d7f5f697daad534169f90efa980177e5e8e9)

For detailed instructions on how to compile, deploy, and interact with the FreezeGuardToken contract on the Stellar Testnet, please refer to our comprehensive guide:

➡️ [**Testnet Interaction Guide (INTERACTION_GUIDE.md)**](./INTERACTION_GUIDE.md)

---