use bevy::prelude::*;
use plugin::MyPlugin;
use setup::setup;

mod constants;
mod plugin;
mod setup;

fn say_hello() {
    println!("Hello, world!");
}


fn main() {
    
    App::new()
        .add_systems(Startup, setup)
        .add_plugins(MyPlugin)
        .run();
}
