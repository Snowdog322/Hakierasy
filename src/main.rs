mod graphics;
mod commands;

use graphics::graphics as graphicsmod;
use commands::commands as commandsmod;


fn main() {
    graphicsmod::main();
    commandsmod::main();
}
