global move

; <-- AX move(DI pos, SI roll) -->
move:
    xor eax, eax            ; AX <- the result
    mov eax, esi
    mov esi, 0x2
    mul esi
    add eax, edi
    ret
