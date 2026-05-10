//! # Open-Cognitive: Logic Gate Core (Sistem 2) - Bus Master
//! 
//! Bu çekirdek, paylaşımlı bellek üzerinden emirleri yayınlar.

mod state_machine;

use state_machine::{AgentCore, CognitiveState};
use open_cognitive_protocol::{CognitiveSignal, CMD_FORWARD_PASS, CMD_IDLE};
use open_cognitive_protocol::ipc::MemoryBus;

fn main() -> std::io::Result<()> {
    println!("=== Open-Cognitive Logic Gate Core (BUS MASTER) Başlatılıyor ===");
    
    // 1. Paylaşımlı Bellek Otobüsünü oluştur (Dosya yolu: /tmp/cog.bus)
    let mut bus = MemoryBus::new("/tmp/cog.bus")?;
    println!("[SİSTEM] Bellek Otobüsü bağlandı: /tmp/cog.bus");

    let mut agent = AgentCore::new(10);
    
    loop {
        println!("\n[DÜŞÜNCE ADIMI: {}] Durum: {:?}", agent.step_counter, agent.current_state);
        
        // 2. Duruma göre sinyal hazırla
        let mut signal = CognitiveSignal::new();
        signal.cognitive_state = agent.current_state as u8;

        if agent.current_state == CognitiveState::Project {
            println!("[BUS] Nöral Motor'a 'Forward Pass' emri gönderiliyor...");
            signal.command_type = CMD_FORWARD_PASS;
            signal.context_length = 512; // Örnek bağlam uzunluğu
        } else {
            signal.command_type = CMD_IDLE;
        }

        // 3. Sinyali Belleğe (RAM) Yaz
        bus.write_signal(&signal);

        let next_state = agent.tick();
        if next_state == CognitiveState::Halt { break; }

        // Sistemin diğer modüllerinin tepki vermesi için kısa bekleme
        std::thread::sleep(std::time::Duration::from_millis(1500));
    }

    println!("\n[SİSTEM] Görev bitti. Bellek temizleniyor.");
    Ok(())
}