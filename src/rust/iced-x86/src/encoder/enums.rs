// SPDX-License-Identifier: MIT
// Copyright (C) 2018-present iced project and contributors

use crate::encoder::iced_constants::IcedConstants;
use crate::encoder::iced_error::IcedError;
use core::iter::{ExactSizeIterator, FusedIterator, Iterator};
use core::{fmt, mem};

// GENERATOR-BEGIN: DisplSize
// ⚠️This was generated by GENERATOR!🦹‍♂️
#[derive(Copy, Clone, Eq, PartialEq)]
#[allow(non_camel_case_types)]
#[allow(dead_code)]
pub(crate) enum DisplSize {
	None,
	Size1,
	Size2,
	Size4,
	Size8,
	RipRelSize4_Target32,
	RipRelSize4_Target64,
}
#[rustfmt::skip]
static GEN_DEBUG_DISPL_SIZE: [&str; 7] = [
	"None",
	"Size1",
	"Size2",
	"Size4",
	"Size8",
	"RipRelSize4_Target32",
	"RipRelSize4_Target64",
];
impl fmt::Debug for DisplSize {
	#[inline]
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", GEN_DEBUG_DISPL_SIZE[*self as usize])
	}
}
impl Default for DisplSize {
	#[inline]
	fn default() -> Self {
		DisplSize::None
	}
}
// GENERATOR-END: DisplSize

// GENERATOR-BEGIN: ImmSize
// ⚠️This was generated by GENERATOR!🦹‍♂️
#[derive(Copy, Clone, Eq, PartialEq)]
#[allow(non_camel_case_types)]
#[allow(dead_code)]
pub(crate) enum ImmSize {
	None,
	Size1,
	Size2,
	Size4,
	Size8,
	/// `ENTER xxxx,yy`
	Size2_1,
	/// `EXTRQ/INSERTQ xx,yy`
	Size1_1,
	/// `CALL16 FAR x:y`
	Size2_2,
	/// `CALL32 FAR x:y`
	Size4_2,
	RipRelSize1_Target16,
	RipRelSize1_Target32,
	RipRelSize1_Target64,
	RipRelSize2_Target16,
	RipRelSize2_Target32,
	RipRelSize2_Target64,
	RipRelSize4_Target32,
	RipRelSize4_Target64,
	SizeIbReg,
	Size1OpCode,
}
#[rustfmt::skip]
static GEN_DEBUG_IMM_SIZE: [&str; 19] = [
	"None",
	"Size1",
	"Size2",
	"Size4",
	"Size8",
	"Size2_1",
	"Size1_1",
	"Size2_2",
	"Size4_2",
	"RipRelSize1_Target16",
	"RipRelSize1_Target32",
	"RipRelSize1_Target64",
	"RipRelSize2_Target16",
	"RipRelSize2_Target32",
	"RipRelSize2_Target64",
	"RipRelSize4_Target32",
	"RipRelSize4_Target64",
	"SizeIbReg",
	"Size1OpCode",
];
impl fmt::Debug for ImmSize {
	#[inline]
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", GEN_DEBUG_IMM_SIZE[*self as usize])
	}
}
impl Default for ImmSize {
	#[inline]
	fn default() -> Self {
		ImmSize::None
	}
}
// GENERATOR-END: ImmSize

// GENERATOR-BEGIN: EncoderFlags
// ⚠️This was generated by GENERATOR!🦹‍♂️
pub(crate) struct EncoderFlags;
#[allow(dead_code)]
impl EncoderFlags {
	pub(crate) const NONE: u32 = 0x0000_0000;
	pub(crate) const B: u32 = 0x0000_0001;
	pub(crate) const X: u32 = 0x0000_0002;
	pub(crate) const R: u32 = 0x0000_0004;
	pub(crate) const W: u32 = 0x0000_0008;
	pub(crate) const MOD_RM: u32 = 0x0000_0010;
	pub(crate) const SIB: u32 = 0x0000_0020;
	pub(crate) const REX: u32 = 0x0000_0040;
	pub(crate) const P66: u32 = 0x0000_0080;
	pub(crate) const P67: u32 = 0x0000_0100;
	/// `EVEX.R'`
	pub(crate) const R2: u32 = 0x0000_0200;
	pub(crate) const BROADCAST: u32 = 0x0000_0400;
	pub(crate) const HIGH_LEGACY_8_BIT_REGS: u32 = 0x0000_0800;
	pub(crate) const DISPL: u32 = 0x0000_1000;
	pub(crate) const PF0: u32 = 0x0000_2000;
	pub(crate) const REG_IS_MEMORY: u32 = 0x0000_4000;
	pub(crate) const MUST_USE_SIB: u32 = 0x0000_8000;
	pub(crate) const VVVVV_SHIFT: u32 = 0x0000_001B;
	pub(crate) const VVVVV_MASK: u32 = 0x0000_001F;
}
// GENERATOR-END: EncoderFlags

