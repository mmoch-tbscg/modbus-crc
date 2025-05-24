use std::io;
use std::time::Instant;
use rust_crc_project::{parse_hex_input, compute_batch_crcs_optimized};

fn main() {
    println!("Podaj sekwencję bajtów (HEX, spacja oddziela):");
    let mut hex_input_str = String::new();
    if let Err(e) = io::stdin().read_line(&mut hex_input_str) {
        eprintln!("Błąd odczytu linii: {}", e);
        return;
    }

    let data = match parse_hex_input(&hex_input_str) {
        Ok(d) => d,
        Err(e) => {
            eprintln!("Błąd parsowania HEX: {}", e);
            return;
        }
    };

    println!("Podaj liczbę powtórzeń (1 do 1000000000):");
    let mut n_str = String::new();
    if let Err(e) = io::stdin().read_line(&mut n_str) {
        eprintln!("Błąd odczytu linii: {}", e);
        return;
    }

    let n: u64 = match n_str.trim().parse() {
        Ok(num) => {
            if !(1..=1_000_000_000).contains(&num) {
                eprintln!("Liczba powtórzeń poza zakresem.");
                return;
            }
            num
        }
        Err(e) => {
            eprintln!("Błąd parsowania liczby powtórzeń: {}", e);
            return;
        }
    };

    println!("\n=== OBLICZANIE CRC ===");
    run_crc_test(&data, n);
}

fn run_crc_test(data: &[u8], n: u64) {
    let start = Instant::now();
    let crc_val = compute_batch_crcs_optimized(data, n, false);
    let duration = start.elapsed();

    let crc_lsb = (crc_val & 0xFF) as u8;
    let crc_msb = (crc_val >> 8) as u8;

    println!("CRC (Modbus RTU format): {:02X} {:02X}", crc_lsb, crc_msb);
    println!("CRC (hex): 0x{:04X}", crc_val);
    println!("Czas dla {} iteracji: {:.3} ms", n, duration.as_secs_f64() * 1000.0);
    
    if n > 0 {
        let ops_per_sec = n as f64 / duration.as_secs_f64();
        println!("Wydajność: {:.0} CRC/s", ops_per_sec);
    }
    
    if n >= 100_000 {
        println!("Tryb: Równoległe przetwarzanie");
    }
}
