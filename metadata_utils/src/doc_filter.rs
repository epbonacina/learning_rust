use super::document::Document;

pub trait DocFilter {
    fn apply(&self, documents: &[Document]) -> Vec<Document>;
    fn matches(&self, document: &Document) -> bool {
        self.apply(&[document.clone()]).len() == 1
    }
}
