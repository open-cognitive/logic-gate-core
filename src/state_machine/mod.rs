//! # Bilişsel Durum Makinesi (Cognitive State Machine)
//! 
//! Bu modül, ajanın o anki "Düşünce Durumunu" yönetir. 
//! Olasılıksal yapay zekayı deterministik bir çerçeveye oturtan ana mekanizmadır.
//! Her bir durum geçişi (Transition) kesin kurallara bağlıdır.

/// Ajanın içinde bulunabileceği mantıksal durumlar.
/// ReAct (Reason + Act) felsefesinin donanım seviyesindeki karşılığıdır.
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum CognitiveState {
    /// Yeni bir görevin sisteme girdiği ve belleğin tarandığı an.
    Ingest,
    /// Nöral Motordan (Sistem 1) ileri yayılım (Forward Pass) ile hipotez istendiği an.
    Project,
    /// Nöral Motordan gelen olasılıkların şema ve güvenlik kurallarına göre sınandığı an.
    Evaluate,
    /// Sistemin dış dünyaya (WASM Sandbox) etki etmek için araç çağırdığı an.
    Act,
    /// Aracın veya düşüncenin sonucunun değerlendirilip hata düzeltmesi (Self-Correction) yapıldığı an.
    Reflect,
    /// Görevin başarıyla tamamlandığı veya durdurulduğu an.
    Halt,
}

/// Bilişsel Ajanın çekirdek yapısı.
/// Tüm durum geçişlerini ve belleği (ileride eklenecek) bu struct yönetir.
pub struct AgentCore {
    /// Ajanın mevcut düşünce durumu
    pub current_state: CognitiveState,
    /// Döngüsel düşünme hatalarını (Infinite Loop) önlemek için adım sayacı
    pub step_counter: u32,
    /// Maksimum izin verilen düşünce adımı
    pub max_steps: u32,
}

impl AgentCore {
    /// Yeni ve taze bir Ajan Çekirdeği oluşturur.
    pub fn new(max_steps: u32) -> Self {
        Self {
            current_state: CognitiveState::Ingest, // Her zaman Ingest ile başlar
            step_counter: 0,
            max_steps,
        }
    }

    /// Durum makinesini bir sonraki mantıksal adıma geçirir.
    /// 
    /// # Örnek Döngü
    /// `Ingest` -> `Project` -> `Evaluate` -> `Act` -> `Reflect` -> `Halt`
    pub fn tick(&mut self) -> CognitiveState {
        if self.step_counter >= self.max_steps {
            println!("[SİSTEM UYARISI] Maksimum düşünce adımına ulaşıldı. Zorunlu Halt.");
            self.current_state = CognitiveState::Halt;
            return self.current_state;
        }

        self.step_counter += 1;

        // Deterministik geçiş kuralları (Transition Logic)
        self.current_state = match self.current_state {
            CognitiveState::Ingest => CognitiveState::Project,
            CognitiveState::Project => CognitiveState::Evaluate,
            CognitiveState::Evaluate => {
                // Şimdilik varsayılan olarak her düşüncenin onaylandığını ve Act'a geçtiğini varsayıyoruz.
                // İleride burada Neural Engine'den gelen logitler denetlenecek.
                CognitiveState::Act
            },
            CognitiveState::Act => CognitiveState::Reflect,
            CognitiveState::Reflect => CognitiveState::Halt, // Şimdilik başarılı bitir
            CognitiveState::Halt => CognitiveState::Halt,
        };

        self.current_state
    }
}