use qqml_lexer::Lexer;
use crate::MultichoiceData;
use crate::error::ErrorReport;

pub fn parse_multichoice(l: &mut Lexer) -> Result<MultichoiceData, ErrorReport> {
    Ok(MultichoiceData::default())
}
