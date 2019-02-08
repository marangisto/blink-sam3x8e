:
# generate binary and upload to MCU
ELF=blink-sam3x8e
cargo size --bin $ELF -- -A
cargo objcopy --bin $ELF -- -Obinary /tmp/$ELF.bin
bossac --port=ttyACM0 --info -Ufalse -e -w -v -b /tmp/$ELF.bin -v -R
