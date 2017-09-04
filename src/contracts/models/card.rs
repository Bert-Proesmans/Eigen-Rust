use std::fmt::{Debug, Display};

use contracts::effects::IEffect;
use enums::{EGameTags,ECardSets,ECardClasses,ECardTypes};

pub trait ICard: Debug + Display + Sync {
    fn id(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn card_type(&self) -> Option<ECardTypes>;

    fn tag_value(&self, tag: EGameTags) -> u32;

    fn has_implemented_effects(&self) -> bool;
    // +'static -> underlying object is NOT a reference (which typically has a lower lifetime duration).
    fn effects(&self) -> Option<&Vec<Box<IEffect + 'static>>>;

    fn is_collectible(&self) -> bool;
    fn card_set(&self) -> Option<ECardSets>;
    fn card_class(&self) -> Option<ECardClasses>;

    fn has_combo(&self) -> bool;
    fn card_cost(&self) -> u32;
    fn has_overload(&self) -> bool;
    fn overload(&self) -> u32;
    fn max_allowed_in_deck(&self) -> u32;
}
