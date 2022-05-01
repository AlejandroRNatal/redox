# Instructions
 Instruction | Bytes 
|---| ---
 Data Processing | [Cond 4-bits] 0 0 I [Opcode 4-bits] S [Rn 4-bits] [Rd 4-bits] [Operand 2 12-bits]  
 Branch and Exchange | Cond 0 0 0 0 0 0 A S Rd Rn Rs 1 0 0 1 Rm 
 Multiply / Multiply Accumulate | Cond 0 0 0 0 0 0 A S Rd Rn Rs 1 0 0 1 Rm 
 Multiply Long / Multiply-Accumulate Long | Cond 0 0 0 0 1 U A S RdHi RdLo Rn 1 0 0 1 Rm 
 Single Data Transfer | Cond 0 0 0 1 0 B 0 0 Rn Rd 0 0 0 0 1 0 0 1 Rm
 Block Data Transfer | Cond 1 0 0 P U S W L Rn Register List
 Software Interrupt | Cond 1 1 1 1 Ignored by processor
 Coprocessor Data Operations | Cond 1 1 1 0 CP Opc CRn CRd CP# CP 0 CRm 
 Coprocessor Data Transfer | Cond 1 1 0 P U N W L Rn CRd CP# Offset
 Undefined Instruction | Cond 0 1 1 1
