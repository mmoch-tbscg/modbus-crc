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

## Kompilacja

### Wymagania
```bash
# Zainstaluj Xcode Command Line Tools (macOS)
xcode-select --install

# Opcjonalnie: deps dla Windows cross-compilation
make install-deps
```

### Podstawowa kompilacja
```bash
# Build release version dla maksymalnej wydajności
cargo build --release

# Lub użyj Makefile
make build-release
```

### Tworzenie plików wykonywalnych

#### macOS - natywne binaria
```bash
# Automatycznie z script
make build-macos

# Lub ręcznie
./build.sh

# Wynik: folder dist/ z portable aplikacją
```

#### Windows - pliki .exe  
```bash
# Cross-compile do Windows
make build-windows

# Lub ręcznie  
./build_windows.sh

# Wynik: folder dist_windows/ z .exe i .bat
```

#### Wszystkie platformy
```bash
# Stwórz archiwa dla wszystkich platform
make package-all

# Wynik:
# - crc_calculator_macos.tar.gz
# - crc_calculator_windows.zip
```

## Użytkowanie

### Wersja konsolowa

```bash
# Uruchom aplikację konsolową
cargo run --bin console --release
# lub
make run-console

# Lub bezpośrednio (po build)
./target/release/console
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
# lub
make run-gui

# Lub bezpośrednio (po build)
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

## Przykładowe dane do testów

| Dane HEX | Opis |
|----------|------|
| `01 04 00 00 00 0A` | Modbus Read Input Registers |
| `01 03 00 01 00 02` | Modbus Read Holding Registers |
| `AA BB CC DD` | Test data |

## Wydajność

Na współczesnym CPU można oczekiwać:
- **~1-5 milionów CRC/s** dla oryginalnej wersji
- **~10-20% boost** dla zoptymalizowanej wersji
- **Liniowe skalowanie** z liczbą rdzeni dla dużych iteracji

## Struktura projektu

```
src/
├── lib.rs          # Shared CRC functions & optimizations
├── main.rs         # Console application
└── gui.rs          # GUI application

build.sh            # macOS build script
build_windows.sh    # Windows cross-compile script
Makefile            # Build automation
```

## Zależności

- `rayon` - Równoległe przetwarzanie
- `egui` + `eframe` - GUI framework

## Plany rozwoju

- [ ] SIMD optymalizacje (AVX2/NEON)
- [ ] Benchmark suite z różnymi rozmiarami danych
- [ ] Export wyników do CSV
- [ ] WebAssembly version dla przeglądarki 