// GENERATOR-BEGIN: LegacyOpCodeTable
// ⚠️This was generated by GENERATOR!🦹‍♂️
#[derive(Copy, Clone, Eq, PartialEq)]
#[allow(non_camel_case_types)]
#[allow(dead_code)]
pub(crate) enum LegacyOpCodeTable {
	MAP0,
	MAP0F,
	MAP0F38,
	MAP0F3A,
}
#[rustfmt::skip]
static GEN_DEBUG_LEGACY_OP_CODE_TABLE: [&str; 4] = [
	"MAP0",
	"MAP0F",
	"MAP0F38",
	"MAP0F3A",
];
impl fmt::Debug for LegacyOpCodeTable {
	#[inline]
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", GEN_DEBUG_LEGACY_OP_CODE_TABLE[*self as usize])
	}
}
impl Default for LegacyOpCodeTable {
	#[inline]
	fn default() -> Self {
		LegacyOpCodeTable::MAP0
	}
}
// GENERATOR-END: LegacyOpCodeTable

// GENERATOR-BEGIN: VexOpCodeTable
// ⚠️This was generated by GENERATOR!🦹‍♂️
#[derive(Copy, Clone, Eq, PartialEq)]
#[allow(non_camel_case_types)]
#[cfg(not(feature = "no_vex"))]
#[allow(dead_code)]
pub(crate) enum VexOpCodeTable {
	MAP0,
	MAP0F,
	MAP0F38,
	MAP0F3A,
}
#[cfg(not(feature = "no_vex"))]
#[rustfmt::skip]
static GEN_DEBUG_VEX_OP_CODE_TABLE: [&str; 4] = [
	"MAP0",
	"MAP0F",
	"MAP0F38",
	"MAP0F3A",
];
#[cfg(not(feature = "no_vex"))]
impl fmt::Debug for VexOpCodeTable {
	#[inline]
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", GEN_DEBUG_VEX_OP_CODE_TABLE[*self as usize])
	}
}
#[cfg(not(feature = "no_vex"))]
impl Default for VexOpCodeTable {
	#[inline]
	fn default() -> Self {
		VexOpCodeTable::MAP0
	}
}
// GENERATOR-END: VexOpCodeTable

// GENERATOR-BEGIN: XopOpCodeTable
// ⚠️This was generated by GENERATOR!🦹‍♂️
#[derive(Copy, Clone, Eq, PartialEq)]
#[allow(non_camel_case_types)]
#[cfg(not(feature = "no_xop"))]
#[allow(dead_code)]
pub(crate) enum XopOpCodeTable {
	MAP8,
	MAP9,
	MAP10,
}
#[cfg(not(feature = "no_xop"))]
#[rustfmt::skip]
static GEN_DEBUG_XOP_OP_CODE_TABLE: [&str; 3] = [
	"MAP8",
	"MAP9",
	"MAP10",
];
#[cfg(not(feature = "no_xop"))]
impl fmt::Debug for XopOpCodeTable {
	#[inline]
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", GEN_DEBUG_XOP_OP_CODE_TABLE[*self as usize])
	}
}
#[cfg(not(feature = "no_xop"))]
impl Default for XopOpCodeTable {
	#[inline]
	fn default() -> Self {
		XopOpCodeTable::MAP8
	}
}
// GENERATOR-END: XopOpCodeTable

// GENERATOR-BEGIN: EvexOpCodeTable
// ⚠️This was generated by GENERATOR!🦹‍♂️
#[derive(Copy, Clone, Eq, PartialEq)]
#[allow(non_camel_case_types)]
#[cfg(not(feature = "no_evex"))]
#[allow(dead_code)]
pub(crate) enum EvexOpCodeTable {
	MAP0F = 1,
	MAP0F38,
	MAP0F3A,
	MAP5 = 5,
	MAP6,
}
#[cfg(not(feature = "no_evex"))]
impl Default for EvexOpCodeTable {
	#[inline]
	fn default() -> Self {
		EvexOpCodeTable::MAP0F
	}
}
// GENERATOR-END: EvexOpCodeTable

