pub use crate::combatant_generated::Combatant;

use crate::{
    aspect::Aspect,
    attribute::Attribute,
    fraction::Fraction,
    store::{
        BODYWEAR_STORE,
        FOOTWEAR_STORE,
        HANDWEAR_STORE,
        HEADWEAR_STORE,
        LEGWEAR_STORE,
    },
};

impl Combatant {
    pub fn absorbtion(&self, _aspect: Aspect) -> u32 {
        0 // TODO: determine where to get absorbtion values from
    }

    pub fn alive(&self) -> bool {
        self.hp > 0 && self.hp_max() > 0
    }

    pub fn attribute(&self, attribute: Attribute) -> u32 {
        use Attribute::*;

        let attribute_modifiers = match attribute {
            ATTRIBUTE_AGILITY => &self.agility_modifiers,
            ATTRIBUTE_DEXTERITY => &self.dexterity_modifiers,
            ATTRIBUTE_INTELLIGENCE => &self.intelligence_modifiers,
            ATTRIBUTE_MIND => &self.mind_modifiers,
            ATTRIBUTE_STRENGTH => &self.strength_modifiers,
            ATTRIBUTE_VIGOR => &self.vigor_modifiers,
            ATTRIBUTE_VITALITY => &self.vitality_modifiers,
        };

        let (add, subtract, multiply) = attribute_modifiers.iter().fold((0, 0, Fraction::one()), |mut acc, modifier| {
                if modifier.has_add() { return (acc.0 + modifier.get_add(), acc.1, acc.2); }
                if modifier.has_subtract() { return (acc.0, acc.1 + modifier.get_subtract(), acc.2); }
                if modifier.has_multiply() {
                    acc.2.multiply(modifier.get_multiply());
                    return (acc.0, acc.1, acc.2);
                }
                panic!();
            }
        );

        let mut value = self.attribute_raw(attribute);
        value += add;
        value -= std::cmp::min(value, subtract);
        value *= multiply.numerator;
        value /= multiply.denominator;

        value
    }

    pub fn attribute_raw(&self, attribute: Attribute) -> u32 {
        use Attribute::*;

        match attribute {
            ATTRIBUTE_AGILITY => self.agility,
            ATTRIBUTE_DEXTERITY => self.dexterity,
            ATTRIBUTE_INTELLIGENCE => self.intelligence,
            ATTRIBUTE_MIND => self.mind,
            ATTRIBUTE_STRENGTH => self.strength,
            ATTRIBUTE_VIGOR => self.vigor,
            ATTRIBUTE_VITALITY => self.vitality,
        }
    }

    pub fn dead(&self) -> bool {
        !self.alive()
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

    pub fn hp_max(&self) -> u32 {
        self.attribute(Attribute::ATTRIBUTE_VIGOR)
    }

    pub fn hp_max_initial(vigor: u32) -> u32 {
        vigor
    }

    pub fn raw_damage(&self, _aspect: Aspect) -> u32 {
        1 // TODO: determine where to get damage values from
    }

    pub fn ready(&self) -> bool {
        self.alive() && self.fatigue == 0
    }
}
