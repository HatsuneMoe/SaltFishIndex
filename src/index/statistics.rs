pub struct Field {
    // lucene: Terms::getSumTotalTermFreq()
    sum_total_term_freq: u64,
    // lucene: Terms::size()
    total_terms: u32,
    // lucene: Terms::getSumDocFreq()
    sum_term_doc_freq: u64,
    docs_count: u32,
}

pub fn new_field_statistics() -> Field {
    Field {
        sum_total_term_freq: 0,
        total_terms: 0,
        sum_term_doc_freq: 0,
        docs_count: 0
    }
}