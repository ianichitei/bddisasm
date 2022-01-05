/*
 * Copyright (c) 2021 Bitdefender
 * SPDX-License-Identifier: Apache-2.0
 */
//! Instruction categories.

use super::decode_error::DecodeError;
use core::convert::TryFrom;

/// Instruction category.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum Category {
    I3dnow,
    Aes,
    Aeskl,
    Amx,
    Arith,
    Avx,
    Avx2,
    Avx2gather,
    Avx512,
    Avx512bf16,
    Avx512fp16,
    Avx512vbmi,
    Avx512vp2intersect,
    Avxvnni,
    Bitbyte,
    Blend,
    Bmi1,
    Bmi2,
    Broadcast,
    Call,
    Cet,
    Cldemote,
    Cmov,
    Compress,
    CondBr,
    Conflict,
    Convert,
    Dataxfer,
    Decimal,
    Enqcmd,
    Expand,
    Flagop,
    Fma4,
    Fred,
    Gather,
    Gfni,
    Hreset,
    I386,
    Ifma,
    Interrupt,
    Io,
    Iostringop,
    Kl,
    Kmask,
    Knl,
    Lkgs,
    Logic,
    Logical,
    LogicalFp,
    Lwp,
    Lzcnt,
    Misc,
    Mmx,
    Movdir64b,
    Movdiri,
    Mpx,
    Nop,
    Padlock,
    Pclmulqdq,
    Pconfig,
    Pop,
    Prefetch,
    Ptwrite,
    Push,
    Rdpid,
    Rdrand,
    Rdseed,
    Rdwrfsgs,
    Ret,
    Rotate,
    Scatter,
    Segop,
    Semaphore,
    Sgx,
    Sha,
    Shift,
    Smap,
    Sse,
    Sse2,
    Stringop,
    Sttni,
    Syscall,
    Sysret,
    System,
    Tdx,
    Ud,
    Uintr,
    UncondBr,
    Unknown,
    Vaes,
    Vfma,
    Vfmaps,
    Vnni,
    Vnniw,
    Vpclmulqdq,
    Vpopcnt,
    Vtx,
    Waitpkg,
    Wbnoinvd,
    Widenop,
    WideKl,
    X87Alu,
    Xop,
    Xsave,
}

#[doc(hidden)]
impl TryFrom<ffi::ND_INS_CATEGORY> for Category {
    type Error = DecodeError;

