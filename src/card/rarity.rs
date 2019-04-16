use serde::Deserialize;

#[derive(Deserialize,Debug)]
#[serde(rename_all = "snake_case")]
pub enum Rarity {
    Common,
    Uncommon,
    Rare,
    Mythic,
}
