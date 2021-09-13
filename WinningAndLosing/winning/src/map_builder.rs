use crate::prelude::*;

const NUM_ROOMS: usize = 20;
//START: mapbuild
pub struct MapBuilder {
    pub map : Map,
    pub rooms : Vec<Rect>,
    pub player_start : Point,
    //START_HIGHLIGHT
    pub amulet_start : Point
    //END_HIGHLIGHT
}
//END: mapbuild

impl MapBuilder {
    //START: mapbuild_const
    pub fn new(rng: &mut RandomNumberGenerator) -> Self {

        let mut mb = MapBuilder{
            map : Map::new(),
            rooms : Vec::new(),
            player_start : Point::zero(),
            //START_HIGHLIGHT
            amulet_start : Point::zero()
            //END_HIGHLIGHT
        };
        //END: mapbuild_const
        mb.fill(TileType::Wall);
        mb.build_random_rooms(rng);
        mb.build_corridors(rng);
        mb.player_start = mb.rooms[0].center();

        // START: adijkstra
        let dijkstra_map = DijkstraMap::new(
            SCREEN_WIDTH,
            SCREEN_HEIGHT,
            &vec![mb.map.point2d_to_index(mb.player_start)],
            &mb.map,
            1024.0
        );
        //END: adijkstra
        //START: amulet_distant
        const UNREACHABLE : &f32 = &f32::MAX;// <callout id="co.winning.unreachable" />
        mb.amulet_start = mb.map.index_to_point2d// <callout id="co.winning.i2p" />
        ( 
            dijkstra_map.map
                .iter()
                .enumerate()// <callout id="co.winning.enumerate" />
                .filter(|(_,dist)| *dist < UNREACHABLE)// <callout id="co.winning.filter" />
                .max_by(|a,b| a.1.partial_cmp(b.1).unwrap())// <callout id="co.winning.max" />
                .unwrap().0// <callout id="co.winning.unwrap" />
        );
        //END: amulet_distant

        mb
    }

    fn fill(&mut self, tile : TileType) {
        self.map.tiles.iter_mut().for_each(|t| *t = tile);
    }

    fn build_random_rooms(&mut self, rng : &mut RandomNumberGenerator) {
        while self.rooms.len() < NUM_ROOMS {
            let room = Rect::with_size(
                rng.range(1, SCREEN_WIDTH - 10),
                rng.range(1, SCREEN_HEIGHT - 10),
                rng.range(2, 10),
                rng.range(2, 10),
            );
            let mut overlap = false;
            for r in self.rooms.iter() {
                if r.intersect(&room) {
                    overlap = true;
                }
            }
            if !overlap {
                room.for_each(|p| {
                    if p.x > 0 && p.x < SCREEN_WIDTH && p.y > 0 && p.y < SCREEN_HEIGHT {
                        let idx = map_idx(p.x, p.y);
                        self.map.tiles[idx] = TileType::Floor;
                    }
                });

                self.rooms.push(room)
            }
        }
    }

    fn apply_horizontal_tunnel(&mut self, x1:i32, x2:i32, y:i32) {
        use std::cmp::{min, max};
        for x in min(x1,x2) ..= max(x1,x2) {
            if let Some(idx) = self.map.try_idx(Point::new(x, y)) {
                self.map.tiles[idx as usize] = TileType::Floor;
            }
        }
    }

    fn apply_vertical_tunnel(&mut self, y1:i32, y2:i32, x:i32) {
        use std::cmp::{min, max};
        for y in min(y1,y2) ..= max(y1,y2) {
            if let Some(idx) = self.map.try_idx(Point::new(x, y)) {
                self.map.tiles[idx as usize] = TileType::Floor;
            }
        }
    }

    fn build_corridors(&mut self, rng: &mut RandomNumberGenerator) {
        let mut rooms = self.rooms.clone();
        rooms.sort_by(|a,b| a.center().x.cmp(&b.center().x));

        for (i, room) in rooms.iter().enumerate().skip(1) {
            let prev = rooms[i-1].center();
            let new = room.center();

            if rng.range(0,2) == 1 {
                self.apply_horizontal_tunnel(prev.x, new.x, prev.y);
                self.apply_vertical_tunnel(prev.y, new.y, new.x);
            } else {
                self.apply_vertical_tunnel(prev.y, new.y, prev.x);
                self.apply_horizontal_tunnel(prev.x, new.x, new.y);
            }
        }
    }
}