//! # Open-Cognitive: Logic Gate Core - CLI Master

mod state_machine;

use state_machine::{AgentCore, CognitiveState};
use open_cognitive_protocol::{CognitiveSignal, CMD_FORWARD_PASS, CMD_EXECUTE_TOOL, CMD_IDLE};
use open_cognitive_protocol::ipc::MemoryBus;
use std::env;

fn main() -> std::io::Result<()> {
    // 1. Kullanıcıdan görevi al
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Kullanım: cargo run -- <çalıştırılacak_sayı>");
        println!("Örnek: cargo run -- 7");
        return Ok(());
    }

    // Görev girdisini u8 (byte) olarak al
    let user_input: u8 = args[1].parse().unwrap_or(0);

    println!("=== Open-Cognitive Logic Gate Core Başlatılıyor ===");
    println!("[GÖREV ALINDI] İşlenecek Veri: {}", user_input);

    let mut bus = MemoryBus::new("/tmp/cog.bus")?;
    let mut agent = AgentCore::new(10);
    bus.write_signal(&CognitiveSignal::new());

    loop {
        println!("\n[DÜŞÜNCE ADIMI: {}] Durum: {:?}", agent.step_counter, agent.current_state);
        let mut signal = CognitiveSignal::new();
        
        // Veriyi taşıma çantasına (payload) koy (Sadece ilk bayta yazıyoruz)
        signal.payload[0] = user_input;
        
        let mut wait_for_workers = false;

        match agent.current_state {
            CognitiveState::Project => {
                println!("[MASTER] Nöral Motor'a 'Forward Pass' emri gönderildi.");
                signal.command_type = CMD_FORWARD_PASS;
                wait_for_workers = true;
            },
            CognitiveState::Act => {
                println!("[MASTER] Sandbox'a veri ({}) gönderiliyor...", user_input);
                signal.command_type = CMD_EXECUTE_TOOL;
                wait_for_workers = true;
            },
            CognitiveState::Reflect => {
                // YENİ: Sandbox'ın bellekte bıraktığı sonucu oku
                let current_signal = bus.read_signal();
                let tool_result = current_signal.payload[0];
                println!("[MASTER] Öz-Denetim (Reflect): Sandbox'ın ürettiği sonuç = {}", tool_result);
                signal.command_type = CMD_IDLE;
            },
            _ => { signal.command_type = CMD_IDLE; }
        }

        bus.write_signal(&signal);

        if wait_for_workers {
            loop {
                let check_signal = bus.read_signal();
                if check_signal.command_type == CMD_IDLE { break; }
                std::thread::sleep(std::time::Duration::from_millis(50));
            }
        }

        let next_state = agent.tick();
        if next_state == CognitiveState::Halt { break; }
        std::thread::sleep(std::time::Duration::from_millis(500));
    }

    println!("\n[SİSTEM] Görev başarıyla tamamlandı.");
    Ok(())
}