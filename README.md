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

For detailed instructions on how to compile, deploy, and interact with the FreezeGuardToken contract on the Stellar Testnet, please refer to our comprehensive guide:

➡️ [**Testnet Interaction Guide (INTERACTION_GUIDE.md)**](./INTERACTION_GUIDE.md)

---