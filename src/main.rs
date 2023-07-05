use macroquad::prelude::*;

use macroquad_tiled as tiled;

#[macroquad::main("Fishgame")]
async fn main() {
    let tileset = load_texture("src/assets/backgrounds/grass_hill.png").await.unwrap();

    let tiled_map_json = load_string("src/assets/maps/test-level.json").await.unwrap();

    
    let tiled_map = tiled::load_map(
        &tiled_map_json,
        &[("grass_hill.png", tileset)],
        &[],
    )
    .unwrap();

    loop {
        clear_background(BLACK);
        tiled_map.draw_tiles(
            // The name of the layer in assets/map.json
            "Tile Layer 1",
            Rect::new(0.0, 0.0, screen_width(), screen_height()),
            None,
        );
        next_frame().await;
    }
}