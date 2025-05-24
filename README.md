# CRC Modbus RTU Calculator

Projekt w Rust do szybkiego obliczania CRC Modbus RTU z możliwością porównania wydajności różnych implementacji.

## Funkcje

- ✅ **Wersja konsolowa** - tryb interaktywny z porównaniem wydajności
- ✅ **Aplikacja GUI** - proste graficzne UI
- ✅ **Optymalizacje wydajności**:
  - Oryginalny algorytm (8-byte unrolling)
  - Zoptymalizowany algorytm (16-byte unrolling + unsafe pointers)
  - Równoległe przetwarzanie z Rayon dla dużych iteracji
- ✅ **Pomiar wydajności** - mierzenie czasu i CRC/s

## Wymagania

- Rust (najnowsza stabilna wersja)
- Na macOS: Xcode Command Line Tools mogą być wymagane dla niektórych zależności

```bash
# Zainstaluj Xcode Command Line Tools (tylko macOS)
xcode-select --install
```

## Instalacja i uruchomienie


### Aplikacja konsolowa
```bash
# Uruchom aplikację konsolową
cargo run --release

# Lub zbuduj i uruchom osobno
cargo build --release
./target/release/rust_crc_project
```

Przykład użycia:
```
Podaj sekwencję bajtów (HEX, spacja oddziela):
01 04 00 00 00 0A

Podaj liczbę powtórzeń (1 do 1000000000):
1000000

Wybierz tryb:
1) Oryginalny algorytm (8-byte unrolling)
2) Zoptymalizowany algorytm (16-byte unrolling)
3) Porównanie obydwu
```

### Aplikacja GUI

```bash
# Uruchom aplikację GUI
cargo run --bin gui --release

# Lub zbuduj i uruchom osobno
cargo build --release
./target/release/gui
```

GUI oferuje:
- 📝 Pole do wprowadzania danych HEX
- 🔢 Pole dla liczby iteracji
- ⚙️ Checkbox do wyboru zoptymalizowanej wersji
- 📊 Wyświetlanie wyników (CRC w różnych formatach + wydajność)
- 🎯 Przyciski z przykładowymi danymi

## Optymalizacje

### 1. **16-byte unrolling** (vs oryginalny 8-byte)
- Większe bloki danych przetwarzane na raz
- Lepsza utylizacja cache CPU
- Użycie `unsafe` dla szybszego dostępu do pamięci

### 2. **Unikanie klonowania danych**
- Współdzielenie referencji zamiast `Arc::clone`
- Mniejsze zużycie pamięci

### 3. **Równoległe przetwarzanie**
- Automatyczne dla > 100,000 iteracji
- Podział na batche dla lepszej balansacji obciążenia

## Wydajność

Na współczesnym CPU można oczekiwać:
- **~1-5 milionów CRC/s** dla oryginalnej wersji
- **~10-20% boost** dla zoptymalizowanej wersji
- **Liniowe skalowanie** z liczbą rdzeni dla dużych iteracji

## Dodatkowe komendy

```bash
# Uruchom testy
cargo test

# Sprawdź kod (linting)
cargo clippy

# Formatowanie kodu
cargo fmt

# Uruchom w trybie deweloperskim (szybsza kompilacja)
cargo run
cargo run --bin gui

# Sprawdź zależności
cargo tree
```

## Zależności

- `rayon` - Równoległe przetwarzanie
- `egui` + `eframe` - GUI framework