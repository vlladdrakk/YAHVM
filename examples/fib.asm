; Initialize
SET $0 0
SET $1 1
SET $2 0
PRT $0
PRT $1
EQL $0 0
JMP #loop

#printAndLoop
PRT $1
#loop
ADD $2 $0
ADD $2 $1
SET $0 $1
SET $1 $2
SET $2 0
CLP $1 126
JMP #checkGreaterThanZero
JNP #done
#checkGreaterThanZero
CBP $1 0
JMP #printAndLoop

#done