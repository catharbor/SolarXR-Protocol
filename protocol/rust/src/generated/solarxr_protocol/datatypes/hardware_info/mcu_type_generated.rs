// automatically generated by the FlatBuffers compiler, do not modify
// @generated
extern crate alloc;
extern crate flatbuffers;
use alloc::boxed::Box;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::mem;
use core::cmp::Ordering;
use self::flatbuffers::{EndianScalar, Follow};
use super::*;
#[deprecated(since = "2.0.0", note = "Use associated constants instead. This will no longer be generated in 2021.")]
pub const ENUM_MIN_MCU_TYPE: u16 = 0;
#[deprecated(since = "2.0.0", note = "Use associated constants instead. This will no longer be generated in 2021.")]
pub const ENUM_MAX_MCU_TYPE: u16 = 2;
#[deprecated(since = "2.0.0", note = "Use associated constants instead. This will no longer be generated in 2021.")]
#[allow(non_camel_case_types)]
pub const ENUM_VALUES_MCU_TYPE: [McuType; 3] = [
  McuType::Other,
  McuType::ESP8266,
  McuType::ESP32,
];

/// Currently firmware only reports ESP8266 or if the device uses ESP-IDF
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct McuType(pub u16);
#[allow(non_upper_case_globals)]
impl McuType {
  pub const Other: Self = Self(0);
  pub const ESP8266: Self = Self(1);
  pub const ESP32: Self = Self(2);

  pub const ENUM_MIN: u16 = 0;
  pub const ENUM_MAX: u16 = 2;
  pub const ENUM_VALUES: &'static [Self] = &[
    Self::Other,
    Self::ESP8266,
    Self::ESP32,
  ];
  /// Returns the variant's name or "" if unknown.
  pub fn variant_name(self) -> Option<&'static str> {
    match self {
      Self::Other => Some("Other"),
      Self::ESP8266 => Some("ESP8266"),
      Self::ESP32 => Some("ESP32"),
      _ => None,
    }
  }
}
impl core::fmt::Debug for McuType {
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    if let Some(name) = self.variant_name() {
      f.write_str(name)
    } else {
      f.write_fmt(format_args!("<UNKNOWN {:?}>", self.0))
    }
  }
}
impl<'a> flatbuffers::Follow<'a> for McuType {
  type Inner = Self;
  #[inline]
  unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    let b = flatbuffers::read_scalar_at::<u16>(buf, loc);
    Self(b)
  }
}

impl flatbuffers::Push for McuType {
    type Output = McuType;
    #[inline]
    unsafe fn push(&self, dst: &mut [u8], _written_len: usize) {
        flatbuffers::emplace_scalar::<u16>(dst, self.0);
    }
}

impl flatbuffers::EndianScalar for McuType {
  type Scalar = u16;
  #[inline]
  fn to_little_endian(self) -> u16 {
    self.0.to_le()
  }
  #[inline]
  #[allow(clippy::wrong_self_convention)]
  fn from_little_endian(v: u16) -> Self {
    let b = u16::from_le(v);
    Self(b)
  }
}

impl<'a> flatbuffers::Verifiable for McuType {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    u16::run_verifier(v, pos)
  }
}

impl flatbuffers::SimpleToVerifyInSlice for McuType {}
