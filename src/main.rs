use crate::codec::{Encoder, Decoder};
use std::ptr::null;
use crate::terms::{new_term, pack_terms};

mod codec;
mod common;
mod buffer;
mod terms;
mod index;

fn codec_test() {
    codec::test();
    let mut buf = buffer::io::new_buffer();
    buf.encode_varbyte32(35);
    let data: &[u8] = buf.data();
    print!("{:?}\n", buf.decode_varbyte32(data));

    let mut s = codec::encoder::new_salt_fish_enc();
    s.begin_term();
    s.begin_document(1);
    s.new_hit(1);
    s.end_document();
    s.begin_document(2);
    s.new_hit(3);
    s.new_hit(5);
    s.end_document();
    s.begin_document(3);
    s.new_hit(7);
    s.new_hit(9);
    s.end_document();
    s.end_term(&mut new_term("a"));

    s.debug_print();
}

fn segment_test() {
    let mut s = codec::encoder::new_salt_fish_enc();
    let mut seg = index::segment::new_segment(s);
    seg.new_document(1, vec!["hello", "world", "world", "is", "hello"]);
    seg.new_document(2, vec!["hello", "salt", "fish"]);
    seg.commit();

    print!("find hello: {:?}\n", seg.find("hello"));
    print!("find world: {:?}\n", seg.find("world"));
    print!("find salt: {:?}\n", seg.find("salt"));
}

fn term_test() {
    pack_terms(vec![
        terms::new_term("bcd"),
        terms::new_term("abc"),
        terms::new_term("acd"),
        terms::new_term("🍣")]);
}

fn main() {
    segment_test();
}
