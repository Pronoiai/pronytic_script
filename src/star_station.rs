use std::{fmt, str::FromStr};

use lalrpop_util::lalrpop_mod;
use logos::{self, Logos};
use rust_decimal::prelude::*;

use crate::{
    LexicalError,
    common::{DataParser, Placement},
};

#[derive(Logos, Clone, Debug, PartialEq)]
#[logos(skip r"[\s\t\f]+", error = LexicalError)]
#[logos(skip r"//[^\n\r]*?")]
pub enum StarStationToken {
    #[regex(r#""[^"]*""#, |lex| lex.slice().trim_matches('"').to_string())]
    String(String),

    #[regex(r"(\d+)", |lex|lex.slice().parse::<u8>().expect("parsing u8"), priority = 5)]
    Number(u8),

    #[regex(r"(-?\d+\.?\d*)", |lex| Decimal::from_str(lex.slice()).expect("parsed_decimal"), priority = 4)]
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

    #[token("name")]
    Name,

    #[token("station_class")]
    StationClass,

    #[token("construction")]
    Construction,

    #[token("population")]
    Population,

    #[token("resource")]
    Resource,

    #[token("statite")]
    Statite,

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

impl fmt::Display for StarStationToken {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

lalrpop_mod!(pub star_station);

#[derive(Clone, Default, Debug)]
pub struct StarStationData {
    pub level: u8,
    pub name: String,
    pub station_class: StationClass,
    pub placement: Placement,
}

#[derive(Clone, Default, Debug, PartialEq)]
pub enum StationClass {
    #[default]
    Construction,
    Population,
    Resource,
    Statite,
}

pub enum Field {
    Name(String),
    StationClass(StationClass),
    Placement(Placement),
}

pub enum PlacementField {
    Right(f32),
    Up(f32),
    Back(f32),
    Scale(f32),
    Path(String),
}

impl<'s> DataParser<'s> for StarStationData {
    type Token = StarStationToken;

    fn parse_tokens(
        tokens: Vec<(usize, Self::Token, usize)>,
    ) -> Result<Vec<Self>, lalrpop_util::ParseError<usize, Self::Token, String>> {
        star_station::StarStationDataParser::new().parse(tokens)
    }
}