// GENERATOR-BEGIN: MvexOpCodeTable
// ⚠️This was generated by GENERATOR!🦹‍♂️
#[derive(Copy, Clone, Eq, PartialEq)]
#[allow(non_camel_case_types)]
#[cfg(feature = "mvex")]
#[allow(dead_code)]
pub(crate) enum MvexOpCodeTable {
	MAP0F = 1,
	MAP0F38,
	MAP0F3A,
}
#[cfg(feature = "mvex")]
impl Default for MvexOpCodeTable {
	#[inline]
	fn default() -> Self {
		MvexOpCodeTable::MAP0F
	}
}
// GENERATOR-END: MvexOpCodeTable

// GENERATOR-BEGIN: EncFlags1
// ⚠️This was generated by GENERATOR!🦹‍♂️
pub(crate) struct EncFlags1;
#[allow(dead_code)]
impl EncFlags1 {
	pub(crate) const NONE: u32 = 0x0000_0000;
	pub(crate) const LEGACY_OP_MASK: u32 = 0x0000_007F;
	pub(crate) const LEGACY_OP0_SHIFT: u32 = 0x0000_0000;
	pub(crate) const LEGACY_OP1_SHIFT: u32 = 0x0000_0007;
	pub(crate) const LEGACY_OP2_SHIFT: u32 = 0x0000_000E;
	pub(crate) const LEGACY_OP3_SHIFT: u32 = 0x0000_0015;
	pub(crate) const VEX_OP_MASK: u32 = 0x0000_003F;
	pub(crate) const VEX_OP0_SHIFT: u32 = 0x0000_0000;
	pub(crate) const VEX_OP1_SHIFT: u32 = 0x0000_0006;
	pub(crate) const VEX_OP2_SHIFT: u32 = 0x0000_000C;
	pub(crate) const VEX_OP3_SHIFT: u32 = 0x0000_0012;
	pub(crate) const VEX_OP4_SHIFT: u32 = 0x0000_0018;
	pub(crate) const XOP_OP_MASK: u32 = 0x0000_001F;
	pub(crate) const XOP_OP0_SHIFT: u32 = 0x0000_0000;
	pub(crate) const XOP_OP1_SHIFT: u32 = 0x0000_0005;
	pub(crate) const XOP_OP2_SHIFT: u32 = 0x0000_000A;
	pub(crate) const XOP_OP3_SHIFT: u32 = 0x0000_000F;
	pub(crate) const EVEX_OP_MASK: u32 = 0x0000_001F;
	pub(crate) const EVEX_OP0_SHIFT: u32 = 0x0000_0000;
	pub(crate) const EVEX_OP1_SHIFT: u32 = 0x0000_0005;
	pub(crate) const EVEX_OP2_SHIFT: u32 = 0x0000_000A;
	pub(crate) const EVEX_OP3_SHIFT: u32 = 0x0000_000F;
	pub(crate) const MVEX_OP_MASK: u32 = 0x0000_000F;
	pub(crate) const MVEX_OP0_SHIFT: u32 = 0x0000_0000;
	pub(crate) const MVEX_OP1_SHIFT: u32 = 0x0000_0004;
	pub(crate) const MVEX_OP2_SHIFT: u32 = 0x0000_0008;
	pub(crate) const MVEX_OP3_SHIFT: u32 = 0x0000_000C;
	pub(crate) const IGNORES_ROUNDING_CONTROL: u32 = 0x4000_0000;
	pub(crate) const AMD_LOCK_REG_BIT: u32 = 0x8000_0000;
}
// GENERATOR-END: EncFlags1

