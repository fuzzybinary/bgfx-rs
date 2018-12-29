// Copyright (c) 2015-2016, Johan Sköld.
// License: http://opensource.org/licenses/ISC

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

extern crate std;

use bindings::*;

pub type size_t = usize;
pub type int32_t = i32;
pub type uint8_t = u8;
pub type uint16_t = u16;
pub type uint32_t = u32;
pub type uint64_t = u64;

pub const BGFX_PCI_ID_NONE: u16 = 0x0000;
pub const BGFX_PCI_ID_SOFTWARE_RASTERIZER: u16 = 0x0001;
pub const BGFX_PCI_ID_AMD: u16 = 0x1002;
pub const BGFX_PCI_ID_INTEL: u16 = 0x8086;
pub const BGFX_PCI_ID_NVIDIA: u16 = 0x10de;

// Clear flags

pub const BGFX_CLEAR_NONE: u16 = 0x0000;
pub const BGFX_CLEAR_COLOR: u16 = 0x0001;
pub const BGFX_CLEAR_DEPTH: u16 = 0x0002;
pub const BGFX_CLEAR_STENCIL: u16 = 0x0004;
pub const BGFX_CLEAR_DISCARD_COLOR_0: u16 = 0x0008;
pub const BGFX_CLEAR_DISCARD_COLOR_1: u16 = 0x0010;
pub const BGFX_CLEAR_DISCARD_COLOR_2: u16 = 0x0020;
pub const BGFX_CLEAR_DISCARD_COLOR_3: u16 = 0x0040;
pub const BGFX_CLEAR_DISCARD_COLOR_4: u16 = 0x0080;
pub const BGFX_CLEAR_DISCARD_COLOR_5: u16 = 0x0100;
pub const BGFX_CLEAR_DISCARD_COLOR_6: u16 = 0x0200;
pub const BGFX_CLEAR_DISCARD_COLOR_7: u16 = 0x0400;
pub const BGFX_CLEAR_DISCARD_DEPTH: u16 = 0x0800;
pub const BGFX_CLEAR_DISCARD_STENCIL: u16 = 0x1000;

pub const BGFX_CLEAR_DISCARD_COLOR_MASK: u16 = (BGFX_CLEAR_DISCARD_COLOR_0
    | BGFX_CLEAR_DISCARD_COLOR_1
    | BGFX_CLEAR_DISCARD_COLOR_2
    | BGFX_CLEAR_DISCARD_COLOR_3
    | BGFX_CLEAR_DISCARD_COLOR_4
    | BGFX_CLEAR_DISCARD_COLOR_5
    | BGFX_CLEAR_DISCARD_COLOR_6
    | BGFX_CLEAR_DISCARD_COLOR_7);

pub const BGFX_CLEAR_DISCARD_MASK: u16 =
    (BGFX_CLEAR_DISCARD_COLOR_MASK | BGFX_CLEAR_DISCARD_DEPTH | BGFX_CLEAR_DISCARD_STENCIL);

// Debug flags

pub const BGFX_DEBUG_NONE: u32 = 0x00000000;
pub const BGFX_DEBUG_WIREFRAME: u32 = 0x00000001;
pub const BGFX_DEBUG_IFH: u32 = 0x00000002;
pub const BGFX_DEBUG_STATS: u32 = 0x00000004;
pub const BGFX_DEBUG_TEXT: u32 = 0x00000008;

// Reset flags

pub const BGFX_RESET_NONE: u32 = 0x00000000;
pub const BGFX_RESET_FULLSCREEN: u32 = 0x00000001;
pub const BGFX_RESET_FULLSCREEN_MASK: u32 = 0x00000001;
pub const BGFX_RESET_MSAA_X2: u32 = 0x00000010;
pub const BGFX_RESET_MSAA_X4: u32 = 0x00000020;
pub const BGFX_RESET_MSAA_X8: u32 = 0x00000030;
pub const BGFX_RESET_MSAA_X16: u32 = 0x00000040;
pub const BGFX_RESET_MSAA_MASK: u32 = 0x00000070;
pub const BGFX_RESET_VSYNC: u32 = 0x00000080;
pub const BGFX_RESET_MAXANISOTROPY: u32 = 0x00000100;
pub const BGFX_RESET_CAPTURE: u32 = 0x00000200;
pub const BGFX_RESET_HMD: u32 = 0x00000400;
pub const BGFX_RESET_HMD_DEBUG: u32 = 0x00000800;
pub const BGFX_RESET_HMD_RECENTER: u32 = 0x00001000;
pub const BGFX_RESET_FLUSH_AFTER_RENDER: u32 = 0x00002000;
pub const BGFX_RESET_FLIP_AFTER_RENDER: u32 = 0x00004000;
pub const BGFX_RESET_SRGB_BACKBUFFER: u32 = 0x00008000;
pub const BGFX_RESET_HIDPI: u32 = 0x00010000;

