
# Summary

This project is just for fun and its aim is to Emulate a GBA for playing ROMs. The emulator is written in Rust as a way to learn proper Rust development/ architecting . This is just a scratchpad/throwaway project not meant to be criticized harshly rather as a neat learning tool.

## Architecture

- BIOS / Boot ROM

- Register File
  - RAM
   - IWRAM
   - VRAM
   - OAM
  - Registers
    - General Purpose Registers[15:0]
    - PC
    - SP (CPSR / SPSR)

- CPU
  - ALU
  - CU
  - Interrupts
    - Software
    - Hardware
  - Timer / Clock

- Controller
  -  Up ⬆
  -  Down ⬇
  -  Left ⬅
  -  Right ➡
  -  Start 
  -  Select
  -  A 🅰️
  -  B 🅱️

- PPU
  - Screen
    - Width: 240 px
    - Height: 160 px
    - Refresh Rate: 60 Hz

- Instructions
  - ARM v4
    - Data Processing
    - Branch and Exchange
    - Branch with Link
    - Condition Field
    - PSR
    - Multiply / Multiply Accumulate
    - Multiply Long / Multiply-Accumulate Long
    - Single Data Transfer
    - Block Data Transfer
    - Software Interrupt
    - Coprocessor Data Operations
    - Coprocessor Data Transfers
    - Undefined Instruction
  - THUMB

### References
- (ReGBA BIOS)[https://github.com/Nebuleon/ReGBA/tree/master/bios]
- (ARM ISA)[https://iitd-plos.github.io/col718/ref/arm-instructionset.pdf]
- (ARM OpCodes)[https://imrannazar.com/ARM-Opcode-Map]
- (ARM 7 TDMI)[https://www.dwedit.org/files/ARM7TDMI.pdf]