// GENERATOR-BEGIN: EncFlags2
// ⚠️This was generated by GENERATOR!🦹‍♂️
pub(crate) struct EncFlags2;
#[allow(dead_code)]
impl EncFlags2 {
	pub(crate) const NONE: u32 = 0x0000_0000;
	pub(crate) const OP_CODE_SHIFT: u32 = 0x0000_0000;
	pub(crate) const OP_CODE_IS2_BYTES: u32 = 0x0001_0000;
	pub(crate) const TABLE_SHIFT: u32 = 0x0000_0011;
	pub(crate) const TABLE_MASK: u32 = 0x0000_0007;
	pub(crate) const MANDATORY_PREFIX_SHIFT: u32 = 0x0000_0014;
	pub(crate) const MANDATORY_PREFIX_MASK: u32 = 0x0000_0003;
	pub(crate) const WBIT_SHIFT: u32 = 0x0000_0016;
	pub(crate) const WBIT_MASK: u32 = 0x0000_0003;
	pub(crate) const LBIT_SHIFT: u32 = 0x0000_0018;
	pub(crate) const LBIT_MASK: u32 = 0x0000_0007;
	pub(crate) const GROUP_INDEX_SHIFT: u32 = 0x0000_001B;
	pub(crate) const GROUP_INDEX_MASK: u32 = 0x0000_0007;
	pub(crate) const HAS_MANDATORY_PREFIX: u32 = 0x4000_0000;
	pub(crate) const HAS_GROUP_INDEX: u32 = 0x8000_0000;
}
// GENERATOR-END: EncFlags2

// GENERATOR-BEGIN: EncFlags3
// ⚠️This was generated by GENERATOR!🦹‍♂️
pub(crate) struct EncFlags3;
#[allow(dead_code)]
impl EncFlags3 {
	pub(crate) const NONE: u32 = 0x0000_0000;
	pub(crate) const ENCODING_SHIFT: u32 = 0x0000_0000;
	pub(crate) const ENCODING_MASK: u32 = 0x0000_0007;
	pub(crate) const OPERAND_SIZE_SHIFT: u32 = 0x0000_0003;
	pub(crate) const OPERAND_SIZE_MASK: u32 = 0x0000_0003;
	pub(crate) const ADDRESS_SIZE_SHIFT: u32 = 0x0000_0005;
	pub(crate) const ADDRESS_SIZE_MASK: u32 = 0x0000_0003;
	pub(crate) const TUPLE_TYPE_SHIFT: u32 = 0x0000_0007;
	pub(crate) const TUPLE_TYPE_MASK: u32 = 0x0000_001F;
	pub(crate) const DEFAULT_OP_SIZE64: u32 = 0x0000_1000;
	pub(crate) const HAS_RM_GROUP_INDEX: u32 = 0x0000_2000;
	pub(crate) const INTEL_FORCE_OP_SIZE64: u32 = 0x0000_4000;
	pub(crate) const FWAIT: u32 = 0x0000_8000;
	pub(crate) const BIT16OR32: u32 = 0x0001_0000;
	pub(crate) const BIT64: u32 = 0x0002_0000;
	pub(crate) const LOCK: u32 = 0x0004_0000;
	pub(crate) const XACQUIRE: u32 = 0x0008_0000;
	pub(crate) const XRELEASE: u32 = 0x0010_0000;
	pub(crate) const REP: u32 = 0x0020_0000;
	pub(crate) const REPNE: u32 = 0x0040_0000;
	pub(crate) const BND: u32 = 0x0080_0000;
	pub(crate) const HINT_TAKEN: u32 = 0x0100_0000;
	pub(crate) const NOTRACK: u32 = 0x0200_0000;
	pub(crate) const BROADCAST: u32 = 0x0400_0000;
	pub(crate) const ROUNDING_CONTROL: u32 = 0x0800_0000;
	pub(crate) const SUPPRESS_ALL_EXCEPTIONS: u32 = 0x1000_0000;
	pub(crate) const OP_MASK_REGISTER: u32 = 0x2000_0000;
	pub(crate) const ZEROING_MASKING: u32 = 0x4000_0000;
	pub(crate) const REQUIRE_OP_MASK_REGISTER: u32 = 0x8000_0000;
}
// GENERATOR-END: EncFlags3

