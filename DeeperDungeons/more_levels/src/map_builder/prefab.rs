//START: prefab
use crate::prelude::*;

const FORTRESS : (&str, i32, i32) = ("
------------
---######---
---#----#---
---#-M--#---
-###----###-
--M------M--
-###----###-
---#----#---
---#----#---
---######---
------------
", 12, 11);
//END: prefab

//START: new_fun
pub fn apply_prefab(mb: &mut MapBuilder, rng: &mut RandomNumberGenerator) {
    //END: new_fun
    //START: dijkstra
    let mut placement = None;

    let dijkstra_map = DijkstraMap::new(
        SCREEN_WIDTH,
        SCREEN_HEIGHT,
        &vec![mb.map.point2d_to_index(mb.player_start)],
        &mb.map,
        1024.0
    );
    //END: dijkstra

    //START: try_add
    let mut attempts = 0;// <callout id="co.prefab.attempt0" />
    while placement.is_none() && attempts < 10 {// <callout id="co.prefab.while" />
        let dimensions = Rect::with_size(// <callout id="co.prefab.rect" />
            rng.range(0, SCREEN_WIDTH - FORTRESS.1),
            rng.range(0, SCREEN_HEIGHT - FORTRESS.2),
            FORTRESS.1,
            FORTRESS.2
        );

        let mut can_place = false;// <callout id="co.prefab.can_place" />
        dimensions.for_each(|pt| {// <callout id="co.prefab.dim_iter" />
            let idx = mb.map.point2d_to_index(pt);
            let distance = dijkstra_map.map[idx];
            if distance < 2000.0 && distance > 20.0 && mb.amulet_start != pt {// <callout id="co.prefab.dim_comp" />
                can_place = true;
            }
        });

        if can_place {// <callout id="co.prefab.if_can_place" />
            placement = Some(Point::new(dimensions.x1, dimensions.y1));
            let points = dimensions.point_set();
            mb.monster_spawns.retain(|pt| !points.contains(pt) );// <callout id="co.prefab.retain" />
        }
        attempts += 1;
    }
    //END: try_add

    //START: place
    if let Some(placement) = placement {// <callout id="co.prefab.iflet" />
        let string_vec : Vec<char> = FORTRESS.0
            .chars().filter(|a| *a != '\r' && *a !='\n')
            .collect();// <callout id="co.prefab.filter" />
        let mut i = 0;// <callout id="co.prefab.i" />
        for ty in placement.y .. placement.y + FORTRESS.2 {// <callout id="co.prefab.for_ty" />
            for tx in placement.x .. placement.x + FORTRESS.1 {
                let idx = map_idx(tx, ty);
                let c = string_vec[i];// <callout id="co.prefab.string_access" />
                match c {// <callout id="co.prefab.match" />
                    'M' => {// <callout id="co.prefab.monster" />
                        mb.map.tiles[idx] = TileType::Floor;
                        mb.monster_spawns.push(Point::new(tx, ty));
                    }
                    '-' => mb.map.tiles[idx] = TileType::Floor,// <callout id="co.prefab.floorwall" />
                    '#' => mb.map.tiles[idx] = TileType::Wall,
                    _ => println!("No idea what to do with [{}]", c)// <callout id="co.prefab.ohdear" />
                }
                i += 1;
            }
        }
    }
    //END: place
}