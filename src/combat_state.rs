use crate::{
    aspect::Aspect,
    attribute::Attribute,
    combat_event::CombatEvent,
    combatant::Combatant,
    consumable::Consumable,
    dot::DOT,
    effect::SubEffect,
    fraction::Fraction,
    lifetime::Lifetime,
    party::Party,
    skill::Skill,
    target::Target,
    weapon::Weapon,
};

use serde::{
    Deserialize,
    Serialize
};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CombatState {
    pub parties: Vec<Party>,
}

impl CombatState {
    pub fn transform(&mut self, combat_event: CombatEvent) {
        use CombatEvent::*;
        match combat_event {
            AttackEvent { source, targets } => {
                for target in targets {
                    let (source, target) = self.get_combatant_handles(source, target);
                    let weapon = match source {
                        None => target.weapon,
                        Some(source) => source.weapon,
                    };

                    match weapon {
                        None => (), // TODO, fist attack or something
                        Some(weapon) => for sub_effect in &<&Weapon>::from(weapon).effect.sub_effects {
                            handle_sub_effect(source, target, *sub_effect);
                        }
                    }
                }
                self.post_turn(source);
            },
            ConsumableEvent { source, targets, consumable } => {
                let consumable = <&Consumable>::from(consumable);
                for target in targets {
                    let (source, target) = self.get_combatant_handles(source, target);
                    for sub_effect in &consumable.effect.sub_effects { handle_sub_effect(source, target, *sub_effect); }
                }
                self.post_turn(source)
            },
            SkillEvent { source, targets, skill } => {
                for target in targets {
                    let (source, target) = self.get_combatant_handles(source, target);
                    let skill = <&Skill>::from(skill);
                    for sub_effect in &skill.effect.sub_effects { handle_sub_effect(source, target, *sub_effect); }
                }

                self.post_turn(source);
            },
            SkipEvent => (),
        }
    }

    fn get_combatant_handles(&mut self, source: Target, target: Target) -> (Option<&Combatant>, &mut Combatant) {
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
    }

    fn post_turn(&mut self, source: Target) {
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

    pub fn calculate_ready(&mut self) -> Target {
        loop {
            let mut readied = vec![];

            for party_index in 0..self.parties.len() {
                for member_index in 0..self.parties[party_index].members.len() {
                    if self.parties[party_index].members[member_index].fatigue == 0 {
                        readied.push(Target { party_index, member_index })
                    }
                }
            }

            if let Some(ready) = readied.first() { return *ready; }
            self.ready_all();
        }
    }

    fn ready_all(&mut self) {
        for party in &mut self.parties {
            for member in &mut party.members {
                member.ready_up();
            }
        }
    }
}

fn handle_sub_effect(source: Option<&Combatant>, target: &mut Combatant, sub_effect: SubEffect) {
    match sub_effect {
        SubEffect::Damage { aspect,  scaling } => {
            let damage_value = calculate_damage_value(source, target, aspect, scaling);
            process_damage(target, aspect, damage_value);
        },
        SubEffect::DOT { aspect, scaling, lifetime } => {
            let damage_value = calculate_damage_value(source, target, aspect, scaling);

            let dot = DOT {
                aspect,
                damage_value,
                lifetime,
            };

            target.dots.push(dot);
        },
        SubEffect::Modifier { modifier, attribute } => {
            use Attribute::*;
            match attribute {
                Agility => target.agility_modifiers.push(modifier),
                Dexterity => target.dexterity_modifiers.push(modifier),
                Intelligence => target.intelligence_modifiers.push(modifier),
                Mind => target.mind_modifiers.push(modifier),
                Strength => target.strength_modifiers.push(modifier),
                Vigor => target.vigor_modifiers.push(modifier),
                Vitality => target.vitality_modifiers.push(modifier),
            }
        }
    }
}

fn process_damage(target: &mut Combatant, aspect: Aspect, damage_value: u32) {
    let defense_value = target.defense(aspect);
    let absorbtion_value = target.absorbtion(aspect);

    if damage_value < defense_value { return; }
    let damage_value = damage_value - defense_value;

    if absorbtion_value > 100 { target.hp += damage_value * (absorbtion_value - 100) / 100; }
    else { target.hp -= std::cmp::min(target.hp, damage_value * (100 - absorbtion_value) / 100); }
}

fn calculate_damage_value(source: Option<&Combatant>, target: &Combatant, aspect: Aspect, scaling: Fraction) -> u32 {
    let raw_damage = match source {
        None => target.raw_damage(aspect),
        Some(source) => source.raw_damage(aspect),
    };

    raw_damage * scaling.numerator / scaling.denominator
}
