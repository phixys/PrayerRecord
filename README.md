# 🕋 Mutaba'ah On-Chain: Soroban Prayer Tracker

Sebuah *smart contract* terdesentralisasi (dApp) yang dibangun di atas jaringan **Stellar (Soroban)** untuk mencatat riwayat sholat harian (Mutaba'ah Yaumiyah) secara *immutable* (permanen dan tidak bisa dimanipulasi).

Aplikasi ini mendemonstrasikan operasi dasar CRUD (Create, Read, Delete) pada *blockchain* menggunakan bahasa pemrograman Rust dan Soroban SDK.

## ✨ Fitur Utama

* **Catat Sholat (`record_prayer`):** Menyimpan data sholat (tanggal, waktu sholat, dan status pelaksanaan seperti berjamaah/sendiri) ke dalam jaringan blockchain.
* **Lihat Riwayat (`get_prayers`):** Mengambil dan menampilkan seluruh daftar riwayat sholat yang pernah dicatat oleh pengguna.
* **Hapus Catatan (`delete_prayer`):** Menghapus catatan spesifik berdasarkan ID (berguna untuk mengoreksi kesalahan *input* data).

## 🛠️ Teknologi yang Digunakan

* **Bahasa Pemrograman:** Rust (`#![no_std]`)
* **Smart Contract Platform:** Soroban (Stellar Network)
* **Environment:** Stellar Testnet / Futurenet

## 📋 Prasyarat Sistem

Sebelum menjalankan *project* ini secara lokal, pastikan sistem kamu sudah menginstal:

1. [Rust toolchain](https://www.rust-lang.org/tools/install) (termasuk `cargo`)
2. Target WebAssembly: `rustup target add wasm32-unknown-unknown`
3. [Stellar CLI](https://developers.stellar.org/docs/build/smart-contracts/getting-started/setup)

## 🚀 Panduan Instalasi & Deployment

### 1. Build Smart Contract
Kompilasi kode Rust menjadi *file* WebAssembly (.wasm) yang siap diunggah ke jaringan:
```bash
cargo build --target wasm32-unknown-unknown --release
