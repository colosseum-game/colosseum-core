use crate::{
    aspect::Aspect,
    fraction::Fraction,
    lifetime::Lifetime,
    modifier::{
        Modifier,
        ModifierExpression,
    },
    store::{
        get_bodywear,
        get_footwear,
        get_handwear,
        get_headwear,
        get_legwear,
    },
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

pub const SKILL_COUNT_MAX: usize = 32;
pub const MODIFIER_COUNT_MAX: usize = 32;
pub const DOT_COUNT_MAX: usize = 32;

#[derive(Debug, Deserialize, Serialize)]
pub struct Combatant {
    pub name: String,
    pub gender: Gender,
    pub skills: [Option<String>; SKILL_COUNT_MAX],

    pub agility: u32,
    pub dexterity: u32,
    pub intelligence: u32,
    pub mind: u32,
    pub strength: u32,
    pub vigor: u32,
    pub vitality: u32,

    pub bodywear: Option<String>,
    pub footwear: Option<String>,
    pub handwear: Option<String>,
    pub headwear: Option<String>,
    pub legwear: Option<String>,
    pub weapon: Option<String>,

    pub hp: u32,
    pub fatigue: u32,
    pub dots: [Option<DOT>; DOT_COUNT_MAX],

    pub agility_modifiers: [Option<Modifier>; MODIFIER_COUNT_MAX],
    pub dexterity_modifiers: [Option<Modifier>; MODIFIER_COUNT_MAX],
    pub intelligence_modifiers: [Option<Modifier>; MODIFIER_COUNT_MAX],
    pub mind_modifiers: [Option<Modifier>; MODIFIER_COUNT_MAX],
    pub strength_modifiers: [Option<Modifier>; MODIFIER_COUNT_MAX],
    pub vigor_modifiers: [Option<Modifier>; MODIFIER_COUNT_MAX],
    pub vitality_modifiers: [Option<Modifier>; MODIFIER_COUNT_MAX],
}

impl Combatant {
    pub fn hp_max(&self) -> u32 {
        self.attribute(Attribute::Vigor)
    }

    pub fn hp_max_initial(vigor: u32) -> u32 {
        vigor
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
            match modifier {
                None => (acc.0, acc.1, acc.2),
                Some(modifier) => match modifier.expression {
                    ModifierExpression::Add(value) => (acc.0 + value, acc.1, acc.2),
                    ModifierExpression::Multiply(value) => (acc.0, acc.1.multiply(value), acc.2),
                    ModifierExpression::Subtract(value) => (acc.0, acc.1, acc.2 + value),
                }
            }
        );

        let mut value = self.attribute_raw(attribute);
        value += add;
        value -= std::cmp::min(value, subtract);
        value *= multiply.numerator;
        value /= multiply.denominator;

        value
    }

    pub fn raw_damage(&self, _aspect: Aspect) -> u32 {
        1 // TODO: determine where to get damage values from
    }

    pub fn defense(&self, aspect: Aspect) -> u32 {
        let mut value = 0;
        if let Some(identifier) = &self.bodywear { value += get_bodywear(identifier).unwrap().get_defense(aspect); }
        if let Some(identifier) = &self.footwear { value += get_footwear(identifier).unwrap().get_defense(aspect); }
        if let Some(identifier) = &self.handwear { value += get_handwear(identifier).unwrap().get_defense(aspect); }
        if let Some(identifier) = &self.headwear { value += get_headwear(identifier).unwrap().get_defense(aspect); }
        if let Some(identifier) = &self.legwear { value += get_legwear(identifier).unwrap().get_defense(aspect); }
        value
    }

    pub fn absorbtion(&self, _aspect: Aspect) -> u32 {
        0 // TODO: determine where to get absorbtion values from
    }
}
