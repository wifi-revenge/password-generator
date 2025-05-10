# Password Generator (Rust)

Generatore di password ad alte prestazioni che utilizza zykgen in parallelo.

## 🚀 Funzionalità
- Generazione parallela con N core
- Progress bar visuale
- Supporta chunk da 50 elementi
- Log delle password speciali

## 📦 Installazione
1. Installa Rust:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

2. Clona il repository:
```bash
git clone https://github.com/wifi-revenge/password-generator.git
cd password-generator
```

3. Compila:
```bash
cargo build --release
```

## 🛠 Utilizzo con 8 processori
```bash
./target/release/password-generator inputs/serials.txt passwords.txt 8
```
## con 16 processori
```bash
./target/release/password-generator inputs/serials.txt passwords.txt 16
```

## 🧩 Dipendenze
- [zykgen](https://github.com/luc10/zykgen) installato nel PATH


