# rust-6502-emu

## Status

* Implemented Opcodes: https://docs.google.com/spreadsheets/d/1L-864UBW4wkh5y9SUwTcCbbFewtKfADZxqDlOhk8pA4

## Sources

* https://en.wikipedia.org/wiki/MOS_Technology_6502
* https://www.masswerk.at/6502/6502_instruction_set.html
* https://web.archive.org/web/20190925014923/http://www.obelisk.me.uk/6502/
  * https://web.archive.org/web/20190926210651/http://www.obelisk.me.uk/6502/reference.html
  * https://web.archive.org/web/20190908035706/http://www.obelisk.me.uk/6502/addressing.html
* https://sta.c64.org/cbm64mem.html
* https://www.nesdev.org/wiki/Status_flags
* https://www.pagetable.com/?p=410

## Tools

* https://www.masswerk.at/6502/
* https://cc65.github.io/

## TODO

* sort instructions by type
* refactor opcode constants into enum?
* merge tests (e.g. AND + EOR + ORA)
* logging
* map ROM
* method doc
* refactor: set ZN flags for value
* address helper for tests
* move most unit tests of cpu to integration tests?