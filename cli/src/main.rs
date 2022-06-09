#![allow(confusable_idents, uncommon_codepoints)]

mod part;
mod util;

use rustyline;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "collision-black-box",
    about = "U.S. Physics Team Training Camp collision black box simulator"
)]
enum Opt {
    #[structopt(name = "wall", about = "collision with wall")]
    Wall,
    #[structopt(name = "disk", about = "collision with unknown disk")]
    Disk,
}

#[cfg(windows)]
fn terminal_setup() {
    use colored;
    colored::control::set_virtual_terminal(true);
}

#[cfg(unix)]
fn terminal_setup() {}

fn main() -> Result<(), rustyline::error::ReadlineError> {
    let opt = Opt::from_args();

    terminal_setup();

    match opt {
        Opt::Wall => part::wall::run(),
        Opt::Disk => part::disk::run(),
    }
}
