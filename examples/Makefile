CA65 = ca65
LD65 = ld65

CONFIG = rust-6502-emu.cfg

ASM_FILES := $(wildcard *.asm)
BIN_FILES := $(patsubst %.asm, %.bin, $(ASM_FILES))

all: $(BIN_FILES)

%.o: %.asm
	$(CA65) -o $@ $<

%.bin: %.o
	$(LD65) -o $@ -C $(CONFIG) $<

.PHONY: clean
clean:
	rm -f *.o
