.text
.globl main

main:
    movq $6, %rax # первое число и результат сложения
    movq $7, %rdi # второе число

    addq %rdi, %rax

    ret
