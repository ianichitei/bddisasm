/*
 * Copyright (c) 2021 Bitdefender
 * SPDX-License-Identifier: Apache-2.0
 */
//! Instruction sets.

use super::decode_error::DecodeError;
use core::convert::TryFrom;

/// ISA set.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
#[allow(non_camel_case_types)]
pub enum IsaSet {
    I3DNOW,
    ADX,
    AES,
    AMD,
    AMXBF16,
    AMXCOMPLEX,
    AMXFP16,
    AMXINT8,
    AMXTILE,
    APX_F,
    AVX,
    AVX102,
    AVX2,
    AVX2GATHER,
    AVX5124FMAPS,
    AVX5124VNNIW,
    AVX512BF16,
    AVX512BITALG,
    AVX512BW,
    AVX512CD,
    AVX512DQ,
    AVX512ER,
    AVX512F,
    AVX512FP16,
    AVX512IFMA,
    AVX512PF,
    AVX512VBMI,
    AVX512VBMI2,
    AVX512VNNI,
    AVX512VP2INTERSECT,
    AVX512VPOPCNTDQ,
    AVXIFMA,
    AVXNECONVERT,
    AVXVNNI,
    AVXVNNIINT16,
    AVXVNNIINT8,
    BMI1,
    BMI2,
    CET_IBT,
    CET_SS,
    CLDEMOTE,
    CLFSH,
    CLFSHOPT,
    CLWB,
    CLZERO,
    CMPCCXADD,
    CMPXCHG16B,
    ENQCMD,
    F16C,
    FMA,
    FMA4,
    FRED,
    FXSAVE,
    GFNI,
    HRESET,
    I186,
    I286PROT,
    I286REAL,
    I386,
    I486,
    I486REAL,
    I64,
    I86,
    INVLPGB,
    INVPCID,
    KL,
    LKGS,
    LONGMODE,
    LWP,
    LZCNT,
    MCOMMIT,
    MMX,
    MOVBE,
    MOVDIR64B,
    MOVDIRI,
    MPX,
    MSRLIST,
    MWAITT,
    PAUSE,
    PCLMULQDQ,
    PCONFIG,
    PENTIUMREAL,
    PKU,
    POPCNT,
    PPRO,
    PREFETCHITI,
    PREFETCH_NOP,
    PTWRITE,
    RAOINT,
    RDPID,
    RDPMC,
    RDPRU,
    RDRAND,
    RDSEED,
    RDTSCP,
    RDWRFSGS,
    SERIALIZE,
    SGX,
    SHA,
    SHA512,
    SM3,
    SM4,
    SMAP,
    SMX,
    SNP,
    SSE,
    SSE2,
    SSE3,
    SSE4,
    SSE42,
    SSE4A,
    SSSE3,
    SVM,
    TBM,
    TDX,
    TSE,
    TSX,
    TSXLDTRK,
    UD,
    UINTR,
    UNKNOWN,
    USER_MSR,
    VAES,
    VPCLMULQDQ,
    VTX,
    WAITPKG,
    WBNOINVD,
    WRMSRNS,
    X87,
    XOP,
    XSAVE,
    XSAVEC,
    XSAVES,
}

#[doc(hidden)]
impl TryFrom<ffi::ND_INS_SET> for IsaSet {
    type Error = DecodeError;

