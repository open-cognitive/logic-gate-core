//! # Open-Cognitive: Logic Gate Core (Sistem 2)
//! 
//! Bilişsel İşletim Sisteminin "Mantık Kapısı". 
//! Bu modül, Neural Engine'den gelen verileri alıp otonom karar döngüsünü işletir.

mod state_machine;

use state_machine::{AgentCore, CognitiveState};
use open_cognitive_protocol::CognitiveSignal;

fn main() {
    println!("=== Open-Cognitive Logic Gate Core Başlatılıyor ===");
    println!("Durum Makinesi (State Machine) otonomi testi başlatıldı...\n");

    // Maksimum 10 adım düşünebilen bir ajan oluştur
    let mut agent = AgentCore::new(10);
    
    // İşletim sisteminin ana döngüsü (Event Loop)
    loop {
        println!("[DÜŞÜNCE ADIMI: {}] Mevcut Durum: {:?}", agent.step_counter, agent.current_state);
        
        // Simülasyon: Protokol sinyali hazırlama
        let mut _signal = CognitiveSignal::new();
        // İleride state'e göre _signal doldurulup Shared Memory'ye yazılacak.

        // Durumu bir adım ileri götür
        let next_state = agent.tick();

        if next_state == CognitiveState::Halt {
            println!("[DÜŞÜNCE ADIMI: {}] Mevcut Durum: {:?}", agent.step_counter, agent.current_state);
            println!("\nGörev başarıyla tamamlandı. Mantık kapısı kapanıyor.");
            break;
        }

        // Simülasyonun rahat okunması için işletim sistemini çok kısa uyutuyoruz
        std::thread::sleep(std::time::Duration::from_millis(800));
    }
}