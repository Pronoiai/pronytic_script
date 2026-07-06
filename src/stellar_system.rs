use std::fmt;

use lalrpop_util::lalrpop_mod;
use logos::Logos;
use rust_decimal::prelude::*;

use crate::{
    LexicalError,
    common::{DataParser, Temperature},
};

#[derive(Logos, Clone, Debug, PartialEq)]
#[logos(skip r"[\s\t\f]+", error = LexicalError)]
#[logos(skip r"//[^\n\r]*")]
pub enum StellarToken {
    #[token("true")]
    True,
    #[token("false")]
    False,
    #[regex(r#""[^"]*""#, |lex| lex.slice().trim_matches('"').to_string())]
    String(String),

    #[regex(r"(\d+)", |lex|lex.slice().parse::<u16>().expect("parsing u8"), priority = 5)]
    Number(u16),

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

    #[token(":")]
    Colon,

    #[token("name")]
    Name,

    #[token("asset_location")]
    AssetLocation,

    #[token("size")]
    Size,

    #[token("surveyed")]
    Surveyed,

    #[token("star_data")]
    StarData,

    #[token("planet_data")]
    PlanetData,

    #[token("moon_data")]
    MoonData,

    #[token("asteroid_belt")]
    AsteroidBelt,

    #[token("star_type")]
    StarType,

    #[token("stapledon")]
    Stapledon,

    #[token("planet_type")]
    PlanetType,

    #[token("magnetosphere")]
    Magnetosphere,
    #[token("atmosphere")]
    Atmosphere,
    #[token("temperature_kelvin")]
    TemperatureKelvin,
    #[token("temperature_celsius")]
    TemperatureCelsius,
    #[token("water")]
    Water,
    #[token("breathability")]
    Breathability,

    #[token("natural_resources")]
    NaturalResources,

    #[token("ring")]
    Ring,

    #[token("good_id")]
    GoodId,

    #[token("amount")]
    Amount,

    #[token("capital")]
    Capital,

    #[token("buildings")]
    Buildings,

    #[token("level")]
    Level,

    #[token("orbital")]
    Orbital,

    #[token("owner")]
    Owner,

    #[token("none")]
    None,

    #[token("human")]
    Human,
    #[token("homeworld")]
    Homeworld,

    #[token("colony")]
    Colony,

    #[token("pops")]
    Pops,

    #[token("count")]
    Count,

    #[token("designation")]
    Designation,

    #[token("shipyard")]
    Shipyard,
}

impl fmt::Display for StellarToken {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

lalrpop_mod!(pub stellar_system);

#[derive(Clone, Default, Debug)]
pub struct StellarData {
    pub id: u16,
    pub star_data: StarData,
    pub orbiting: Vec<StellarObject>,
    pub surveyed: bool,
    pub owner: StarOwner,
}

#[derive(Clone, Default, Debug)]
pub enum StarOwner {
    #[default]
    Unowned,
    Human,
    Homeworld,
}

#[derive(Clone, Debug)]
//Majority case is the large data structure variant and it will be cleaned up on program startup
#[allow(clippy::large_enum_variant)]
pub enum StellarObject {
    PlanetData(PlanetData),
    //Dwarf planets
    AsteroidBelt(Vec<PlanetData>),
}

#[derive(Clone, Default, Debug)]
pub struct StarData {
    pub name: String,
    pub asset_location: String,
    pub size: u16,
    pub temperature: Temperature,
    pub star_type: String,
    pub stapledon_level: u16,
}

macro_rules! create_default {
    ($title:ident {$(pub $field:ident : $ty:ty),*$(,)*}) => {
        #[derive(Clone, Debug)]
        pub struct $title {
            $(pub $field: $ty,)*
        }

        impl Default for $title {
            fn default() -> Self {
                PlanetData {
                    $(
                        $field: create_default!(@internal $field),
                    )*
                }
            }
        }
    };

    (@internal size) => { 10 };
    (@internal $other:ident) => { Default::default() };
}

create_default!( PlanetData
    {
        pub name: String,

        pub asset_location: String,
        pub size: u16,

        pub planet_type: String,
        pub magnetosphere: Decimal,
        pub atmosphere: Decimal,
        pub temperature: Temperature,
        pub water: Decimal,
        pub breathability: Decimal,

        pub natural_resources: Vec<NaturalResource>,

        pub capital: bool,

        pub ring: bool,

        pub moons: Vec<MoonData>,
        pub orbital: Option<Orbital>,
        pub buildings: Vec<String>,
        pub colony: Option<ColonyDetails>,
        pub shipyard: Option<Shipyard>,

});

#[derive(Clone, Default, Debug)]
pub struct MoonData {
    pub name: String,
    pub asset_location: String,

