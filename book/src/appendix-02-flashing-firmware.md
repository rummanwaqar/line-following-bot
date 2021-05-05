## Appendix B: Flashing firmware to hardware

### Flashing with OpenOCD and GDB

Start OpenOCD
```
openocd -f interface/stlink-v2.cfg -f target/stm32f4x.cfg
```

Start GDB with the elf file
```
gdb-multiarch target/thumbv7em-none-eabihf/debug/line-following-bot
```

Then run the following commands in GDB
1. `target remote :3333`
2. `load`
3. `quit`

*Note: Semihosting can be enabled with the following gdb command:
`monitor arm semihosting enable`*

### Flashing with DFU-util

DFU-util requires a binary file. The rust ELF file can be converted using objcopy
```
arm-none-eabi-objcopy -O binary target/thumbv7em-none-eabihf/release/line-following-bot target/thumbv7em-none-eabihf/release/line-following-bot.bin
```

Flash with DFU-util
```
dfu-util -a0 --dfuse-address 0x08000000 -D target/thumbv7em-none-eabihf/release/line-following-bot.bin
```