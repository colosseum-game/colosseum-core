use crate::{
    combatant::{
        Combatant,
        Stat,
    },
    damage::{
        ActiveDamage,
        DamageType,
    },
    modifiers::Modifier,
};

#[allow(dead_code)]
#[derive(Debug)]
pub enum EffectSource<'a> {
    None,
    Other(&'a Combatant<'a>),
    Target,
}

#[derive(Debug)]
pub enum Effect {
    ActiveDamage(DamageType, u32, u32, u32),
    ActiveModifier(Modifier, Stat, u32),
    Damage(DamageType, u32, u32),
    Modifier(Modifier, Stat),
}

pub enum EffectFunction {
    Sourced(Box<dyn Fn(&mut Combatant, EffectSource)>),
    Unsourced(Box<dyn Fn(&mut Combatant)>),
}

impl Effect {
    pub fn function(&self) -> EffectFunction {
        match *self {
            Effect::ActiveDamage(damage_type, multiplier, divisor, turns_to_live) => EffectFunction::Sourced(Box::new(move |target, source| apply_active_damage(target, source, damage_type, multiplier, divisor, turns_to_live))),
            Effect::ActiveModifier(modifier, stat, turns_to_live) => EffectFunction::Unsourced(Box::new(move |target| apply_active_modifier(target, modifier, stat, turns_to_live))),
            Effect::Damage(damage_type, multiplier, divisor) => EffectFunction::Sourced(Box::new(move |target, source| apply_damage(target, source, damage_type, multiplier, divisor))),
            Effect::Modifier(modifier, stat) => EffectFunction::Unsourced(Box::new(move |target| apply_modifier(target, modifier, stat))),
        }
    }
}

fn apply_active_damage(target: &mut Combatant, source: EffectSource, damage_type: DamageType, multiplier: u32, divisor: u32, turns_to_live: u32) {
    let active_damage = ActiveDamage {
        damage_type: damage_type,
        value: damage_type.damage_from_source(target, source, multiplier, divisor),
    };

    target.active_damage.push((active_damage, turns_to_live));
}

fn apply_active_modifier(target: &mut Combatant, modifier: Modifier, stat: Stat, turns_to_live: u32) {
    target.active_stat_modifiers[stat as usize].push((modifier, turns_to_live))
}

fn apply_damage(target: &mut Combatant, source: EffectSource, damage_type: DamageType, multiplier: u32, divisor: u32) {
    let damage_value = damage_type.damage_from_source(target, source, multiplier, divisor);
    let damage_reduction = damage_type.damage_reduction_from_target(target);
    target.hp -= damage_value - damage_reduction; // TODO: overflow stuff nananana
}

fn apply_modifier(target: &mut Combatant, modifier: Modifier, stat: Stat) {
    target.stat_modifiers[stat as usize].push(modifier);
}
