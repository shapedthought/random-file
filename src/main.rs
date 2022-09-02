use fake::Fake;
use rand::Rng;
use rayon::prelude::*;
use std::fs::File;
use std::io::prelude::*;
use clap::Parser;
// use spinners::{Spinner, Spinners};
use num_cpus;
use indicatif::ProgressBar;
use dialoguer::Confirm;


#[derive(Parser)]
#[clap(author, version)]
pub struct Cli {
    #[clap(short, long, value_parser)]
    files: i32,

    #[clap(short, long, value_parser)]
    lower: usize,

    #[clap(short, long, value_parser)]
    higher: usize,

    #[clap(short, long, value_parser)]
    threads: usize
}


fn main() {
    let cli = Cli::parse();


    let num_cpu = num_cpus::get();
    let mut _threads_check: usize = 0;
    if cli.threads > num_cpu {
        _threads_check = num_cpu;
        println!("Specified threads higher than available, using: {}", _threads_check);
    } else {
        _threads_check = cli.threads
    }

    let max_cap = (cli.files as i64 * cli.higher as i64) / 1048576;
    println!("Max capacity is {} MB", max_cap);

    if Confirm::new().with_prompt(format!("Are you sure you want to create {} files?", cli.files)).interact().unwrap() {
        rayon::ThreadPoolBuilder::new().num_threads(_threads_check).build_global().unwrap();

        let bar = ProgressBar::new(cli.files.try_into().unwrap());
        
        (0..cli.files).into_par_iter().for_each(|x| {
            let file_size = rand::thread_rng().gen_range(cli.lower..cli.higher);
            let fake_string = file_size.fake::<String>();
            let file_name = format!("file{}.txt", x);
            let mut file = File::create(file_name).unwrap();
            file.write_all(fake_string.as_bytes()).unwrap();
            bar.inc(1)
        });
    } else {
        println!("Quitting")
    }


        // sp.stop();


}
    



