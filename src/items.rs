use crate::{
    actions::TargetFlag,
    effects::Effect,
};

use std::fmt;

#[derive(Debug)]
pub struct Item<'a> {
    pub display_name: &'a str,
    pub effects: &'a [Effect],
    pub target_flags: &'a [&'a [TargetFlag]],
    pub target_count: u32,
}

impl<'a> fmt::Display for Item<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.display_name)
    }
}
