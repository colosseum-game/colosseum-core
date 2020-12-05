use crate::{
    aspect::Aspect,
    attribute::Attribute,
    combat_event::CombatEvent,
    combatant::Combatant,
    dot::DOT,
    effect::SubEffect,
    fraction::Fraction,
    lifetime::Lifetime,
    party::Party,
    store::SKILL_STORE,
    target::Target,
};

use serde::{
    Deserialize,
    Serialize
};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CombatState {
    parties: Vec<Party>,
}

impl CombatState {
    pub fn transform(&mut self, source: Target, combat_event: CombatEvent) {
        use CombatEvent::*;
        match combat_event {
            AttackEvent { targets: _ } => (),
            ConsumableEvent { consumable_identifier: _, targets: _ } => (),
            ForfeitEvent => (),
            SkipEvent => (),
            SkillEvent { skill_identifier, targets } => {
                for target in &targets {
                    let (source, target) = {
                        if source.party_index > target.party_index {
                            let (target_container, source_container) = self.parties.split_at_mut(source.party_index);
                            let source = Some(&source_container[0].members[source.member_index]);
                            let target = &mut target_container[target.party_index].members[target.member_index];
                            (source, target)
                        } else if source.party_index < target.party_index {
                            let (source_container, target_container) = self.parties.split_at_mut(target.party_index);
                            let source = Some(&source_container[source.party_index].members[source.member_index]);
                            let target = &mut target_container[0].members[target.member_index];
                            (source, target)
                        } else {
                            if source.member_index > target.member_index {
                                let (target_container, source_container) = self.parties[source.party_index].members.split_at_mut(source.member_index);
                                let source = Some(&source_container[0]);
                                let target = &mut target_container[target.member_index];
                                (source, target)
                            } else if source.member_index < target.member_index {
                                let (source_container, target_container) = self.parties[source.party_index].members.split_at_mut(target.member_index);
                                let source = Some(&source_container[source.member_index]);
                                let target = &mut target_container[0];
                                (source, target)
                            } else {
                                let target = &mut self.parties[source.party_index].members[source.member_index];
                                (None, target)
                            }
                        }
                    };

                    // apply every effect in the current sub_action
                    let skill = SKILL_STORE.get(&skill_identifier).unwrap();
                    for sub_effect in &skill.effect.sub_effects {
                        match sub_effect {
                            SubEffect::Damage { aspect,  scaling } => {
                                process_damage(target, *aspect, calculate_damage_value(target, source, *aspect, *scaling))
                            },
                            SubEffect::DOT { aspect, scaling, lifetime } => {
                                let damage_value = calculate_damage_value(target, source, *aspect, *scaling);

                                let dot = DOT {
                                    aspect: *aspect,
                                    damage_value,
                                    lifetime: *lifetime,
                                };

                                target.dots.push(dot);
                            },
                            SubEffect::Modifier { modifier, attribute } => {
                                use Attribute::*;
                                match attribute {
                                    Agility => target.agility_modifiers.push(*modifier),
                                    Dexterity => target.dexterity_modifiers.push(*modifier),
                                    Intelligence => target.intelligence_modifiers.push(*modifier),
                                    Mind => target.mind_modifiers.push(*modifier),
                                    Strength => target.strength_modifiers.push(*modifier),
                                    Vigor => target.vigor_modifiers.push(*modifier),
                                    Vitality => target.vitality_modifiers.push(*modifier),
                                }
                            }
                        }
                    }
                }
            },
        }

        let source = &mut self.parties[source.party_index].members[source.member_index];

        // update dots
        for i in 0..source.dots.len() {
            process_damage(source, source.dots[i].aspect, source.dots[i].damage_value);
        }

        for dot in &mut source.dots {
            use Lifetime::*;
            match dot.lifetime {
                Active { ref mut duration } => if *duration > 0 { *duration -= 1 },
                Constant => (),
            }
        }

        // update modifiers
        macro_rules! update_modifiers {
            ($modifiers: ident) => {
                for modifier in &mut source.$modifiers {
                    use Lifetime::*;
                    match modifier.lifetime {
                        Active { ref mut duration } => if *duration > 0 { *duration -= 1 },
                        Constant => (),
                    }
                }
            };
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

fn calculate_damage_value(target: &Combatant, source: Option<&Combatant>, aspect: Aspect, scaling: Fraction) -> u32 {
    let raw_damage = match source {
        None => target.raw_damage(aspect),
        Some(source) => source.raw_damage(aspect),
    };

    raw_damage * scaling.numerator / scaling.denominator
}

fn process_damage(target: &mut Combatant, aspect: Aspect, damage_value: u32) {
    let defense_value = target.defense(aspect);
    let absorbtion_value = target.absorbtion(aspect);

    if damage_value < defense_value { return; }
    let damage_value = damage_value - defense_value;

    if absorbtion_value > 100 { target.hp += damage_value * (absorbtion_value - 100) / 100; }
    else { target.hp -= std::cmp::min(target.hp, damage_value * (100 - absorbtion_value) / 100); }
}
