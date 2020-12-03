pub use crate::combat_state_generated::CombatState;

use crate::{
    aspect::Aspect,
    attribute::Attribute,
    combat_event::{
        combat_event,
        CombatEvent,
    },
    combatant::Combatant,
    dot::Dot,
    effect::sub_effect::SubEffect,
    fraction::Fraction,
    lifetime::lifetime,
    store::SKILL_STORE,
    target::Target,
};

impl CombatState {
    pub fn transform(&mut self, source: Target, mut combat_event: CombatEvent) {
        let source_party_index = source.party_index as usize;
        let source_member_index = source.member_index as usize;

        use combat_event::CombatEvent::*;
        match combat_event.combat_event.unwrap() {
            AttackEvent(event) => (),
            ConsumableEvent(event) => (),
            EquipableEvent(equipable) => (),
            ForfeitEvent(_) => (),
            SkipEvent(_) => (),
            SkillEvent(event) => {
                let skill = SKILL_STORE.get(&event.skill).unwrap();
                let targets = event.targets;
    
                for target in &targets {
                    let target_party_index = target.party_index as usize;
                    let target_member_index = target.member_index as usize;
    
                    let (source, target) = {
                        if source_party_index > target_party_index {
                            let (target_container, source_container) = self.parties.split_at_mut(source_party_index);
                            let source = Some(&source_container[0].members[source_member_index]);
                            let target = &mut target_container[target_party_index].members[target_member_index];
                            (source, target)
                        } else if source_party_index < target_party_index {
                            let (source_container, target_container) = self.parties.split_at_mut(target_party_index);
                            let source = Some(&source_container[source_party_index].members[source_member_index]);
                            let target = &mut target_container[0].members[target_member_index];
                            (source, target)
                        } else {
                            if source_member_index > target_member_index {
                                let (target_container, source_container) = self.parties[source_party_index].members.split_at_mut(source_member_index);
                                let source = Some(&source_container[0]);
                                let target = &mut target_container[target_member_index];
                                (source, target)
                            } else if source_member_index < target_member_index {
                                let (source_container, target_container) = self.parties[source_party_index].members.split_at_mut(target_member_index);
                                let source = Some(&source_container[source_member_index]);
                                let target = &mut target_container[0];
                                (source, target)
                            } else {
                                let source = None;
                                let target = &mut self.parties[source_party_index].members[source_member_index];
                                (source, target)
                            }
                        }
                    };

                    // apply every effect in the current sub_action
                    for sub_effect in &skill.effect.unwrap().sub_effects {
                        use SubEffect::*;
                        match sub_effect.sub_effect.unwrap() {
                            DamageEffect(effect) => {
                                let aspect = effect.aspect;
                                let scaling = effect.scaling;
                                process_damage(target, Aspect::try_from(aspect), calculate_damage_value(target, source, aspect.into().unwrap(), &scaling.unwrap()))
                            },
                            DotEffect(effect) => {
                                let aspect = effect.aspect;
                                let scaling = effect.scaling;
                                let lifetime = effect.lifetime;
                                let damage_value = calculate_damage_value(target, source, aspect.into().unwrap(), &scaling.unwrap());

                                let mut dot = Dot {
                                    aspect,
                                    damage_value,
                                    lifetime,
                                };

                                target.dots.push(dot);
                            }
                            ModifierEffect(effect) => {
                                let modifier = effect.modifier.unwrap();
                                let attribute = effect.attribute;

                                use Attribute::*;
                                match attribute.into().unwrap() {
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
                }
            },
        }

        let source = &mut self.parties[source_party_index].members[source_member_index];

        // update dots
        for i in 0..source.dots.len() {
            process_damage(source, source.dots[i].aspect.into().unwrap(), source.dots[i].damage_value);
        }

        for dot in &mut source.dots {
            use lifetime::Lifetime::*;
            match dot.lifetime.unwrap().lifetime {
                None => (),
                Some(ref mut lifetime) => {
                    match lifetime {
                        Active(ref mut duration) => if *duration > 0 { *duration -= 1 }, 
                    }
                }
            }
        }

        // update modifiers
        macro_rules! update_modifiers {
            ($modifiers: ident) => {
                for modifier in &mut source.$modifiers {
                    use lifetime::Lifetime::*;
                    match modifier.lifetime.unwrap().lifetime {
                        None => (),
                        Some(ref mut lifetime) => {
                            match lifetime {
                                Active(ref mut duration) => if *duration > 0 { *duration -= 1 }, 
                            }
                        }
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

fn calculate_damage_value(target: &Combatant, source: Option<&Combatant>, aspect: Aspect, scaling: &Fraction) -> u32 {
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
