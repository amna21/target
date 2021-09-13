use crate::prelude::*;

mod map_render;
mod entity_render;
mod player_input;
mod random_move;
mod chasing;
mod end_turn;
mod movement;
mod hud;
mod tooltips;
mod combat;
mod fov;
mod use_items;

pub fn build_input_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(player_input::player_input_system())
        .add_system(fov::fov_system())
        .flush()
        .add_system(map_render::map_render_system())
        .add_system(entity_render::entity_render_system())
        .add_system(hud::hud_system())
        .add_system(tooltips::tooltips_system())
        .build()
}

//START: player_scheduler
pub fn build_player_scheduler() -> Schedule {
    Schedule::builder()
        // START_HIGHLIGHT
        .add_system(use_items::use_items_system())
        // END_HIGHLIGHT
        .add_system(combat::combat_system())
        .flush()
        //END: player_scheduler
        .add_system(movement::movement_system())
        .flush()
        .add_system(fov::fov_system())
        .flush()
        .add_system(map_render::map_render_system())
        .add_system(entity_render::entity_render_system())
        .add_system(hud::hud_system())
        .add_system(end_turn::end_turn_system())
        .build()
}

//START: monster_scheduler
pub fn build_monster_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(random_move::random_move_system())
        .add_system(chasing::chasing_system())
        .flush()
        // START_HIGHLIGHT
        .add_system(use_items::use_items_system())
        // END_HIGHLIGHT
        .add_system(combat::combat_system())
        .flush()
        //END: monster_scheduler
        .add_system(movement::movement_system())
        .flush()
        .add_system(fov::fov_system())
        .flush()
        .add_system(map_render::map_render_system())
        .add_system(entity_render::entity_render_system())
        .add_system(hud::hud_system())
        .add_system(end_turn::end_turn_system())
        .build()
}
