    
    bits 64

_start:
    mov     rax, 0xBDBDBDBDBDBDBDBD
    mov     rcx, 1

    ; mov r31, 0xFFFFFFFFFFFFFFFF
    db 0xD5, 0x7F, 0xBF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF
    ; add r31, rcx
    db 0xD5, 0x19, 0x01, 0xCF
    ; add r31, rcx
    db 0xD5, 0x19, 0x01, 0xCF
    ; mov r16, r31
    db 0xD5, 0x5C, 0x89, 0xF8
    
    lea rdx, [rel _start]
    
    ; mov r30b, byte [rdx + r31 * 8 + 0x0]
    db 0xD5, 0x66, 0x8A, 0x34, 0xFA
    
    int3
    
