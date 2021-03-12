section .data
msg:
  db "hello world",10

section .text

global start
start:
  mov rsi, msg
  mov rdx, 12
	mov rax, 1
	mov rdi, rax
	syscall

	xor rdi, rdi
	mov rax, 0x3c
	syscall
