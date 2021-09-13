pub use crate::prelude::*;
//START: hashset
use std::collections::HashSet;
//END: hashset

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Render {
    pub color : ColorPair,
    pub glyph : FontCharType
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Player;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Enemy;

//START: amulet
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Item;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AmuletOfYala;
//END: amulet

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MovingRandomly;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ChasingPlayer;


#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WantsToMove {
    pub entity : Entity,
    pub destination : Point
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WantsToAttack {
    pub attacker : Entity,
    pub victim : Entity
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Health {
    pub current: i32,
    pub max: i32
}

#[derive(Clone, PartialEq)]
pub struct Name(pub String);

//START: field_of_view_component
#[derive(Clone, Debug, PartialEq)]// <callout id="co.wcis.fov.nocopy" />
pub struct FieldOfView{
    pub visible_tiles : HashSet<Point>,// <callout id="co.wcis.fov.hashset" />
    pub radius: i32,// <callout id="co.wcis.fov.radius" />
    pub is_dirty: bool// <callout id="co.wcis.fov.is_dirty" />
}
//END: field_of_view_component

//START: impl_fov
impl FieldOfView {
    pub fn new(radius: i32) -> Self {
        Self{
            visible_tiles: HashSet::new(),// <callout id="co.wcis.fov.new_hashset" />
            radius,
            is_dirty: true// <callout id="co.wcis.fov.start_dirty" />
        }
    }

    pub fn clone_dirty(&self) -> Self {// <callout id="co.wcis.fov.clone_dirty" />
        Self {
            visible_tiles: HashSet::new(),
            radius: self.radius,
            is_dirty: true,
        }
    }
}
//END: impl_fov