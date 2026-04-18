// ÖNEMLİ: Kendi Testnet ID'ni buraya yapıştır!
const CONTRACT_ID = "C...SENIN_TESTNET_IDIN"; 
const SECRET_VALUE = 12345; // Test için sabit kalsın

const btnCommit = document.getElementById('btn-commit');
const btnReveal = document.getElementById('btn-reveal');
const strip = document.getElementById('strip');
const status = document.getElementById('status');

// 1. Şeridi Rastgele Itemlarla Doldur
function fillStrip() {
    strip.innerHTML = '';
    for(let i=0; i<50; i++) {
        const item = document.createElement('div');
        item.className = 'item';
        item.innerText = 'MAVİ';
        if (i === 45) { // 45. sıradaki "kazanma" itemı olsun
            item.className = 'item rare';
            item.innerText = '🔥 50 XLM';
        }
        strip.appendChild(item);
    }
}

// 2. Commit Fonksiyonu (Sözleşmeye mühür atar)
btnCommit.onclick = async () => {
    status.innerText = "Mühürleniyor (Commit)...";
    // Burada normalde Freighter ile kontrat invoke edilir.
    // Şimdilik mantığı simüle edelim:
    setTimeout(() => {
        status.innerText = "Mühürlendi! 10 saniye bekle...";
        btnCommit.disabled = true;
        setTimeout(() => {
            btnReveal.disabled = false;
            status.innerText = "Hazır! Kasayı açabilirsin.";
        }, 10000);
    }, 2000);
};

// 3. Reveal Fonksiyonu (Animasyonu başlatır ve sonucu alır)
btnReveal.onclick = async () => {
    status.innerText = "Kasa açılıyor...";
    btnReveal.disabled = true;
    
    // Animasyonu Başlat (Rastgele bir yere kaydır)
    const offset = 45 * 150 - (window.innerWidth * 0.4); // 45. item'a denk getir
    strip.style.transform = `translateX(-${offset}px)`;

    setTimeout(() => {
        status.innerText = "TEBRİKLER! Kırmızı Item ve 50 XLM kazandın!";
        // Gerçekte burada 'kasa_ac' fonksiyonunu invoke edip 
        // Transaction Hash'i ekrana yazdıracağız.
    }, 5500);
};

fillStrip();
