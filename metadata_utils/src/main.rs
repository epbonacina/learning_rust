mod doc_type;
mod doc_utils;
mod document;

use doc_type::DocType;
use document::Document;

fn make_documents() -> Vec<Document> {
    vec![
        Document{tipo: DocType::Decisao, descricao: String::from("Decisão")},
        Document{tipo: DocType::AtoOrdinatorio, descricao: String::from("Ato Ordinatório")},
        Document{tipo: DocType::Certidao, descricao: String::from("Certidão")},
    ]
}

fn main() {
    let documents = make_documents();

    println!("Includes 'certidao'? {}", 
             doc_utils::includes_doc_type(&documents, DocType::Certidao));
    println!("Includes 'peticao inicial'? {}", 
             doc_utils::includes_doc_type(&documents, DocType::PeticaoInicial));
}
