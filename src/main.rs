//! # Open-Cognitive: Logic Gate Core - TCP Daemon

mod state_machine;

use state_machine::{AgentCore, CognitiveState};
use open_cognitive_protocol::{CognitiveSignal, CMD_FORWARD_PASS, CMD_EXECUTE_TOOL, CMD_IDLE};
use open_cognitive_protocol::ipc::MemoryBus;
use serde::{Deserialize, Serialize};
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

#[derive(Deserialize)]
struct TaskRequest {
    prompt: String, // String alıyor
}

#[derive(Serialize)]
struct TaskResponse {
    result: String, // String dönüyor
    status: String,
}

fn handle_client(mut stream: TcpStream, bus: &mut MemoryBus) {
    let mut buffer = [0; 1024];
    
    match stream.read(&mut buffer) {
        Ok(size) if size > 0 => {
            let request_str = String::from_utf8_lossy(&buffer[..size]);
            if let Ok(request) = serde_json::from_str::<TaskRequest>(&request_str) {
                println!("\n[DAEMON] Yeni Prompt Alındı: \"{}\"", request.prompt);
                
                let final_result = execute_cognitive_loop(bus, &request.prompt);
                
                let response = TaskResponse {
                    result: final_result,
                    status: "Başarılı".to_string(),
                };
                let response_json = serde_json::to_string(&response).unwrap();
                stream.write_all(response_json.as_bytes()).unwrap();
            }
        }
        _ => {},
    }
}

fn execute_cognitive_loop(bus: &mut MemoryBus, prompt: &str) -> String {
    let mut agent = AgentCore::new(10);
    bus.write_signal(&CognitiveSignal::new());

    loop {
        println!("[DÜŞÜNCE ADIMI: {}] Durum: {:?}", agent.step_counter, agent.current_state);
        let mut signal = bus.read_signal();
        let mut wait_for_workers = false;

        match agent.current_state {
            CognitiveState::Project => {
                println!("[MASTER] Nöral Motor'a metin iletiliyor...");
                signal.set_prompt(prompt); // Metni C-String olarak belleğe yaz!
                signal.command_type = CMD_FORWARD_PASS;
                wait_for_workers = true;
                bus.write_signal(&signal);
            },
            CognitiveState::Act => {
                println!("[MASTER] Sandbox'a veri gönderiliyor...");
                // Şimdilik sandbox'ı atlıyoruz (Sandbox henüz string okumayı bilmiyor)
                signal.command_type = CMD_IDLE; 
                bus.write_signal(&signal);
            },
            CognitiveState::Reflect => {
                let answered_prompt = signal.get_prompt();
                println!("[MASTER] Ajanın nihai düşüncesi: {}", answered_prompt);
                signal.command_type = CMD_IDLE;
                bus.write_signal(&signal);
            },
            _ => { 
                signal.command_type = CMD_IDLE; 
                bus.write_signal(&signal);
            }
        }

        if wait_for_workers {
            loop {
                let check_signal = bus.read_signal();
                if check_signal.command_type == CMD_IDLE { break; }
                std::thread::sleep(std::time::Duration::from_millis(50));
            }
        }

        let next_state = agent.tick();
        if next_state == CognitiveState::Halt { break; }
        std::thread::sleep(std::time::Duration::from_millis(200));
    }
    
    // İşlem bitince bellekteki son durumu (Ajanın yanıtını) geri dönüyoruz
    bus.read_signal().get_prompt()
}

fn main() -> std::io::Result<()> {
    println!("=== Open-Cognitive Logic Gate Core (DAEMON MODE) Başlatılıyor ===");
    let mut bus = MemoryBus::new("/tmp/cog.bus").expect("Memory Bus oluşturulamadı!");
    
    let listener = TcpListener::bind("127.0.0.1:8080")?;
    println!("[DAEMON] Çekirdek 127.0.0.1:8080 portunda bağlantıları bekliyor...");

    for stream in listener.incoming() {
        if let Ok(stream) = stream {
            handle_client(stream, &mut bus);
        }
    }
    Ok(())
}