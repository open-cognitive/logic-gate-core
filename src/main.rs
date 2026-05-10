//! # Open-Cognitive: Logic Gate Core - Synchronized Master

mod state_machine;

use state_machine::{AgentCore, CognitiveState};
use open_cognitive_protocol::{CognitiveSignal, CMD_FORWARD_PASS, CMD_EXECUTE_TOOL, CMD_IDLE};
use open_cognitive_protocol::ipc::MemoryBus;

fn main() -> std::io::Result<()> {
    println!("=== Open-Cognitive Logic Gate Core (SYNC MASTER) Başlatılıyor ===");
    let mut bus = MemoryBus::new("/tmp/cog.bus")?;
    let mut agent = AgentCore::new(10);
    
    // Temiz bir başlangıç için belleği sıfırla
    bus.write_signal(&CognitiveSignal::new());

    loop {
        println!("\n[DÜŞÜNCE ADIMI: {}] Durum: {:?}", agent.step_counter, agent.current_state);
        let mut signal = CognitiveSignal::new();
        let mut wait_for_workers = false;

        match agent.current_state {
            CognitiveState::Project => {
                println!("[MASTER] Nöral Motor'a 'Forward Pass' emri gönderildi.");
                signal.command_type = CMD_FORWARD_PASS;
                wait_for_workers = true;
            },
            CognitiveState::Act => {
                println!("[MASTER] Sandbox'a 'Execute Tool' emri gönderildi.");
                signal.command_type = CMD_EXECUTE_TOOL;
                wait_for_workers = true;
            },
            _ => {
                signal.command_type = CMD_IDLE;
            }
        }

        // Komutu RAM'e yaz
        bus.write_signal(&signal);

        // İşçilerin (Workers) işi bitirip komutu CMD_IDLE yapmasını bekle (Blocking IPC)
        if wait_for_workers {
            println!("[MASTER] İşçilerin işlemi tamamlaması bekleniyor...");
            loop {
                let check_signal = bus.read_signal();
                if check_signal.command_type == CMD_IDLE {
                    println!("[MASTER] İşçiler onay (ACK) verdi. Döngüye devam ediliyor.");
                    break;
                }
                std::thread::sleep(std::time::Duration::from_millis(50));
            }
        }

        let next_state = agent.tick();
        if next_state == CognitiveState::Halt { break; }
        std::thread::sleep(std::time::Duration::from_millis(800));
    }

    println!("\n[SİSTEM] Görev başarıyla tamamlandı.");
    Ok(())
}