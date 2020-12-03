use crate::aspect::Aspect;

use serde::{
    Deserialize,
    Serialize,
};

macro_rules! generate_wearable {
    ($wearable: ident) => {
        #[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
        pub struct $wearable {
            pub display_name: String,
            pub description: String,
            pub fire_defense: u32,
            pub frost_defense: u32,
            pub lightning_defense: u32,
            pub physical_defense: u32,
        }

        impl $wearable {
            pub fn get_defense(&self, aspect: Aspect) -> u32 {
                match aspect {
                    Aspect::Fire => self.fire_defense,
                    Aspect::Frost => self.frost_defense,
                    Aspect::Lightning => self.lightning_defense,
                    Aspect::Physical => self.physical_defense,
                }
            }
        }
    }
}

generate_wearable!(Bodywear);
generate_wearable!(Footwear);
generate_wearable!(Handwear);
generate_wearable!(Headwear);
generate_wearable!(Legwear);