// Buffer flags

pub const BGFX_BUFFER_NONE: u16 = 0x0000;
pub const BGFX_BUFFER_COMPUTE_FORMAT_8X1: u16 = 0x0001;
pub const BGFX_BUFFER_COMPUTE_FORMAT_8X2: u16 = 0x0002;
pub const BGFX_BUFFER_COMPUTE_FORMAT_8X4: u16 = 0x0003;
pub const BGFX_BUFFER_COMPUTE_FORMAT_16X1: u16 = 0x0004;
pub const BGFX_BUFFER_COMPUTE_FORMAT_16X2: u16 = 0x0005;
pub const BGFX_BUFFER_COMPUTE_FORMAT_16X4: u16 = 0x0006;
pub const BGFX_BUFFER_COMPUTE_FORMAT_32X1: u16 = 0x0007;
pub const BGFX_BUFFER_COMPUTE_FORMAT_32X2: u16 = 0x0008;
pub const BGFX_BUFFER_COMPUTE_FORMAT_32X4: u16 = 0x0009;
pub const BGFX_BUFFER_COMPUTE_FORMAT_MASK: u16 = 0x000f;
pub const BGFX_BUFFER_COMPUTE_TYPE_UINT: u16 = 0x0010;
pub const BGFX_BUFFER_COMPUTE_TYPE_INT: u16 = 0x0020;
pub const BGFX_BUFFER_COMPUTE_TYPE_FLOAT: u16 = 0x0030;
pub const BGFX_BUFFER_COMPUTE_TYPE_MASK: u16 = 0x0030;
pub const BGFX_BUFFER_COMPUTE_READ: u16 = 0x0100;
pub const BGFX_BUFFER_COMPUTE_WRITE: u16 = 0x0200;
pub const BGFX_BUFFER_DRAW_INDIRECT: u16 = 0x0400;
pub const BGFX_BUFFER_ALLOW_RESIZE: u16 = 0x0800;
pub const BGFX_BUFFER_INDEX32: u16 = 0x1000;

pub const BGFX_BUFFER_COMPUTE_READ_WRITE: u16 =
    (BGFX_BUFFER_COMPUTE_READ | BGFX_BUFFER_COMPUTE_WRITE);

// State flags

