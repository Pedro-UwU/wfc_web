use std::collections::{HashMap, HashSet};

use crate::{
    handlers::events::{BuildMessage, TileMessage},
    models::wfc::{Dirs, Tile},
};
use socketioxide::extract::SocketRef;
use tracing::info;

// ------------- Entry function ------------- //
pub fn build(_socket: SocketRef, msg: BuildMessage) {
    let tiles = tile_msg_to_tiles(&msg.tiles);
    let edges = generate_edge_map(&tiles);
}

fn tile_msg_to_tiles(tiles: &Vec<TileMessage>) -> Vec<Tile> {
    let mut result = vec![];
    for t in tiles {
        let new_tile: Tile = t.clone().into();
        if t.can_rotate {
            let mut tile = new_tile.clone();
            for _ in 0..4 {
                tile = tile.rotate();
                result.push(tile.rotate());
            }
        } else {
            result.push(new_tile);
        }
    }
    result
}

fn generate_edge_map(tiles: &Vec<Tile>) -> HashMap<u64, HashMap<Dirs, HashSet<u32>>> {
    let mut result = HashMap::new();
    for t in tiles {
        let north_edge = result.entry(t.north).or_insert_with(|| HashMap::new());
        let dir = north_edge.entry(Dirs::North).or_insert_with(|| HashSet::new());
        dir.insert(t.id);
        let east_edge = result.entry(t.east).or_insert_with(|| HashMap::new());
        let dir = east_edge.entry(Dirs::East).or_insert_with(|| HashSet::new());
        dir.insert(t.id);
        let south_edge = result.entry(t.south).or_insert_with(|| HashMap::new());
        let dir = south_edge.entry(Dirs::South).or_insert_with(|| HashSet::new());
        dir.insert(t.id);
        let west_edge = result.entry(t.west).or_insert_with(|| HashMap::new());
        let dir = west_edge.entry(Dirs::West).or_insert_with(|| HashSet::new());
        dir.insert(t.id);
    }
    result
}
