cargo build
if ERRORLEVEL 1 exit 1
espflash COM3 target/riscv32imc-esp-espidf/debug/esp32c3-idf-led-example
if ERRORLEVEL 1 exit 1
espmonitor --chip esp32c3 --speed 115200 COM3
