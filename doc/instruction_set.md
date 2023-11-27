# Instruction Set

## Registers

### Data Types

| Index | Type | Description |
| ----- | ---- | ----------- |
| 0 | u8 | 8 bit unsigned integer |
| 1 | u16 | 16 bit unsigned integer |
| 2 | u32 | 32 bit unsigned integer |
| 3 | i8 | 8 bit signed integer |
| 4 | i16 | 16 bit signed integer |
| 5 | i32 | 32 bit signed integer |
| 6 | vu8 | A cache line of 8 bit unsigned integers |
| 7 | vu16 | A cache line of 16 bit unsigned integers |
| 8 | vu32 | A cache line of 32 bit unsigned integers |
| 9 | vi8 | A cache line of 8 bit signed integers |
| 10 | vi16 | A cache line of 16 bit signed integers |
| 11 | vi32 | A cache line of 32 bit signed integers |


### Belt Registers

| Index | Abbreviation | Name | Description |
| ----- | ---- | ----------- | - |
| 0 | R0 | RZ | Always zero |
| 1-15 | R1-15 | Belt Registers | The last 15 results to be produced where R1 is most recent and R15 is least recent |
| 16-255 | R16-255 | Extended belt registers | Register values produced more than 15 results ago |

Belt registers are used to store the results of operations. The most recent result is stored in R1 and the oldest result is stored in R15. When a new result is produced, the value in R15 is lost and all other registers are shifted down. The value in R1 is always the most recent result and the value in R15 is always the oldest result.


### Special Registers

| Index | Abbreviation | Name | Description |
| ----- | ---- | ----------- | - |
| 0 | PC | Program Counter | Pointer to the start of the currently executing instruction |
| 1 | SP | Stack Pointer | Pointer to the top of the current stack |
| 2 | TP | Thread Pointer | Pointer to the start of the threads memory structure including thread local variables and the base of the stack |
| 3 | PP | Process Pointer | Pointer to the start of the processes memory structure |

## Instruction Format

<!-- 3 bits for instruction catagory, 5 bits for instruction and 8 bits for registers and immediates with an optional second 16 bit or 32bit parameter -->

| 15 - 13 | 12 - 8 | 7 - 0 |
| ------- | ------ | ----- |
| Catagory | Instruction | Register/Imidiate |

## Instruction Catagories

| Index | Category | Description |
| ----- | -------- | ----------- |
| 0 | General | Miscellaneous instructions |
| 1 | Load | Instructions to load data from RAM into registers |
| 2 | Store | Instructions to Store data from registers to RAM |
| 3 | Arithmetic | Instructions to perform mathematical operations |
| 4 | Binary Logic | Instructions to perform bit manipulation |
| 5 | Flow Control | Instructions to implement flow control including if, loop, and call |
| 6 | System | Instructions to perform system level control like creating memory regions |
| 7 | Extension | Reserved for future extension |


## Instruction Set

### General Instructions

| Instruction | Encoding | Description |
| ----------- | ----------- |
| NOP | 0000h | No operation. Does not change the CPU state other than advancing the Program Counter one instruction (2 bytes) |

### Load Instructions

#### Load Types

| Index | Type | Description |
| ----- | ---- | ----------- |
| 0 | Immediate | Load a constant value into a register |
| 1 | Register | Load relative to a Special Register |
| 2 | Indirect | Load from a memory address stored in a register |
| 3 | Indirect with Offset | Load from a memory address stored in a register with an offset |


| Instruction | Encoding | Description |
| ----------- | ----------- | ----- |
| LI8 | 0100h | Load a 16 bit value from RAM into a register |