.data
hello:
    .string "Hello world!\n"

nl:
    .string "\n"

.global _start
.text

_start:
    # write(1, hello, strlen(hello))
    movq $1, %rax # 1 это номер системного вызова write
    movq $1, %rdi # 1 это номер файлового дескрипттора stdout
    movq $hello, %rsi # адрес буфера, который мы будем писать
    movq $13, %rdx    # сколько байт пишем
    syscall

    movq $1, %rax # 1 это номер системного вызова write
    movq $1, %rdi # 1 это номер файлового дескрипттора stdout
    movq $nl, %rsi # адрес буфера, который мы будем писать
    movq $1, %rdx    # сколько байт пишем
    syscall

    # exit(0)
    movq    $60, %rax  # 60 это номер системного вызова exit
    xorq    %rdi, %rdi # 0 это код выхода
    syscall
