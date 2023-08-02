# rust-6502-emu

My Rust learning project writing a simple 6502 emulator.

> :warning: Although there are some unit tests, the functionality is not yet completely tested or proven. And some functionality such as BCD mode still needs to be implemented. This is just a tinkering project - don't expect too much!

Key data:

* Memory size: `64K`
* Stack: `0x0100` to `0x01FF`
* Reset vector address: `0xE000`

## Building

```shell
cargo build --release
```

This results in the release binary `./target/release/rust-6502-emu`.

## Running

### Synopsis

```
Usage: rust-6502-emu [OPTIONS]

Options:
  -c, --cycles <CYCLES>  Cycles to execute
  -d, --demo             Load demo data
  -f, --file <FILE>      Load data from file
  -i, --interactive      Interactive mode
  -v, --verbose...       Verbosity; can be specified multiple times
  -h, --help             Print help
  -V, --version          Print version
```

### Example invocation

Running an example program:

```shell
./target/release/rust-6502-emu -f examples/fibonacci.bin
```

Running an example program step-by-step in interactive mode:

```shell
./target/release/rust-6502-emu -i -f examples/fibonacci.bin
```

Running with demo code:

```shell
./target/release/rust-6502-emu -d
```

Running without a program initialized in memory is less exciting, as we're hitting a BRK as first instruction:

```shell
./target/release/rust-6502-emu
```

## Example programs

There are some example programs in `./examples` including a Makefile. [CC65](https://cc65.github.io/) is used for assembling and linking.

Pre-built `*.bin` files can be loaded using the `-f <binfile>` option.

The examples can be (re)build using `make all`.

## Useful resources / documents

* https://en.wikipedia.org/wiki/MOS_Technology_6502
* https://www.masswerk.at/6502/6502_instruction_set.html
* https://web.archive.org/web/20190925014923/http://www.obelisk.me.uk/6502/
  * https://web.archive.org/web/20190926210651/http://www.obelisk.me.uk/6502/reference.html
  * https://web.archive.org/web/20190908035706/http://www.obelisk.me.uk/6502/addressing.html
* https://sta.c64.org/cbm64mem.html
* https://www.nesdev.org/wiki/Status_flags
* https://www.pagetable.com/?p=410
* https://en.wikibooks.org/wiki/6502_Assembly
* https://codeburst.io/running-programs-on-the-apple-ii-cc183aab268
* https://en.wikipedia.org/wiki/Interrupts_in_65xx_processors
* Testing
  * https://www.nesdev.org/wiki/Visual6502wiki/6502TestPrograms
  * https://github.com/Klaus2m5/6502_65C02_functional_tests

## Tools

* https://www.masswerk.at/6502/
* https://cc65.github.io/
