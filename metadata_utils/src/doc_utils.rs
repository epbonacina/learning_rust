use super::doc_type::DocType;
use super::document::Document;

pub fn includes_doc_type(documents: &[Document], doc_type: DocType) -> bool {
    documents.iter().any(|d| d.tipo == doc_type)
}