pub const BGFX_STATE_RGB_WRITE: u64 = 0x0000000000000001_u64;
pub const BGFX_STATE_ALPHA_WRITE: u64 = 0x0000000000000002_u64;
pub const BGFX_STATE_DEPTH_WRITE: u64 = 0x0000000000000004_u64;
pub const BGFX_STATE_DEPTH_TEST_LESS: u64 = 0x0000000000000010_u64;
pub const BGFX_STATE_DEPTH_TEST_LEQUAL: u64 = 0x0000000000000020_u64;
pub const BGFX_STATE_DEPTH_TEST_EQUAL: u64 = 0x0000000000000030_u64;
pub const BGFX_STATE_DEPTH_TEST_GEQUAL: u64 = 0x0000000000000040_u64;
pub const BGFX_STATE_DEPTH_TEST_GREATER: u64 = 0x0000000000000050_u64;
pub const BGFX_STATE_DEPTH_TEST_NOTEQUAL: u64 = 0x0000000000000060_u64;
pub const BGFX_STATE_DEPTH_TEST_NEVER: u64 = 0x0000000000000070_u64;
pub const BGFX_STATE_DEPTH_TEST_ALWAYS: u64 = 0x0000000000000080_u64;
pub const BGFX_STATE_DEPTH_TEST_MASK: u64 = 0x00000000000000f0_u64;
pub const BGFX_STATE_BLEND_ZERO: u64 = 0x0000000000001000_u64;
pub const BGFX_STATE_BLEND_ONE: u64 = 0x0000000000002000_u64;
pub const BGFX_STATE_BLEND_SRC_COLOR: u64 = 0x0000000000003000_u64;
pub const BGFX_STATE_BLEND_INV_SRC_COLOR: u64 = 0x0000000000004000_u64;
pub const BGFX_STATE_BLEND_SRC_ALPHA: u64 = 0x0000000000005000_u64;
pub const BGFX_STATE_BLEND_INV_SRC_ALPHA: u64 = 0x0000000000006000_u64;
pub const BGFX_STATE_BLEND_DST_ALPHA: u64 = 0x0000000000007000_u64;
pub const BGFX_STATE_BLEND_INV_DST_ALPHA: u64 = 0x0000000000008000_u64;
pub const BGFX_STATE_BLEND_DST_COLOR: u64 = 0x0000000000009000_u64;
pub const BGFX_STATE_BLEND_INV_DST_COLOR: u64 = 0x000000000000a000_u64;
pub const BGFX_STATE_BLEND_SRC_ALPHA_SAT: u64 = 0x000000000000b000_u64;
pub const BGFX_STATE_BLEND_FACTOR: u64 = 0x000000000000c000_u64;
pub const BGFX_STATE_BLEND_INV_FACTOR: u64 = 0x000000000000d000_u64;
pub const BGFX_STATE_BLEND_MASK: u64 = 0x000000000ffff000_u64;
pub const BGFX_STATE_BLEND_EQUATION_ADD: u64 = 0x0000000000000000_u64;
pub const BGFX_STATE_BLEND_EQUATION_SUB: u64 = 0x0000000010000000_u64;
pub const BGFX_STATE_BLEND_EQUATION_REVSUB: u64 = 0x0000000020000000_u64;
pub const BGFX_STATE_BLEND_EQUATION_MIN: u64 = 0x0000000030000000_u64;
pub const BGFX_STATE_BLEND_EQUATION_MAX: u64 = 0x0000000040000000_u64;
pub const BGFX_STATE_BLEND_EQUATION_MASK: u64 = 0x00000003f0000000_u64;
pub const BGFX_STATE_BLEND_INDEPENDENT: u64 = 0x0000000400000000_u64;
pub const BGFX_STATE_CULL_CW: u64 = 0x0000001000000000_u64;
pub const BGFX_STATE_CULL_CCW: u64 = 0x0000002000000000_u64;
pub const BGFX_STATE_CULL_MASK: u64 = 0x0000003000000000_u64;
pub const BGFX_STATE_ALPHA_REF_MASK: u64 = 0x0000ff0000000000_u64;
pub const BGFX_STATE_PT_TRISTRIP: u64 = 0x0001000000000000_u64;
pub const BGFX_STATE_PT_LINES: u64 = 0x0002000000000000_u64;
pub const BGFX_STATE_PT_LINESTRIP: u64 = 0x0003000000000000_u64;
pub const BGFX_STATE_PT_POINTS: u64 = 0x0004000000000000_u64;
pub const BGFX_STATE_PT_MASK: u64 = 0x0007000000000000_u64;
pub const BGFX_STATE_POINT_SIZE_MASK: u64 = 0x0ff0000000000000_u64;
pub const BGFX_STATE_MSAA: u64 = 0x1000000000000000_u64;
pub const BGFX_STATE_RESERVED_MASK: u64 = 0xe000000000000000_u64;
pub const BGFX_STATE_NONE: u64 = 0x0000000000000000_u64;
pub const BGFX_STATE_MASK: u64 = 0xffffffffffffffff_u64;

pub const BGFX_STATE_DEFAULT: u64 = (BGFX_STATE_RGB_WRITE
    | BGFX_STATE_ALPHA_WRITE
    | BGFX_STATE_DEPTH_TEST_LESS
    | BGFX_STATE_DEPTH_WRITE
    | BGFX_STATE_CULL_CW
    | BGFX_STATE_MSAA);

#[macro_export]
macro_rules! BGFX_STATE_ALPHA_REF {
    ($aref:expr) => {
        (($aref as u64) << BGFX_STATE_ALPHA_REF_SHIFT) & BGFX_STATE_ALPHA_REF_MASK
    };
}

#[macro_export]
macro_rules! BGFX_STATE_POINT_SIZE {
    ($size:expr) => {
        (($size as u64) << BGFX_STATE_POINT_SIZE_SHIFT) & BGFX_STATE_POINT_SIZE_MASK
    };
}

#[macro_export]
macro_rules! BGFX_STATE_BLEND_FUNC_SEPARATE {
    ($srcrgb:expr, $dstrgb:expr, $srca:expr, $dsta:expr) => {
        ($srcrgb as u64)
            | (($dstrgb as u64) << 4)
            | (($srca as u64) << 8)
            | (($dsta as u64) << 12)
    };
}

#[macro_export]
macro_rules! BGFX_STATE_BLEND_EQUATION_SEPARATE {
    ($rgb:expr, $a:expr) => {
        ($rgb as u64) | (($a as u64) << 3)
    };
}

