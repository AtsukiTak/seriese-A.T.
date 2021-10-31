;; TEST: Exit Code "0"
;; TEST: STDOUT "Hello world!"

section .text
global _main
_main:
  ; prepare data
  push 0x21646c72  ; "!dlr"
  mov rax, 0x6f77206f6c6c6548 ; "ow olleH"
  push rax

  ; write syscall
  mov rsi, rsp
  mov rdi, 1
  mov rdx, 12
  mov rax, 0x2000004
  syscall

  ; exit syscall
  mov rax, 0x2000001
  mov rdi, 0
  syscall
