# FreezeGuardToken
<img src="image.jpg" alt="Logo" width="300"/>

## Overview  
**FreezeGuardToken** is a smart contract built on the **Soroban Blockchain**, enhancing standard token behavior with advanced account freezing features. It enables administrators to freeze accounts temporarily or permanently, assign reasons, and tag them with compliance statuses like `KycPending`, `AmlFlagged`, or `Sanctioned`. Frozen accounts are unable to transfer, burn, or spend tokens.

Inspired by real-world financial compliance scenarios (such as AML/KYC), this project provides a powerful tool for secure and compliant asset management in decentralized systems.

---

## Vision Statement  
FreezeGuardToken aims to create a secure, compliant token standard that fits seamlessly into regulated environments. By integrating freeze mechanics and compliance tagging, it offers institutions the flexibility and control needed to manage risk and uphold transparency. This project envisions a future where blockchain is trusted and adopted not just for its decentralization, but also for its responsible design.

---

## Software Development Plan

### 1. Token Core & Admin Control  
- Functions: `initialize`, `mint`, `set_admin`, `approve`, `transfer`, `transfer_from`, `burn`, `burn_from`, `balance`, `name`, `symbol`, `decimals`  
- Admin authorization for critical operations

### 2. Advanced Freeze System  
- `freeze_account(account, duration, reason, compliance_tag)`  
- `unfreeze_account(account)`  
- `FreezeInfo` struct with details on freeze expiration, reason, and tag  
- Internal storage maps account addresses to their freeze status

### 3. Query Functions  
- `is_frozen(account) -> bool`  
- `get_account_freeze_details(account) -> Option<FreezeInfo>`  
- `get_compliance_tag_for_account(account) -> Option<ComplianceStatus>`

### 4. Logic Integration  
- Modify `approve`, `transfer`, `transfer_from`, `burn`, and `burn_from` to block frozen accounts

### 5. Testing & Deployment  
- All core functionalities of the FreezeGuardToken contract are thoroughly tested to ensure security and reliability. The test suite includes:
- Token operations (mint, transfer, burn, approve)
- Account freezing and unfreezing (with duration, reason, and compliance tag)
- Compliance checks and freeze status queries
- Timed automatic unfreeze logic

---

## Personal Story  
I'm **Oben**, a final-year Computer Engineering student at **Mersin University**. My main interests lie in **AI**, **web development**, and **IoT**. With FreezeGuardToken, I aimed to turn my passion for blockchain into a real-world application that solves compliance and security challenges.

---
