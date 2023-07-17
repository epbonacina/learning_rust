use std::collections::HashMap;
use lazy_static::lazy_static;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum DocType {
    AtoOrdinatorio,
    Certidao,
    Decisao,
    PeticaoInicial,
}

lazy_static!{
    static ref TYPE_DESCRIPTIONS_MAP: HashMap<DocType, Vec<&'static str>> = vec![
        (DocType::AtoOrdinatorio, vec!["ato ordinatório"]),
        (DocType::Certidao, vec!["certidão", "certidões", "certidão (outras)"]),
        (DocType::Decisao, vec!["decisão", "decisões"]),
        (DocType::PeticaoInicial, vec!["petição inicial"]),
    ].into_iter().collect();
}

impl DocType {
    pub fn descricoes(self) -> &'static[&'static str] {
        TYPE_DESCRIPTIONS_MAP.get(&self).unwrap()
    }
}