#[macro_export]
macro_rules! BGFX_STATE_BLEND_FUNC {
    ($src:expr, $dst:expr) => {
        BGFX_STATE_BLEND_FUNC_SEPARATE!($src, $dst, $src, $dst)
    };
}

#[macro_export]
macro_rules! BGFX_STATE_BLEND_EQUATION {
    ($equation:expr) => {
        BGFX_STATE_BLEND_EQUATION_SEPARATE!($equation, $equation)
    };
}

pub const BGFX_STATE_BLEND_ADD: u64 =
    (BGFX_STATE_BLEND_FUNC!(BGFX_STATE_BLEND_ONE, BGFX_STATE_BLEND_ONE));
pub const BGFX_STATE_BLEND_ALPHA: u64 =
    (BGFX_STATE_BLEND_FUNC!(BGFX_STATE_BLEND_SRC_ALPHA, BGFX_STATE_BLEND_INV_SRC_ALPHA));
pub const BGFX_STATE_BLEND_DARKEN: u64 =
    (BGFX_STATE_BLEND_FUNC!(BGFX_STATE_BLEND_ONE, BGFX_STATE_BLEND_ONE)
        | BGFX_STATE_BLEND_EQUATION!(BGFX_STATE_BLEND_EQUATION_MIN));
pub const BGFX_STATE_BLEND_LIGHTEN: u64 =
    (BGFX_STATE_BLEND_FUNC!(BGFX_STATE_BLEND_ONE, BGFX_STATE_BLEND_ONE)
        | BGFX_STATE_BLEND_EQUATION!(BGFX_STATE_BLEND_EQUATION_MAX));
pub const BGFX_STATE_BLEND_MULTIPLY: u64 =
    (BGFX_STATE_BLEND_FUNC!(BGFX_STATE_BLEND_DST_COLOR, BGFX_STATE_BLEND_ZERO));
pub const BGFX_STATE_BLEND_NORMAL: u64 =
    (BGFX_STATE_BLEND_FUNC!(BGFX_STATE_BLEND_ONE, BGFX_STATE_BLEND_INV_SRC_ALPHA));
pub const BGFX_STATE_BLEND_SCREEN: u64 =
    (BGFX_STATE_BLEND_FUNC!(BGFX_STATE_BLEND_ONE, BGFX_STATE_BLEND_INV_SRC_COLOR));
pub const BGFX_STATE_BLEND_LINEAR_BURN: u64 =
    (BGFX_STATE_BLEND_FUNC!(BGFX_STATE_BLEND_DST_COLOR, BGFX_STATE_BLEND_INV_DST_COLOR)
        | BGFX_STATE_BLEND_EQUATION!(BGFX_STATE_BLEND_EQUATION_SUB));

#[macro_export]
macro_rules! BGFX_STATE_BLEND_FUNC_RT_x {
    ($src:expr, $dst:expr) => {
        (($src >> BGFX_STATE_BLEND_SHIFT) as u32)
            | ((($dst >> BGFX_STATE_BLEND_SHIFT) as u32) << 4)
    };
}

#[macro_export]
macro_rules! BGFX_STATE_BLEND_FUNC_RT_xE {
    ($src:expr, $dst:expr, $equation:expr) => {
        BGFX_STATE_BLEND_FUNC_RT_x!($src, $dst)
            | ((($equation >> BGFX_STATE_BLEND_EQUATION_SHIFT) as u32) << 8)
    };
}

#[macro_export]
macro_rules! BGFX_STATE_BLEND_FUNC_RT_1 {
    ($src:expr, $dst:expr) => {
        BGFX_STATE_BLEND_FUNC_RT_x!($src, $dst)
    };
}

#[macro_export]
macro_rules! BGFX_STATE_BLEND_FUNC_RT_2 {
    ($src:expr, $dst:expr) => {
        BGFX_STATE_BLEND_FUNC_RT_x!($src, $dst) << 11
    };
}

#[macro_export]
macro_rules! BGFX_STATE_BLEND_FUNC_RT_3 {
    ($src:expr, $dst:expr) => {
        BGFX_STATE_BLEND_FUNC_RT_x!($src, $dst) << 22
    };
}

#[macro_export]
macro_rules! BGFX_STATE_BLEND_FUNC_RT_1E {
    ($src:expr, $dst:expr, $equation:expr) => {
        BGFX_STATE_BLEND_FUNC_RT_xE!($src, $dst, $equation)
    };
}

#[macro_export]
macro_rules! BGFX_STATE_BLEND_FUNC_RT_2E {
    ($src:expr, $dst:expr, $equation:expr) => {
        BGFX_STATE_BLEND_FUNC_RT_xE!($src, $dst, $equation) << 11
    };
}

