//! # Open-Cognitive: Logic Gate Core - TCP Daemon
//! 
//! Bilişsel İşletim Sisteminin Ana Servisi.
//! Dış dünyadan (SDK'lar) gelen TCP bağlantılarını dinler, görevleri
//! donanım seviyesindeki IPC (Memory Bus) otobüsüne aktarır.

mod state_machine;

use state_machine::{AgentCore, CognitiveState};
use open_cognitive_protocol::{CognitiveSignal, CMD_FORWARD_PASS, CMD_EXECUTE_TOOL, CMD_IDLE};
use open_cognitive_protocol::ipc::MemoryBus;
use serde::{Deserialize, Serialize};
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

/// Dış dünyadan (Örn: TypeScript SDK) gelen JSON formatı
#[derive(Deserialize)]
struct TaskRequest {
    task_input: u8,
}

/// Dış dünyaya gönderilecek JSON cevabı
#[derive(Serialize)]
struct TaskResponse {
    result: u8,
    status: String,
}

fn handle_client(mut stream: TcpStream, bus: &mut MemoryBus) {
    let mut buffer = [0; 512];
    
    // SDK'dan veriyi oku
    match stream.read(&mut buffer) {
        Ok(size) if size > 0 => {
            let request_str = String::from_utf8_lossy(&buffer[..size]);
            if let Ok(request) = serde_json::from_str::<TaskRequest>(&request_str) {
                println!("\n[DAEMON] Yeni görev alındı! İşlenecek Veri: {}", request.task_input);
                
                // Görevi Bilişsel Döngüye Sok
                let final_result = execute_cognitive_loop(bus, request.task_input);
                
                // Sonucu JSON olarak SDK'ya geri yolla
                let response = TaskResponse {
                    result: final_result,
                    status: "Başarılı".to_string(),
                };
                let response_json = serde_json::to_string(&response).unwrap();
                stream.write_all(response_json.as_bytes()).unwrap();
            }
        }
        _ => eprintln!("[DAEMON] Hatalı bağlantı denemesi."),
    }
}

/// Ajanın Düşünce (Reasoning) ve Eylem (Act) döngüsünü işletir
fn execute_cognitive_loop(bus: &mut MemoryBus, user_input: u8) -> u8 {
    let mut agent = AgentCore::new(10);
    let mut final_result = 0;

    // Temiz başlangıç
    bus.write_signal(&CognitiveSignal::new());

    loop {
        println!("[DÜŞÜNCE ADIMI: {}] Durum: {:?}", agent.step_counter, agent.current_state);
        let mut signal = bus.read_signal();
        let mut wait_for_workers = false;

        match agent.current_state {
            CognitiveState::Project => {
                println!("[MASTER] Nöral Motor'a 'Forward Pass' emri gönderildi.");
                signal.payload[0] = user_input; 
                signal.command_type = CMD_FORWARD_PASS;
                wait_for_workers = true;
                bus.write_signal(&signal);
            },
            CognitiveState::Act => {
                println!("[MASTER] Sandbox'a veri gönderiliyor...");
                signal.command_type = CMD_EXECUTE_TOOL;
                wait_for_workers = true;
                bus.write_signal(&signal);
            },
            CognitiveState::Reflect => {
                final_result = signal.payload[0];
                println!("[MASTER] Öz-Denetim (Reflect): Sandbox'ın ürettiği sonuç = {}", final_result);
                signal.command_type = CMD_IDLE;
                bus.write_signal(&signal);
            },
            _ => { 
                signal.command_type = CMD_IDLE; 
                bus.write_signal(&signal);
            }
        }

        // İşçilerin işi bitirmesini bekle
        if wait_for_workers {
            loop {
                let check_signal = bus.read_signal();
                if check_signal.command_type == CMD_IDLE { break; }
                std::thread::sleep(std::time::Duration::from_millis(50));
            }
        }

        let next_state = agent.tick();
        if next_state == CognitiveState::Halt { break; }
        std::thread::sleep(std::time::Duration::from_millis(100)); // Hızlandırılmış döngü
    }
    
    final_result
}

fn main() -> std::io::Result<()> {
    println!("=== Open-Cognitive Logic Gate Core (DAEMON MODE) Başlatılıyor ===");
    let mut bus = MemoryBus::new("/tmp/cog.bus").expect("Memory Bus oluşturulamadı!");
    
    let listener = TcpListener::bind("127.0.0.1:8080")?;
    println!("[DAEMON] Çekirdek 127.0.0.1:8080 portunda bağlantıları bekliyor...");

    // Sonsuz döngü: Her gelen isteği (Örn: TypeScript'ten) dinle
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_client(stream, &mut bus);
            }
            Err(e) => eprintln!("Bağlantı hatası: {}", e),
        }
    }
    Ok(())
}