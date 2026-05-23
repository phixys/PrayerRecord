#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Env, String, Symbol, Vec};

// Struktur data yang akan menyimpan riwayat sholat
#[contracttype]
#[derive(Clone, Debug)]
pub struct PrayerRecord {
    id: u64,
    date: String,        // Contoh: "23-05-2026"
    prayer_name: String, // Contoh: "Subuh", "Dzuhur"
    status: String,      // Contoh: "Berjamaah", "Sendiri", "Terlambat"
}

// Storage key untuk data sholat
// Catatan: Symbol maksimal 9 karakter, jadi kita pakai "PRAYERS"
const PRAYER_DATA: Symbol = symbol_short!("PRAYERS");

#[contract]
pub struct PrayerTrackerContract;

#[contractimpl]
impl PrayerTrackerContract {
    // Fungsi untuk melihat semua riwayat sholat yang sudah dicatat
    pub fn get_prayers(env: Env) -> Vec<PrayerRecord> {
        // 1. Ambil data riwayat sholat dari storage
        return env.storage().instance().get(&PRAYER_DATA).unwrap_or(Vec::new(&env));
    }

    // Fungsi untuk mencatat sholat baru
    pub fn record_prayer(env: Env, date: String, prayer_name: String, status: String) -> String {
        // 1. Ambil data riwayat dari storage
        let mut records: Vec<PrayerRecord> = env.storage().instance().get(&PRAYER_DATA).unwrap_or(Vec::new(&env));
        
        // 2. Buat object riwayat sholat baru
        let new_record = PrayerRecord {
            id: env.prng().gen::<u64>(), // Menghasilkan ID unik secara acak
            date: date,
            prayer_name: prayer_name,
            status: status,
        };
        
        // 3. Tambahkan riwayat baru ke daftar lama
        records.push_back(new_record);
        
        // 4. Simpan kembali daftar ke storage
        env.storage().instance().set(&PRAYER_DATA, &records);
        
        return String::from_str(&env, "Alhamdulillah, sholat berhasil dicatat");
    }

    // Fungsi untuk menghapus catatan sholat (misal: jika ada salah ketik)
    pub fn delete_prayer(env: Env, id: u64) -> String {
        // 1. Ambil data riwayat dari storage 
        let mut records: Vec<PrayerRecord> = env.storage().instance().get(&PRAYER_DATA).unwrap_or(Vec::new(&env));

        // 2. Cari index catatan yang akan dihapus menggunakan perulangan
        for i in 0..records.len() {
            if records.get(i).unwrap().id == id {
                records.remove(i);

                // 3. Simpan update terbaru ke storage
                env.storage().instance().set(&PRAYER_DATA, &records);
                return String::from_str(&env, "Berhasil menghapus catatan sholat");
            }
        }

        return String::from_str(&env, "Catatan sholat tidak ditemukan")
    }
}