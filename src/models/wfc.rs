use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct BuildInfo {
    pub width: usize,
    pub height: usize,
}

#[derive(Deserialize, Debug)]
pub struct Neighbors {
    pub n: Vec<usize>,
    pub e: Vec<usize>,
    pub s: Vec<usize>,
    pub w: Vec<usize>,
}

use core::{fmt, panic};
use std::fmt::Display;

use crate::handlers::events::TileMessage;

#[derive(Debug, Clone)]
pub struct Tile {
    pub north: u64,
    pub east: u64,
    pub south: u64,
    pub west: u64,
    pub id: u32,
    pub sections: u32,
}

impl Tile {
    pub fn rotate(&self) -> Self {
        let mut current_rotation = (self.id >> 30) + 1;
        current_rotation <<= 30;
        let new_id = (self.id & 0x3FFFFFFF) | current_rotation;
        Tile {
            // Id -> 32 bits -> 2 bits of rotation and 30 for the index
            id: new_id,
            sections: self.sections,
            north: self.west,
            east: self.north,
            south: self.east,
            west: self.south,
        }
    }

    pub fn get_section_u64_from_vec(sections: &Vec<u16>) -> u64 {
        if sections.len() > 4 {
            panic!("TILE WITH MORE THAN 4 sections");
        }
        let mut result: u64 = 0;
        for val in sections.iter() {
            result <<= 16;
            result += (*val) as u64;
        }
        result
    }

    pub fn extract_sections(value: u64, sections: u32) -> Vec<u16> {
        (0..sections)
            .map(|i| ((value >> (i * 16)) & 0xFFFF) as u16)
            .collect()
    }

    pub fn get_rotation(&self) -> u32 {
        self.id >> 30
    }

    pub fn get_id_without_rotation(&self) -> u32 {
        self.id & 0x3FFFFFFF
    }
}

impl Into<Tile> for TileMessage {
    fn into(self) -> Tile {
        Tile {
            id: self.tile_id,
            sections: self.north.len() as u32,
            north: Tile::get_section_u64_from_vec(&self.north),
            east: Tile::get_section_u64_from_vec(&self.east),
            south: Tile::get_section_u64_from_vec(&self.south),
            west: Tile::get_section_u64_from_vec(&self.west),
        }
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let north_sections = Tile::extract_sections(self.north, self.sections);
        let east_sections = Tile::extract_sections(self.east, self.sections);
        let south_sections = Tile::extract_sections(self.south, self.sections);
        let west_sections = Tile::extract_sections(self.west, self.sections);

        write!(
            f,
            "ID: {}\nTile ID: {}, Rotation: {}\nNorth: {:?}\nEast:  {:?}\nSouth: {:?}\nWest:  {:?}",
            self.id,
            self.get_id_without_rotation(),
            self.get_rotation(),
            north_sections,
            east_sections,
            south_sections,
            west_sections
        )
    }
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum Dirs {
    North = 0,
    East = 1,
    South = 2,
    West = 3,
}
