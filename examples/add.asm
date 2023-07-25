; Simple arithmetic example adding two numbers
; https://codeburst.io/running-programs-on-the-apple-ii-cc183aab268

CLC      ; CLEAR CARRY BIT
CLD      ; CLEAR DECIMAL BIT
ADR1 = $6100 ; WHERE IN MEMORY ARE THESE THINGS
ADR2 = $6101
ADR3 = $6102
LDA #01
STA ADR1 ;load ADR1 with the value 1
LDA #02
STA ADR2 ;load ADR2 with the value 2
LDA ADR1 ; LOAD CONTENTS OF ADR1 INTO ACCUMULATOR
ADC ADR2 ; ADD CONTENTS OF ADR2 INTO ACCUMULATOR 
STA ADR3 ; TRANSFER CONTENT OF ACC TO ADR3
RTS
