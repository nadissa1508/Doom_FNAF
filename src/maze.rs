// maze.rs
use std::fs::File;
use std::io::{BufRead, BufReader, Result};

pub type Maze = Vec<Vec<char>>;

/// Carga un laberinto desde un archivo de texto
pub fn load_maze(filename: &str) -> Result<Maze> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let maze: Maze = reader
        .lines()
        .filter_map(|line| line.ok()) // ignora líneas con errores
        .map(|line| line.chars().collect())
        .collect();

    Ok(maze)
}
