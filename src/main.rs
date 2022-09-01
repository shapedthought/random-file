use fake::Fake;
use rayon::prelude::*;
use std::fs::File;
use std::io::prelude::*;
use clap::{Args, Parser, Subcommand};
// use spinners::{Spinner, Spinners};
use dialoguer::Confirm;
use num_cpus;
use indicatif::ProgressBar;

#[derive(Parser)]
#[clap(author, version)]
pub struct Cli {
    #[clap(subcommand)]
    command: Commands,

}

#[derive(Subcommand)]
enum Commands {
    Check(CheckCommand),
    Run(FileCommands),
}

#[derive(Args)]
struct CheckCommand {
    #[clap(short, long, action)]
    run: bool
}

#[derive(Args)]
struct FileCommands {
    #[clap(short, long, value_parser)]
    files: i32,
    #[clap(short, long, value_parser)]
    size: usize, 
    #[clap(short, long, value_parser)]
    threads: usize
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Check(run) => {
            if run.run {
                let num_cpu = num_cpus::get();
                println!("Total threads available: {}", num_cpu);
            } else {
                println!("Exit");
            };
        },
        Commands::Run(cli) => {
            let file_size: usize = cli.size;
            let mut total_size = (cli.files * cli.size as i32) / 1048576;
            let num_cpu = num_cpus::get();
            let mut _threads_check: usize = 0;
            if cli.threads > num_cpu {
                _threads_check = num_cpu;
                println!("Specified threads higher than available, using: {}", _threads_check);
            } else {
                _threads_check = cli.threads
            }
            let mut cal_val = String::from("MB");
            if total_size == 0 {
                total_size = cli.size as i32 / 1024;
                cal_val = String::from("KB")
            }
            println!("Total File Capacity to be created: {}{}", total_size, cal_val);
        
            if Confirm::new().with_prompt("Do you want to continue?").interact().unwrap() {
        
                rayon::ThreadPoolBuilder::new().num_threads(_threads_check).build_global().unwrap();
        
                // let mut sp = Spinner::new(Spinners::Dots9, "Creating files...".into());
                let bar = ProgressBar::new(cli.files.try_into().unwrap());
                
                (0..cli.files).into_par_iter().for_each(|x| {
                    let fake_string = file_size.fake::<String>();
                    let file_name = format!("file{}.txt", x);
                    let mut file = File::create(file_name).unwrap();
                    file.write_all(fake_string.as_bytes()).unwrap();
                    bar.inc(1)
                });
            
                // sp.stop();
        
            } else {
                println!("Quitting")
            }
        }
    }


}
