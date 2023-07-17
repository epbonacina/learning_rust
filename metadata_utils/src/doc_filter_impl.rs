use super::document::Document;
use super::doc_filter::DocFilter;
use super::doc_type::DocType;
use super::doc_utils::is_doc_type;

pub struct DType {
    pub doc_types: Vec<DocType>,
}

impl DocFilter for DType {
    fn apply(&self, documents: &[Document]) -> Vec<Document> {
        let mut filtered_docs = vec![];

        for doc in documents {
            if is_doc_type(doc, &self.doc_types) {
                filtered_docs.push(doc.clone());
            }
        }

        filtered_docs
    }
}
