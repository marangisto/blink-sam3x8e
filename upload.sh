:
# generate binary and upload to MCU
ELF=rusty-blink
cargo size --bin $ELF -- -A
cargo objcopy --bin $ELF -- -Obinary /tmp/$ELF.bin
bossac --port=ttyACM0 --info -Ufalse -e -w -v -b /tmp/$ELF.bin -v -R
