# DCPU-16 Assembler
A library and CLI wrapper to assemble DCPU-16 assembly into DCPU-16 binary.

## CLI Usage
```
dasm file [-o output_file]
```

## Library Usage
Call `dcpu16_assembler::assemble(input_string)` to get a Vec&lt;u16&gt;.

## Known Issues
The assembler will fail if there is any whitespace (including a comment) at the end of the file.