use crate::{
    aspect::Aspect,
    attribute::Attribute,
    dot::DOT,
    fraction::Fraction,
    gender::Gender,
    store::{
        BODYWEAR_STORE,
        FOOTWEAR_STORE,
        HANDWEAR_STORE,
        HEADWEAR_STORE,
        LEGWEAR_STORE,
    },
    modifier::{
        Modifier,
        ModifierExpression,
    },
};

use serde::{
    Deserialize,
    Serialize,
};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Combatant {
    pub name: String,
    pub gender: Gender,
    pub skills: Vec<String>,

    pub agility: u32,
    pub dexterity: u32,
    pub intelligence: u32,
    pub mind: u32,
    pub strength: u32,
    pub vigor: u32,
    pub vitality: u32,

    pub bodywear: String,
    pub footwear: String,
    pub handwear: String,
    pub headwear: String,
    pub legwear: String,
    pub weapon: String,

    pub hp: u32,
    pub fatigue: u32,
    pub dots: Vec<DOT>,

    pub agility_modifiers: Vec<Modifier>,
    pub dexterity_modifiers: Vec<Modifier>,
    pub intelligence_modifiers: Vec<Modifier>,
    pub mind_modifiers: Vec<Modifier>,
    pub strength_modifiers: Vec<Modifier>,
    pub vigor_modifiers: Vec<Modifier>,
    pub vitality_modifiers: Vec<Modifier>,
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
        use Attribute::*;
        match attribute {
            Agility => self.agility,
            Dexterity => self.dexterity,
            Intelligence => self.intelligence,
            Mind => self.mind,
            Strength => self.strength,
            Vigor => self.vigor,
            Vitality => self.vitality,
        }
    }

    pub fn attribute(&self, attribute: Attribute) -> u32 {
        use Attribute::*;
        let attribute_modifiers = match attribute {
            Agility => &self.agility_modifiers,
            Dexterity => &self.dexterity_modifiers,
            Intelligence => &self.intelligence_modifiers,
            Mind => &self.mind_modifiers,
            Strength => &self.strength_modifiers,
            Vigor => &self.vigor_modifiers,
            Vitality => &self.vitality_modifiers,
        };

        let (add, multiply, subtract) = attribute_modifiers.iter().fold((0, Fraction::one(), 0), |acc, modifier| {
            use ModifierExpression::*;
            match modifier.expression {
                Add(value) => (acc.0 + value, acc.1, acc.2),
                Multiply(value) => (acc.0, acc.1.multiply(value), acc.2),
                Subtract(value) => (acc.0, acc.1, acc.2 + value),
            }
        });

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
        if !self.bodywear.is_empty() { value += BODYWEAR_STORE.get(&self.bodywear).unwrap().get_defense(aspect); }
        if !self.footwear.is_empty() { value += FOOTWEAR_STORE.get(&self.footwear).unwrap().get_defense(aspect); }
        if !self.handwear.is_empty() { value += HANDWEAR_STORE.get(&self.handwear).unwrap().get_defense(aspect); }
        if !self.headwear.is_empty() { value += HEADWEAR_STORE.get(&self.headwear).unwrap().get_defense(aspect); }
        if !self.legwear.is_empty() { value += LEGWEAR_STORE.get(&self.legwear).unwrap().get_defense(aspect); }
        value
    }

    pub fn absorbtion(&self, _aspect: Aspect) -> u32 {
        0 // TODO: determine where to get absorbtion values from
    }
}