#[macro_export]
macro_rules! BGFX_STATE_BLEND_FUNC_RT_3E {
    ($src:expr, $dst:expr, $equation:expr) => {
        BGFX_STATE_BLEND_FUNC_RT_xE!($src, $dst, $equation) << 22
    };
}

pub type BgfxInitParams = bgfx_init_t;
impl std::default::Default for BgfxInitParams {
    fn default() -> BgfxInitParams {
        unsafe {
            let mut params: BgfxInitParams = std::mem::uninitialized();
            bgfx_init_ctor(&mut params);
            params
        }
    }
}

bitflags! {
    flags StateFlags: u64 {
        const STATE_RGB_WRITE = BGFX_STATE_RGB_WRITE,
        const STATE_ALPHA_WRITE = BGFX_STATE_ALPHA_WRITE,
        const STATE_DEPTH_WRITE = BGFX_STATE_DEPTH_WRITE,
        const STATE_DEPTH_TEST_LESS = BGFX_STATE_DEPTH_TEST_LESS,
        const STATE_DEPTH_TEST_LEQUAL = BGFX_STATE_DEPTH_TEST_LEQUAL,
        const STATE_DEPTH_TEST_EQUAL = BGFX_STATE_DEPTH_TEST_EQUAL,
        const STATE_DEPTH_TEST_GEQUAL = BGFX_STATE_DEPTH_TEST_GEQUAL,
        const STATE_DEPTH_TEST_GREATER = BGFX_STATE_DEPTH_TEST_GREATER,
        const STATE_DEPTH_TEST_NOTEQUAL = BGFX_STATE_DEPTH_TEST_NOTEQUAL,
        const STATE_DEPTH_TEST_NEVER = BGFX_STATE_DEPTH_TEST_NEVER,
        const STATE_DEPTH_TEST_ALWAYS = BGFX_STATE_DEPTH_TEST_ALWAYS,
        const STATE_DEPTH_TEST_SHIFT = BGFX_STATE_DEPTH_TEST_SHIFT as u64,
        const STATE_DEPTH_TEST_MASK = BGFX_STATE_DEPTH_TEST_MASK,
        const STATE_BLEND_ZERO = BGFX_STATE_BLEND_ZERO,
        const STATE_BLEND_ONE = BGFX_STATE_BLEND_ONE,
        const STATE_BLEND_SRC_COLOR = BGFX_STATE_BLEND_SRC_COLOR,
        const STATE_BLEND_INV_SRC_COLOR = BGFX_STATE_BLEND_INV_SRC_COLOR,
        const STATE_BLEND_SRC_ALPHA = BGFX_STATE_BLEND_SRC_ALPHA,
        const STATE_BLEND_INV_SRC_ALPHA = BGFX_STATE_BLEND_INV_SRC_ALPHA,
        const STATE_BLEND_DST_ALPHA = BGFX_STATE_BLEND_DST_ALPHA,
        const STATE_BLEND_INV_DST_ALPHA = BGFX_STATE_BLEND_INV_DST_ALPHA,
        const STATE_BLEND_DST_COLOR = BGFX_STATE_BLEND_DST_COLOR,
        const STATE_BLEND_INV_DST_COLOR = BGFX_STATE_BLEND_INV_DST_COLOR,
        const STATE_BLEND_SRC_ALPHA_SAT = BGFX_STATE_BLEND_SRC_ALPHA_SAT,
        const STATE_BLEND_FACTOR = BGFX_STATE_BLEND_FACTOR,
        const STATE_BLEND_INV_FACTOR = BGFX_STATE_BLEND_INV_FACTOR,
        const STATE_BLEND_SHIFT = BGFX_STATE_BLEND_SHIFT as u64,
        const STATE_BLEND_MASK = BGFX_STATE_BLEND_MASK,
        const STATE_BLEND_EQUATION_ADD = BGFX_STATE_BLEND_EQUATION_ADD,
        const STATE_BLEND_EQUATION_SUB = BGFX_STATE_BLEND_EQUATION_SUB,
        const STATE_BLEND_EQUATION_REVSUB = BGFX_STATE_BLEND_EQUATION_REVSUB,
        const STATE_BLEND_EQUATION_MIN = BGFX_STATE_BLEND_EQUATION_MIN,
        const STATE_BLEND_EQUATION_MAX = BGFX_STATE_BLEND_EQUATION_MAX,
        const STATE_BLEND_EQUATION_SHIFT = BGFX_STATE_BLEND_EQUATION_SHIFT as u64,
        const STATE_BLEND_EQUATION_MASK = BGFX_STATE_BLEND_EQUATION_MASK,
        const STATE_BLEND_INDEPENDENT = BGFX_STATE_BLEND_INDEPENDENT,
        const STATE_CULL_CW = BGFX_STATE_CULL_CW,
        const STATE_CULL_CCW = BGFX_STATE_CULL_CCW,
        const STATE_CULL_SHIFT = BGFX_STATE_CULL_SHIFT as u64,
        const STATE_CULL_MASK = BGFX_STATE_CULL_MASK,
        const STATE_ALPHA_REF_SHIFT = BGFX_STATE_ALPHA_REF_SHIFT as u64,
        const STATE_ALPHA_REF_MASK = BGFX_STATE_ALPHA_REF_MASK,
        const STATE_PT_TRISTRIP = BGFX_STATE_PT_TRISTRIP,
        const STATE_PT_LINES = BGFX_STATE_PT_LINES,
        const STATE_PT_LINESTRIP = BGFX_STATE_PT_LINESTRIP,
        const STATE_PT_POINTS = BGFX_STATE_PT_POINTS,
        const STATE_PT_SHIFT = BGFX_STATE_PT_SHIFT as u64,
        const STATE_PT_MASK = BGFX_STATE_PT_MASK,
        const STATE_POINT_SIZE_SHIFT = BGFX_STATE_POINT_SIZE_SHIFT as u64,
        const STATE_POINT_SIZE_MASK = BGFX_STATE_POINT_SIZE_MASK,
        const STATE_MSAA = BGFX_STATE_MSAA,
        const STATE_RESERVED_MASK = BGFX_STATE_RESERVED_MASK,
        const STATE_NONE = BGFX_STATE_NONE,
        const STATE_MASK = BGFX_STATE_MASK,
        const STATE_DEFAULT = BGFX_STATE_DEFAULT,
        const STATE_BLEND_ADD = BGFX_STATE_BLEND_ADD,
        const STATE_BLEND_ALPHA = BGFX_STATE_BLEND_ALPHA,
        const STATE_BLEND_DARKEN = BGFX_STATE_BLEND_DARKEN,
        const STATE_BLEND_LIGHTEN = BGFX_STATE_BLEND_LIGHTEN,
        const STATE_BLEND_MULTIPLY = BGFX_STATE_BLEND_MULTIPLY,
        const STATE_BLEND_NORMAL = BGFX_STATE_BLEND_NORMAL,
        const STATE_BLEND_SCREEN = BGFX_STATE_BLEND_SCREEN,
        const STATE_BLEND_LINEAR_BURN = BGFX_STATE_BLEND_LINEAR_BURN,
    }
}

