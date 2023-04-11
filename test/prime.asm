PRT 2
PRT 3
SET $0 0 5 ; $0 = i
SET $1 0 3
#loop_start
CBP $0 125
JMP #end
SET $2 2 0 ; $2 = $0
DIV $2 2 $1
MUL $2 2 $1
EQL $2 2 $0
JMP #loop_start
MUL $2
#end