# Opcodes
## PRT (0001)
Print output to the screen

```
PRT $0 0 1 ; Print the number out
PRT 1 ; Short form

PRT $0 2 0 ; Print variable $0
PRT $0 ; Short form
```

## SET (0010)
Set the value of a variable

```
SET $1 0 12 ; Sets $1 to 12
SET $1 12 ; Short form
```

## ADD (0011)
Add a number to a variable

```
ADD $0 0 1 ; Add 1 to variable $0
ADD $0 1 ; Short form
```

## SUB (0100)
Subtract a number from a variable

```
SUB $0 0 1 ; Subtract 1 from variable $0
SUB $0 1 ; Short form
```

## MUL (0101)
Multiply a variable by a number

```
MUL $0 0 2 ; Multiply variable $0 by 2
MUL $0 2 ; Short form
```

## DIV (0110)
Divide a variable by a number

```
DIV $0 0 2 ; Divide variable $0 by 2
DIV $0 2 ; Short form
```

## JMP (0111)
Jump to specified line of the program, It is a conditional jump, so the most recent condition has to be true for the jump to occur.

```
JMP $0 0 12 ; Jump to line 12 if recent condition is truthy
JMP 12 ; Short form
```

## JNP (1000)
Jump to specified line of the program, It is a conditional jump, so the most recent condition has to be false for the jump to occur.

```
JNP $0 0 12 ; Jump to line 12 if recent condition is falsey
JNP 12 ; Short form
```

## EQL (1001)
Compares a variable to a number, condition result is true if they are equal

```
EQL $2 0 100 ; Compare variable $2 to 100
EQL $2 100 ; Short form
```

## CBP (1010)
Compares a variable to a number, condition result is true if the variable is bigger

```
CBP $4 0 10 ; Compare variable $4 to 10
CBP $4 10 ; Short form
```

## CLP (1011)
Compares a variable to a number, condition result is true if the variable is smaller

```
CLP $4 0 10 ; Compare variable $4 to 10
CLP $4 10 ; Short form
```
