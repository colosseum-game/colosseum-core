use crate::{
    actions::CombatAction,
    ability::Ability,
    aspects::Aspect,
    combatant::{
        Attribute,
        Combatant,
        DOT,
    },
    effect::{
        EffectSource,
        SubEffect,
    },
    fraction::Fraction,
    lifetimes::Lifetime,
    modifier::Modifier,
    party::Party,
    targeting::Target,
};

use serde::{
    Deserialize,
    Serialize,
};

#[derive(Debug, Deserialize, Serialize)]
pub struct CombatState {
    pub parties: Vec<Party>,
}

impl CombatState {
    pub fn transform(&mut self, action: CombatAction, source: Target) {
        match action {
            CombatAction::Ability { ability_index, target_groups } => {
                let ability_identifier = self.parties[source.party_index].members[source.member_index].abilities[ability_index];
                let ability = <&Ability>::from(ability_identifier);

                for i in 0..ability.effects.len() {
                    for target in &target_groups[i] {
                        // getting the source and target from our collection is complicated here since we have to show rust that the source and target aren't the same thing
                        // we can only do this by iterating the collection, or by splitting the collection at a threshold between them
                        // we split the collections here as needed since the functional approach isn't viable?
                        // I'm blaming nested map calls since I couldn't get them to work with/without type arguments
                        let (source, target) = match source.party_index {
                            party_index if party_index > target.party_index => {
                                let (target_containing_parties, source_containing_parties) = self.parties.split_at_mut(party_index);
                                let source = EffectSource::Other(&source_containing_parties[0].members[source.member_index]);
                                let target = &mut target_containing_parties[target.party_index].members[target.member_index];
                                (source, target)
                            },
                            party_index if party_index < target.party_index => {
                                let (source_containing_parties, target_containing_parties) = self.parties.split_at_mut(party_index);
                                let source = EffectSource::Other(&source_containing_parties[source.party_index].members[source.member_index]);
                                let target = &mut target_containing_parties[0].members[target.member_index];
                                (source, target)
                            },
                            party_index => {
                                match source.member_index {
                                    member_index if member_index > target.member_index => {
                                        let (target_container, source_container) = self.parties[party_index].members.split_at_mut(member_index);
                                        let source = EffectSource::Other(&source_container[0]);
                                        let target = &mut target_container[target.member_index];
                                        (source, target)
                                    },
                                    member_index if member_index < target.member_index => {
                                        let (source_container, target_container) = self.parties[party_index].members.split_at_mut(member_index);
                                        let source = EffectSource::Other(&source_container[source.member_index]);
                                        let target = &mut target_container[0];
                                        (source, target)
                                    },
                                    member_index => {
                                        let source = EffectSource::Origin;
                                        let target = &mut self.parties[party_index].members[member_index];
                                        (source, target)
                                    },
                                }
                            },
                        };

                        // apply every effect in the current sub_action
                        for sub_effect in ability.effects[i].sub_effects {
                            match sub_effect {
                                SubEffect::Damage { aspect, scaling } => process_damage(target, *aspect, calculate_damage_value(target, source, *aspect, *scaling)),
                                SubEffect::Modifier(modifier, attribute) => push_modifier(target, *modifier, *attribute),
                                SubEffect::DamageOverTime { aspect, scaling, lifetime } => push_dot(target, *aspect, calculate_damage_value(target, source, *aspect, *scaling), *lifetime),
                            };
                        }
                    }
                }
            },
            //Action::Item { item_index, arguments } => {
            //    let item = self.parties[source.party_index].inventory[item_index].as_ref().unwrap(); // TODO: this can fail
            //    match item {
            //        ItemIdentifier::Consumable(identifier) => {
            //            let consumable = <&Consumable>::from(*identifier);
            //        },
            //        ItemIdentifier::Equipable(identifier) => {
            //            let equipable = <&Equipable>::from(*identifier);
            //            match arguments {
            //                ItemArguments::
            //            }
            //        },
            //        ItemIdentifier::Wearable(identifier) => {
            //            macro_rules! equip_wearable {
            //                ($W:ident, $WT:ident, $I:ident) => {{
            //                    let wearable = self.parties[source.party_index].members[source.member_index].$W;
            //                    self.parties[source.party_index].members[source.member_index].$W = Some(*$I);
            //                    if let Some(wearable) = wearable { self.parties[source.party_index].inventory[item_index] = Some(ItemIdentifier::Wearable(WearableIdentifier::$WT(wearable))); }
            //                }};
            //            }
//
            //            match identifier {
            //                WearableIdentifier::Bodywear(identifier) => equip_wearable!(bodywear, Bodywear, identifier),
            //                WearableIdentifier::Footwear(identifier) => equip_wearable!(footwear, Footwear, identifier),
            //                WearableIdentifier::Handwear(identifier) => equip_wearable!(handwear, Handwear, identifier),
            //                WearableIdentifier::Headwear(identifier) => equip_wearable!(headwear, Headwear, identifier),
            //                WearableIdentifier::Legwear(identifier) => equip_wearable!(legwear, Legwear, identifier),
            //            };
            //        },
            //    }
            //},
            _ => (),
        }

        {
            let mut source = &mut self.parties[source.party_index].members[source.member_index];

            // update dots
            for dot_index in 0..source.dots.len() {
                let dot = source.dots[dot_index];
                process_damage(&mut source, dot.aspect, dot.damage_value);
                if let Lifetime::Active { duration } = dot.lifetime {
                    if duration > 0 {
                        source.dots[dot_index].lifetime = Lifetime::Active { duration: duration - 1 };
                    }
                }
            }
            source.dots.retain(|dot| if let Lifetime::Active { duration } = dot.lifetime { duration > 0 } else { true });

            // update modifiers
            macro_rules! update_modifiers {
                ($M:ident) => {{
                    for modifier in &mut source.$M {
                        if let Lifetime::Active { ref mut duration } = modifier.lifetime {
                            *duration -= std::cmp::min(*duration, 1)
                        }
                    }

                    source.$M.retain(|modifier| if let Lifetime::Active { duration } = modifier.lifetime { duration > 0 } else { true });
                }};
            }

            update_modifiers!(agility_modifiers);
            update_modifiers!(dexterity_modifiers);
            update_modifiers!(intelligence_modifiers);
            update_modifiers!(mind_modifiers);
            update_modifiers!(strength_modifiers);
            update_modifiers!(vigor_modifiers);
            update_modifiers!(vitality_modifiers);
        }
    }
}

