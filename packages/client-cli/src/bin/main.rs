extern crate clap;
extern crate shared;

use clap::{App, Arg};
use shared::emulators::game_boy_color::GameBoyColor;

pub fn main() {
    let matches = App::new("client-cli")
        .arg(
            Arg::with_name("CARTRIDGE")
                .help("The cartridge to be started")
                .index(1)
                .required(true),
        )
        .get_matches();
    let cartridge = matches.value_of("CARTRIDGE").unwrap(); // safe unwrap as being `required`
    let mut emulator = GameBoyColor::new();
    emulator.load(cartridge);
    emulator.boot();
}
