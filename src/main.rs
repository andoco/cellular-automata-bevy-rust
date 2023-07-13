use std::time::Duration;

use bevy::{prelude::*, time::common_conditions::on_fixed_timer, utils::HashMap};
use bevy_pixel_buffer::prelude::*;
use rand::Rng;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, PixelBufferPlugin))
        .add_systems(
            PreStartup,
            PixelBufferBuilder::new()
                .with_size(PixelBufferSize::pixel_size((16, 16))) // only set pixel_size as size will be dynamically updated
                .with_fill(Fill::window()) // set fill to the window
                .setup(),
        )
        .add_systems(Startup, setup)
        .add_systems(
            FixedUpdate,
            (update, die)
                .chain()
                .run_if(on_fixed_timer(Duration::from_secs_f32(0.1))),
        )
        .add_systems(Update, draw)
        .run();
}

#[derive(Component, Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct Cell((u32, u32));

#[derive(Component)]
struct Die;

impl Into<UVec2> for Cell {
    fn into(self) -> UVec2 {
        self.0.into()
    }
}

impl Cell {
    fn neighbours(&self, bounds: UVec2) -> Vec<Cell> {
        let x = self.0 .0 as i32;
        let y = self.0 .1 as i32;

        let all = [
            (x - 1, y - 1),
            (x, y - 1),
            (x + 1, y - 1),
            (x - 1, y),
            (x + 1, y),
            (x - 1, y + 1),
            (x, y + 1),
            (x + 1, y + 1),
        ];

        all.iter()
            .filter(|(x, y)| *x >= 0 && *x < bounds.x as i32 && *y >= 0 && *y < bounds.y as i32)
            .map(|(x, y)| Cell((*x as u32, *y as u32)))
            .collect()
    }
}

fn setup(mut commands: Commands, mut pb: QueryPixelBuffer) {
    let frame = pb.frame();
    let size = frame.size();
    let mut rng = rand::thread_rng();

    for _ in 0..50 {
        let x = rng.gen_range(0..size.x);
        let y = rng.gen_range(0..size.y);
        commands.spawn(Cell((x, y)));
    }
}

fn update(cell_query: Query<(Entity, &Cell)>, mut commands: Commands, mut pb: QueryPixelBuffer) {
    let frame = pb.frame();

    let map: HashMap<Cell, Entity> = HashMap::from_iter(cell_query.iter().map(|(e, c)| (*c, e)));

    let mut neighbor_counts: HashMap<Cell, usize> = HashMap::new();

    for cell in map.keys() {
        neighbor_counts.insert(*cell, 0);
    }

    for cell in map.keys() {
        for neighbor_cell in cell.neighbours(frame.size()) {
            let entry = neighbor_counts.entry(neighbor_cell).or_insert(0);
            *entry += 1;
        }
    }

    let mut new_map: HashMap<Cell, Entity> = HashMap::new();

    for (cell, n) in neighbor_counts {
        let live = map.get(&cell);

        if live.is_some() && (n == 2 || n == 3) {
            new_map.insert(cell, *live.unwrap());
        } else if live.is_none() && n == 3 {
            let entity = commands.spawn(cell).id();
            new_map.insert(cell, entity);
        } else if live.is_some() {
            commands.entity(*live.unwrap()).insert(Die);
        }
    }
}

fn die(
    mut commands: Commands,
    mut pb: QueryPixelBuffer,
    cell_query: Query<(Entity, &Cell), With<Die>>,
) {
    let mut frame = pb.frame();

    for (entity, cell) in cell_query.iter() {
        let _ = frame.set(*cell, Pixel::TRANSPARENT);
        commands.entity(entity).despawn_recursive();
    }
}

fn draw(mut pb: QueryPixelBuffer, cell_query: Query<&Cell>) {
    let mut frame = pb.frame();

    for cell in cell_query.iter() {
        let _ = frame.set(*cell, Pixel::WHITE);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cell_neighbors_should_return_all() {
        assert_eq!(
            vec![
                Cell((0, 0)),
                Cell((1, 0)),
                Cell((2, 0)),
                Cell((0, 1)),
                Cell((2, 1)),
                Cell((0, 2)),
                Cell((1, 2)),
                Cell((2, 2))
            ],
            Cell((1, 1)).neighbours(UVec2::new(3, 3))
        );
    }

    #[test]
    fn cell_neighbors_should_return_in_bounds() {
        assert_eq!(
            vec![Cell((1, 0)), Cell((0, 1)), Cell((1, 1)),],
            Cell((0, 0)).neighbours(UVec2::new(2, 2))
        );
    }
}