// GENERATOR-BEGIN: OpCodeInfoFlags1
// ⚠️This was generated by GENERATOR!🦹‍♂️
#[cfg(all(feature = "encoder", feature = "op_code_info"))]
pub(crate) struct OpCodeInfoFlags1;
#[cfg(all(feature = "encoder", feature = "op_code_info"))]
#[allow(dead_code)]
impl OpCodeInfoFlags1 {
	pub(crate) const NONE: u32 = 0x0000_0000;
	pub(crate) const CPL0_ONLY: u32 = 0x0000_0001;
	pub(crate) const CPL3_ONLY: u32 = 0x0000_0002;
	pub(crate) const INPUT_OUTPUT: u32 = 0x0000_0004;
	pub(crate) const NOP: u32 = 0x0000_0008;
	pub(crate) const RESERVED_NOP: u32 = 0x0000_0010;
	pub(crate) const SERIALIZING_INTEL: u32 = 0x0000_0020;
	pub(crate) const SERIALIZING_AMD: u32 = 0x0000_0040;
	pub(crate) const MAY_REQUIRE_CPL0: u32 = 0x0000_0080;
	pub(crate) const CET_TRACKED: u32 = 0x0000_0100;
	pub(crate) const NON_TEMPORAL: u32 = 0x0000_0200;
	pub(crate) const FPU_NO_WAIT: u32 = 0x0000_0400;
	pub(crate) const IGNORES_MOD_BITS: u32 = 0x0000_0800;
	pub(crate) const NO66: u32 = 0x0000_1000;
	pub(crate) const NFX: u32 = 0x0000_2000;
	pub(crate) const REQUIRES_UNIQUE_REG_NUMS: u32 = 0x0000_4000;
	pub(crate) const PRIVILEGED: u32 = 0x0000_8000;
	pub(crate) const SAVE_RESTORE: u32 = 0x0001_0000;
	pub(crate) const STACK_INSTRUCTION: u32 = 0x0002_0000;
	pub(crate) const IGNORES_SEGMENT: u32 = 0x0004_0000;
	pub(crate) const OP_MASK_READ_WRITE: u32 = 0x0008_0000;
	pub(crate) const MOD_REG_RM_STRING: u32 = 0x0010_0000;
	pub(crate) const DEC_OPTION_VALUE_MASK: u32 = 0x0000_001F;
	pub(crate) const DEC_OPTION_VALUE_SHIFT: u32 = 0x0000_0015;
	pub(crate) const FORCE_OP_SIZE64: u32 = 0x4000_0000;
	pub(crate) const REQUIRES_UNIQUE_DEST_REG_NUM: u32 = 0x8000_0000;
}
// GENERATOR-END: OpCodeInfoFlags1

// GENERATOR-BEGIN: OpCodeInfoFlags2
// ⚠️This was generated by GENERATOR!🦹‍♂️
#[cfg(all(feature = "encoder", feature = "op_code_info"))]
pub(crate) struct OpCodeInfoFlags2;
#[cfg(all(feature = "encoder", feature = "op_code_info"))]
#[allow(dead_code)]
impl OpCodeInfoFlags2 {
	pub(crate) const NONE: u32 = 0x0000_0000;
	pub(crate) const REAL_MODE: u32 = 0x0000_0001;
	pub(crate) const PROTECTED_MODE: u32 = 0x0000_0002;
	pub(crate) const VIRTUAL8086_MODE: u32 = 0x0000_0004;
	pub(crate) const COMPATIBILITY_MODE: u32 = 0x0000_0008;
	pub(crate) const USE_OUTSIDE_SMM: u32 = 0x0000_0010;
	pub(crate) const USE_IN_SMM: u32 = 0x0000_0020;
	pub(crate) const USE_OUTSIDE_ENCLAVE_SGX: u32 = 0x0000_0040;
	pub(crate) const USE_IN_ENCLAVE_SGX1: u32 = 0x0000_0080;
	pub(crate) const USE_IN_ENCLAVE_SGX2: u32 = 0x0000_0100;
	pub(crate) const USE_OUTSIDE_VMX_OP: u32 = 0x0000_0200;
	pub(crate) const USE_IN_VMX_ROOT_OP: u32 = 0x0000_0400;
	pub(crate) const USE_IN_VMX_NON_ROOT_OP: u32 = 0x0000_0800;
	pub(crate) const USE_OUTSIDE_SEAM: u32 = 0x0000_1000;
	pub(crate) const USE_IN_SEAM: u32 = 0x0000_2000;
	pub(crate) const TDX_NON_ROOT_GEN_UD: u32 = 0x0000_4000;
	pub(crate) const TDX_NON_ROOT_GEN_VE: u32 = 0x0000_8000;
	pub(crate) const TDX_NON_ROOT_MAY_GEN_EX: u32 = 0x0001_0000;
	pub(crate) const INTEL_VM_EXIT: u32 = 0x0002_0000;
	pub(crate) const INTEL_MAY_VM_EXIT: u32 = 0x0004_0000;
	pub(crate) const INTEL_SMM_VM_EXIT: u32 = 0x0008_0000;
	pub(crate) const AMD_VM_EXIT: u32 = 0x0010_0000;
	pub(crate) const AMD_MAY_VM_EXIT: u32 = 0x0020_0000;
	pub(crate) const TSX_ABORT: u32 = 0x0040_0000;
	pub(crate) const TSX_IMPL_ABORT: u32 = 0x0080_0000;
	pub(crate) const TSX_MAY_ABORT: u32 = 0x0100_0000;
	pub(crate) const INTEL_DECODER16OR32: u32 = 0x0200_0000;
	pub(crate) const INTEL_DECODER64: u32 = 0x0400_0000;
	pub(crate) const AMD_DECODER16OR32: u32 = 0x0800_0000;
	pub(crate) const AMD_DECODER64: u32 = 0x1000_0000;
	pub(crate) const INSTR_STR_FMT_OPTION_MASK: u32 = 0x0000_0007;
	pub(crate) const INSTR_STR_FMT_OPTION_SHIFT: u32 = 0x0000_001D;
}
// GENERATOR-END: OpCodeInfoFlags2

