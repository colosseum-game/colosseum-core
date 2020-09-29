use crate::{
    combatant::Combatant,
    io,
    items::Item,
};

use std::fmt;

#[derive(Debug)]
pub enum Action {
    Attack,
    Cry,
    Skip,
    UseItem,
}

impl Action {
    pub fn function(&self) -> fn(&mut [&mut Combatant], &[usize], usize) {
        match *self {
            Action::Attack => attack,
            Action::Cry => cry,
            Action::Skip => skip,
            Action::UseItem => use_item,
        }
    }
}

impl fmt::Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let display_name = match *self {
            Action::Attack => "Attack".to_string(),
            Action::Cry => "Cry".to_string(),
            Action::Skip => "Skip".to_string(),
            Action::UseItem => "Use Item".to_string(),
        };

        write!(f, "{}", display_name)
    }
}

fn attack(combatants: &mut [&mut Combatant], targets: &[usize], caster: usize) {
    for target in targets {
        if combatants[*target].physical_resistance > combatants[caster].physical_attack { continue; }

        combatants[*target].hp -= std::cmp::min(
            combatants[caster].physical_attack - combatants[*target].physical_resistance,
            combatants[*target].hp
        );

        println!(
            "{} attacked {} for {} damage!",
            combatants[caster],
            combatants[*target],
            combatants[caster].physical_attack - combatants[*target].physical_resistance
        );
    };
}

fn cry(combatants: &mut [&mut Combatant], targets: &[usize], caster: usize) {
    println!("{} whined like a little bitch", combatants[caster]);
    for target in targets {
        println!(
            "{} felt bad for {}",
            combatants[*target],
            combatants[caster],
        );
    };
}

fn skip(combatants: &mut [&mut Combatant], _: &[usize], caster: usize) {
    println!("{} skipped their turn", combatants[caster]);
}

fn get_item_index(items: &[Item]) -> usize {
    items.iter().enumerate().for_each(|(i, item)| {
        println!("{}: {}", i, item);
    });

    let input = io::get_input();
    if input >= items.len() { get_item_index(items) }
    else { input }
}

fn use_item(combatants: &mut [&mut Combatant], targets: &[usize], caster: usize) {
    let items = vec![Item::HealthPotion];
    let item = &items[get_item_index(&items)];
    println!("{} uses {}", combatants[caster], item);
    item.function()(combatants, targets);
}
