use super::doc_type::DocType;

#[derive(Clone, Debug, PartialEq)]
pub struct Document {
    pub tipo: DocType,
    pub descricao: String,
}
