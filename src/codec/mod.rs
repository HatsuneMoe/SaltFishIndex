pub mod encoder;
pub mod decoder;

use crate::common;
use crate::terms::Term;

pub trait Encoder {
    fn begin_term(&mut self);
    fn end_term(&mut self, term_info: &mut Term);
    fn begin_document(&mut self, document_id: common::TDocId);
    fn end_document(&mut self);
    fn commit_block(&mut self);
    fn new_hit(&mut self, pos: u32);
    fn data(&self) -> &[u8];

    fn debug_print(&self);
}

pub trait Decoder {

}

pub fn test() {
    let a: common::TDocId = 10;
    print!("hello world: {} \n", a);
}