# rust-6502-emu

My Rust learning project writing a simple 6502 emulator.

## Building

```shell
cargo build --release
```

This results in the release binary `./target/release/rust-6502-emu`.

## Running

### Synopsis

```
$ ./target/release/rust-6502-emu --help
Usage: rust-6502-emu [OPTIONS] [CYCLES_TO_EXECUTE]

Arguments:
  [CYCLES_TO_EXECUTE]  Maximum cycles to execute [default: 1]

Options:
  -d, --demo         Load demo data
  -f, --file <FILE>  Load data from file
  -v, --verbose...   Verbosity; can be specified multiple times
  -h, --help         Print help
  -V, --version      Print version
```

### Example invocation

Running an example program:

```shell
./target/release/rust-6502-emu -f examples/fibonacci.bin 100
```

Running with demo code:

```shell
./target/release/rust-6502-emu -d 100
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

## Tools

* https://www.masswerk.at/6502/
* https://cc65.github.io/
