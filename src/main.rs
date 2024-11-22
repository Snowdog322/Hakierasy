mod graphics;
mod commands;

use bevy::prelude::*;
use graphics::graphics as graphicsmod;
use commands::commands as commandsmod;


fn main() {
    commandsmod::main();
    App::new()
    .add_systems(Startup,graphicsmod::main)
    .run();
}
