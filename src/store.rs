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
    ($dir_const: ident, $path: literal, $stored: ident, $store: ident, $store_const: ident, $getter: ident) => {
        const $dir_const: Dir = include_dir!($path);

        pub struct $store<'a>(HashMap<&'a str, $stored>);

        lazy_static! {
            pub static ref $store_const: $store<'static> = $store({
                let mut hashmap = HashMap::new();
                for file in $dir_const.files {
                    let file_name = file.path().file_stem().unwrap().to_str().unwrap();
                    let stored = serde_json::from_str(file.contents_utf8().unwrap()).expect(&format!("failed to deserialize {}: {}", $path, file_name));
                    hashmap.insert(file_name, stored);
                }

                hashmap
            });
        }

        pub fn $getter(identifier: &str) -> Option<&$stored> {
            $store_const.0.get(identifier)
        }
    };
}

generate_store!(BODYWEAR_DIRECTORY, "content/bodywear", Bodywear, BodywearStore, BODYWEAR_STORE, get_bodywear);
generate_store!(CONSUMABLE_DIRECTORY, "content/consumable", Consumable, ConsumableStore, CONSUMABLE_STORE, get_consumable);
generate_store!(FOOTWEAR_DIRECTORY, "content/footwear", Footwear, FootwearStore, FOOTWEAR_STORE, get_footwear);
generate_store!(HANDWEAR_DIRECTORY, "content/handwear", Handwear, HandwearStore, HANDWEAR_STORE, get_handwear);
generate_store!(HEADWEAR_DIRECTORY, "content/headwear", Headwear, HeadwearStore, HEADWEAR_STORE, get_headwear);
generate_store!(LEGWEAR_DIRECTORY, "content/legwear", Legwear, LegwearStore, LEGWEAR_STORE, get_legwear);
generate_store!(SKILL_DIRECTORY, "content/skill", Skill, SkillStore, SKILL_STORE, get_skill);
generate_store!(WEAPON_DIRECTORY, "content/weapon", Weapon, WeaponStore, WEAPON_STORE, get_weapon);

pub fn initialize() {
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
        initialize();
    }

    #[test]
    fn quick_wearable_check() {
        let bodywear = get_bodywear("breakers_longsleeve");
        let bodywear2 = serde_json::from_str(include_str!("../content/bodywear/breakers_longsleeve.json")).unwrap();
        assert_eq!(bodywear, Some(&bodywear2));
    }
}
