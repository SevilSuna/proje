#🚀 Soroban CS2 Case Opening System

Bu proje, Stellar ağının Soroban akıllı sözleşme platformu üzerinde geliştirilmiş, CS2 tarzı bir "Kasa Açma" mekanizmasıdır. Siber güvenlik prensipleri gözetilerek Commit-Reveal protokolü ile tasarlanmıştır. Bu sayede blok zinciri üzerindeki şeffaflıktan kaynaklanabilecek rastgelelik manipülasyonlarının önüne geçilmiştir.

#Özellikler:

Güvenli Rastgelelik: Commit-Reveal şeması ile oyuncuların sonucu önceden tahmin etmesi engellenir.

Cooldown Mekanizması: Spam ve DoS saldırılarını önlemek için 10 saniyelik işlem sınırı uygulanmıştır.

Token Entegrasyonu: Başarılı kasa açılışlarında oyunculara otomatik olarak XLM ödülü transfer edilir.

Kalıcı Veri (Persistent Storage): Oyuncu verileri ve mühürler zincir üzerinde güvenli bir şekilde saklanır.


This project is a CS2-style "Case Opening" system developed on the Soroban smart contract platform of the Stellar network. It utilizes the Commit-Reveal protocol to ensure fairness and prevent randomness manipulation often found in transparent blockchain environments.

#Features:

Secure Randomness: Uses a Commit-Reveal scheme to ensure players cannot predict or manipulate the outcome.

Cooldown Mechanism: A 10-second cooldown is enforced to prevent spamming and automated DoS attempts.

Token Integration: Automatically transfers XLM rewards to the player upon successful "red" drops.

Persistent Storage: Player states and commitments are securely stored on-chain.

🛠 Teknik Detaylar / Technical Details
Language: Rust

SDK: Soroban SDK v22.0.1

Tools: Stellar CLI, Docker (Standalone Network)

🚀 Çalıştırma / How to Run
Bash
# Projeyi derle / Build the project
stellar contract build

# Sözleşmeyi deploy et / Deploy the contract
stellar contract deploy --wasm target/wasm32v1-none/release/hello_world.wasm --source SunaCuzdan --network-passphrase "Standalone Network ; February 2017" --rpc-url http://localhost:8000/soroban/rpc
🔐 Siber Güvenlik Notu / Cybersecurity Note
Bu proje, blok zinciri tabanlı oyunlarda karşılaşılan yaygın güvenlik açıklarını (malleability, front-running) Commit-Reveal katmanı ile minimize etmeyi amaçlayan bir akademik çalışmadır.
