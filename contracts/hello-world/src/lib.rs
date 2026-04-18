#![no_std]
use soroban_sdk::{
    contract, contractimpl, contracttype, token, Address, Env, log
};

#[contract]
pub struct CS2Kasasi;

#[contracttype]
pub enum DataKey {
    Commit(Address),
    LastPlay(Address),
}

#[contractimpl]
impl CS2Kasasi {

    pub fn commit(env: Env, oyuncu: Address, commit_hash: u64) {
        oyuncu.require_auth();
        env.storage().persistent().set(&DataKey::Commit(oyuncu.clone()), &commit_hash);
        
        // Terminale log basalım
        log!(&env, "Mühür kaydedildi: {}", commit_hash);
    }

    pub fn kasa_ac(
        env: Env,
        oyuncu: Address,
        token_adresi: Address,
        secret: u64,
    ) {
        oyuncu.require_auth();
        let now = env.ledger().timestamp();

        // 1. Cooldown Kontrolü
        if let Some(last) = env.storage().persistent().get::<_, u64>(&DataKey::LastPlay(oyuncu.clone())) {
            if now - last < 5 {
                log!(&env, "Hata: Cooldown aktif!");
                panic!("Cok hizli oynuyorsun!");
            }
        }

        // 2. Commit Verisi Var mı?
        let stored_commit = match env.storage().persistent().get::<_, u64>(&DataKey::Commit(oyuncu.clone())) {
            Some(val) => val,
            None => {
                log!(&env, "Hata: Hafizada commit bulunamadi!");
                panic!("Once commit yapmalisin!");
            }
        };

        // 3. Matematik Kontrolü
        let calculated = secret ^ 0xABCDEF; 
        log!(&env, "Hafizadaki: {}, Hesaplanan: {}", stored_commit, calculated);

        if stored_commit != calculated {
            log!(&env, "Hata: Muhurler uyusmuyor!");
            panic!("Muhur eslesmiyor!");
        }

        // Başarılıysa commiti sil
        env.storage().persistent().remove(&DataKey::Commit(oyuncu.clone()));

        // 4. Para İşlemleri
        let client = token::Client::new(&env, &token_adresi);
        let contract_addr = env.current_contract_address();
        let odul: i128 = 50_0000000;

        // Rastgelelik ve Çark
        let random_seed = now ^ secret;
        let sans = random_seed % 100;
        log!(&env, "Sans numaran: {}", sans);

        let balance = client.balance(&contract_addr);

        if sans > 50 && balance >= odul {
            log!(&env, "TEBRIKLER! Odul gonderiliyor.");
            client.transfer(&contract_addr, &oyuncu, &odul);
        } else {
            log!(&env, "Maalesef cikmadi.");
        }

        env.storage().persistent().set(&DataKey::LastPlay(oyuncu), &now);
    }
}