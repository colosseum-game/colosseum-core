pub use crate::combat_state_generated::{
    CombatState,
    file_descriptor,
};

use crate::{
    aspect::Aspect,
    attribute::Attribute,
    combat_event::CombatEvent,
    combatant::Combatant,
    dot::DOT,
    fraction::Fraction,
    store::SKILL_STORE,
    target::Target,
};

impl CombatState {
    pub fn transform(&mut self, source: Target, mut event: CombatEvent) {
        let source_party_index = source.party_index as usize;
        let source_member_index = source.member_index as usize;

        if event.has_attack_event() {

        } else if event.has_consumable_event() {

        } else if event.has_equipable_event() {
            
        } else if event.has_forfeit_event() {

        } else if event.has_skill_event() {
            let skill_event = event.take_skill_event();
            let skill = SKILL_STORE.get(&skill_event.skill).unwrap();
            let targets = skill_event.targets;

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
                for sub_effect in &skill.effect.get_ref().sub_effects {
                    if sub_effect.has_damage_effect() {
                        let damage_effect = sub_effect.get_damage_effect();
                        let aspect = damage_effect.aspect.enum_value().unwrap();
                        let scaling = damage_effect.scaling.get_ref();
                        process_damage(target, aspect, calculate_damage_value(target, source, aspect, scaling))
                    } else if sub_effect.has_dot_effect() {
                        let dot_effect = sub_effect.get_dot_effect();
                        let aspect = dot_effect.aspect.enum_value().unwrap();
                        let scaling = dot_effect.scaling.get_ref();
                        let lifetime = if dot_effect.has_lifetime_duration() { Some(dot_effect.get_lifetime_duration()) } else { None };
                        let damage_value = calculate_damage_value(target, source, aspect, scaling);

                        let mut dot = DOT::new();
                        dot.aspect = protobuf::ProtobufEnumOrUnknown::from(aspect);
                        dot.damage_value = damage_value;
                        if let Some(duration) = lifetime { dot.set_lifetime_duration(duration) };

                        target.dots.push(dot);
                    } else if sub_effect.has_modifier_effect() {
                        use Attribute::*;

                        let modifier_effect = sub_effect.get_modifier_effect();
                        let modifier = modifier_effect.modifier.clone().unwrap();
                        let attribute = modifier_effect.attribute.unwrap();

                        match attribute {
                            ATTRIBUTE_AGILITY => target.agility_modifiers.push(modifier),
                            ATTRIBUTE_DEXTERITY => target.dexterity_modifiers.push(modifier),
                            ATTRIBUTE_INTELLIGENCE => target.intelligence_modifiers.push(modifier),
                            ATTRIBUTE_MIND => target.mind_modifiers.push(modifier),
                            ATTRIBUTE_STRENGTH => target.strength_modifiers.push(modifier),
                            ATTRIBUTE_VIGOR => target.vigor_modifiers.push(modifier),
                            ATTRIBUTE_VITALITY => target.vitality_modifiers.push(modifier),
                        }
                    }
                }
            }
        }

        let source = &mut self.parties[source_party_index].members[source_member_index];

        // update dots
        for i in 0..source.dots.len() {
            process_damage(source, source.dots[i].aspect.enum_value().unwrap(), source.dots[i].damage_value);
        }

        for dot in &mut source.dots {
            if dot.has_lifetime_duration() && dot.get_lifetime_duration() > 0 {
                dot.set_lifetime_duration(dot.get_lifetime_duration() - 1);
            }
        }

        // update modifiers
        macro_rules! update_modifiers {
            ($modifiers: ident) => {
                for modifier in &mut source.$modifiers {
                    if modifier.has_lifetime_duration() && modifier.get_lifetime_duration() > 0 {
                        modifier.set_lifetime_duration(modifier.get_lifetime_duration() - 1);
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
