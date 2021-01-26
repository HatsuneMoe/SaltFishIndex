use crate::buffer::io::{Buffer, Serialize, new_buffer};
use std::fmt::{Debug, Formatter, Result};
use crate::common::{TDocId, TTokenPos};
use std::borrow::Borrow;

pub struct Term<'term> {
    name: &'term str,
    documents: u32,
    // todo: range hits data
    // index_len: u32,
    // index_offset: u32,
    hits: Vec<(TDocId, Vec<TTokenPos>)>,
}

impl<'term> Term<'term> {
    pub fn append(&mut self, data: (TDocId, Vec<TTokenPos>)) {
        match self.hits.binary_search_by(|probe| probe.0.cmp(&data.0)) {
            Ok(pos) => { self.hits[pos] = data; }
            Err(pos) => {
                self.documents += 1;
                self.hits.insert(pos,data);
            }
        }
    }

    pub fn get_docs(&self) -> Vec<TDocId> {
        self.hits.iter().map(|i| i.0).collect()
    }

    pub fn get_data(&self) -> Vec<(TDocId, Vec<TTokenPos>)> {
        self.hits.to_owned()
    }
}

impl<'term> Debug for Term<'term> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.debug_struct("Term")
            .field("name", &self.name)
            .field("documents", &self.documents)
            .field("hits", &self.hits)
            .finish()
    }
}

pub fn new_term(term: &str) -> Term {
    Term{
        name: term,
        documents: 0,
        hits: Vec::new(),
        // index_len: 0,
        // index_offset: 0
    }
}

fn common_prefix(s1: &str, s2: &str) -> u8 {
    s1.as_bytes().iter().zip(s2.as_bytes()).take_while(|(x, y)| x == y).count() as u8
}

pub fn pack_terms(mut terms: Vec<Term>) {
    let mut out = new_buffer();
    terms.sort_by(|a, b| a.name.cmp(b.name));
    let mut prev: &str = "";
    for term in terms {
        let prefix = common_prefix(prev, term.name);
        print!("prefix: {}\n", prefix);
        let (_, suffix) = term.name.split_at(prefix as usize);
        print!("suffix: {}, len: {}\n", suffix, suffix.len());

        out.pack(prefix);
        out.pack(suffix.len() as u8);
        out.pack(suffix.as_bytes());
        out.encode_varbyte32(term.documents);
        // self.out.encode_varbyte32(term.index_len);
        // self.out.pack(term.index_offset);
        prev = term.name;
    }
}