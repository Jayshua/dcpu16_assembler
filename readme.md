# DCPU-16 Assembler
A library and CLI wrapper to assemble DCPU-16 assembly into DCPU-16 binary.


## CLI Usage
After installing the binary CLI (try `cargo install`) call:
```
dasm [-h|--help] [-o output_file] [-a|--ast] input_file
```
+ -h outputs help
+ -o output_file sets the output file (defaults to the input file with the .bin prefix attached)
+ -a print the parsed Abstract Syntax Tree (for debugging the parser)


## Library Usage
Call `dcpu16_assembler::assemble(input_string)` to get a Vec&lt;u16&gt; of the assembled program.


## Known Issues
The assembler will fail if there is any whitespace (including a comment) at the end of the file.