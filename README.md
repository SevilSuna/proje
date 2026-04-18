🎮 CS2Kasasi Smart Contract
🇹🇷 Türkçe
📌 Proje Hakkında

CS2Kasasi, Soroban üzerinde yazılmış bir akıllı kontrattır. Commit-reveal mekanizması kullanarak adil bir “kasa açma” (loot box) sistemi sağlar.

Oyuncular önce bir “commit” (mühür) gönderir, ardından secret değeri ile kasayı açarak ödül kazanma şansı elde eder.

⚙️ Nasıl Çalışır?
1. Commit (Mühürleme)

Oyuncu, bir commit_hash gönderir:

commit(env, oyuncu, commit_hash)
commit_hash = secret ^ 0xABCDEF
Bu işlem oyuncunun gelecekteki hamlesini gizler.
2. Kasa Açma

Oyuncu kasayı açmak için:

kasa_ac(env, oyuncu, token_adresi, secret)

Kontrat şu kontrolleri yapar:

✅ Cooldown kontrolü (5 saniye)
✅ Commit doğrulaması
✅ Secret doğrulaması

🎲 Rastgelelik (Randomness)

Rastgelelik şu şekilde hesaplanır:

random_seed = now ^ secret
sans = random_seed % 100
sans > 50 → Oyuncu kazanır 🎉
sans <= 50 → Kaybeder 😢
💰 Ödül Sistemi
Kazanan oyuncuya 50 token gönderilir
Token transferi için Soroban Token Interface kullanılır
🔐 Güvenlik Özellikleri
✔️ Commit-Reveal mekanizması
✔️ Cooldown koruması
✔️ Yetkilendirme (require_auth)
✔️ Storage temizleme (commit silinir)

🇬🇧 English
📌 About the Project

CS2Kasasi is a smart contract built on Soroban. It implements a fair loot-box system using a commit-reveal scheme.

Players first submit a commitment, then reveal a secret to open the case and try their luck.

⚙️ How It Works
1. Commit Phase

The player submits a hash:

commit(env, oyuncu, commit_hash)
commit_hash = secret ^ 0xABCDEF
This hides the player's move until reveal.
2. Open Case
kasa_ac(env, oyuncu, token_adresi, secret)

The contract checks:

✅ Cooldown (5 seconds)
✅ Stored commit exists
✅ Secret matches commit

🎲 Randomness
random_seed = now ^ secret
chance = random_seed % 100
chance > 50 → Win 🎉
chance <= 50 → Lose 😢
💰 Reward System
Winners receive 50 tokens
Uses Soroban Token Interface for transfers
🔐 Security Features
✔️ Commit-Reveal scheme
✔️ Cooldown protection
✔️ Authorization (require_auth)
✔️ Storage cleanup


test 

Durum: Başarılı (Success)
<img width="1476" height="619" alt="Ekran görüntüsü 2026-04-18 171246" src="https://github.com/user-attachments/assets/5ff755f7-2768-4e5d-b5c9-3b6f07be1047" />
📸 Test Aşaması Çıktısı:
Yukarıdaki ekran görüntüsünde, akıllı sözleşmenin üç ana fonksiyonunun (Commit -> Reveal -> Balance) Standalone ağındaki başarılı çalışma döngüsü görülmektedir. İşlem hash değerleri (Transaction Hash) üzerinden ağdaki doğrulama teyit edilebilir.
1. örnek: commit (mühürleme) işleminin hashi= 736ee086ed10d464151c4c180d56b14e7c3c6ef2303cfadc9e7199ebbdb67657
2. Örnek: Kasa Açma İşleminin Hash'i(reveal)= b9f8158ee684699204ece902580addfde2918692322a8789f72e5740129559a1

docker:
<img width="1603" height="897" alt="image" src="https://github.com/user-attachments/assets/be125f85-2ecf-44d1-b391-e4ded84d320e" />





<img width="736" height="478" alt="download (8)" src="https://github.com/user-attachments/assets/c1464017-a5a6-4ecb-8a2e-ee57218ad3fd" />




