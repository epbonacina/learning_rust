use std::collections::HashMap;
use lazy_static::lazy_static;

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub enum DocType {
    AtoOrdinatorio,
    Certidao,
    Decisao,
    PeticaoInicial,
}

lazy_static!{
    static ref TYPE_DESCRIPTIONS_MAP: HashMap<DocType, Vec<&'static str>> = vec![
        (DocType::AtoOrdinatorio, vec!["ato ordinatório"]),
        (DocType::Certidao, vec!["certidao", "certidões", "certidão (outras)"]),
        (DocType::Decisao, vec!["decisao", "decisoes"]),
        (DocType::PeticaoInicial, vec!["petição inicial"]),
    ].into_iter().collect();
}

impl DocType {
    pub fn descricoes(self) -> &'static[&'static str] {
        TYPE_DESCRIPTIONS_MAP.get(&self).unwrap()
    }
}
