section .text
global start
start:
  mov rax, 0x2000004    ;sys_write
  mov rdi, 1 ; stdout
  mov rsi, msg ; lea rsi, [rel msg] でも可
  mov rdx, 12
  syscall

  mov rax, 0x2000000 + 1    ;sys_exit
  mov rdi, 0
  syscall

section .data
  msg db  'hello, world'
