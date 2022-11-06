// automatically generated by the FlatBuffers compiler, do not modify
extern crate alloc;
extern crate flatbuffers;
use alloc::boxed::Box;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::mem;
use core::cmp::Ordering;
use self::flatbuffers::{EndianScalar, Follow};
use super::*;
pub enum TopicHandleOffset {}
#[derive(Copy, Clone, PartialEq)]

/// A handle for the topic, allows referencing a topic without sending a huge
/// `TopicId`.
pub struct TopicHandle<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for TopicHandle<'a> {
  type Inner = TopicHandle<'a>;
  #[inline]
  fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    Self { _tab: flatbuffers::Table { buf, loc } }
  }
}

impl<'a> TopicHandle<'a> {
  pub const VT_ID: flatbuffers::VOffsetT = 4;

  #[inline]
  pub fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
    TopicHandle { _tab: table }
  }
  #[allow(unused_mut)]
  pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
    _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
    args: &'args TopicHandleArgs
  ) -> flatbuffers::WIPOffset<TopicHandle<'bldr>> {
    let mut builder = TopicHandleBuilder::new(_fbb);
    builder.add_id(args.id);
    builder.finish()
  }


  #[inline]
  pub fn id(&self) -> u16 {
    self._tab.get::<u16>(TopicHandle::VT_ID, Some(0)).unwrap()
  }
}

impl flatbuffers::Verifiable for TopicHandle<'_> {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.visit_table(pos)?
     .visit_field::<u16>("id", Self::VT_ID, false)?
     .finish();
    Ok(())
  }
}
pub struct TopicHandleArgs {
    pub id: u16,
}
impl<'a> Default for TopicHandleArgs {
  #[inline]
  fn default() -> Self {
    TopicHandleArgs {
      id: 0,
    }
  }
}

pub struct TopicHandleBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> TopicHandleBuilder<'a, 'b> {
  #[inline]
  pub fn add_id(&mut self, id: u16) {
    self.fbb_.push_slot::<u16>(TopicHandle::VT_ID, id, 0);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> TopicHandleBuilder<'a, 'b> {
    let start = _fbb.start_table();
    TopicHandleBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<TopicHandle<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

impl core::fmt::Debug for TopicHandle<'_> {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut ds = f.debug_struct("TopicHandle");
      ds.field("id", &self.id());
      ds.finish()
  }
}
