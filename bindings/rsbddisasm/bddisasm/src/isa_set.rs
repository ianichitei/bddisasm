/*
 * Copyright (c) 2021 Bitdefender
 * SPDX-License-Identifier: Apache-2.0
 */
//! Instruction sets.

use super::decode_error::DecodeError;
use core::convert::TryFrom;

/// ISA set.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum IsaSet {
    I3dnow,
    Adx,
    Aes,
    Amd,
    Amxbf16,
    Amxfp16,
    Amxint8,
    Amxtile,
    Avx,
    Avx2,
    Avx2gather,
    Avx5124fmaps,
    Avx5124vnniw,
    Avx512bf16,
    Avx512bitalg,
    Avx512bw,
    Avx512cd,
    Avx512dq,
    Avx512er,
    Avx512f,
    Avx512fp16,
    Avx512ifma,
    Avx512pf,
    Avx512vbmi,
    Avx512vbmi2,
    Avx512vnni,
    Avx512vp2intersect,
    Avx512vpopcntdq,
    Avxifma,
    Avxneconvert,
    Avxvnni,
    Avxvnniint8,
    Bmi1,
    Bmi2,
    CetIbt,
    CetSs,
    Cldemote,
    Clfsh,
    Clfshopt,
    Clwb,
    Clzero,
    Cmpccxadd,
    Cmpxchg16b,
    Cyrix,
    CyrixSmm,
    Enqcmd,
    F16c,
    Fma,
    Fma4,
    Fred,
    Fxsave,
    Gfni,
    Hreset,
    I186,
    I286prot,
    I286real,
    I386,
    I486,
    I486real,
    I64,
    I86,
    Invlpgb,
    Invpcid,
    Kl,
    Lkgs,
    Longmode,
    Lwp,
    Lzcnt,
    Mcommit,
    Mmx,
    Movbe,
    Movdir64b,
    Movdiri,
    Mpx,
    Msrlist,
    Mwaitt,
    Pause,
    Pclmulqdq,
    Pconfig,
    Pentiumreal,
    Pku,
    Popcnt,
    Ppro,
    Prefetchiti,
    PrefetchNop,
    Ptwrite,
    Raoint,
    Rdpid,
    Rdpmc,
    Rdpru,
    Rdrand,
    Rdseed,
    Rdtscp,
    Rdwrfsgs,
    Serialize,
    Sgx,
    Sha,
    Smap,
    Smx,
    Snp,
    Sse,
    Sse2,
    Sse3,
    Sse4,
    Sse42,
    Sse4a,
    Ssse3,
    Svm,
    Tbm,
    Tdx,
    Tsx,
    Tsxldtrk,
    Ud,
    Uintr,
    Unknown,
    Vaes,
    Vpclmulqdq,
    Vtx,
    Waitpkg,
    Wbnoinvd,
    Wrmsrns,
    X87,
    Xop,
    Xsave,
    Xsavec,
    Xsaves,
}

#[doc(hidden)]
impl TryFrom<ffi::ND_INS_SET> for IsaSet {
    type Error = DecodeError;

