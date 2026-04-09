# 🗳️ Voting DApp (Soroban Smart Contract)

## 📌 Project Description

The Voting DApp is a decentralized voting application built on the Stellar blockchain using Soroban smart contracts. It allows users to create proposals and securely vote on them in a transparent and tamper-proof way.

This project demonstrates how decentralized governance and voting mechanisms can be implemented using Soroban smart contracts.

---

## 🚀 What It Does

The smart contract enables:

- Creating voting proposals
- Casting votes securely
- Preventing double voting
- Retrieving proposal results
- Tracking total number of proposals

All voting logic is executed on-chain, ensuring transparency, immutability, and security.

---

## ✨ Features

### 🗳️ Create Proposal
Anyone can create a new proposal with a unique ID.

### 🔒 Secure Voting
- Users must authenticate before voting.
- Each address can vote only once.
- Votes are permanently recorded on-chain.

### 📊 View Results
Anyone can:
- Fetch proposal details
- View total vote count per proposal
- Get total number of proposals

### ⚡ Built with Soroban
- Written in Rust
- Uses Soroban SDK
- Deployed on Stellar network

---

## 🛠 Smart Contract Functions

| Function | Description |
|----------|------------|
| `initialize()` | Initializes proposal counter |
| `create_proposal(name)` | Creates a new proposal |
| `vote(voter, proposal_id)` | Cast a vote |
| `get_proposal(proposal_id)` | Get proposal details |
| `get_proposal_count()` | Get total proposals |

---

## 🌍 Deployed Smart Contract Link

Voting DApp Contract on Stellar Explorer:

[https://stellar.expert/explorer/public/contract/CB7AP63HZAU2WI4AW76UQYWD56ON4WB6W4AJ6DLJ2SJ2OLT7J35XNMIT
](https://stellar.expert/explorer/testnet/contract/CB7AP63HZAU2WI4AW76UQYWD56ON4WB6W4AJ6DLJ2SJ2OLT7J35XNMIT)

<img width="1919" height="855" alt="image" src="https://github.com/user-attachments/assets/f013d961-b84e-47cc-aaf9-9a79f263fe3a" />


You can inspect storage, transactions, and contract state directly from the explorer.

---

## 🔧 How to Deploy

1. Install Stellar CLI
2. Build contract:
   ```bash
   cargo build --target wasm32-unknown-unknown --release
