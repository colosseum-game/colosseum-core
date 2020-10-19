use crate::{
    combatant::{
        Combatant,
        Stat,
    },
    effects::EffectSource,
};

#[derive(Clone, Copy, Debug)]
pub enum DamageType {
    Fire,
    Physical,
}

impl DamageType {
    pub fn damage_from_source(&self, target: &Combatant, source: EffectSource, multiplier: u32, divisor: u32) -> u32 {
        match *self {
            DamageType::Fire => {
                let fire_damage = match source {
                    EffectSource::None => 1,
                    EffectSource::Origin => target.get_stat(Stat::FireAttack),
                    EffectSource::Other(source) => source.get_stat(Stat::FireAttack),
                } * multiplier / divisor;
    
                let fire_resistance = target.get_stat(Stat::FireResistance);
    
                if fire_resistance == 0 { fire_damage }
                else { fire_damage / fire_resistance }
            },
            DamageType::Physical => {
                let physical_damage = match source {
                    EffectSource::None => 1,
                    EffectSource::Origin => target.get_stat(Stat::PhysicalAttack),
                    EffectSource::Other(source) => source.get_stat(Stat::PhysicalAttack),
                } * multiplier / divisor;
    
                let physical_resistance = target.get_stat(Stat::PhysicalResistance);
    
                if physical_resistance > physical_damage { 0 }
                else { physical_damage - physical_resistance }
            },
        }
    }

    pub fn damage_reduction_from_target(&self, target: &Combatant) -> u32 {
        0
    }
}

#[derive(Clone, Copy, Debug)]
pub struct DamagePerTurn {
    pub damage_type: DamageType,
    pub value: u32,
    pub turns_to_live: u32,
}
