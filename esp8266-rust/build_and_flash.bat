cargo.exe build --release --color=always --message-format=json-diagnostic-rendered-ansi --package a_clockwork_blue --bin a_clockwork_blue
espflash.exe COM3 .\target\xtensa-esp8266-none-elf\release\a_clockwork_blue
