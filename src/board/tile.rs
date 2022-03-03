use bevy::prelude::*;

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum TileType {
    Blue,
    Red,
    Green,
}

#[derive(Component)]
pub struct Tile {
    pub tile_type: TileType,
    pub next: Vec<Entity>,
}

pub struct TileDef {
    pub position: Vec2,
    pub tile_type: TileType,
}

pub fn tiles() -> Vec<TileDef> {
    let mut vec = vec![];
    vec.push(TileDef {
        position: Vec2::new(935., 934.),
        tile_type: TileType::Blue,
    });
    vec.push(TileDef {
        position: Vec2::new(847., 900.),
        tile_type: TileType::Blue,
    });
    vec.push(TileDef {
        position: Vec2::new(770., 928.),
        tile_type: TileType::Blue,
    });
    vec.push(TileDef {
        position: Vec2::new(679., 931.),
        tile_type: TileType::Red,
    });
    vec.push(TileDef {
        position: Vec2::new(600., 909.),
        tile_type: TileType::Blue,
    });
    vec.push(TileDef {
        position: Vec2::new(537., 877.),
        tile_type: TileType::Blue,
    });
    vec.push(TileDef {
        position: Vec2::new(501., 828.),
        tile_type: TileType::Red,
    });
    vec.push(TileDef {
        position: Vec2::new(484., 778.),
        tile_type: TileType::Blue,
    });
    vec.push(TileDef {
        position: Vec2::new(465., 729.),
        tile_type: TileType::Blue,
    });
    vec.push(TileDef {
        position: Vec2::new(455., 680.),
        tile_type: TileType::Red,
    });
    vec.push(TileDef {
        position: Vec2::new(487., 637.),
        tile_type: TileType::Blue,
    });
    vec.push(TileDef {
        position: Vec2::new(531., 602.),
        tile_type: TileType::Blue,
    });
    vec.push(TileDef {
        position: Vec2::new(584., 570.),
        tile_type: TileType::Red,
    });
    vec.push(TileDef {
        position: Vec2::new(604., 524.),
        tile_type: TileType::Green,
    });
    vec.push(TileDef {
        position: Vec2::new(579., 480.),
        tile_type: TileType::Blue,
    });
    vec.push(TileDef {
        position: Vec2::new(535., 438.),
        tile_type: TileType::Red,
    });
    vec.push(TileDef {
        position: Vec2::new(523., 394.),
        tile_type: TileType::Blue,
    });
    vec.push(TileDef {
        position: Vec2::new(584., 372.),
        tile_type: TileType::Blue,
    });
    vec.push(TileDef {
        position: Vec2::new(655., 370.),
        tile_type: TileType::Red,
    });
    vec.push(TileDef {
        position: Vec2::new(724., 367.),
        tile_type: TileType::Blue,
    });
    vec.push(TileDef {
        position: Vec2::new(786., 379.),
        tile_type: TileType::Blue,
    });
    vec.push(TileDef {
        position: Vec2::new(846., 402.),
        tile_type: TileType::Red,
    });
    vec.push(TileDef {
        position: Vec2::new(891., 437.),
        tile_type: TileType::Blue,
    });
    vec.push(TileDef {
        position: Vec2::new(922., 484.),
        tile_type: TileType::Blue,
    });
    vec.push(TileDef {
        position: Vec2::new(937., 531.),
        tile_type: TileType::Red,
    });
    vec.push(TileDef {
        position: Vec2::new(946., 584.),
        tile_type: TileType::Blue,
    });
    vec.push(TileDef {
        position: Vec2::new(938., 635.),
        tile_type: TileType::Green,
    });
    vec.push(TileDef {
        position: Vec2::new(931., 691.),
        tile_type: TileType::Red,
    });
    vec.push(TileDef {
        position: Vec2::new(915., 746.),
        tile_type: TileType::Blue,
    });
    vec.push(TileDef {
        position: Vec2::new(900., 801.),
        tile_type: TileType::Blue,
    });
    vec.push(TileDef {
        position: Vec2::new(879., 849.),
        tile_type: TileType::Red,
    });
    for tile_def in vec.iter_mut() {
        tile_def.position.x = tile_def.position.x - 740.5;
        tile_def.position.y = -tile_def.position.y + 690.5;
    }
    vec
}
