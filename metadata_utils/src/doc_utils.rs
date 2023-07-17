use super::doc_type::DocType;
use super::document::Document;

pub fn includes_doc_type(documents: &[Document], doc_type: DocType) -> bool {
    documents.iter().any(|d| d.tipo == doc_type)
}

pub fn is_doc_type(doc_meta: &Document, doc_types: &[DocType]) -> bool {
    doc_types
        .iter()
        .any(|t| t.descricoes().iter().any(|&d| d == doc_meta.descricao.to_lowercase()))
}


#[cfg(test)]
mod tests {
    use super::*;

    fn make_documents() -> Vec<Document> {
        vec![
            Document {
                tipo: DocType::Decisao,
                descricao: String::from("Decisão"),
            },
            Document {
                tipo: DocType::AtoOrdinatorio,
                descricao: String::from("Ato Ordinatório"),
            },
            Document {
                tipo: DocType::Certidao,
                descricao: String::from("Certidão"),
            },
        ]
    }

    #[test]
    fn first_document_should_be_of_type_decisao() {
        let documents = make_documents();
        
        assert!(is_doc_type(&documents[0], &[DocType::Decisao]));
    }
    
    #[test]
    fn second_document_should_not_be_of_type_certidao() {
        let documents = make_documents();
        
        assert!(!is_doc_type(&documents[1], &[DocType::Certidao]));
    }
    
    #[test]
    fn doc_list_should_include_ato_ordinatorio() {
        let documents = make_documents();

        assert!(includes_doc_type(&documents, DocType::AtoOrdinatorio));
    }
    
    #[test]
    fn doc_list_should_not_include_peticao_inicial() {
        let documents = make_documents();

        assert!(!includes_doc_type(&documents, DocType::PeticaoInicial));
    }
}
