;; TEST: Exit Code "0"
;; TEST: STDOUT "Hello, World!!"

section .text
global _main
_main:
  mov rax, 0x2000000 + 4    ;sys_write
  mov rdi, 1 ; stdout
  mov rsi, msg ; lea rsi, [rel msg] でも可
  mov rdx, 14
  syscall

  mov rax, 0x2000000 + 1    ;sys_exit
  mov rdi, 0
  syscall

section .data
  msg db  "Hello, World!!"