// GENERATOR-BEGIN: DecOptionValue
// ⚠️This was generated by GENERATOR!🦹‍♂️
#[derive(Copy, Clone, Eq, PartialEq)]
#[allow(non_camel_case_types)]
#[cfg(all(feature = "encoder", feature = "op_code_info"))]
#[allow(dead_code)]
pub(crate) enum DecOptionValue {
	None,
	ALTINST,
	Cl1invmb,
	Cmpxchg486A,
	Cyrix,
	Cyrix_DMI,
	Cyrix_SMINT_0F7E,
	Jmpe,
	Loadall286,
	Loadall386,
	MovTr,
	MPX,
	OldFpu,
	Pcommit,
	Umov,
	Xbts,
	Udbg,
	KNC,
}
#[cfg(all(feature = "encoder", feature = "op_code_info"))]
#[rustfmt::skip]
static GEN_DEBUG_DEC_OPTION_VALUE: [&str; 18] = [
	"None",
	"ALTINST",
	"Cl1invmb",
	"Cmpxchg486A",
	"Cyrix",
	"Cyrix_DMI",
	"Cyrix_SMINT_0F7E",
	"Jmpe",
	"Loadall286",
	"Loadall386",
	"MovTr",
	"MPX",
	"OldFpu",
	"Pcommit",
	"Umov",
	"Xbts",
	"Udbg",
	"KNC",
];
#[cfg(all(feature = "encoder", feature = "op_code_info"))]
impl fmt::Debug for DecOptionValue {
	#[inline]
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", GEN_DEBUG_DEC_OPTION_VALUE[*self as usize])
	}
}
#[cfg(all(feature = "encoder", feature = "op_code_info"))]
impl Default for DecOptionValue {
	#[inline]
	fn default() -> Self {
		DecOptionValue::None
	}
}
// GENERATOR-END: DecOptionValue

// GENERATOR-BEGIN: InstrStrFmtOption
// ⚠️This was generated by GENERATOR!🦹‍♂️
#[derive(Copy, Clone, Eq, PartialEq)]
#[allow(non_camel_case_types)]
#[cfg(all(feature = "encoder", feature = "op_code_info"))]
#[allow(dead_code)]
pub(crate) enum InstrStrFmtOption {
	None,
	OpMaskIsK1_or_NoGprSuffix,
	IncVecIndex,
	NoVecIndex,
	SwapVecIndex12,
	SkipOp0,
	VecIndexSameAsOpIndex,
}
#[cfg(all(feature = "encoder", feature = "op_code_info"))]
#[rustfmt::skip]
static GEN_DEBUG_INSTR_STR_FMT_OPTION: [&str; 7] = [
	"None",
	"OpMaskIsK1_or_NoGprSuffix",
	"IncVecIndex",
	"NoVecIndex",
	"SwapVecIndex12",
	"SkipOp0",
	"VecIndexSameAsOpIndex",
];
#[cfg(all(feature = "encoder", feature = "op_code_info"))]
impl fmt::Debug for InstrStrFmtOption {
	#[inline]
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", GEN_DEBUG_INSTR_STR_FMT_OPTION[*self as usize])
	}
}
#[cfg(all(feature = "encoder", feature = "op_code_info"))]
impl Default for InstrStrFmtOption {
	#[inline]
	fn default() -> Self {
		InstrStrFmtOption::None
	}
}
// GENERATOR-END: InstrStrFmtOption

