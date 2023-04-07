; func var type num

; PRT  = 0001
; SET  = 0010
; ADD  = 0011
; SUB  = 0100
; MUL  = 0101
; DIV  = 0110
; JMP  = 0111
; JNP  = 1000
; EQL  = 1001
; CBP  = 1010
; CLP  = 1011
;
; Var => $0 $1 $2 $3 $4 $5 $6 $7 $8 $9 $a $b $c $d $e $f
;
; type => 0 1 2 3
;
; num => -127 to 127

; Example:

SET $0 0 1
ADD $0 0 2
PRT $0 2 0
