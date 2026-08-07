use std::{fmt, str::FromStr};

use lalrpop_util::lalrpop_mod;
use logos::{self, Logos};
use rust_decimal::prelude::*;

use crate::{
    LexicalError,
    common::{DataParser, GoodConsumes},
};

#[derive(Logos, Clone, Debug, PartialEq)]
#[logos(skip r"[\s\t\f]+", error = LexicalError)]
#[logos(skip r"//[^\n\r]*?")]
pub enum OrbitalToken {
    #[regex(r#""[^"]*""#, |lex| lex.slice().trim_matches('"').to_string())]
    String(String),

    #[regex(r"(\d+)", |lex|lex.slice().parse::<u8>().expect("parsing u8"), priority = 5)]
    Number(u8),

    #[regex(r"(\d+\.?\d*)", |lex| Decimal::from_str(lex.slice()).expect("parsed_decimal"), priority = 4)]
    DecimalNumber(Decimal),

    #[token("=")]
    Equal,

    #[token("{")]
    LeftCurly,
    #[token("}")]
    RightCurly,
    #[token("[")]
    LeftSquare,
    #[token("]")]
    RightSquare,

    #[token(":")]
    Colon,

    #[token("name")]
    Name,

    #[token("consumes")]
    Consumes,
    #[token("good_id")]
    GoodId,
    #[token("amount")]
    Amount,

    #[token("time")]
    Time,
    #[token("building_limit")]
    BuildingLimit,

    #[token("magnetosphere")]
    Magnetosphere,
    #[token("atmosphere")]
    Atmosphere,

    #[token("placement")]
    Placement,

    #[token("right")]
    Right,
    #[token("up")]
    Up,
    #[token("back")]
    Back,
    #[token("scale")]
    Scale,
    #[token("path")]
    Path,
}

impl fmt::Display for OrbitalToken {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

lalrpop_mod!(pub orbital);
#[derive(Clone, Default, Debug)]
pub struct OrbitalData {
    pub level: u8,
    pub name: String,

    pub costs: Vec<GoodConsumes>,

    pub placement: Placement,

    pub time: u8,
    pub building_limit: u8,
}

#[derive(Clone, Debug)]
pub struct Placement {
    pub right: f32,
    pub up: f32,
    pub back: f32,

    pub scale: f32,
    pub asset_location: String,
}

impl Default for Placement {
    fn default() -> Self {
        Self {
            right: Default::default(),
            up: Default::default(),
            back: Default::default(),
            scale: 1.0,
            asset_location: Default::default(),
        }
    }
}

/// Differentiates between each field when parsing
/// This allows fields to be done in arbitrary order in lalrpop files
pub enum Field {
    Name(String),
    Consumes(Vec<GoodConsumes>),
    Time(u8),
    BuildingLimit(u8),
    Placement(Placement),
}

pub enum PlacementField {
    Right(f32),
    Up(f32),
    Back(f32),
    Scale(f32),
    Path(String),
}

impl<'s> DataParser<'s> for OrbitalData {
    type Token = OrbitalToken;
    fn parse_tokens(
        tokens: Vec<(usize, Self::Token, usize)>,
    ) -> Result<Vec<OrbitalData>, lalrpop_util::ParseError<usize, Self::Token, String>> {
        orbital::OrbitalDataParser::new().parse(tokens)
    }
}