    fn try_from(value: ffi::ND_INS_CATEGORY) -> Result<Self, Self::Error> {
        match value {
            ffi::_ND_INS_TYPE::ND_CAT_INVALID => Err(DecodeError::InternalError(value as u64)),
            ffi::_ND_INS_TYPE::ND_CAT_3DNOW => Ok(Category::I3dnow),
            ffi::_ND_INS_TYPE::ND_CAT_AES => Ok(Category::Aes),
            ffi::_ND_INS_TYPE::ND_CAT_AESKL => Ok(Category::Aeskl),
            ffi::_ND_INS_TYPE::ND_CAT_AMX => Ok(Category::Amx),
            ffi::_ND_INS_TYPE::ND_CAT_ARITH => Ok(Category::Arith),
            ffi::_ND_INS_TYPE::ND_CAT_AVX => Ok(Category::Avx),
            ffi::_ND_INS_TYPE::ND_CAT_AVX2 => Ok(Category::Avx2),
            ffi::_ND_INS_TYPE::ND_CAT_AVX2GATHER => Ok(Category::Avx2gather),
            ffi::_ND_INS_TYPE::ND_CAT_AVX512 => Ok(Category::Avx512),
            ffi::_ND_INS_TYPE::ND_CAT_AVX512BF16 => Ok(Category::Avx512bf16),
            ffi::_ND_INS_TYPE::ND_CAT_AVX512FP16 => Ok(Category::Avx512fp16),
            ffi::_ND_INS_TYPE::ND_CAT_AVX512VBMI => Ok(Category::Avx512vbmi),
            ffi::_ND_INS_TYPE::ND_CAT_AVX512VP2INTERSECT => Ok(Category::Avx512vp2intersect),
            ffi::_ND_INS_TYPE::ND_CAT_AVXVNNI => Ok(Category::Avxvnni),
            ffi::_ND_INS_TYPE::ND_CAT_BITBYTE => Ok(Category::Bitbyte),
            ffi::_ND_INS_TYPE::ND_CAT_BLEND => Ok(Category::Blend),
            ffi::_ND_INS_TYPE::ND_CAT_BMI1 => Ok(Category::Bmi1),
            ffi::_ND_INS_TYPE::ND_CAT_BMI2 => Ok(Category::Bmi2),
            ffi::_ND_INS_TYPE::ND_CAT_BROADCAST => Ok(Category::Broadcast),
            ffi::_ND_INS_TYPE::ND_CAT_CALL => Ok(Category::Call),
            ffi::_ND_INS_TYPE::ND_CAT_CET => Ok(Category::Cet),
            ffi::_ND_INS_TYPE::ND_CAT_CLDEMOTE => Ok(Category::Cldemote),
            ffi::_ND_INS_TYPE::ND_CAT_CMOV => Ok(Category::Cmov),
            ffi::_ND_INS_TYPE::ND_CAT_COMPRESS => Ok(Category::Compress),
            ffi::_ND_INS_TYPE::ND_CAT_COND_BR => Ok(Category::CondBr),
            ffi::_ND_INS_TYPE::ND_CAT_CONFLICT => Ok(Category::Conflict),
            ffi::_ND_INS_TYPE::ND_CAT_CONVERT => Ok(Category::Convert),
            ffi::_ND_INS_TYPE::ND_CAT_DATAXFER => Ok(Category::Dataxfer),
            ffi::_ND_INS_TYPE::ND_CAT_DECIMAL => Ok(Category::Decimal),
            ffi::_ND_INS_TYPE::ND_CAT_ENQCMD => Ok(Category::Enqcmd),
            ffi::_ND_INS_TYPE::ND_CAT_EXPAND => Ok(Category::Expand),
            ffi::_ND_INS_TYPE::ND_CAT_FLAGOP => Ok(Category::Flagop),
            ffi::_ND_INS_TYPE::ND_CAT_FMA4 => Ok(Category::Fma4),
            ffi::_ND_INS_TYPE::ND_CAT_GATHER => Ok(Category::Gather),
            ffi::_ND_INS_TYPE::ND_CAT_GFNI => Ok(Category::Gfni),
            ffi::_ND_INS_TYPE::ND_CAT_HRESET => Ok(Category::Hreset),
            ffi::_ND_INS_TYPE::ND_CAT_I386 => Ok(Category::I386),
            ffi::_ND_INS_TYPE::ND_CAT_IFMA => Ok(Category::Ifma),
            ffi::_ND_INS_TYPE::ND_CAT_INTERRUPT => Ok(Category::Interrupt),
            ffi::_ND_INS_TYPE::ND_CAT_IO => Ok(Category::Io),
            ffi::_ND_INS_TYPE::ND_CAT_IOSTRINGOP => Ok(Category::Iostringop),
            ffi::_ND_INS_TYPE::ND_CAT_KL => Ok(Category::Kl),
            ffi::_ND_INS_TYPE::ND_CAT_KMASK => Ok(Category::Kmask),
            ffi::_ND_INS_TYPE::ND_CAT_KNL => Ok(Category::Knl),
            ffi::_ND_INS_TYPE::ND_CAT_LKGS => Ok(Category::Lkgs),
            ffi::_ND_INS_TYPE::ND_CAT_LOGIC => Ok(Category::Logic),
            ffi::_ND_INS_TYPE::ND_CAT_LOGICAL => Ok(Category::Logical),
            ffi::_ND_INS_TYPE::ND_CAT_LOGICAL_FP => Ok(Category::LogicalFp),
            ffi::_ND_INS_TYPE::ND_CAT_LWP => Ok(Category::Lwp),
            ffi::_ND_INS_TYPE::ND_CAT_LZCNT => Ok(Category::Lzcnt),
            ffi::_ND_INS_TYPE::ND_CAT_MISC => Ok(Category::Misc),
            ffi::_ND_INS_TYPE::ND_CAT_MMX => Ok(Category::Mmx),
            ffi::_ND_INS_TYPE::ND_CAT_MOVDIR64B => Ok(Category::Movdir64b),
            ffi::_ND_INS_TYPE::ND_CAT_MOVDIRI => Ok(Category::Movdiri),
            ffi::_ND_INS_TYPE::ND_CAT_MPX => Ok(Category::Mpx),
            ffi::_ND_INS_TYPE::ND_CAT_NOP => Ok(Category::Nop),
            ffi::_ND_INS_TYPE::ND_CAT_PADLOCK => Ok(Category::Padlock),
            ffi::_ND_INS_TYPE::ND_CAT_PCLMULQDQ => Ok(Category::Pclmulqdq),
            ffi::_ND_INS_TYPE::ND_CAT_PCONFIG => Ok(Category::Pconfig),
            ffi::_ND_INS_TYPE::ND_CAT_POP => Ok(Category::Pop),
            ffi::_ND_INS_TYPE::ND_CAT_PREFETCH => Ok(Category::Prefetch),
            ffi::_ND_INS_TYPE::ND_CAT_PTWRITE => Ok(Category::Ptwrite),
            ffi::_ND_INS_TYPE::ND_CAT_PUSH => Ok(Category::Push),
            ffi::_ND_INS_TYPE::ND_CAT_RDPID => Ok(Category::Rdpid),
            ffi::_ND_INS_TYPE::ND_CAT_RDRAND => Ok(Category::Rdrand),
            ffi::_ND_INS_TYPE::ND_CAT_RDSEED => Ok(Category::Rdseed),
            ffi::_ND_INS_TYPE::ND_CAT_RDWRFSGS => Ok(Category::Rdwrfsgs),
            ffi::_ND_INS_TYPE::ND_CAT_RET => Ok(Category::Ret),
            ffi::_ND_INS_TYPE::ND_CAT_ROTATE => Ok(Category::Rotate),
            ffi::_ND_INS_TYPE::ND_CAT_SCATTER => Ok(Category::Scatter),
            ffi::_ND_INS_TYPE::ND_CAT_SEGOP => Ok(Category::Segop),
            ffi::_ND_INS_TYPE::ND_CAT_SEMAPHORE => Ok(Category::Semaphore),
            ffi::_ND_INS_TYPE::ND_CAT_SGX => Ok(Category::Sgx),
            ffi::_ND_INS_TYPE::ND_CAT_SHA => Ok(Category::Sha),
            ffi::_ND_INS_TYPE::ND_CAT_SHIFT => Ok(Category::Shift),
            ffi::_ND_INS_TYPE::ND_CAT_SMAP => Ok(Category::Smap),
            ffi::_ND_INS_TYPE::ND_CAT_SSE => Ok(Category::Sse),
            ffi::_ND_INS_TYPE::ND_CAT_SSE2 => Ok(Category::Sse2),
            ffi::_ND_INS_TYPE::ND_CAT_STRINGOP => Ok(Category::Stringop),
            ffi::_ND_INS_TYPE::ND_CAT_STTNI => Ok(Category::Sttni),
            ffi::_ND_INS_TYPE::ND_CAT_SYSCALL => Ok(Category::Syscall),
            ffi::_ND_INS_TYPE::ND_CAT_SYSRET => Ok(Category::Sysret),
            ffi::_ND_INS_TYPE::ND_CAT_SYSTEM => Ok(Category::System),
            ffi::_ND_INS_TYPE::ND_CAT_TDX => Ok(Category::Tdx),
            ffi::_ND_INS_TYPE::ND_CAT_UD => Ok(Category::Ud),
            ffi::_ND_INS_TYPE::ND_CAT_UINTR => Ok(Category::Uintr),
            ffi::_ND_INS_TYPE::ND_CAT_UNCOND_BR => Ok(Category::UncondBr),
            ffi::_ND_INS_TYPE::ND_CAT_UNKNOWN => Ok(Category::Unknown),
            ffi::_ND_INS_TYPE::ND_CAT_VAES => Ok(Category::Vaes),
            ffi::_ND_INS_TYPE::ND_CAT_VFMA => Ok(Category::Vfma),
            ffi::_ND_INS_TYPE::ND_CAT_VFMAPS => Ok(Category::Vfmaps),
            ffi::_ND_INS_TYPE::ND_CAT_VNNI => Ok(Category::Vnni),
            ffi::_ND_INS_TYPE::ND_CAT_VNNIW => Ok(Category::Vnniw),
            ffi::_ND_INS_TYPE::ND_CAT_VPCLMULQDQ => Ok(Category::Vpclmulqdq),
            ffi::_ND_INS_TYPE::ND_CAT_VPOPCNT => Ok(Category::Vpopcnt),
            ffi::_ND_INS_TYPE::ND_CAT_VTX => Ok(Category::Vtx),
            ffi::_ND_INS_TYPE::ND_CAT_WAITPKG => Ok(Category::Waitpkg),
            ffi::_ND_INS_TYPE::ND_CAT_WBNOINVD => Ok(Category::Wbnoinvd),
            ffi::_ND_INS_TYPE::ND_CAT_WIDENOP => Ok(Category::Widenop),
            ffi::_ND_INS_TYPE::ND_CAT_WIDE_KL => Ok(Category::WideKl),
            ffi::_ND_INS_TYPE::ND_CAT_X87_ALU => Ok(Category::X87Alu),
            ffi::_ND_INS_TYPE::ND_CAT_XOP => Ok(Category::Xop),
            ffi::_ND_INS_TYPE::ND_CAT_XSAVE => Ok(Category::Xsave),
        }
    }
}