impl Default for StateFlags {
    #[inline]
    fn default() -> StateFlags {
        STATE_DEFAULT
    }
}

#[inline]
pub fn state_alpha_ref(aref: u8) -> StateFlags {
    StateFlags::from_bits(BGFX_STATE_ALPHA_REF!(aref)).unwrap()
}

#[inline]
pub fn state_point_size(size: u8) -> StateFlags {
    StateFlags::from_bits(BGFX_STATE_POINT_SIZE!(size)).unwrap()
}

#[inline]
pub fn state_blend_func_separate(srcrgb: u32, dstrgb: u32, srca: u8, dsta: u8) -> StateFlags {
    StateFlags::from_bits(BGFX_STATE_BLEND_FUNC_SEPARATE!(srcrgb, dstrgb, srca, dsta)).unwrap()
}

#[inline]
pub fn state_blend_equation_separate(rgb: u32, a: u8) -> StateFlags {
    StateFlags::from_bits(BGFX_STATE_BLEND_EQUATION_SEPARATE!(rgb, a)).unwrap()
}

#[inline]
pub fn state_blend_func(src: u32, dst: u32) -> StateFlags {
    StateFlags::from_bits(BGFX_STATE_BLEND_FUNC_SEPARATE!(src, dst, src, dst)).unwrap()
}

#[inline]
pub fn state_blend_equation(equation: u32) -> StateFlags {
    StateFlags::from_bits(BGFX_STATE_BLEND_EQUATION_SEPARATE!(equation, equation)).unwrap()
}

#[inline]
pub fn state_blend_func_rt_x(src: u32, dst: u32) -> StateFlags {
    StateFlags::from_bits(BGFX_STATE_BLEND_FUNC_RT_x!(src, dst) as u64).unwrap()
}

#[inline]
pub fn state_blend_func_rt_xe(src: u32, dst: u32, equation: u64) -> StateFlags {
    StateFlags::from_bits(BGFX_STATE_BLEND_FUNC_RT_xE!(src, dst, equation) as u64).unwrap()
}

