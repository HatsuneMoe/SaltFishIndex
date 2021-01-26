use crate::codec::Encoder;
use crate::common;
use crate::buffer::io::{new_buffer, Serialize, Buffer};

use std::mem;
use crate::terms::Term;

const BLOCK_SIZE: usize = 32;

pub struct SaltFishEnc {
    cur_doc_id: common::TDocId,
    last_committed_doc_id: common::TDocId,
    prev_block_last_document_id: common::TDocId,
    cur_block_size: u8,
    last_pos: u32,
    term_docs: u32,
    cur_term_offset: u32,
    block_freqs: [u32; BLOCK_SIZE],
    doc_deltas: [common::TDocId; BLOCK_SIZE],
    out: Buffer,
    block: Buffer,
    hits_data: Buffer,
}

pub fn new_salt_fish_enc() -> Box<dyn Encoder> {
    Box::new(SaltFishEnc{
        cur_doc_id: 0,
        last_committed_doc_id: 0,
        prev_block_last_document_id: 0,
        cur_block_size: 0,
        last_pos: 0,
        term_docs: 0,
        cur_term_offset: 0,
        block_freqs: [0; BLOCK_SIZE],
        doc_deltas: [0; BLOCK_SIZE],
        out: new_buffer(),
        block: new_buffer(),
        hits_data: new_buffer()
    })
}

impl Encoder for SaltFishEnc {
    fn begin_term(&mut self) {
        self.cur_block_size = 0;
        self.last_committed_doc_id = 0;
        self.prev_block_last_document_id = 0;
        self.term_docs = 0;

        // skiplist header, alloc in begin_term, fill in end_term
        self.out.alloc(mem::size_of::<u16>())
    }

    fn end_term(&mut self, term_info: &mut Term) {
        if self.cur_block_size > 0 {
            self.commit_block()
        }

        print!("ENDING term {} \n", self.cur_term_offset);

        // todo: skiplist


    }

    fn begin_document(&mut self, document_id: u32) {
        // todo: unlikely
        if document_id <= self.last_committed_doc_id {
            panic!("Unexpected document_id({}) <= last_committed_doc_id({})\n",
                   document_id,
                   self.last_committed_doc_id
            );
        }

        self.cur_doc_id = document_id;
        self.last_pos = 0;
        self.block_freqs[self.cur_block_size as usize] = 0;
    }

    fn end_document(&mut self) {
        print!("end document {} {} {}\n", self.cur_doc_id, self.last_committed_doc_id, self.cur_block_size);

        self.doc_deltas[self.cur_block_size as usize] = self.cur_doc_id - self.last_committed_doc_id;
        self.cur_block_size += 1;

        if self.cur_block_size == BLOCK_SIZE as u8 {
            self.commit_block();
        }

        self.last_committed_doc_id = self.cur_doc_id;
        self.term_docs += 1;
    }

    fn commit_block(&mut self) {
        let delta = self.cur_doc_id - self.prev_block_last_document_id;
        let n: usize = self.cur_block_size as usize;
        // let n: usize = self.cur_block_size as usize - 1;

        print!("Committing block, cur_block_size = {}, cur_doc_id = {}, prev_block_last_document_id = {}, delta = {}  out->size() + sess->indexOutFlushed\n", self.cur_block_size, self.cur_doc_id, self.prev_block_last_document_id, delta);

        self.block.clear();

        print!("doc_delta: {:?}\n", self.doc_deltas);
        for i in 0..n {
            print!("<< {}\n", self.doc_deltas[i]);
            self.block.encode_varbyte32(self.doc_deltas[i]);
        }

        print!("block_freqs: {:?}\n", self.block_freqs);
        for i in 0..n {
            print!("<< freq {}\n", self.block_freqs[i]);
            self.block.encode_varbyte32(self.block_freqs[i]);
        }

        let block_length: u32 = self.block.size() as u32 + self.hits_data.size() as u32;

        // todo: skiplist

        print!("<< block_length {}\n", block_length);
        self.out.encode_varbyte32(delta);
        self.out.encode_varbyte32(block_length);
        self.out.pack(self.cur_block_size);
        self.out.pack(&self.block);
        print!("<< block size: {}\n", self.block.size());
        self.out.pack(&self.hits_data);
        print!("<< hits_data size: {}\n", self.hits_data.size());
        self.hits_data.clear();

        self.prev_block_last_document_id = self.cur_doc_id;
        self.cur_block_size = 0;
    }

    fn new_hit(&mut self, pos: u32) {
        // assert!(pos < Limits::MaxPosition);
        // assert!(pos >= self.last_pos);

        let delta: u32 = pos - self.last_pos;
        // todo: payload
        print!("HIT {} => {}\n", pos, delta);

        self.block_freqs[self.cur_block_size as usize] += 1;
        self.hits_data.encode_varbyte32(delta);
        self.last_pos = pos;
    }

    fn data(&self) -> &[u8] {
        self.out.data()
    }

    fn debug_print(&self) {
        print!("{:?}", self.out.data());
    }
}