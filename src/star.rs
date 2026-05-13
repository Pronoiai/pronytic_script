use std::fmt;

use lalrpop_util::lalrpop_mod;
use logos::Logos;
use rust_decimal::prelude::*;

use crate::{LexicalError, common::DataParser};

#[derive(Logos, Clone, Debug, PartialEq)]
#[logos(skip r"[\s\t\f]+", error = LexicalError)]
#[logos(skip r"//[^\n\r]*")]
pub enum StarToken {
    #[regex(r#""[^"]*""#, |lex| lex.slice().trim_matches('"').to_string())]
    String(String),

    #[regex(r"(-?\d+\.?\d*)", |lex| Decimal::from_str(lex.slice()).expect("parsed_decimal"), priority = 4)]
    DecimalNumber(Decimal),

    #[token("=")]
    Equal,

    #[token("{")]
    LeftCurly,
    #[token("}")]
    RightCurly,

    #[token("x")]
    X,
    #[token("y")]
    Y,
    #[token("z")]
    Z,

    #[token("asset_location")]
    AssetLocation,

    #[token("claim_asset_location")]
    ClaimAssetLocation,

    #[token("scale")]
    Scale,
    #[token("claim_asset_scale")]
    ClaimAssetScale,

    #[token("claim_asset_offset")]
    ClaimAssetOffset,

    #[token("claim_asset_field")]
    ClaimAssetField,
}

impl fmt::Display for StarToken {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{self:?}")
    }
}
lalrpop_mod!(pub star);

#[derive(Clone, Default, Debug)]
pub struct StarData {
    pub id: String,
    pub asset_location: String,
    pub claim_asset_location: String,
    pub scale: Decimal,
    pub claim_asset_scale: Decimal,
    pub asset_offset: ClaimAssetOffset,
}

#[derive(Clone, Default, Debug)]
pub struct ClaimAssetOffset {
    pub x: Decimal,
    pub y: Decimal,
    pub z: Decimal,
}

pub enum StarField {
    AssetLocation(String),
    ClaimAssetLocation(String),
    Scale(Decimal),
    ClaimAssetScale(Decimal),
    ClaimAssetOffset(ClaimAssetOffset),
}

pub enum OffsetField {
    X(Decimal),
    Y(Decimal),
    Z(Decimal),
}

impl<'s> DataParser<'s> for StarData {
    type Token = StarToken;

    fn parse_tokens(
        tokens: Vec<(usize, Self::Token, usize)>,
    ) -> Result<Vec<Self>, lalrpop_util::ParseError<usize, Self::Token, String>> {
        star::StarDataParser::new().parse(tokens)
    }
}
