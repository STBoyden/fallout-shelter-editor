use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use std::{collections::HashMap, fmt::Display};

type Other = HashMap<String, serde_json::Value>;

#[derive(Debug, Serialize, Deserialize)]
pub struct DwellerHappiness {
    #[serde(rename = "happinessValue")]
    pub happiness_value: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DwellerHealth {
    #[serde(rename = "healthValue")]
    pub health: f32,
    #[serde(rename = "radiationValue")]
    pub radiation: f32,
    #[serde(rename = "maxHealth")]
    pub max_health: f32,
    #[serde(flatten)]
    other: Other,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DwellerExperience {
    #[serde(rename = "experienceValue")]
    pub experience_value: f32,
    #[serde(rename = "currentLevel")]
    pub current_level: i32,
    #[serde(rename = "needLvUp")]
    pub need_lvl_up: bool,
    #[serde(flatten)]
    other: Other,
}

#[derive(Debug, Serialize_repr, Deserialize_repr, PartialEq, Eq)]
#[repr(u8)]
pub enum DwellerGender {
    Female = 1,
    Male = 2,
}

impl Display for DwellerGender {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let gender;

        if *self == Self::Female {
            gender = "Female"
        } else {
            gender = "Male"
        }

        write!(f, "{gender}")
    }
}

#[derive(Debug, Serialize_repr, Deserialize_repr, PartialEq, Eq)]
#[repr(u8)]
pub enum DwellerSpecial {
    Unknown = 0,
    Strength = 1,
    Perception = 2,
    Endurance = 3,
    Charisma = 4,
    Intelligence = 5,
    Agility = 6,
    Luck = 7,
}

impl DwellerSpecial {
    pub fn from_index(index: usize) -> Self {
        match index {
            1 => Self::Strength,
            2 => Self::Perception,
            3 => Self::Endurance,
            4 => Self::Charisma,
            5 => Self::Intelligence,
            6 => Self::Agility,
            7 => Self::Luck,
            _ => Self::Unknown,
        }
    }
}

impl Display for DwellerSpecial {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let special;

        match self {
            Self::Unknown => special = "Unknown",
            Self::Strength => special = "Strength",
            Self::Perception => special = "Perception",
            Self::Endurance => special = "Endurance",
            Self::Charisma => special = "Charisma",
            Self::Intelligence => special = "Intelligence",
            Self::Agility => special = "Agility",
            Self::Luck => special = "Luck",
        }

        write!(f, "{special}")
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DwellerStat {
    pub value: i32,
    #[serde(flatten)]
    other: Other,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DwellerStats {
    pub stats: [DwellerStat; 8],
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DwellerRelations {
    pub ascendents: [i32; 6],
    #[serde(flatten)]
    other: Other,
}

impl DwellerRelations {
    pub fn is_child(&self) -> bool {
        self.ascendents.iter().filter(|a| **a != -1).count() == 0
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DwellerInner {
    #[serde(rename = "name")]
    pub first_name: String,
    #[serde(rename = "lastName")]
    pub last_name: String,
    pub happiness: DwellerHappiness,
    pub gender: DwellerGender,
    pub stats: DwellerStats,
    pub pregnant: bool,
    #[serde(rename = "babyReady")]
    pub baby_ready: bool,
    pub health: DwellerHealth,
    pub experience: DwellerExperience,

    #[serde(flatten)]
    other: Other,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Dwellers {
    pub dwellers: Vec<DwellerInner>,
    #[serde(flatten)]
    other: Other,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Rock {
    r: i32,
    c: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VaultResources {
    #[serde(rename = "Nuka")]
    pub caps: f32,
    #[serde(rename = "Food")]
    pub food: f32,
    #[serde(rename = "Energy")]
    pub energy: f32,
    #[serde(rename = "Water")]
    pub water: f32,
    #[serde(rename = "StimPack")]
    pub stim_pack: f32,
    #[serde(rename = "RadAway")]
    pub rad_away: f32,
    #[serde(rename = "Lunchbox")]
    pub lunchbox: f32,
    #[serde(rename = "MrHandy")]
    pub mr_handy: f32,
    #[serde(rename = "PetCarrier")]
    pub pet_carrier: f32,
    #[serde(rename = "CraftedOutfit")]
    pub crafted_outfit: f32,
    #[serde(rename = "CraftedWeapon")]
    pub crafted_weapon: f32,
    #[serde(rename = "NukaColaQuantum")]
    pub nuka_cola_quantum: f32,
    #[serde(rename = "CraftedTheme")]
    pub crafted_theme: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VaultStorage {
    pub resources: VaultResources,
    pub bonus: VaultResources,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VaultInventoryItem {
    pub id: String,
    #[serde(rename = "type")]
    pub _type: String,
    #[serde(rename = "hasBeenAssigned")]
    pub has_been_assigned: bool,
    #[serde(rename = "hasRandonWeaponBeenAssigned")]
    pub has_random_weapon_been_assigned: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VaultInventory {
    pub items: Vec<VaultInventoryItem>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Vault {
    pub rocks: Vec<Rock>,
    pub storage: VaultStorage,
    pub inventory: VaultInventory,
    #[serde(rename = "LunchboxesCount", default)]
    pub lunchboxes_count: i32,
    #[serde(rename = "VaultName")]
    pub vault_name: String,

    #[serde(flatten)]
    pub other: Other,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Save {
    pub dwellers: Dwellers,
    pub vault: Vault,
    #[serde(flatten)]
    other: Other,
}