fn calculate_damage_value(target: &Combatant, source: EffectSource, aspect: Aspect, scaling: Fraction) -> u32 {
    let raw_damage = match source {
        EffectSource::None => 1,
        EffectSource::Origin => target.get_raw_damage(aspect),
        EffectSource::Other(source) => source.get_raw_damage(aspect),
    };

    raw_damage * scaling.numerator / scaling.denominator
}

fn process_damage(target: &mut Combatant, aspect: Aspect, damage_value: u32) {
    let defense_value = target.get_defense(aspect);
    let absorbtion_value = target.get_absorbtion(aspect);

    if damage_value < defense_value { return; }
    let damage_value = damage_value - defense_value;

    if absorbtion_value > 100 { target.hp += damage_value * (absorbtion_value - 100) / 100; }
    else { target.hp -= std::cmp::min(target.hp, damage_value * (100 - absorbtion_value) / 100); }
}

fn push_modifier(target: &mut Combatant, modifier: Modifier, attribute: Attribute) {
    match attribute {
        Attribute::Agility => target.agility_modifiers.push(modifier),
        Attribute::Dexterity => target.dexterity_modifiers.push(modifier),
        Attribute::Intelligence => target.intelligence_modifiers.push(modifier),
        Attribute::Mind => target.mind_modifiers.push(modifier),
        Attribute::Strength => target.strength_modifiers.push(modifier),
        Attribute::Vigor => target.vigor_modifiers.push(modifier),
        Attribute::Vitality => target.vitality_modifiers.push(modifier),
    }
}

fn push_dot(target: &mut Combatant, aspect: Aspect, damage_value: u32, lifetime: Lifetime) {
    target.dots.push(DOT { aspect, damage_value, lifetime });
}
