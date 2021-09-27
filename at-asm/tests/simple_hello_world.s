push 0x21646c72  ; "!dlr"
mov rax, 0x6f77206f6c6c6548 ; "ow olleH"
push rax
mov rsi, rsp
mov rdi, 1
mov rdx, 12
mov rax, 0x2000004
syscall
mov rax, 0x2000001
mov rdi, 0
syscall
