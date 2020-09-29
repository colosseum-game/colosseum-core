use crate::combatant::Combatant;

use std::fmt;
use std::fmt::{
    Display,
    Formatter,
};

#[derive(Debug)]
pub enum Item {
    HealthPotion,
}

impl Item {
    pub fn function(&self) -> fn(&mut [&mut Combatant], &[usize]) {
        match *self {
            Item::HealthPotion => health_potion,
        }
    }
}

impl Display for Item {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let display_name = match *self {
            Item::HealthPotion => "Health Potion".to_string(),
        };

        write!(f, "{}", display_name)
    }
}

fn health_potion(combatants: &mut [&mut Combatant], targets: &[usize]) {
    targets.iter().for_each(|target| {
        println!("{} restores 25 hp", combatants[*target]);
        combatants[*target].hp += std::cmp::min(25, combatants[*target].hp_max - combatants[*target].hp)
    });
}
