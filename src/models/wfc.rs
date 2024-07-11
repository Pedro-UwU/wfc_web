use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct BuildInfo {
    pub width: usize,
    pub height: usize 
}

#[derive(Deserialize, Debug)]
pub struct Neighbors {
    pub n: Vec<usize>,
    pub e: Vec<usize>,
    pub s: Vec<usize>,
    pub w: Vec<usize>
}

