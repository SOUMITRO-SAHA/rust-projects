# Memory Concepts

## Basic Memory Concept

- Memory is stored using binary
  - Bits: 0 or 1
- Computer optimized for bytes
  - 1 byte == 8 contiguous bits
- Fully contiguous

## Addresses

- All data in memory has an "addresses"
  - Used to locate data
  - Always the same - only data changes
- Usually don't utilize addresses directly
  - Variables handle most of the work

## Offsets

- Items can be located at an address using an "offset"
- Offsets begin at 0
- Represent the number of bytes away from the original address
  - Normally deal with indexes instead

![Address & Offsets](image.png)