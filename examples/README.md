# Examples

* `add.asm` - Simple arithmetic example of adding two numbers
* `fibonacci.asm` - Calculates the Nth fibonacci number

## Build

Using [CC65](https://cc65.github.io/):

```shell
ca65 example.asm
ld65 -o example.bin -C rust-6502-emu.cfg example.o
```
