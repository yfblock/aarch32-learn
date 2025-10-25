
.section .text
.global _start
.type _start, #function
_start:
  bl  _clear_bss
  ldr  sp, =_boot_stack_top
  bl  boot_rust_stage1

.Lend:
  wfi
  bl  .Lend
.size _start, . - _start

_clear_bss:
  ldr  r1, =_boot_stack
  ldr  r2, =_boot_stack_top
  mov  r0, 0
.L1:
  add  r1, r1, #4
  str  r0, [r1]
  cmp  r1, r2
  blt  .L1
  BX   LR
.size _clear_bss, . - _clear_bss


.section .bss
.global _boot_stack
.global _boot_stack_top
_boot_stack:
.p2align 2
.fill 4096
_boot_stack_top:
.size _boot_stack, . - _boot_stack
