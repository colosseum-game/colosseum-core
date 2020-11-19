use crate::{
    ability::AbilityIdentifier,
    aspects::Aspect,
    item::{
        Bodywear,
        BodywearIdentifier,
        EquipableIdentifier,
        Footwear,
        FootwearIdentifier,
        Handwear,
        HandwearIdentifier,
        Headwear,
        HeadwearIdentifier,
        Legwear,
        LegwearIdentifier,
    },
    lifetimes::Lifetime,
    modifier::{
        Modifier,
        ModifierExpression,
    },
    fraction::Fraction,
};

use serde::{
    Deserialize,
    Serialize,
};

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum Attribute {
    Agility,
    Dexterity,
    Intelligence,
    Mind,
    Strength,
    Vigor,
    Vitality,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum Gender {
    Female,
    Male,
    None,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct DOT {
    pub aspect: Aspect,
    pub damage_value: u32,
    pub lifetime: Lifetime,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Combatant {
    pub name: String,
    pub gender: Gender,
    pub abilities: Vec<AbilityIdentifier>,

    pub agility: u32,
    pub dexterity: u32,
    pub intelligence: u32,
    pub mind: u32,
    pub strength: u32,
    pub vigor: u32,
    pub vitality: u32,

    pub agility_modifiers: Vec<Modifier>,
    pub dexterity_modifiers: Vec<Modifier>,
    pub intelligence_modifiers: Vec<Modifier>,
    pub mind_modifiers: Vec<Modifier>,
    pub strength_modifiers: Vec<Modifier>,
    pub vigor_modifiers: Vec<Modifier>,
    pub vitality_modifiers: Vec<Modifier>,

    pub bodywear: Option<BodywearIdentifier>,
    pub equipable: Option<EquipableIdentifier>,
    pub footwear: Option<FootwearIdentifier>,
    pub handwear: Option<HandwearIdentifier>,
    pub headwear: Option<HeadwearIdentifier>,
    pub legwear: Option<LegwearIdentifier>,

    pub hp: u32,
    pub fatigue: u32,
    pub dots: Vec<DOT>,
}

impl Combatant {
    pub fn hp_max(&self) -> u32 {
        self.attribute(Attribute::Vigor)
    }

    pub fn alive(&self) -> bool {
        self.hp > 0 && self.hp_max() > 0
    }

    pub fn dead(&self) -> bool {
        !self.alive()
    }

    pub fn ready(&self) -> bool {
        self.alive() && self.fatigue == 0
    }

    pub fn attribute_raw(&self, attribute: Attribute) -> u32 {
        match attribute {
            Attribute::Agility => self.agility,
            Attribute::Dexterity => self.dexterity,
            Attribute::Intelligence => self.intelligence,
            Attribute::Mind => self.mind,
            Attribute::Strength => self.strength,
            Attribute::Vigor => self.vigor,
            Attribute::Vitality => self.vitality,
        }
    }

    pub fn attribute(&self, attribute: Attribute) -> u32 {
        let attribute_modifiers = match attribute {
            Attribute::Agility => &self.agility_modifiers,
            Attribute::Dexterity => &self.dexterity_modifiers,
            Attribute::Intelligence => &self.intelligence_modifiers,
            Attribute::Mind => &self.mind_modifiers,
            Attribute::Strength => &self.strength_modifiers,
            Attribute::Vigor => &self.vigor_modifiers,
            Attribute::Vitality => &self.vitality_modifiers,
        };

        let (add, multiply, subtract) = attribute_modifiers.iter().fold((0, Fraction::one(), 0), |acc, modifier|
            match modifier.expression {
                ModifierExpression::Add(value) => (acc.0 + value, acc.1, acc.2),
                ModifierExpression::Multiply(value) => (acc.0, acc.1.multiply(value), acc.2),
                ModifierExpression::Subtract(value) => (acc.0, acc.1, acc.2 + value),
            }
        );

        let mut value = self.attribute_raw(attribute);
        value += add;
        value -= std::cmp::min(value, subtract);
        value *= multiply.numerator;
        value /= multiply.denominator;

        value
    }

    pub fn get_raw_damage(&self, aspect: Aspect) -> u32 {
        1 // TODO: determine where to get damage values from
    }

    pub fn get_defense(&self, aspect: Aspect) -> u32 {
        let mut value = 0;
        if let Some(identifier) = self.bodywear { value += <&Bodywear>::from(identifier).get_defense(aspect); }
        if let Some(identifier) = self.footwear { value += <&Footwear>::from(identifier).get_defense(aspect); }
        if let Some(identifier) = self.handwear { value += <&Handwear>::from(identifier).get_defense(aspect); }
        if let Some(identifier) = self.headwear { value += <&Headwear>::from(identifier).get_defense(aspect); }
        if let Some(identifier) = self.legwear { value += <&Legwear>::from(identifier).get_defense(aspect); }
        value
    }

    pub fn get_absorbtion(&self, aspect: Aspect) -> u32 {
        0 // TODO: determine where to get absorbtion values from
    }
}
