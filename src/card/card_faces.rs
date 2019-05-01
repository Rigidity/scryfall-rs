use serde::Deserialize;

use super::color::Color;
use crate::util::UUID;

use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct CardFace {
    pub artist: Option<String>,
    pub color_indicator: Option<Vec<Color>>,
    pub colors: Option<Vec<Color>>,
    pub flavor_text: Option<String>,
    pub illustration_id: Option<UUID>,
    pub image_uris: Option<HashMap<String, String>>,
    pub loyalty: Option<String>,
    pub mana_cost: String,
    pub name: String,
    pub oracle_text: Option<String>,
    pub power: Option<String>,
    pub printed_name: Option<String>,
    pub printed_text: Option<String>,
    pub printed_type_line: Option<String>,
    pub toughness: Option<String>,
    pub type_line: String,
    pub watermark: Option<String>,
}