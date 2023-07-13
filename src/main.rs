use bevy::prelude::*;
use bevy_pixel_buffer::prelude::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, PixelBufferPlugin))
        .add_systems(
            Startup,
            PixelBufferBuilder::new()
                .with_size(PixelBufferSize::pixel_size((16, 16))) // only set pixel_size as size will be dynamically updated
                .with_fill(Fill::window()) // set fill to the window
                .setup(),
        )
        .add_systems(Startup, setup)
        .add_systems(Update, update)
        .run();
}

#[derive(Component)]
struct Cell(UVec2);

fn setup(mut commands: Commands) {
    commands.spawn(Cell(UVec2::new(0, 0)));
}

fn update(mut pb: QueryPixelBuffer, cell_query: Query<&Cell>) {
    let mut frame = pb.frame();

    for cell in cell_query.iter() {
        let _ = frame.set(cell.0, Pixel::WHITE);
    }
}
