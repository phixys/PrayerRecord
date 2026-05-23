# 🕋 Mutaba'ah On-Chain: Soroban Prayer Tracker

A decentralized smart contract (dApp) built on the **Stellar (Soroban)** network to record daily prayer history (Mutaba'ah Yaumiyah) immutably (permanent and tamper-proof).

This application demonstrates basic CRUD (Create, Read, Delete) operations on the blockchain using the Rust programming language and the Soroban SDK.

## ✨ Key Features

* **Record Prayer (`record_prayer`):** Saves prayer data (date, prayer name, and execution status such as congregational/alone) to the blockchain network.
* **View History (`get_prayers`):** Retrieves and displays the entire list of prayer records logged by the user.
* **Delete Record (`delete_prayer`):** Removes a specific record based on its ID (useful for correcting data input errors).

## 🛠️ Tech Stack

* **Programming Language:** Rust (`#![no_std]`)
* **Smart Contract Platform:** Soroban (Stellar Network)
* **Environment:** Stellar Testnet / Futurenet

## 📋 System Prerequisites

Before running this project locally, ensure your system has the following installed:

1. [Rust toolchain](https://www.rust-lang.org/tools/install) (including `cargo`)
2. WebAssembly target: `rustup target add wasm32-unknown-unknown`
3. [Stellar CLI](https://developers.stellar.org/docs/build/smart-contracts/getting-started/setup)

## 🚀 Installation & Deployment Guide

### 1. Build Smart Contract
Compile the Rust code into a WebAssembly (.wasm) file ready to be uploaded to the network:
```bash
cargo build --target wasm32-unknown-unknown --release
```
## Contract ID
CD53HCX5ON2EQXLYRSXB6BGVYCMGS7CHL5MGYHHUVEWSRVOPJ25QSLYE

