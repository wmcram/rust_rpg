pub struct Map {
    width: usize,
    height: usize,
    tiles: Vec<Vec<Tile>>,
}

enum Occupied {
    Entity(Entity),
    None,
}

struct Tile {
    occupant: Occupied,
}

impl Tile {
    pub fn new() -> Tile {
        Tile {
            occupant: Occupied::None,
        }
    }
}

pub struct Entity {
    name: String,
    health: u32,
    attack: u32,
    defense: u32,
}

impl Entity {
    pub fn new(name: String, health: u32, attack: u32, defense: u32) -> Entity {
        Entity {
            name,
            health,
            attack,
            defense,
        }
    }
}

impl Map {
    pub fn new(width: usize, height: usize) -> Map {
        let mut tiles = Vec::new();
        for _ in 0..height {
            let mut row = Vec::new();
            for _ in 0..width {
                row.push(Tile::new());
            }
            tiles.push(row);
        }
        Map {
            width,
            height,
            tiles,
        }
    }

    pub fn get_tile(&self, x: usize, y: usize) -> &Tile {
        &self.tiles[x][y]
    }

    pub fn spawn_entity(&mut self, entity: Entity, x: usize, y: usize) -> Result<(), MapError> {
        if x >= self.width || y >= self.height {
            return Err(MapError::OutOfBounds);
        }
        match self.tiles[x][y].occupant {
            Occupied::Entity(_) => return Err(MapError::Occupied),
            Occupied::None => self.tiles[x][y].occupant = Occupied::Entity(entity),
        }
        Ok(())
    }

    pub fn render(&self) {
        for row in &self.tiles {
            for tile in row {
                match tile.occupant {
                    Occupied::Entity(_) => print!("E"),
                    Occupied::None => print!("."),
                }
            }
            println!();
        }
    }
}

#[derive(Debug)]
pub enum MapError {
    OutOfBounds,
    Occupied,
}