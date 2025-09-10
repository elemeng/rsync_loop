use clap::Parser;
use std::io::{self, Write};
use std::process::{Command, Stdio};
use std::thread;
use std::time::Duration;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about=None)]
struct Args {
    /// SSH password
    #[arg(short, long)]
    password: String,

    /// How many times to run
    #[arg(short, long, default_value_t = 20)]
    times: usize,

    /// Seconds to wait between rounds
    #[arg(short, long, default_value_t = 300)]
    wait: u64,

    /// All arguments passed to rsync
    #[arg(trailing_var_arg = true, required = true)]
    rsync_args: Vec<String>,
}

fn countdown(seconds: u64) {
    for rem in (0..seconds).rev() {
        let mins = rem / 60;
        let secs = rem % 60;
        print!("\rNext sync in {:02}:{:02}", mins, secs);
        io::stdout().flush().unwrap();
        thread::sleep(Duration::from_secs(1));
    }
    println!();
}

fn run_sync(password: &str, args: &[String]) -> bool {
    let mut cmd = Command::new("sshpass")
        .arg("-p")
        .arg(password)
        .arg("rsync")
        .args(args)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .expect("failed to spawn sshpass/rsync");
    cmd.wait().unwrap().success()
}

fn main() {
    let args = Args::parse();
    let mut ok = 0;
    let mut fail = 0;

    // 优雅 Ctrl-C
    ctrlc::set_handler(|| {
        println!("\nAborted by user");
        std::process::exit(0);
    })
    .unwrap();

    for i in 1..=args.times {
        println!("\n===== Round {}/{} =====", i, args.times);
        if run_sync(&args.password, &args.rsync_args) {
            ok += 1;
            println!("Round {} succeeded.", i);
        } else {
            fail += 1;
            eprintln!("Round {} FAILED.", i);
        }

        if i != args.times {
            countdown(args.wait);
        }
    }

    println!("\nFinished: {} success, {} failure(s)", ok, fail);
}