// GENERATOR-BEGIN: WBit
// ⚠️This was generated by GENERATOR!🦹‍♂️
#[derive(Copy, Clone, Eq, PartialEq)]
#[allow(non_camel_case_types)]
#[cfg(any(not(feature = "no_vex"), not(feature = "no_xop"), not(feature = "no_evex"), feature = "mvex"))]
#[allow(dead_code)]
pub(crate) enum WBit {
	W0,
	W1,
	WIG,
	WIG32,
}
#[cfg(any(not(feature = "no_vex"), not(feature = "no_xop"), not(feature = "no_evex"), feature = "mvex"))]
#[rustfmt::skip]
static GEN_DEBUG_WBIT: [&str; 4] = [
	"W0",
	"W1",
	"WIG",
	"WIG32",
];
#[cfg(any(not(feature = "no_vex"), not(feature = "no_xop"), not(feature = "no_evex"), feature = "mvex"))]
impl fmt::Debug for WBit {
	#[inline]
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", GEN_DEBUG_WBIT[*self as usize])
	}
}
#[cfg(any(not(feature = "no_vex"), not(feature = "no_xop"), not(feature = "no_evex"), feature = "mvex"))]
impl Default for WBit {
	#[inline]
	fn default() -> Self {
		WBit::W0
	}
}
// GENERATOR-END: WBit

// GENERATOR-BEGIN: LBit
// ⚠️This was generated by GENERATOR!🦹‍♂️
#[derive(Copy, Clone, Eq, PartialEq)]
#[allow(non_camel_case_types)]
#[cfg(any(not(feature = "no_vex"), not(feature = "no_xop"), not(feature = "no_evex"), feature = "mvex"))]
#[allow(dead_code)]
pub(crate) enum LBit {
	L0,
	L1,
	LIG,
	LZ,
	L128,
	L256,
	L512,
}
#[cfg(any(not(feature = "no_vex"), not(feature = "no_xop"), not(feature = "no_evex"), feature = "mvex"))]
#[rustfmt::skip]
static GEN_DEBUG_LBIT: [&str; 7] = [
	"L0",
	"L1",
	"LIG",
	"LZ",
	"L128",
	"L256",
	"L512",
];
#[cfg(any(not(feature = "no_vex"), not(feature = "no_xop"), not(feature = "no_evex"), feature = "mvex"))]
impl fmt::Debug for LBit {
	#[inline]
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", GEN_DEBUG_LBIT[*self as usize])
	}
}
#[cfg(any(not(feature = "no_vex"), not(feature = "no_xop"), not(feature = "no_evex"), feature = "mvex"))]
impl Default for LBit {
	#[inline]
	fn default() -> Self {
		LBit::L0
	}
}
// GENERATOR-END: LBit

