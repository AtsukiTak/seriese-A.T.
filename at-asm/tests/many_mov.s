;; TEST: Exit Code "42"

global _main
_main:
  mov edi, 42
  mov r8d, edi
  mov cx, r8w
  mov rax, rcx
  ret
