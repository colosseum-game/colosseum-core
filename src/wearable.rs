pub use crate::{
    bodywear::Bodywear,
    footwear::Footwear,
    handwear::Handwear,
    headwear::Headwear,
    legwear::Legwear,
};

use crate::aspect::Aspect;

macro_rules! generate_wearable_impl {
    ($wearable: ident) => {
        impl $wearable {
            pub fn get_defense(&self, aspect: Aspect) -> u32 {
                use Aspect::*;

                match aspect {
                    ASPECT_NONE => 0,
                    ASPECT_FIRE => self.fire_defense,
                    ASPECT_FROST => self.frost_defense,
                    ASPECT_LIGHTNING => self.lightning_defense,
                    ASPECT_PHYSICAL => self.physical_defense,
                }
            }
        }
    }
}

generate_wearable_impl!(Bodywear);
generate_wearable_impl!(Footwear);
generate_wearable_impl!(Handwear);
generate_wearable_impl!(Headwear);
generate_wearable_impl!(Legwear);
