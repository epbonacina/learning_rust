use super::document::Document;
use super::doc_filter::DocFilter;
use super::doc_type::DocType;

pub struct DType {
    pub doc_types: Vec<DocType>,
}

impl DocFilter for DType {
    fn apply(&self, documents: &[Document]) -> Vec<Document> {
        documents.to_vec()
    }
}
