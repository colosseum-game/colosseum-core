use crate::{
    actions::ActionIdentifier,
    damage::{
        Aspect,
        StatusEffectEntry,
    },
    item::equipable::{
        BodyEquipable,
        BodyEquipableIdentifier,
        FeetEquipable,
        FeetEquipableIdentifier,
        HandsEquipable,
        HandsEquipableIdentifier,
        HeadEquipable,
        HeadEquipableIdentifier,
        LegsEquipable,
        LegsEquipableIdentifier,
        WaistEquipable,
        WaistEquipableIdentifier,
    },
    math::Fraction,
    modifier::{
        Modifier,
        ModifierExpression,
    },
};

use serde::{
    Deserialize,
    Serialize,
};

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
pub enum Gender {
    Female,
    Male,
    None,
}

#[derive(Clone, Copy, Debug)]
pub enum Attribute {
    Vigor,
    Vitality,
    Strength,
    Agility,
    Dexterity,
    Intelligence,
    Mind,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Combatant {
    pub name: String,
    pub gender: Gender,
    pub actions: Vec<ActionIdentifier>,

    pub vigor: u32,
    pub vitality: u32,
    pub agility: u32,
    pub strength: u32,
    pub dexterity: u32,
    pub intelligence: u32,
    pub mind: u32,

    pub head_equipable: Option<HeadEquipableIdentifier>,
    pub body_equipable: Option<BodyEquipableIdentifier>,
    pub hands_equipable: Option<HandsEquipableIdentifier>,
    pub waist_equipable: Option<WaistEquipableIdentifier>,
    pub legs_equipable: Option<LegsEquipableIdentifier>,
    pub feet_equipable: Option<FeetEquipableIdentifier>,

    #[serde(skip)] hp: u32,
    #[serde(skip)] pub status_effects: Vec<StatusEffectEntry>,

    #[serde(skip)] pub vigor_modifiers: Vec<Modifier>,
    #[serde(skip)] pub vitality_modifiers: Vec<Modifier>,
    #[serde(skip)] pub agility_modifiers: Vec<Modifier>,
    #[serde(skip)] pub strength_modifiers: Vec<Modifier>,
    #[serde(skip)] pub dexterity_modifiers: Vec<Modifier>,
    #[serde(skip)] pub intelligence_modifiers: Vec<Modifier>,
    #[serde(skip)] pub mind_modifiers: Vec<Modifier>,
}

impl Combatant {
    pub fn get_hp(&self) -> u32 {
        std::cmp::min(self.hp, self.get_hp_max())
    }

    pub fn get_hp_max(&self) -> u32 {
        self.get_attribute(Attribute::Vigor) * 2
    }

    pub fn modify_hp(&mut self, modifier: Modifier) {
        self.hp = self.get_hp();
        match modifier.expression {
            ModifierExpression::Add(value) => self.hp = std::cmp::min(self.hp + value, self.get_hp_max()),
            ModifierExpression::Multiply(value) => {
                self.hp *= value.0;
                self.hp /= value.1;
                self.hp = std::cmp::min(self.hp, self.get_hp_max());
            }
            ModifierExpression::Subtract(value) => self.hp -= std::cmp::min(value, self.hp),
        }
    }

    pub fn alive(&self) -> bool {
        self.vigor > 0 && self.get_hp() > 0
    }

    pub fn dead(&self) -> bool {
        !self.alive()
    }

    pub fn get_attribute_raw(&self, attribute: Attribute) -> u32 {
        match attribute {
            Attribute::Vigor => self.vigor,
            Attribute::Vitality => self.vitality,
            Attribute::Strength => self.strength,
            Attribute::Agility => self.agility,
            Attribute::Dexterity => self.dexterity,
            Attribute::Intelligence => self.intelligence,
            Attribute::Mind => self.mind,
        }
    }

    pub fn get_attribute(&self, attribute: Attribute) -> u32 {
        let attribute_modifiers = match attribute {
            Attribute::Vigor => &self.vigor_modifiers,
            Attribute::Vitality => &self.vitality_modifiers,
            Attribute::Strength => &self.strength_modifiers,
            Attribute::Agility => &self.agility_modifiers,
            Attribute::Dexterity => &self.dexterity_modifiers,
            Attribute::Intelligence => &self.intelligence_modifiers,
            Attribute::Mind => &self.mind_modifiers,
        };

        let (add, multiply, subtract) = attribute_modifiers.iter().fold((0, Fraction::one(), 0), |acc, modifier|
            match modifier.expression {
                ModifierExpression::Add(value) => (acc.0 + value, acc.1, acc.2),
                ModifierExpression::Multiply(value) => (acc.0, acc.1.multiply(value), acc.2),
                ModifierExpression::Subtract(value) => (acc.0, acc.1, acc.2 + value),
            }
        );

        let mut value = self.get_attribute_raw(attribute);
        value += add;
        value -= std::cmp::min(value, subtract);
        value *= multiply.0;
        value /= multiply.1;

        value
    }

    pub fn get_defense(&self, aspect: Aspect) -> u32 {
        let mut value = 0;
        if let Some(identifier) = self.head_equipable { value += <&HeadEquipable>::from(identifier).get_defense(aspect); }
        if let Some(identifier) = self.body_equipable { value += <&BodyEquipable>::from(identifier).get_defense(aspect); }
        if let Some(identifier) = self.hands_equipable { value += <&HandsEquipable>::from(identifier).get_defense(aspect); }
        if let Some(identifier) = self.waist_equipable { value += <&WaistEquipable>::from(identifier).get_defense(aspect); }
        if let Some(identifier) = self.legs_equipable { value += <&LegsEquipable>::from(identifier).get_defense(aspect); }
        if let Some(identifier) = self.feet_equipable { value += <&FeetEquipable>::from(identifier).get_defense(aspect); }
        value
    }
}
