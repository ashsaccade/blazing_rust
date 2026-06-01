.text
.globl main

main:
    movq $8, %rdi      # n = 8 индекс n (какое число Фибоначчи ищем)

    movq $0, %rbx      # F(0) = 0 предыдущее число
    movq $1, %rax      # F(1) = 1 текущее число
    movq $1, %rcx      # i = 1 счётчик цикла

loop:
    # Перейти на done, если rcx >= rdi
    cmpq %rdi, %rcx  # Jump if Greater than or Equal to (rcx - rdi)
    jge done


    movq %rax, %rdx    # сохранить текущее значение
    addq %rbx, %rax    # current = current + previous
    movq %rdx, %rbx    # previous = старое current

    incq %rcx          # i++
    jmp loop

done:
    ret
