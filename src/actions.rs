use crate::{
    combatant::{
        Combatant,
        Gender,
    },
    io,
    items::Item,
};

use std::fmt;

#[derive(Debug)]
pub enum Action {
    Attack,
    Burn,
    Cry,
    MisoAttack,
    Skip,
    UseItem,
}

impl Action {
    pub fn function(&self) -> fn(&mut [&mut Combatant], &[usize], usize) {
        match *self {
            Action::Attack => attack,
            Action::Burn => burn,
            Action::Cry => cry,
            Action::MisoAttack => miso_attack,
            Action::Skip => skip,
            Action::UseItem => use_item,
        }
    }
}

impl fmt::Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let display_name = match *self {
            Action::Attack => "Attack".to_string(),
            Action::Burn => "Burn".to_string(),
            Action::Cry => "Cry".to_string(),
            Action::MisoAttack => "Misogynist Attack".to_string(),
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
            combatants[caster].physical_attack - std::cmp::min(
                combatants[*target].physical_resistance,
                combatants[caster].physical_attack
            ),
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

fn burn(combatants: &mut [&mut Combatant], targets: &[usize], caster: usize) {
    for target in targets {
        let temp: u32 = combatants[*target].hp;

        combatants[*target].hp -= std::cmp::min(
            combatants[caster].flammability - std::cmp::min(
                combatants[*target].physical_resistance,
                combatants[caster].flammability
            ),
            combatants[*target].hp
        );

        println! (
            "{} burnt the shit out of {} for {} damage!",
            combatants[caster],
            combatants[*target],
            temp - combatants[*target].hp,
        );
        println! (
            "{} took {} burn damage from previous wounds!",
            combatants[*target],
            combatants[*target].damage_over_time,
        );

        combatants[*target].damage_over_time += 1;
    }
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

fn miso_attack(combatants: &mut [&mut Combatant], targets: &[usize], caster: usize) {
    for target in targets {
        match combatants[*target].gender {
            Gender::Male => {
                combatants[*target].hp -= std::cmp::min(
                    combatants[caster].physical_attack / 2 - std::cmp::min(
                        combatants[*target].physical_resistance,
                        combatants[caster].physical_attack / 2
                    ),
                    combatants[*target].hp
                );

                println! (
                    "{} saw that {} was a man and overestimated him, hitting for {} damage",
                    combatants[caster],
                    combatants[*target],
                    combatants[caster].physical_attack / 2 - combatants[*target].physical_resistance,
                );
            },
            _ => {
                combatants[*target].hp -= std::cmp::min(
                    combatants[caster].physical_attack * 2 - std::cmp::min(
                        combatants[*target].physical_resistance,
                        combatants[caster].physical_attack * 2
                    ),
                    combatants[*target].hp
                );

                println! (
                    "{} saw that {} was a woman and was filled with rage, hitting for {} damage",
                    combatants[caster],
                    combatants[*target],
                    combatants[caster].physical_attack * 2 - combatants[*target].physical_resistance,
                );

                combatants[caster].physical_attack /= 2;
                combatants[caster].physical_resistance /= 2;

                println! (
                    "{}'s saw this and harassed him to the point he could no longer attend the gym. As a result, he lost half of his physical attack and resistance.",
                    combatants[caster],
                );
            }
        }
    }
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
