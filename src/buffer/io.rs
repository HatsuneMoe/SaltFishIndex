use std::mem::transmute;
use std::convert::TryInto;
use varint_simd::{encode, decode};
use std::borrow::Borrow;
use std::ops::Range;

pub struct Buffer {
    buf: Vec<u8>,
}

pub fn new_buffer() -> Buffer {
    Buffer {
        buf: Vec::new(),
    }
}

impl Buffer {
    pub fn encode_varbyte32(&mut self, input: u32) {
        // todo: varbyte
        let (encoded, len) = encode::<u32>(input);
        self.buf.extend_from_slice(&encoded[0..len as usize]);
    }

    pub fn decode_varbyte32(&self, input: &[u8]) -> (u32, usize) {
        let (decoded, len) = decode::<u32>(input).unwrap();
        (decoded, len as usize)
    }

    pub fn alloc(&mut self, len: usize) {
        self.buf.append(&mut vec![0; len]);
    }

    pub fn clear(&mut self) {
        self.buf.clear();
    }

    pub fn size(&self) -> usize {
        self.buf.len()
    }

    pub fn print(&self) {
        let arr:[u8;4] = self.buf[0..4].try_into().unwrap();
        let num = unsafe { transmute::<[u8; 4], u32>(arr) }.to_le();
        print!("num: {} \n", num);
    }

    pub fn data(&self) -> &[u8] {
        self.buf.borrow()
    }

    pub fn base_ptr(&mut self) -> Range<*mut u8> {
        self.buf.as_mut_ptr_range()
    }
}

pub trait Serialize<T> {
    fn pack(&mut self, data: T);
}

impl Serialize<&[u8]> for Buffer {
    fn pack(&mut self, data: &[u8]) {
        self.buf.extend_from_slice(data);
    }
}

impl Serialize<&Buffer> for Buffer {
    fn pack(&mut self, data: &Buffer) {
        self.buf.extend_from_slice(data.data());
    }
}

impl Serialize<u8> for Buffer {
    fn pack(&mut self, data: u8) {
        self.buf.push(data);
    }
}

impl Serialize<u32> for Buffer {
    fn pack(&mut self, data: u32) {
        let bytes: [u8; 4] = unsafe { transmute(data.to_le()) };
        self.buf.extend_from_slice(&bytes);
    }
}