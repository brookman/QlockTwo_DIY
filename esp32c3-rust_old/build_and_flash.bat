cargo.exe build --release --color=always --message-format=json-diagnostic-rendered-ansi --package a_clockwork_blue --bin a_clockwork_blue
if ERRORLEVEL 1 exit 1
espflash.exe COM3 .\target\riscv32i-unknown-none-elf\release\a_clockwork_blue
