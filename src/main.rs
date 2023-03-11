use humantime::{format_duration, parse_duration};
use self_update::cargo_crate_version;
use std::thread;
use std::time::Duration;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(
    name = "xtimer",
    about = "A simple timer utility",
    author = "Developer X"
)]
struct Opt {
    /// Debug mode
    #[structopt(short)]
    debug: bool,
    /// Don't show messages
    #[structopt(short, long)]
    quiet: bool,
    #[structopt(subcommand)]
    command: Command,
}

#[derive(StructOpt, Debug)]
enum Command {
    /// Update the application
    Update,
    /// Set timer for a specified time
    Set {
        /// Amount of time
        time: String,
        /// Show time remaining
        #[structopt(short, long)]
        show_time_remaining: bool,
    },
}

fn main() -> Result<(), anyhow::Error> {
    let opt = Opt::from_args();
    if opt.debug {
        println!("{opt:#?}");
    }
    match opt.command {
        Command::Update => {
            let target = self_update::get_target();
            let status = self_update::backends::github::Update::configure()
                .repo_owner("developerx-official")
                .repo_name("xtimer")
                .target(target)
                .bin_name("xtimer")
                .show_download_progress(true)
                .current_version(cargo_crate_version!())
                .build()?
                .update()?;
            println!("Update status: `{}`!", status.version());
            thread::sleep(Duration::from_secs(2));
            return Ok(());
        }
        Command::Set {
            time,
            show_time_remaining,
        } => {
            let duration = parse_duration(time.as_str())?;
            if !show_time_remaining || opt.quiet {
                thread::sleep(duration);
            } else {
                let mut seconds = duration.as_secs_f32();
                while seconds > 0_f32 {
                    let time_step = seconds.min(1_f32);
                    thread::sleep(Duration::from_secs_f32(time_step));
                    seconds -= time_step;
                    let remaining = Duration::from_secs_f32(seconds);
                    println!("Time remaining: {}", format_duration(remaining));
                }
            }
            if !opt.quiet {
                println!("Time elapsed.");
            }
        }
    }
    Ok(())
}