// GENERATOR-BEGIN: RepPrefixKind
// ⚠️This was generated by GENERATOR!🦹‍♂️
/// `REP`/`REPE`/`REPNE` prefix
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[cfg_attr(not(feature = "exhaustive_enums"), non_exhaustive)]
pub enum RepPrefixKind {
	/// No `REP`/`REPE`/`REPNE` prefix
	None = 0,
	/// `REP`/`REPE` prefix
	Repe = 1,
	/// `REPNE` prefix
	Repne = 2,
}
#[rustfmt::skip]
static GEN_DEBUG_REP_PREFIX_KIND: [&str; 3] = [
	"None",
	"Repe",
	"Repne",
];
impl fmt::Debug for RepPrefixKind {
	#[inline]
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", GEN_DEBUG_REP_PREFIX_KIND[*self as usize])
	}
}
impl Default for RepPrefixKind {
	#[inline]
	fn default() -> Self {
		RepPrefixKind::None
	}
}
#[allow(non_camel_case_types)]
#[allow(dead_code)]
pub(crate) type RepPrefixKindUnderlyingType = u8;
#[rustfmt::skip]
impl RepPrefixKind {
	/// Iterates over all `RepPrefixKind` enum values
	#[inline]
	pub fn values() -> impl Iterator<Item = RepPrefixKind> + DoubleEndedIterator + ExactSizeIterator + FusedIterator {
		// SAFETY: all values 0-max are valid enum values
		(0..IcedConstants::REP_PREFIX_KIND_ENUM_COUNT).map(|x| unsafe { mem::transmute::<u8, RepPrefixKind>(x as u8) })
	}
}
#[test]
#[rustfmt::skip]
fn test_repprefixkind_values() {
	let mut iter = RepPrefixKind::values();
	assert_eq!(iter.size_hint(), (IcedConstants::REP_PREFIX_KIND_ENUM_COUNT, Some(IcedConstants::REP_PREFIX_KIND_ENUM_COUNT)));
	assert_eq!(iter.len(), IcedConstants::REP_PREFIX_KIND_ENUM_COUNT);
	assert!(iter.next().is_some());
	assert_eq!(iter.size_hint(), (IcedConstants::REP_PREFIX_KIND_ENUM_COUNT - 1, Some(IcedConstants::REP_PREFIX_KIND_ENUM_COUNT - 1)));
	assert_eq!(iter.len(), IcedConstants::REP_PREFIX_KIND_ENUM_COUNT - 1);

	let values: Vec<RepPrefixKind> = RepPrefixKind::values().collect();
	assert_eq!(values.len(), IcedConstants::REP_PREFIX_KIND_ENUM_COUNT);
	for (i, value) in values.into_iter().enumerate() {
		assert_eq!(i, value as usize);
	}

	let values1: Vec<RepPrefixKind> = RepPrefixKind::values().collect();
	let mut values2: Vec<RepPrefixKind> = RepPrefixKind::values().rev().collect();
	values2.reverse();
	assert_eq!(values1, values2);
}
#[rustfmt::skip]
impl TryFrom<usize> for RepPrefixKind {
	type Error = IcedError;
	#[inline]
	fn try_from(value: usize) -> Result<Self, Self::Error> {
		if value < IcedConstants::REP_PREFIX_KIND_ENUM_COUNT {
			// SAFETY: all values 0-max are valid enum values
			Ok(unsafe { mem::transmute(value as u8) })
		} else {
			Err(IcedError::new("Invalid RepPrefixKind value"))
		}
	}
}
#[test]
#[rustfmt::skip]
fn test_repprefixkind_try_from_usize() {
	for value in RepPrefixKind::values() {
		let converted = <RepPrefixKind as TryFrom<usize>>::try_from(value as usize).unwrap();
		assert_eq!(converted, value);
	}
	assert!(<RepPrefixKind as TryFrom<usize>>::try_from(IcedConstants::REP_PREFIX_KIND_ENUM_COUNT).is_err());
	assert!(<RepPrefixKind as TryFrom<usize>>::try_from(usize::MAX).is_err());
}
#[cfg(feature = "serde")]
#[rustfmt::skip]
#[allow(clippy::zero_sized_map_values)]
const _: () = {
	use core::marker::PhantomData;
	use serde::de;
	use serde::{Deserialize, Deserializer, Serialize, Serializer};
	type EnumType = RepPrefixKind;
	impl Serialize for EnumType {
		#[inline]
		fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
		{
			serializer.serialize_u8(*self as u8)
		}
	}
	impl<'de> Deserialize<'de> for EnumType {
		#[inline]
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: Deserializer<'de>,
		{
			struct Visitor<'de> {
				marker: PhantomData<EnumType>,
				lifetime: PhantomData<&'de ()>,
			}
			impl<'de> de::Visitor<'de> for Visitor<'de> {
				type Value = EnumType;
				#[inline]
				fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
					formatter.write_str("enum RepPrefixKind")
				}
				#[inline]
				fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
				where
					E: de::Error,
				{
					if let Ok(v) = <usize as TryFrom<_>>::try_from(v) {
						if let Ok(value) = <EnumType as TryFrom<_>>::try_from(v) {
							return Ok(value);
						}
					}
					Err(de::Error::invalid_value(de::Unexpected::Unsigned(v), &"a valid RepPrefixKind variant value"))
				}
			}
			deserializer.deserialize_u8(Visitor { marker: PhantomData::<EnumType>, lifetime: PhantomData })
		}
	}
};
// GENERATOR-END: RepPrefixKind