    pub size: u16,

    pub planet_type: String,
    pub magnetosphere: Decimal,
    pub atmosphere: Decimal,
    pub temperature: Temperature,
    pub water: Decimal,
    pub breathability: Decimal,

    pub natural_resources: Vec<NaturalResource>,
    pub capital: bool,
    pub buildings: Vec<String>,
    pub orbital: Option<Orbital>,
    pub colony: Option<ColonyDetails>,
    pub shipyard: Option<Shipyard>,
}

#[derive(Clone, Default, Debug)]
pub struct NaturalResource {
    pub id: String,
    pub amount: Decimal,
}

#[derive(Clone, Default, Debug)]
pub struct Orbital {
    pub orbital_level: u16,
    pub buildings: Vec<String>,
    pub colony_details: ColonyDetails,
}

#[derive(Clone, Default, Debug)]
pub struct ColonyDetails {
    pub designation: String,
    pub colony_pops: Vec<ColonyPop>,
}

#[derive(Clone, Debug)]
pub struct ColonyPop {
    pub count: u16,
    pub star_type: String,
}

#[derive(Clone, Default, Debug)]
pub struct Shipyard {
    pub level: u16,
}

#[allow(clippy::large_enum_variant)]
pub enum StellarField {
    StarData(StarData),
    Orbiting(StellarObject),
    Surveyed(bool),
    Owner(StarOwner),
}

pub enum StarField {
    Name(String),
    AssetLocation(String),
    Temperature(Temperature),
    StarType(String),
    Stapledon(u16),
}

#[allow(clippy::large_enum_variant)]
pub enum PlanetField {
    Name(String),
    AssetLocation(String),
    Size(u16),
    PlanetType(String),
    Magnetosphere(Decimal),
    Atmosphere(Decimal),
    Temperature(Temperature),
    Water(Decimal),
    Breathability(Decimal),
    NaturalResources(Vec<NaturalResource>),
    Ring,
    Moon(MoonData),
    Capital(bool),
    Buildings(Vec<String>),
    Orbital(Orbital),
    Colony(ColonyDetails),
    Shipyard(Shipyard),
}

pub enum MoonField {
    Name(String),
    AssetLocation(String),
    Size(u16),
    PlanetType(String),
    Magnetosphere(Decimal),
    Atmosphere(Decimal),
    Temperature(Temperature),
    Water(Decimal),
    Breathability(Decimal),
    NaturalResources(Vec<NaturalResource>),
    Capital(bool),
    Buildings(Vec<String>),
    Orbital(Orbital),
    Colony(ColonyDetails),
    Shipyard(Shipyard),
}

pub enum ColonyField {
    Designation(String),
    Population(Vec<ColonyPop>),
}

pub enum OrbitalField {
    Buildings(Vec<String>),
    Level(u16),
    ColonyField(ColonyDetails),
}

pub enum ShipyardField {
    Level(u16),
}

impl<'s> DataParser<'s> for StellarData {
    type Token = StellarToken;
    fn parse_tokens(
        tokens: Vec<(usize, Self::Token, usize)>,
    ) -> Result<Vec<StellarData>, lalrpop_util::ParseError<usize, Self::Token, String>> {
        stellar_system::StellarDataParser::new().parse(tokens)
    }
}
