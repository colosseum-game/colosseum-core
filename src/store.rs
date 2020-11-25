use crate::{
    consumable::Consumable,
    skill::Skill,
    weapon::Weapon,
    wearable::{
        Bodywear,
        Footwear,
        Handwear,
        Headwear,
        Legwear,
    },
};

use include_dir::{
    Dir,
    include_dir,
};

use lazy_static::lazy_static;

use std::collections::HashMap;

macro_rules! generate_store {
    ($dir_const: ident, $path: literal, $stored: ident, $store: ident, $store_const: ident) => {
        const $dir_const: Dir = include_dir!($path);

        pub struct $store<'a>(HashMap<&'a str, $stored>);

        impl<'a> $store<'a> {
            pub fn get(&self, identifier: &str) -> Option<&$stored> {
                self.0.get(identifier)
            }
        }

        lazy_static! {
            pub static ref $store_const: $store<'static> = $store({
                let mut hashmap = HashMap::new();
                for file in $dir_const.files {
                    let file_name = file.path().file_stem().unwrap().to_str().unwrap();
                    let stored = protobuf::json::parse_from_str(file.contents_utf8().unwrap()).expect("failed to parse content");
                    hashmap.insert(file_name, stored);
                }

                hashmap
            });
        }
    };
}

generate_store!(BODYWEAR_DIRECTORY, "content/bodywear", Bodywear, BodywearStore, BODYWEAR_STORE);
generate_store!(CONSUMABLE_DIRECTORY, "content/consumable", Consumable, ConsumableStore, CONSUMABLE_STORE);
generate_store!(FOOTWEAR_DIRECTORY, "content/footwear", Footwear, FootwearStore, FOOTWEAR_STORE);
generate_store!(HANDWEAR_DIRECTORY, "content/handwear", Handwear, HandwearStore, HANDWEAR_STORE);
generate_store!(HEADWEAR_DIRECTORY, "content/headwear", Headwear, HeadwearStore, HEADWEAR_STORE);
generate_store!(LEGWEAR_DIRECTORY, "content/legwear", Legwear, LegwearStore, LEGWEAR_STORE);
generate_store!(SKILL_DIRECTORY, "content/skill", Skill, SkillStore, SKILL_STORE);
generate_store!(WEAPON_DIRECTORY, "content/weapon", Weapon, WeaponStore, WEAPON_STORE);

pub fn initialize_stores() {
    lazy_static::initialize(&BODYWEAR_STORE);
    lazy_static::initialize(&CONSUMABLE_STORE);
    lazy_static::initialize(&FOOTWEAR_STORE);
    lazy_static::initialize(&HANDWEAR_STORE);
    lazy_static::initialize(&HEADWEAR_STORE);
    lazy_static::initialize(&LEGWEAR_STORE);
    lazy_static::initialize(&SKILL_STORE);
    lazy_static::initialize(&WEAPON_STORE);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initialization() {
        initialize_stores();
    }

    #[test]
    fn quick_wearable_check() {
        let default = BODYWEAR_STORE.get("default");
        let default2 = protobuf::json::parse_from_str(include_str!("../content/bodywear/default.json")).unwrap();
        assert_eq!(default, Some(&default2));
    }
}
