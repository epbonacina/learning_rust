use super::doc_type::DocType;

#[derive(Clone)]
pub struct Document {
    pub tipo: DocType,
    pub descricao: String,
}
