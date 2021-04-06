main:
    li x1, 2
    li x2, 2
    beq x1, x2, success

fail:
    li x3, 1
    nop

success:
    li x3, 2
    nop