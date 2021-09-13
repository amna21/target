use crate::prelude::*;
use super::MapArchitect;//<callout id="co.mid.traits.use_trait" />

pub struct EmptyArchitect {}//<callout id="co.mid.traits.empty_struct" />

impl MapArchitect for EmptyArchitect {//<callout id="co.mid.traits.impl" />
    fn new(&mut self, rng: &mut RandomNumberGenerator) -> MapBuilder {//<callout id="co.mid.traits.build" />
        let mut mb = MapBuilder{//<callout id="co.mid.traits.minimal" />
            map : Map::new(),
            rooms: Vec::new(),
            monster_spawns : Vec::new(),
            player_start : Point::zero(),
            amulet_start : Point::zero()
        };
        mb.fill(TileType::Floor);
        mb.player_start = Point::new(SCREEN_WIDTH/2, SCREEN_HEIGHT/2);
        mb.amulet_start = mb.find_most_distant();//<callout id="co.mid.traits.place_amulet" />
        for _ in 0..50 {//<callout id="co.mid.traits.fifty_mobs" />
            mb.monster_spawns.push(
                Point::new(
                    rng.range(1, SCREEN_WIDTH),
                    rng.range(1, SCREEN_WIDTH)
                )
            )
        }
        mb
    }
}