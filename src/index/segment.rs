use std::collections::HashMap;
use std::collections::hash_map::Entry;

use crate::common;
use crate::codec::{Encoder};
use crate::terms::{Term, new_term};
use crate::index::statistics::{Field, new_field_statistics};

pub struct Segment<'term> {
    encoder: Box<dyn Encoder>,
    // hashmap[term_str] = term_id
    dictionary: HashMap<&'term str, u32>,
    // vec[term_id] = term_info
    invert_dict: Vec<Term<'term>>,

    // todo: buffer range
    uncommitted_buffer: HashMap<common::TDocId, Vec<(u32, Vec<common::TTokenPos>)>>,

    default_stat: Field,
}

pub fn new_segment<'term>(encoder: Box<dyn Encoder>) -> Segment<'term> {
    Segment{
        encoder,
        dictionary: HashMap::new(),
        invert_dict: Vec::new(),
        uncommitted_buffer: HashMap::new(),
        default_stat: new_field_statistics(),
    }
}

impl<'term> Segment<'term> {
    fn term_id(&mut self, term: &'term str) -> u32 {
        let term_id = match self.dictionary.entry(term) {
            Entry::Occupied(id) => {
                *id.get()
            }
            Entry::Vacant(id) => {
                self.invert_dict.push(new_term(term));
                let id = self.dictionary.len() as u32;
                self.dictionary.insert(term, id);
                id
            }
        };
        term_id
    }

    fn insert(&mut self, term: &'term str, pos: common::TTokenPos) {
        print!("term: {}, term_id: {}, pos: {}\n", term, self.term_id(term),pos);
    }

    pub fn new_document(&mut self, doc_id: common::TDocId, terms: Vec<&'term str>) {
        let mut hits: HashMap<u32, Vec<common::TTokenPos>> = HashMap::new();
        for (i, term) in terms.iter().enumerate() {
            let term_id = self.term_id(term);
            let pos = i as common::TTokenPos + 1;
            match hits.entry(term_id) {
                Entry::Occupied(mut entry) => {
                    entry.get_mut().push(pos);
                }
                Entry::Vacant(entry) => {
                    entry.insert(vec![pos]);
                }
            }
        }

        let mut hits_vec: Vec<_> = hits.into_iter().collect();
        hits_vec.sort_by(|x,y| x.0.cmp(&y.0));
        self.uncommitted_buffer.insert(doc_id, hits_vec);
    }

    pub fn commit(&mut self) {
        for (doc_id, hits) in self.uncommitted_buffer.iter() {
            for (term_id, token_pos) in hits.iter() {
                self.invert_dict[*term_id as usize].append((doc_id.clone(), token_pos.clone()));
            }
            print!("doc_id: {}, hits: {:?}\n", doc_id, hits)
        }
        print!("term_dict: {:?}, invert_dict: {:?}\n", self.dictionary, self.invert_dict);

        self.uncommitted_buffer.clear();
    }

    pub fn find(&self, term: &str) -> Option<Vec<common::TDocId>> {
        let term_id = *self.dictionary.get(term)? as usize;
        Some(self.invert_dict[term_id].get_docs())
    }

    pub fn save(&mut self) {
        for term in self.invert_dict.iter_mut() {
            self.encoder.begin_term();
            for (doc_id, pos_data) in term.get_data() {
                self.encoder.begin_document(doc_id);
                for pos in pos_data.iter() {
                    self.encoder.new_hit(pos.clone());
                }
                self.encoder.end_document();
            }
            self.encoder.end_term(term);
        }

    }
}