#[inline]
pub fn state_blend_func_rt_1(src: u32, dst: u32) -> StateFlags {
    StateFlags::from_bits(BGFX_STATE_BLEND_FUNC_RT_1!(src, dst) as u64).unwrap()
}

#[inline]
pub fn state_blend_func_rt_2(src: u32, dst: u32) -> StateFlags {
    StateFlags::from_bits(BGFX_STATE_BLEND_FUNC_RT_2!(src, dst) as u64).unwrap()
}

#[inline]
pub fn state_blend_func_rt_3(src: u32, dst: u32) -> StateFlags {
    StateFlags::from_bits(BGFX_STATE_BLEND_FUNC_RT_3!(src, dst) as u64).unwrap()
}

#[inline]
pub fn state_blend_func_rt_1e(src: u32, dst: u32, equation: u64) -> StateFlags {
    StateFlags::from_bits(BGFX_STATE_BLEND_FUNC_RT_1E!(src, dst, equation) as u64).unwrap()
}

#[inline]
pub fn state_blend_func_rt_2e(src: u32, dst: u32, equation: u64) -> StateFlags {
    StateFlags::from_bits(BGFX_STATE_BLEND_FUNC_RT_2E!(src, dst, equation) as u64).unwrap()
}

#[inline]
pub fn state_blend_func_rt_3e(src: u32, dst: u32, equation: u64) -> StateFlags {
    StateFlags::from_bits(BGFX_STATE_BLEND_FUNC_RT_3E!(src, dst, equation) as u64).unwrap()
}

bitflags! {
    flags BufferFlags: u16 {
        const BUFFER_NONE = BGFX_BUFFER_NONE,
        const BUFFER_COMPUTE_FORMAT_8X1 = BGFX_BUFFER_COMPUTE_FORMAT_8X1,
        const BUFFER_COMPUTE_FORMAT_8X2 = BGFX_BUFFER_COMPUTE_FORMAT_8X2,
        const BUFFER_COMPUTE_FORMAT_8X4 = BGFX_BUFFER_COMPUTE_FORMAT_8X4,
        const BUFFER_COMPUTE_FORMAT_16X1 = BGFX_BUFFER_COMPUTE_FORMAT_16X1,
        const BUFFER_COMPUTE_FORMAT_16X2 = BGFX_BUFFER_COMPUTE_FORMAT_16X2,
        const BUFFER_COMPUTE_FORMAT_16X4 = BGFX_BUFFER_COMPUTE_FORMAT_16X4,
        const BUFFER_COMPUTE_FORMAT_32X1 = BGFX_BUFFER_COMPUTE_FORMAT_32X1,
        const BUFFER_COMPUTE_FORMAT_32X2 = BGFX_BUFFER_COMPUTE_FORMAT_32X2,
        const BUFFER_COMPUTE_FORMAT_32X4 = BGFX_BUFFER_COMPUTE_FORMAT_32X4,
        const BUFFER_COMPUTE_FORMAT_SHIFT = BGFX_BUFFER_COMPUTE_FORMAT_SHIFT as u16,
        const BUFFER_COMPUTE_FORMAT_MASK = BGFX_BUFFER_COMPUTE_FORMAT_MASK,
        const BUFFER_COMPUTE_TYPE_UINT = BGFX_BUFFER_COMPUTE_TYPE_UINT,
        const BUFFER_COMPUTE_TYPE_INT = BGFX_BUFFER_COMPUTE_TYPE_INT,
        const BUFFER_COMPUTE_TYPE_FLOAT = BGFX_BUFFER_COMPUTE_TYPE_FLOAT,
        const BUFFER_COMPUTE_TYPE_SHIFT = BGFX_BUFFER_COMPUTE_TYPE_SHIFT as u16,
        const BUFFER_COMPUTE_TYPE_MASK = BGFX_BUFFER_COMPUTE_TYPE_MASK,
        const BUFFER_COMPUTE_READ = BGFX_BUFFER_COMPUTE_READ,
        const BUFFER_COMPUTE_WRITE = BGFX_BUFFER_COMPUTE_WRITE,
        const BUFFER_DRAW_INDIRECT = BGFX_BUFFER_DRAW_INDIRECT,
        const BUFFER_ALLOW_RESIZE = BGFX_BUFFER_ALLOW_RESIZE,
        const BUFFER_INDEX32 = BGFX_BUFFER_INDEX32,
        const BUFFER_COMPUTE_READ_WRITE = BGFX_BUFFER_COMPUTE_READ_WRITE,
    }
}