    #[allow(clippy::too_many_lines)]
    fn try_from(value: ffi::ND_INS_SET) -> Result<Self, Self::Error> {
        match value {
            ffi::_ND_INS_SET::ND_SET_INVALID => Err(DecodeError::InternalError(value as u64)),
            ffi::_ND_INS_SET::ND_SET_3DNOW => Ok(IsaSet::I3DNOW),
            ffi::_ND_INS_SET::ND_SET_ADX => Ok(IsaSet::ADX),
            ffi::_ND_INS_SET::ND_SET_AES => Ok(IsaSet::AES),
            ffi::_ND_INS_SET::ND_SET_AMD => Ok(IsaSet::AMD),
            ffi::_ND_INS_SET::ND_SET_AMXBF16 => Ok(IsaSet::AMXBF16),
            ffi::_ND_INS_SET::ND_SET_AMXCOMPLEX => Ok(IsaSet::AMXCOMPLEX),
            ffi::_ND_INS_SET::ND_SET_AMXFP16 => Ok(IsaSet::AMXFP16),
            ffi::_ND_INS_SET::ND_SET_AMXINT8 => Ok(IsaSet::AMXINT8),
            ffi::_ND_INS_SET::ND_SET_AMXTILE => Ok(IsaSet::AMXTILE),
            ffi::_ND_INS_SET::ND_SET_APX_F => Ok(IsaSet::APX_F),
            ffi::_ND_INS_SET::ND_SET_AVX => Ok(IsaSet::AVX),
            ffi::_ND_INS_SET::ND_SET_AVX102 => Ok(IsaSet::AVX102),
            ffi::_ND_INS_SET::ND_SET_AVX2 => Ok(IsaSet::AVX2),
            ffi::_ND_INS_SET::ND_SET_AVX2GATHER => Ok(IsaSet::AVX2GATHER),
            ffi::_ND_INS_SET::ND_SET_AVX5124FMAPS => Ok(IsaSet::AVX5124FMAPS),
            ffi::_ND_INS_SET::ND_SET_AVX5124VNNIW => Ok(IsaSet::AVX5124VNNIW),
            ffi::_ND_INS_SET::ND_SET_AVX512BF16 => Ok(IsaSet::AVX512BF16),
            ffi::_ND_INS_SET::ND_SET_AVX512BITALG => Ok(IsaSet::AVX512BITALG),
            ffi::_ND_INS_SET::ND_SET_AVX512BW => Ok(IsaSet::AVX512BW),
            ffi::_ND_INS_SET::ND_SET_AVX512CD => Ok(IsaSet::AVX512CD),
            ffi::_ND_INS_SET::ND_SET_AVX512DQ => Ok(IsaSet::AVX512DQ),
            ffi::_ND_INS_SET::ND_SET_AVX512ER => Ok(IsaSet::AVX512ER),
            ffi::_ND_INS_SET::ND_SET_AVX512F => Ok(IsaSet::AVX512F),
            ffi::_ND_INS_SET::ND_SET_AVX512FP16 => Ok(IsaSet::AVX512FP16),
            ffi::_ND_INS_SET::ND_SET_AVX512IFMA => Ok(IsaSet::AVX512IFMA),
            ffi::_ND_INS_SET::ND_SET_AVX512PF => Ok(IsaSet::AVX512PF),
            ffi::_ND_INS_SET::ND_SET_AVX512VBMI => Ok(IsaSet::AVX512VBMI),
            ffi::_ND_INS_SET::ND_SET_AVX512VBMI2 => Ok(IsaSet::AVX512VBMI2),
            ffi::_ND_INS_SET::ND_SET_AVX512VNNI => Ok(IsaSet::AVX512VNNI),
            ffi::_ND_INS_SET::ND_SET_AVX512VP2INTERSECT => Ok(IsaSet::AVX512VP2INTERSECT),
            ffi::_ND_INS_SET::ND_SET_AVX512VPOPCNTDQ => Ok(IsaSet::AVX512VPOPCNTDQ),
            ffi::_ND_INS_SET::ND_SET_AVXIFMA => Ok(IsaSet::AVXIFMA),
            ffi::_ND_INS_SET::ND_SET_AVXNECONVERT => Ok(IsaSet::AVXNECONVERT),
            ffi::_ND_INS_SET::ND_SET_AVXVNNI => Ok(IsaSet::AVXVNNI),
            ffi::_ND_INS_SET::ND_SET_AVXVNNIINT16 => Ok(IsaSet::AVXVNNIINT16),
            ffi::_ND_INS_SET::ND_SET_AVXVNNIINT8 => Ok(IsaSet::AVXVNNIINT8),
            ffi::_ND_INS_SET::ND_SET_BMI1 => Ok(IsaSet::BMI1),
            ffi::_ND_INS_SET::ND_SET_BMI2 => Ok(IsaSet::BMI2),
            ffi::_ND_INS_SET::ND_SET_CET_IBT => Ok(IsaSet::CET_IBT),
            ffi::_ND_INS_SET::ND_SET_CET_SS => Ok(IsaSet::CET_SS),
            ffi::_ND_INS_SET::ND_SET_CLDEMOTE => Ok(IsaSet::CLDEMOTE),
            ffi::_ND_INS_SET::ND_SET_CLFSH => Ok(IsaSet::CLFSH),
            ffi::_ND_INS_SET::ND_SET_CLFSHOPT => Ok(IsaSet::CLFSHOPT),
            ffi::_ND_INS_SET::ND_SET_CLWB => Ok(IsaSet::CLWB),
            ffi::_ND_INS_SET::ND_SET_CLZERO => Ok(IsaSet::CLZERO),
            ffi::_ND_INS_SET::ND_SET_CMPCCXADD => Ok(IsaSet::CMPCCXADD),
            ffi::_ND_INS_SET::ND_SET_CMPXCHG16B => Ok(IsaSet::CMPXCHG16B),
            ffi::_ND_INS_SET::ND_SET_ENQCMD => Ok(IsaSet::ENQCMD),
            ffi::_ND_INS_SET::ND_SET_F16C => Ok(IsaSet::F16C),
            ffi::_ND_INS_SET::ND_SET_FMA => Ok(IsaSet::FMA),
            ffi::_ND_INS_SET::ND_SET_FMA4 => Ok(IsaSet::FMA4),
            ffi::_ND_INS_SET::ND_SET_FRED => Ok(IsaSet::FRED),
            ffi::_ND_INS_SET::ND_SET_FXSAVE => Ok(IsaSet::FXSAVE),
            ffi::_ND_INS_SET::ND_SET_GFNI => Ok(IsaSet::GFNI),
            ffi::_ND_INS_SET::ND_SET_HRESET => Ok(IsaSet::HRESET),
            ffi::_ND_INS_SET::ND_SET_I186 => Ok(IsaSet::I186),
            ffi::_ND_INS_SET::ND_SET_I286PROT => Ok(IsaSet::I286PROT),
            ffi::_ND_INS_SET::ND_SET_I286REAL => Ok(IsaSet::I286REAL),
            ffi::_ND_INS_SET::ND_SET_I386 => Ok(IsaSet::I386),
            ffi::_ND_INS_SET::ND_SET_I486 => Ok(IsaSet::I486),
            ffi::_ND_INS_SET::ND_SET_I486REAL => Ok(IsaSet::I486REAL),
            ffi::_ND_INS_SET::ND_SET_I64 => Ok(IsaSet::I64),
            ffi::_ND_INS_SET::ND_SET_I86 => Ok(IsaSet::I86),
            ffi::_ND_INS_SET::ND_SET_INVLPGB => Ok(IsaSet::INVLPGB),
            ffi::_ND_INS_SET::ND_SET_INVPCID => Ok(IsaSet::INVPCID),
            ffi::_ND_INS_SET::ND_SET_KL => Ok(IsaSet::KL),
            ffi::_ND_INS_SET::ND_SET_LKGS => Ok(IsaSet::LKGS),
            ffi::_ND_INS_SET::ND_SET_LONGMODE => Ok(IsaSet::LONGMODE),
            ffi::_ND_INS_SET::ND_SET_LWP => Ok(IsaSet::LWP),
            ffi::_ND_INS_SET::ND_SET_LZCNT => Ok(IsaSet::LZCNT),
            ffi::_ND_INS_SET::ND_SET_MCOMMIT => Ok(IsaSet::MCOMMIT),
            ffi::_ND_INS_SET::ND_SET_MMX => Ok(IsaSet::MMX),
            ffi::_ND_INS_SET::ND_SET_MOVBE => Ok(IsaSet::MOVBE),
            ffi::_ND_INS_SET::ND_SET_MOVDIR64B => Ok(IsaSet::MOVDIR64B),
            ffi::_ND_INS_SET::ND_SET_MOVDIRI => Ok(IsaSet::MOVDIRI),
            ffi::_ND_INS_SET::ND_SET_MPX => Ok(IsaSet::MPX),
            ffi::_ND_INS_SET::ND_SET_MSRLIST => Ok(IsaSet::MSRLIST),
            ffi::_ND_INS_SET::ND_SET_MWAITT => Ok(IsaSet::MWAITT),
            ffi::_ND_INS_SET::ND_SET_PAUSE => Ok(IsaSet::PAUSE),
            ffi::_ND_INS_SET::ND_SET_PCLMULQDQ => Ok(IsaSet::PCLMULQDQ),
            ffi::_ND_INS_SET::ND_SET_PCONFIG => Ok(IsaSet::PCONFIG),
            ffi::_ND_INS_SET::ND_SET_PENTIUMREAL => Ok(IsaSet::PENTIUMREAL),
            ffi::_ND_INS_SET::ND_SET_PKU => Ok(IsaSet::PKU),
            ffi::_ND_INS_SET::ND_SET_POPCNT => Ok(IsaSet::POPCNT),
            ffi::_ND_INS_SET::ND_SET_PPRO => Ok(IsaSet::PPRO),
            ffi::_ND_INS_SET::ND_SET_PREFETCHITI => Ok(IsaSet::PREFETCHITI),
            ffi::_ND_INS_SET::ND_SET_PREFETCH_NOP => Ok(IsaSet::PREFETCH_NOP),
            ffi::_ND_INS_SET::ND_SET_PTWRITE => Ok(IsaSet::PTWRITE),
            ffi::_ND_INS_SET::ND_SET_RAOINT => Ok(IsaSet::RAOINT),
            ffi::_ND_INS_SET::ND_SET_RDPID => Ok(IsaSet::RDPID),
            ffi::_ND_INS_SET::ND_SET_RDPMC => Ok(IsaSet::RDPMC),
            ffi::_ND_INS_SET::ND_SET_RDPRU => Ok(IsaSet::RDPRU),
            ffi::_ND_INS_SET::ND_SET_RDRAND => Ok(IsaSet::RDRAND),
            ffi::_ND_INS_SET::ND_SET_RDSEED => Ok(IsaSet::RDSEED),
            ffi::_ND_INS_SET::ND_SET_RDTSCP => Ok(IsaSet::RDTSCP),
            ffi::_ND_INS_SET::ND_SET_RDWRFSGS => Ok(IsaSet::RDWRFSGS),
            ffi::_ND_INS_SET::ND_SET_SERIALIZE => Ok(IsaSet::SERIALIZE),
            ffi::_ND_INS_SET::ND_SET_SGX => Ok(IsaSet::SGX),
            ffi::_ND_INS_SET::ND_SET_SHA => Ok(IsaSet::SHA),
            ffi::_ND_INS_SET::ND_SET_SHA512 => Ok(IsaSet::SHA512),
            ffi::_ND_INS_SET::ND_SET_SM3 => Ok(IsaSet::SM3),
            ffi::_ND_INS_SET::ND_SET_SM4 => Ok(IsaSet::SM4),
            ffi::_ND_INS_SET::ND_SET_SMAP => Ok(IsaSet::SMAP),
            ffi::_ND_INS_SET::ND_SET_SMX => Ok(IsaSet::SMX),
            ffi::_ND_INS_SET::ND_SET_SNP => Ok(IsaSet::SNP),
            ffi::_ND_INS_SET::ND_SET_SSE => Ok(IsaSet::SSE),
            ffi::_ND_INS_SET::ND_SET_SSE2 => Ok(IsaSet::SSE2),
            ffi::_ND_INS_SET::ND_SET_SSE3 => Ok(IsaSet::SSE3),
            ffi::_ND_INS_SET::ND_SET_SSE4 => Ok(IsaSet::SSE4),
            ffi::_ND_INS_SET::ND_SET_SSE42 => Ok(IsaSet::SSE42),
            ffi::_ND_INS_SET::ND_SET_SSE4A => Ok(IsaSet::SSE4A),
            ffi::_ND_INS_SET::ND_SET_SSSE3 => Ok(IsaSet::SSSE3),
            ffi::_ND_INS_SET::ND_SET_SVM => Ok(IsaSet::SVM),
            ffi::_ND_INS_SET::ND_SET_TBM => Ok(IsaSet::TBM),
            ffi::_ND_INS_SET::ND_SET_TDX => Ok(IsaSet::TDX),
            ffi::_ND_INS_SET::ND_SET_TSE => Ok(IsaSet::TSE),
            ffi::_ND_INS_SET::ND_SET_TSX => Ok(IsaSet::TSX),
            ffi::_ND_INS_SET::ND_SET_TSXLDTRK => Ok(IsaSet::TSXLDTRK),
            ffi::_ND_INS_SET::ND_SET_UD => Ok(IsaSet::UD),
            ffi::_ND_INS_SET::ND_SET_UINTR => Ok(IsaSet::UINTR),
            ffi::_ND_INS_SET::ND_SET_UNKNOWN => Ok(IsaSet::UNKNOWN),
            ffi::_ND_INS_SET::ND_SET_USER_MSR => Ok(IsaSet::USER_MSR),
            ffi::_ND_INS_SET::ND_SET_VAES => Ok(IsaSet::VAES),
            ffi::_ND_INS_SET::ND_SET_VPCLMULQDQ => Ok(IsaSet::VPCLMULQDQ),
            ffi::_ND_INS_SET::ND_SET_VTX => Ok(IsaSet::VTX),
            ffi::_ND_INS_SET::ND_SET_WAITPKG => Ok(IsaSet::WAITPKG),
            ffi::_ND_INS_SET::ND_SET_WBNOINVD => Ok(IsaSet::WBNOINVD),
            ffi::_ND_INS_SET::ND_SET_WRMSRNS => Ok(IsaSet::WRMSRNS),
            ffi::_ND_INS_SET::ND_SET_X87 => Ok(IsaSet::X87),
            ffi::_ND_INS_SET::ND_SET_XOP => Ok(IsaSet::XOP),
            ffi::_ND_INS_SET::ND_SET_XSAVE => Ok(IsaSet::XSAVE),
            ffi::_ND_INS_SET::ND_SET_XSAVEC => Ok(IsaSet::XSAVEC),
            ffi::_ND_INS_SET::ND_SET_XSAVES => Ok(IsaSet::XSAVES),
        }
    }
}
