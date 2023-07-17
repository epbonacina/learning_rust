mod doc_filter;
mod doc_filter_impl;
mod doc_type;
mod doc_utils;
mod document;

use doc_filter::DocFilter;
use doc_filter_impl::DType;
use doc_type::DocType;
use document::Document;

fn main() {
    let f = DType {
        doc_types: vec![DocType::Certidao, DocType::Decisao],
    };

    let documents = vec![
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
    ];

    println!(
        "Matches 'certidao' or 'decisao'? {}",
        f.matches(&documents[0])
    );
}
