# ⚙️ No-Compiler
Are you tired of having read all the Intel Developer manuals and not being able to use the information correctly?
Are you proficient in binary but cannot show your skills beyond Assembly?
Do you want to code with no restrictions directly in machine code?
No problem my friend, the no-compiler is here!
No compiler is a revolutionary and complex tool that **doesn't** compile bits from any text file to any file format you want (Since **all** digital information is made out of bits)

## 💫 Features
- Compile binary code
- Decompile any file into binary code
- Flex your ability to code in a more low level way than assembly

## 💾 Installation

Download from [releases](https://github.com/gg0074x/no-compiler/releases)

## 🗒️ Usage

Run the command `noc -h` once installed to receive help about the arguments and options.
You must specify an input file containing 0s or 1s characters to represent your binary code and then you can compile it by using `noc ./path/to/file`
You can decompile other files by using the decomp flag like this: `noc ./path/to/file --decomp`

You can use the option `-o` to provide a name for the output file and `-f` to specify the file extension you want the output file to have.

Any characters that are not 0s or 1s will be recognized as comments.
Decompilation can take a while since its reading every single 0 or 1 from the file.

```sh
Usage: noc [OPTIONS] <FILE PATH>...

Arguments:
  <FILE PATH>...  Input file to compile

Options:
  -o, --output <OUTPUT>  Output file path
  -f, --format <FORMAT>  File extension
      --decomp           Decompile input file
  -h, --help             Print help
  -V, --version          Print version
```