impl Default for BufferFlags {
    #[inline]
    fn default() -> BufferFlags {
        BUFFER_NONE
    }
}

bitflags! {
    flags ClearFlags: u16 {
        const CLEAR_NONE = BGFX_CLEAR_NONE,
        const CLEAR_COLOR = BGFX_CLEAR_COLOR,
        const CLEAR_DEPTH = BGFX_CLEAR_DEPTH,
        const CLEAR_STENCIL = BGFX_CLEAR_STENCIL,
        const CLEAR_DISCARD_COLOR_0 = BGFX_CLEAR_DISCARD_COLOR_0,
        const CLEAR_DISCARD_COLOR_1 = BGFX_CLEAR_DISCARD_COLOR_1,
        const CLEAR_DISCARD_COLOR_2 = BGFX_CLEAR_DISCARD_COLOR_2,
        const CLEAR_DISCARD_COLOR_3 = BGFX_CLEAR_DISCARD_COLOR_3,
        const CLEAR_DISCARD_COLOR_4 = BGFX_CLEAR_DISCARD_COLOR_4,
        const CLEAR_DISCARD_COLOR_5 = BGFX_CLEAR_DISCARD_COLOR_5,
        const CLEAR_DISCARD_COLOR_6 = BGFX_CLEAR_DISCARD_COLOR_6,
        const CLEAR_DISCARD_COLOR_7 = BGFX_CLEAR_DISCARD_COLOR_7,
        const CLEAR_DISCARD_DEPTH = BGFX_CLEAR_DISCARD_DEPTH,
        const CLEAR_DISCARD_STENCIL = BGFX_CLEAR_DISCARD_STENCIL,
        const CLEAR_DISCARD_COLOR_MASK = BGFX_CLEAR_DISCARD_COLOR_MASK,
        const CLEAR_DISCARD_MASK = BGFX_CLEAR_DISCARD_MASK,
    }
}

impl Default for ClearFlags {
    #[inline]
    fn default() -> ClearFlags {
        CLEAR_NONE
    }
}

bitflags! {
    flags DebugFlags: u32 {
        const DEBUG_NONE = BGFX_DEBUG_NONE,
        const DEBUG_WIREFRAME = BGFX_DEBUG_WIREFRAME,
        const DEBUG_IFH = BGFX_DEBUG_IFH,
        const DEBUG_STATS = BGFX_DEBUG_STATS,
        const DEBUG_TEXT = BGFX_DEBUG_TEXT,
    }
}

impl Default for DebugFlags {
    #[inline]
    fn default() -> DebugFlags {
        DEBUG_NONE
    }
}

bitflags! {
    flags ResetFlags: u32 {
        const RESET_NONE = BGFX_RESET_NONE,
        const RESET_FULLSCREEN = BGFX_RESET_FULLSCREEN,
        const RESET_FULLSCREEN_SHIFT = BGFX_RESET_FULLSCREEN_SHIFT,
        const RESET_FULLSCREEN_MASK = BGFX_RESET_FULLSCREEN_MASK,
        const RESET_MSAA_X2 = BGFX_RESET_MSAA_X2,
        const RESET_MSAA_X4 = BGFX_RESET_MSAA_X4,
        const RESET_MSAA_X8 = BGFX_RESET_MSAA_X8,
        const RESET_MSAA_X16 = BGFX_RESET_MSAA_X16,
        const RESET_MSAA_SHIFT = BGFX_RESET_MSAA_SHIFT,
        const RESET_MSAA_MASK = BGFX_RESET_MSAA_MASK,
        const RESET_VSYNC = BGFX_RESET_VSYNC,
        const RESET_MAXANISOTROPY = BGFX_RESET_MAXANISOTROPY,
        const RESET_CAPTURE = BGFX_RESET_CAPTURE,
        const RESET_HMD = BGFX_RESET_HMD,
        const RESET_HMD_DEBUG = BGFX_RESET_HMD_DEBUG,
        const RESET_HMD_RECENTER = BGFX_RESET_HMD_RECENTER,
        const RESET_FLUSH_AFTER_RENDER = BGFX_RESET_FLUSH_AFTER_RENDER,
        const RESET_FLIP_AFTER_RENDER = BGFX_RESET_FLIP_AFTER_RENDER,
        const RESET_SRGB_BACKBUFFER = BGFX_RESET_SRGB_BACKBUFFER,
        const RESET_HIDPI = BGFX_RESET_HIDPI,
    }
}

impl Default for ResetFlags {
    #[inline]
    fn default() -> ResetFlags {
        RESET_NONE
    }
}