    fn try_from(value: ffi::ND_INS_SET) -> Result<Self, Self::Error> {
        match value {
            ffi::_ND_INS_SET::ND_SET_INVALID => Err(DecodeError::InternalError(value as u64)),
            ffi::_ND_INS_SET::ND_SET_3DNOW => Ok(IsaSet::I3dnow),
            ffi::_ND_INS_SET::ND_SET_ADX => Ok(IsaSet::Adx),
            ffi::_ND_INS_SET::ND_SET_AES => Ok(IsaSet::Aes),
            ffi::_ND_INS_SET::ND_SET_AMD => Ok(IsaSet::Amd),
            ffi::_ND_INS_SET::ND_SET_AMXBF16 => Ok(IsaSet::Amxbf16),
            ffi::_ND_INS_SET::ND_SET_AMXFP16 => Ok(IsaSet::Amxfp16),
            ffi::_ND_INS_SET::ND_SET_AMXINT8 => Ok(IsaSet::Amxint8),
            ffi::_ND_INS_SET::ND_SET_AMXTILE => Ok(IsaSet::Amxtile),
            ffi::_ND_INS_SET::ND_SET_AVX => Ok(IsaSet::Avx),
            ffi::_ND_INS_SET::ND_SET_AVX2 => Ok(IsaSet::Avx2),
            ffi::_ND_INS_SET::ND_SET_AVX2GATHER => Ok(IsaSet::Avx2gather),
            ffi::_ND_INS_SET::ND_SET_AVX5124FMAPS => Ok(IsaSet::Avx5124fmaps),
            ffi::_ND_INS_SET::ND_SET_AVX5124VNNIW => Ok(IsaSet::Avx5124vnniw),
            ffi::_ND_INS_SET::ND_SET_AVX512BF16 => Ok(IsaSet::Avx512bf16),
            ffi::_ND_INS_SET::ND_SET_AVX512BITALG => Ok(IsaSet::Avx512bitalg),
            ffi::_ND_INS_SET::ND_SET_AVX512BW => Ok(IsaSet::Avx512bw),
            ffi::_ND_INS_SET::ND_SET_AVX512CD => Ok(IsaSet::Avx512cd),
            ffi::_ND_INS_SET::ND_SET_AVX512DQ => Ok(IsaSet::Avx512dq),
            ffi::_ND_INS_SET::ND_SET_AVX512ER => Ok(IsaSet::Avx512er),
            ffi::_ND_INS_SET::ND_SET_AVX512F => Ok(IsaSet::Avx512f),
            ffi::_ND_INS_SET::ND_SET_AVX512FP16 => Ok(IsaSet::Avx512fp16),
            ffi::_ND_INS_SET::ND_SET_AVX512IFMA => Ok(IsaSet::Avx512ifma),
            ffi::_ND_INS_SET::ND_SET_AVX512PF => Ok(IsaSet::Avx512pf),
            ffi::_ND_INS_SET::ND_SET_AVX512VBMI => Ok(IsaSet::Avx512vbmi),
            ffi::_ND_INS_SET::ND_SET_AVX512VBMI2 => Ok(IsaSet::Avx512vbmi2),
            ffi::_ND_INS_SET::ND_SET_AVX512VNNI => Ok(IsaSet::Avx512vnni),
            ffi::_ND_INS_SET::ND_SET_AVX512VP2INTERSECT => Ok(IsaSet::Avx512vp2intersect),
            ffi::_ND_INS_SET::ND_SET_AVX512VPOPCNTDQ => Ok(IsaSet::Avx512vpopcntdq),
            ffi::_ND_INS_SET::ND_SET_AVXIFMA => Ok(IsaSet::Avxifma),
            ffi::_ND_INS_SET::ND_SET_AVXNECONVERT => Ok(IsaSet::Avxneconvert),
            ffi::_ND_INS_SET::ND_SET_AVXVNNI => Ok(IsaSet::Avxvnni),
            ffi::_ND_INS_SET::ND_SET_AVXVNNIINT8 => Ok(IsaSet::Avxvnniint8),
            ffi::_ND_INS_SET::ND_SET_BMI1 => Ok(IsaSet::Bmi1),
            ffi::_ND_INS_SET::ND_SET_BMI2 => Ok(IsaSet::Bmi2),
            ffi::_ND_INS_SET::ND_SET_CET_IBT => Ok(IsaSet::CetIbt),
            ffi::_ND_INS_SET::ND_SET_CET_SS => Ok(IsaSet::CetSs),
            ffi::_ND_INS_SET::ND_SET_CLDEMOTE => Ok(IsaSet::Cldemote),
            ffi::_ND_INS_SET::ND_SET_CLFSH => Ok(IsaSet::Clfsh),
            ffi::_ND_INS_SET::ND_SET_CLFSHOPT => Ok(IsaSet::Clfshopt),
            ffi::_ND_INS_SET::ND_SET_CLWB => Ok(IsaSet::Clwb),
            ffi::_ND_INS_SET::ND_SET_CLZERO => Ok(IsaSet::Clzero),
            ffi::_ND_INS_SET::ND_SET_CMPCCXADD => Ok(IsaSet::Cmpccxadd),
            ffi::_ND_INS_SET::ND_SET_CMPXCHG16B => Ok(IsaSet::Cmpxchg16b),
            ffi::_ND_INS_SET::ND_SET_CYRIX => Ok(IsaSet::Cyrix),
            ffi::_ND_INS_SET::ND_SET_CYRIX_SMM => Ok(IsaSet::CyrixSmm),
            ffi::_ND_INS_SET::ND_SET_ENQCMD => Ok(IsaSet::Enqcmd),
            ffi::_ND_INS_SET::ND_SET_F16C => Ok(IsaSet::F16c),
            ffi::_ND_INS_SET::ND_SET_FMA => Ok(IsaSet::Fma),
            ffi::_ND_INS_SET::ND_SET_FMA4 => Ok(IsaSet::Fma4),
            ffi::_ND_INS_SET::ND_SET_FRED => Ok(IsaSet::Fred),
            ffi::_ND_INS_SET::ND_SET_FXSAVE => Ok(IsaSet::Fxsave),
            ffi::_ND_INS_SET::ND_SET_GFNI => Ok(IsaSet::Gfni),
            ffi::_ND_INS_SET::ND_SET_HRESET => Ok(IsaSet::Hreset),
            ffi::_ND_INS_SET::ND_SET_I186 => Ok(IsaSet::I186),
            ffi::_ND_INS_SET::ND_SET_I286PROT => Ok(IsaSet::I286prot),
            ffi::_ND_INS_SET::ND_SET_I286REAL => Ok(IsaSet::I286real),
            ffi::_ND_INS_SET::ND_SET_I386 => Ok(IsaSet::I386),
            ffi::_ND_INS_SET::ND_SET_I486 => Ok(IsaSet::I486),
            ffi::_ND_INS_SET::ND_SET_I486REAL => Ok(IsaSet::I486real),
            ffi::_ND_INS_SET::ND_SET_I64 => Ok(IsaSet::I64),
            ffi::_ND_INS_SET::ND_SET_I86 => Ok(IsaSet::I86),
            ffi::_ND_INS_SET::ND_SET_INVLPGB => Ok(IsaSet::Invlpgb),
            ffi::_ND_INS_SET::ND_SET_INVPCID => Ok(IsaSet::Invpcid),
            ffi::_ND_INS_SET::ND_SET_KL => Ok(IsaSet::Kl),
            ffi::_ND_INS_SET::ND_SET_LKGS => Ok(IsaSet::Lkgs),
            ffi::_ND_INS_SET::ND_SET_LONGMODE => Ok(IsaSet::Longmode),
            ffi::_ND_INS_SET::ND_SET_LWP => Ok(IsaSet::Lwp),
            ffi::_ND_INS_SET::ND_SET_LZCNT => Ok(IsaSet::Lzcnt),
            ffi::_ND_INS_SET::ND_SET_MCOMMIT => Ok(IsaSet::Mcommit),
            ffi::_ND_INS_SET::ND_SET_MMX => Ok(IsaSet::Mmx),
            ffi::_ND_INS_SET::ND_SET_MOVBE => Ok(IsaSet::Movbe),
            ffi::_ND_INS_SET::ND_SET_MOVDIR64B => Ok(IsaSet::Movdir64b),
            ffi::_ND_INS_SET::ND_SET_MOVDIRI => Ok(IsaSet::Movdiri),
            ffi::_ND_INS_SET::ND_SET_MPX => Ok(IsaSet::Mpx),
            ffi::_ND_INS_SET::ND_SET_MSRLIST => Ok(IsaSet::Msrlist),
            ffi::_ND_INS_SET::ND_SET_MWAITT => Ok(IsaSet::Mwaitt),
            ffi::_ND_INS_SET::ND_SET_PAUSE => Ok(IsaSet::Pause),
            ffi::_ND_INS_SET::ND_SET_PCLMULQDQ => Ok(IsaSet::Pclmulqdq),
            ffi::_ND_INS_SET::ND_SET_PCONFIG => Ok(IsaSet::Pconfig),
            ffi::_ND_INS_SET::ND_SET_PENTIUMREAL => Ok(IsaSet::Pentiumreal),
            ffi::_ND_INS_SET::ND_SET_PKU => Ok(IsaSet::Pku),
            ffi::_ND_INS_SET::ND_SET_POPCNT => Ok(IsaSet::Popcnt),
            ffi::_ND_INS_SET::ND_SET_PPRO => Ok(IsaSet::Ppro),
            ffi::_ND_INS_SET::ND_SET_PREFETCHITI => Ok(IsaSet::Prefetchiti),
            ffi::_ND_INS_SET::ND_SET_PREFETCH_NOP => Ok(IsaSet::PrefetchNop),
            ffi::_ND_INS_SET::ND_SET_PTWRITE => Ok(IsaSet::Ptwrite),
            ffi::_ND_INS_SET::ND_SET_RAOINT => Ok(IsaSet::Raoint),
            ffi::_ND_INS_SET::ND_SET_RDPID => Ok(IsaSet::Rdpid),
            ffi::_ND_INS_SET::ND_SET_RDPMC => Ok(IsaSet::Rdpmc),
            ffi::_ND_INS_SET::ND_SET_RDPRU => Ok(IsaSet::Rdpru),
            ffi::_ND_INS_SET::ND_SET_RDRAND => Ok(IsaSet::Rdrand),
            ffi::_ND_INS_SET::ND_SET_RDSEED => Ok(IsaSet::Rdseed),
            ffi::_ND_INS_SET::ND_SET_RDTSCP => Ok(IsaSet::Rdtscp),
            ffi::_ND_INS_SET::ND_SET_RDWRFSGS => Ok(IsaSet::Rdwrfsgs),
            ffi::_ND_INS_SET::ND_SET_SERIALIZE => Ok(IsaSet::Serialize),
            ffi::_ND_INS_SET::ND_SET_SGX => Ok(IsaSet::Sgx),
            ffi::_ND_INS_SET::ND_SET_SHA => Ok(IsaSet::Sha),
            ffi::_ND_INS_SET::ND_SET_SMAP => Ok(IsaSet::Smap),
            ffi::_ND_INS_SET::ND_SET_SMX => Ok(IsaSet::Smx),
            ffi::_ND_INS_SET::ND_SET_SNP => Ok(IsaSet::Snp),
            ffi::_ND_INS_SET::ND_SET_SSE => Ok(IsaSet::Sse),
            ffi::_ND_INS_SET::ND_SET_SSE2 => Ok(IsaSet::Sse2),
            ffi::_ND_INS_SET::ND_SET_SSE3 => Ok(IsaSet::Sse3),
            ffi::_ND_INS_SET::ND_SET_SSE4 => Ok(IsaSet::Sse4),
            ffi::_ND_INS_SET::ND_SET_SSE42 => Ok(IsaSet::Sse42),
            ffi::_ND_INS_SET::ND_SET_SSE4A => Ok(IsaSet::Sse4a),
            ffi::_ND_INS_SET::ND_SET_SSSE3 => Ok(IsaSet::Ssse3),
            ffi::_ND_INS_SET::ND_SET_SVM => Ok(IsaSet::Svm),
            ffi::_ND_INS_SET::ND_SET_TBM => Ok(IsaSet::Tbm),
            ffi::_ND_INS_SET::ND_SET_TDX => Ok(IsaSet::Tdx),
            ffi::_ND_INS_SET::ND_SET_TSX => Ok(IsaSet::Tsx),
            ffi::_ND_INS_SET::ND_SET_TSXLDTRK => Ok(IsaSet::Tsxldtrk),
            ffi::_ND_INS_SET::ND_SET_UD => Ok(IsaSet::Ud),
            ffi::_ND_INS_SET::ND_SET_UINTR => Ok(IsaSet::Uintr),
            ffi::_ND_INS_SET::ND_SET_UNKNOWN => Ok(IsaSet::Unknown),
            ffi::_ND_INS_SET::ND_SET_VAES => Ok(IsaSet::Vaes),
            ffi::_ND_INS_SET::ND_SET_VPCLMULQDQ => Ok(IsaSet::Vpclmulqdq),
            ffi::_ND_INS_SET::ND_SET_VTX => Ok(IsaSet::Vtx),
            ffi::_ND_INS_SET::ND_SET_WAITPKG => Ok(IsaSet::Waitpkg),
            ffi::_ND_INS_SET::ND_SET_WBNOINVD => Ok(IsaSet::Wbnoinvd),
            ffi::_ND_INS_SET::ND_SET_WRMSRNS => Ok(IsaSet::Wrmsrns),
            ffi::_ND_INS_SET::ND_SET_X87 => Ok(IsaSet::X87),
            ffi::_ND_INS_SET::ND_SET_XOP => Ok(IsaSet::Xop),
            ffi::_ND_INS_SET::ND_SET_XSAVE => Ok(IsaSet::Xsave),
            ffi::_ND_INS_SET::ND_SET_XSAVEC => Ok(IsaSet::Xsavec),
            ffi::_ND_INS_SET::ND_SET_XSAVES => Ok(IsaSet::Xsaves),
        }
    }
}
