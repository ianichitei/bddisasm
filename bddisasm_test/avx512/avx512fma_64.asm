    bits 64
    
    vpmadd52huq xmm2, xmm7, xmm0
    vpmadd52huq xmm2, xmm7, [rbx]
    vpmadd52huq xmm2, xmm7, [rbx]{1to2}
    vpmadd52huq xmm2, xmm7, [rbx+r11*8+256]
    vpmadd52huq xmm2, xmm7, [rbx+r11*8-256]
    vpmadd52huq xmm2{k5}, xmm7, xmm0
    vpmadd52huq xmm2{k5}, xmm7, [rbx]
    vpmadd52huq xmm2{k5}, xmm7, [rbx]{1to2}
    vpmadd52huq xmm2{k5}, xmm7, [rbx+r11*8+256]
    vpmadd52huq xmm2{k5}, xmm7, [rbx+r11*8-256]
    vpmadd52huq xmm2{k5}{z}, xmm7, xmm0
    vpmadd52huq xmm2{k5}{z}, xmm7, [rbx]
    vpmadd52huq xmm2{k5}{z}, xmm7, [rbx]{1to2}
    vpmadd52huq xmm2{k5}{z}, xmm7, [rbx+r11*8+256]
    vpmadd52huq xmm2{k5}{z}, xmm7, [rbx+r11*8-256]
    vpmadd52huq ymm16, ymm13, ymm15
    vpmadd52huq ymm16, ymm13, [rbx]
    vpmadd52huq ymm16, ymm13, [rbx]{1to4}
    vpmadd52huq ymm16, ymm13, [rbx+r11*8+256]
    vpmadd52huq ymm16, ymm13, [rbx+r11*8-256]
    vpmadd52huq ymm16{k5}, ymm13, ymm15
    vpmadd52huq ymm16{k5}, ymm13, [rbx]
    vpmadd52huq ymm16{k5}, ymm13, [rbx]{1to4}
    vpmadd52huq ymm16{k5}, ymm13, [rbx+r11*8+256]
    vpmadd52huq ymm16{k5}, ymm13, [rbx+r11*8-256]
    vpmadd52huq ymm16{k5}{z}, ymm13, ymm15
    vpmadd52huq ymm16{k5}{z}, ymm13, [rbx]
    vpmadd52huq ymm16{k5}{z}, ymm13, [rbx]{1to4}
    vpmadd52huq ymm16{k5}{z}, ymm13, [rbx+r11*8+256]
    vpmadd52huq ymm16{k5}{z}, ymm13, [rbx+r11*8-256]
    vpmadd52huq zmm24, zmm24, zmm31
    vpmadd52huq zmm24, zmm24, [rbx]
    vpmadd52huq zmm24, zmm24, [rbx]{1to8}
    vpmadd52huq zmm24, zmm24, [rbx+r11*8+256]
    vpmadd52huq zmm24, zmm24, [rbx+r11*8-256]
    vpmadd52huq zmm24{k5}, zmm24, zmm31
    vpmadd52huq zmm24{k5}, zmm24, [rbx]
    vpmadd52huq zmm24{k5}, zmm24, [rbx]{1to8}
    vpmadd52huq zmm24{k5}, zmm24, [rbx+r11*8+256]
    vpmadd52huq zmm24{k5}, zmm24, [rbx+r11*8-256]
    vpmadd52huq zmm24{k5}{z}, zmm24, zmm31
    vpmadd52huq zmm24{k5}{z}, zmm24, [rbx]
    vpmadd52huq zmm24{k5}{z}, zmm24, [rbx]{1to8}
    vpmadd52huq zmm24{k5}{z}, zmm24, [rbx+r11*8+256]
    vpmadd52huq zmm24{k5}{z}, zmm24, [rbx+r11*8-256]
    vpmadd52luq xmm2, xmm7, xmm0
    vpmadd52luq xmm2, xmm7, [rbx]
    vpmadd52luq xmm2, xmm7, [rbx]{1to2}
    vpmadd52luq xmm2, xmm7, [rbx+r11*8+256]
    vpmadd52luq xmm2, xmm7, [rbx+r11*8-256]
    vpmadd52luq xmm2{k5}, xmm7, xmm0
    vpmadd52luq xmm2{k5}, xmm7, [rbx]
    vpmadd52luq xmm2{k5}, xmm7, [rbx]{1to2}
    vpmadd52luq xmm2{k5}, xmm7, [rbx+r11*8+256]
    vpmadd52luq xmm2{k5}, xmm7, [rbx+r11*8-256]
    vpmadd52luq xmm2{k5}{z}, xmm7, xmm0
    vpmadd52luq xmm2{k5}{z}, xmm7, [rbx]
    vpmadd52luq xmm2{k5}{z}, xmm7, [rbx]{1to2}
    vpmadd52luq xmm2{k5}{z}, xmm7, [rbx+r11*8+256]
    vpmadd52luq xmm2{k5}{z}, xmm7, [rbx+r11*8-256]
    vpmadd52luq ymm16, ymm13, ymm15
    vpmadd52luq ymm16, ymm13, [rbx]
    vpmadd52luq ymm16, ymm13, [rbx]{1to4}
    vpmadd52luq ymm16, ymm13, [rbx+r11*8+256]
    vpmadd52luq ymm16, ymm13, [rbx+r11*8-256]
    vpmadd52luq ymm16{k5}, ymm13, ymm15
    vpmadd52luq ymm16{k5}, ymm13, [rbx]
    vpmadd52luq ymm16{k5}, ymm13, [rbx]{1to4}
    vpmadd52luq ymm16{k5}, ymm13, [rbx+r11*8+256]
    vpmadd52luq ymm16{k5}, ymm13, [rbx+r11*8-256]
    vpmadd52luq ymm16{k5}{z}, ymm13, ymm15
    vpmadd52luq ymm16{k5}{z}, ymm13, [rbx]
    vpmadd52luq ymm16{k5}{z}, ymm13, [rbx]{1to4}
    vpmadd52luq ymm16{k5}{z}, ymm13, [rbx+r11*8+256]
    vpmadd52luq ymm16{k5}{z}, ymm13, [rbx+r11*8-256]
    vpmadd52luq zmm24, zmm24, zmm31
    vpmadd52luq zmm24, zmm24, [rbx]
    vpmadd52luq zmm24, zmm24, [rbx]{1to8}
    vpmadd52luq zmm24, zmm24, [rbx+r11*8+256]
    vpmadd52luq zmm24, zmm24, [rbx+r11*8-256]
    vpmadd52luq zmm24{k5}, zmm24, zmm31
    vpmadd52luq zmm24{k5}, zmm24, [rbx]
    vpmadd52luq zmm24{k5}, zmm24, [rbx]{1to8}
    vpmadd52luq zmm24{k5}, zmm24, [rbx+r11*8+256]
    vpmadd52luq zmm24{k5}, zmm24, [rbx+r11*8-256]
    vpmadd52luq zmm24{k5}{z}, zmm24, zmm31
    vpmadd52luq zmm24{k5}{z}, zmm24, [rbx]
    vpmadd52luq zmm24{k5}{z}, zmm24, [rbx]{1to8}
    vpmadd52luq zmm24{k5}{z}, zmm24, [rbx+r11*8+256]
    vpmadd52luq zmm24{k5}{z}, zmm24, [rbx+r11*8-256]
