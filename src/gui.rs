use eframe::egui;
use rust_crc_project::{parse_hex_input, compute_batch_crcs_optimized, CrcResult};
use std::time::Instant;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([900.0, 700.0])
            .with_title("🔧 CRC Modbus RTU Calculator"),
        ..Default::default()
    };
    
    eframe::run_native(
        "CRC Calculator",
        options,
        Box::new(|_cc| Ok(Box::new(CrcApp::default()))),
    )
}

#[derive(Default)]
struct CrcApp {
    hex_input: String,
    iterations_input: String,
    use_optimized: bool,
    result: Option<CrcResult>,
    error_message: String,
    is_calculating: bool,
    last_calculation_time: Option<f64>,
}

impl eframe::App for CrcApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("🔧 CRC Modbus RTU Calculator");
            ui.add_space(10.0);
            ui.separator();
            ui.add_space(15.0);
            
            ui.horizontal(|ui| {
                ui.label("📝 Sekwencja bajtów (HEX):");
                ui.add(egui::TextEdit::singleline(&mut self.hex_input)
                    .desired_width(300.0)
                    .hint_text("AA BB CC DD"));
            });
            ui.small("Format: AA BB CC DD (oddzielone spacjami, max 256 bajtów)");
            
            ui.add_space(10.0);
            
            ui.horizontal(|ui| {
                ui.label("🔢 Liczba powtórzeń:");
                ui.add(egui::TextEdit::singleline(&mut self.iterations_input)
                    .desired_width(150.0)
                    .hint_text("1000000"));
                ui.label("(1 do 1,000,000,000)");
            });
            
            ui.add_space(10.0);
            
            ui.horizontal(|ui| {
                ui.checkbox(&mut self.use_optimized, "⚡ Użyj zoptymalizowanej wersji");
                ui.label("(16-byte unrolling + równoległość)");
            });
            
            ui.add_space(15.0);
                        
            let calc_button = egui::Button::new(if self.is_calculating { 
                "⏳ Obliczanie..." 
            } else { 
                "🚀 Oblicz CRC" 
            }).min_size(egui::vec2(120.0, 30.0));
            
            if ui.add_enabled(!self.is_calculating, calc_button).clicked() {
                self.calculate_crc();
            }
            
            if self.is_calculating {
                ui.add_space(5.0);
                ui.horizontal(|ui| {
                    ui.spinner();
                    ui.label("Trwa obliczanie CRC...");
                });
            }
            
            ui.add_space(15.0);
            
            if !self.error_message.is_empty() {
                ui.horizontal(|ui| {
                    ui.colored_label(egui::Color32::RED, "❌");
                    ui.colored_label(egui::Color32::RED, &self.error_message);
                });
                ui.add_space(10.0);
            }
            
            if let Some(result) = &self.result {
                ui.separator();
                ui.add_space(10.0);
                ui.heading("📊 Wyniki");
                ui.add_space(10.0);
                
                egui::Grid::new("results_grid")
                    .num_columns(2)
                    .spacing([20.0, 8.0])
                    .striped(true)
                    .show(ui, |ui| {
                        ui.label("🎯 CRC (Modbus RTU format):");
                        ui.code(format!("{:02X} {:02X}", result.crc_lsb, result.crc_msb));
                        ui.end_row();
                        
                        ui.label("🔢 CRC (hex):");
                        ui.code(format!("0x{:04X}", result.crc_value));
                        ui.end_row();
                        
                        ui.label("🔢 CRC (dec):");
                        ui.code(format!("{}", result.crc_value));
                        ui.end_row();
                        
                        ui.label("⏱️ Czas wykonania:");
                        ui.code(format!("{:.3} ms", result.duration_ms));
                        ui.end_row();
                        
                        // Performance info
                        if let Ok(iterations) = self.iterations_input.parse::<u64>() {
                            if iterations > 0 {
                                let ops_per_sec = (iterations as f64 / result.duration_ms) * 1000.0;
                                ui.label("⚡ Wydajność:");
                                ui.code(format!("{:.0} CRC/s", ops_per_sec));
                                ui.end_row();
                                
                                if iterations >= 100_000 {
                                    ui.label("🔥 Tryb:");
                                    ui.code("Równoległy");
                                    ui.end_row();
                                }
                            }
                        }
                    });
            }
            
            ui.add_space(20.0);
            ui.separator();
            ui.add_space(10.0);
            
            ui.heading("📋 Przykładowe dane");
            ui.add_space(10.0);
            
            ui.label("Dane HEX:");
            ui.horizontal(|ui| {
                if ui.button("01 04 00 00 00 0A").clicked() {
                    self.hex_input = "01 04 00 00 00 0A".to_string();
                }
                if ui.button("01 03 00 01 00 02").clicked() {
                    self.hex_input = "01 03 00 01 00 02".to_string();
                }
                if ui.button("AA BB CC DD EE FF").clicked() {
                    self.hex_input = "AA BB CC DD EE FF".to_string();
                }
            });
            
            ui.add_space(5.0);
            ui.label("Liczba iteracji:");
            ui.horizontal(|ui| {
                if ui.button("1,000").clicked() {
                    self.iterations_input = "1000".to_string();
                }
                if ui.button("100,000").clicked() {
                    self.iterations_input = "100000".to_string();
                }
                if ui.button("1,000,000").clicked() {
                    self.iterations_input = "1000000".to_string();
                }
                if ui.button("10,000,000").clicked() {
                    self.iterations_input = "10000000".to_string();
                }
            });
            
            ui.add_space(15.0);
            
            ui.separator();
            ui.add_space(10.0);
            ui.horizontal(|ui| {
                ui.label("💡");
                ui.label("Optymalizacja automatycznie włącza równoległe przetwarzanie dla >100k iteracji");
            });
            
            if let Some(calc_time) = self.last_calculation_time {
                ui.horizontal(|ui| {
                    ui.label("⏰");
                    ui.label(format!("Ostatnie obliczenie: {:.1}ms", calc_time));
                });
            }
        });
        
        if self.is_calculating {
            ctx.request_repaint();
        }
    }
}

impl CrcApp {
    fn calculate_crc(&mut self) {
        self.error_message.clear();
        self.is_calculating = true;
        
        let data = match parse_hex_input(&self.hex_input) {
            Ok(d) => d,
            Err(e) => {
                self.error_message = format!("Błąd parsowania HEX: {}", e);
                self.is_calculating = false;
                return;
            }
        };
        
        if data.is_empty() {
            self.error_message = "Wprowadź przynajmniej jeden bajt danych.".to_string();
            self.is_calculating = false;
            return;
        }
        
        let iterations: u64 = match self.iterations_input.trim().parse() {
            Ok(num) => {
                if !(1..=1_000_000_000).contains(&num) {
                    self.error_message = "Liczba powtórzeń poza zakresem (1-1,000,000,000).".to_string();
                    self.is_calculating = false;
                    return;
                }
                num
            }
            Err(_) => {
                self.error_message = "Nieprawidłowa liczba powtórzeń.".to_string();
                self.is_calculating = false;
                return;
            }
        };
        
        let start = Instant::now();
        let crc_val = compute_batch_crcs_optimized(&data, iterations, self.use_optimized);
        let duration = start.elapsed();
        let duration_ms = duration.as_secs_f64() * 1000.0;
        
        self.result = Some(CrcResult::new(crc_val, duration_ms));
        self.last_calculation_time = Some(duration_ms);
        self.is_calculating = false;
    }
} 