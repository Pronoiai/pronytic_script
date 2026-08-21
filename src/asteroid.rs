use std::fmt;

use lalrpop_util::{ParseError, lalrpop_mod};
use logos::{self, Logos};

use crate::{LexicalError, common::DataParser};

#[derive(Logos, Clone, Debug, PartialEq)]
#[logos(skip r"[\s\t\f]+", error = LexicalError)]
#[logos(skip r"//[^\n\r]*?")]
pub enum AsteroidToken {
    #[regex(r#""[^"]*""#, |lex| lex.slice().trim_matches('"').to_string())]
    String(String),

    #[token("=")]
    Equal,

    #[token("[")]
    LeftSquare,
    #[token("]")]
    RightSquare,

    #[token("asset_locations")]
    AssetLocations,
}

impl fmt::Display for AsteroidToken {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

lalrpop_mod!(pub asteroid);

#[derive(Clone, Default, Debug)]
pub struct AsteroidData {
    pub location: String,
}

impl<'s> DataParser<'s> for AsteroidData {
    type Token = AsteroidToken;
    fn parse_tokens(
        tokens: Vec<(usize, Self::Token, usize)>,
    ) -> Result<Vec<Self>, ParseError<usize, Self::Token, String>> {
        asteroid::AsteroidDataParser::new().parse(tokens)
    }
}
