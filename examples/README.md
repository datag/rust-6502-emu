# Examples

* `add.asm` - Simple arithmetic example of adding two numbers
* `fibonacci.asm` - Calculates the Nth fibonacci number

## Toolchain

Using [CC65](https://cc65.github.io/).

```shell
git clone https://github.com/cc65/cc65
cd cc65
make
sudo make install

# OR local install:
mkdir $HOME/local
make PREFIX=$HOME/local/cc65
make install PREFIX=$HOME/local/cc65

export PATH=$HOME/local/cc65/bin:$PATH
```

## Build

```shell
ca65 example.asm
ld65 -o example.bin -C rust-6502-emu.cfg example.o
```
