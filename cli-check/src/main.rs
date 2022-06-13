use colored::Colorize;
use rustyline;
use structopt::StructOpt;

#[cfg(windows)]
fn terminal_setup() {
    use colored;
    colored::control::set_virtual_terminal(true);
}

#[cfg(unix)]
fn terminal_setup() {}

#[derive(StructOpt)]
enum Opt {
    #[structopt(about = "run the check program with `cli-check start`")]
    Start,
}

fn main() -> Result<(), rustyline::error::ReadlineError> {
    Opt::from_args();

    terminal_setup();
    let mut rl = rustyline::Editor::<()>::new();

    println!(concat!(
        "Welcome to the simulation lab tech-check! ",
        "If you're seeing this message, you've launched the program successfully. ",
        "Now, let's make sure everything is running correctly. ",
        "First, make sure the following line contains “normal looking”, colored text ",
        "(as opposed to a bunch of gibberish symbols):",
    ));
    println!(
        "  {} {} {} {} {} {}\n",
        "resistive".red(),
        "gooey".green(),
        "yarn".yellow().dimmed(),
        "breaks".bright_blue(),
        "practice".purple().bold(),
        "competition".cyan(),
    );

    println!("Also make sure the following symbols display correctly:");
    println!("  {}\n", "ohm Ω, degree °, beta β".bright_green());

    println!("Next, respond to the following prompt:");
    let stuff = rl.readline(&format!("  {} ", "Knock knock, who’s there?".bright_blue()))?;

    println!(
        "\nFinally, exit this program by pressing {} (yes, even on Macs) at the following prompt.\n(You'll know you exited successfully if you see a message saying {}.)",
        "ctrl-C".bright_cyan().bold(),
        "Error: Interrupted".bright_red().bold()
    );

    loop {
        let who = rl.readline(&format!(
            "  {} ",
            format!("{} who?", stuff.trim().bold()).yellow()
        ))?;
        println!(
            "Ok, {}, please exit the program by pressing {}.",
            who.bold(),
            "ctrl-C".bright_cyan().bold()
        );
    }
}
