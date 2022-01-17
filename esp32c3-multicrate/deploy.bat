cargo build
if ERRORLEVEL 1 exit 1
@REM espflash COM3 target/riscv32imc-esp-espidf/debug/esp32c3-multicrate
espflash_exe COM3 --partition-table table.csv --bootloader bootloader.bin target\riscv32imc-esp-espidf\debug\esp32c3-multicrate
if ERRORLEVEL 1 exit 1
espmonitor --chip esp32c3 --speed 115200 COM3
