use clap::{Parser, Subcommand};
use fake::Fake;
use rand::Rng;
use rayon::prelude::*;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;
// use spinners::{Spinner, Spinners};
use dialoguer::Confirm;
use indicatif::ProgressBar;

#[derive(Parser)]
#[clap(author, version)]
pub struct Cli {
    /// File Path
    #[clap(short, long, value_parser)]
    path: PathBuf,

    #[clap(subcommand)]
    file_sizes: FileSizes,

    /// Confirms the creation of files without interaction
    #[clap(short, long, default_value = "false")]
    confirm: bool,
}

#[derive(Subcommand)]
enum FileSizes {
    /// static file size
    Static {
        /// Number of files to create
        #[arg(short, long, value_parser)]
        files: i32,

        /// Size of files to create in KB
        #[clap(short, long, value_parser)]
        size: usize,

        /// Number of threads to use, defaults to number of cores
        #[clap(short, long, default_value_t = num_cpus::get())]
        threads: usize,
    },
    /// random file size
    Random {
        /// Number of files to create
        #[clap(short, long, value_parser)]
        files: i32,

        /// Lower bound of file size in KB
        #[clap(long, value_parser)]
        lower: usize,

        /// Upper bound of file size in KB
        #[clap(long, value_parser)]
        higher: usize,

        /// Number of threads to use, defaults to number of cores
        #[clap(short, long, default_value_t = num_cpus::get())]
        threads: usize,
    },
}

fn check_ok(files: &i64) -> bool {
    Confirm::new()
        .with_prompt(format!("Are you sure you want to create {} files?", files))
        .interact()
        .unwrap()
}

fn calculate_total(files: &i64, cap: &i64, random: bool) {
    let total_cap = files * cap;
    let rand_string = if random {
        "Max"
    } else {
        "Total"
    };
    match total_cap {
        0..=1024 => println!("{} capacity is {} KB", rand_string, total_cap),
        1025..=1048576 => println!("{} capacity is {} MB", rand_string, total_cap / 1024),
        _ => println!("{} capacity is {} GB", rand_string, total_cap / 1048576),
    }
}

fn static_files(files: i32, size: usize, threads: usize, path: PathBuf, confirm: bool) {
    calculate_total(&files.into(), &(size as i64), false);

    rayon::ThreadPoolBuilder::new()
        .num_threads(threads)
        .build_global()
        .unwrap();

    let bar = ProgressBar::new(files.try_into().unwrap());

    let size_kb = size * 1024;

    if confirm {
        run_static(files, size_kb, path, bar);
    } else if check_ok(&files.into()) {
        run_static(files, size_kb, path, bar);
    } else {
        println!("Quitting")
    }
}

fn run_static(files: i32, size: usize, path: PathBuf, bar: ProgressBar) {
    (0..files).into_par_iter().for_each(|x| {
        let fake_string = size.fake::<String>();
        let file_name = format!("file{}.txt", x);
        let file_path = path.join(file_name);
        let mut file = File::create(file_path).unwrap();
        file.write_all(fake_string.as_bytes()).unwrap();
        bar.inc(1)
    });
}

fn random_files(
    files: i32,
    lower: usize,
    higher: usize,
    threads: usize,
    path: PathBuf,
    confirm: bool,
) {
    calculate_total(&files.into(), &(higher as i64), true);

    rayon::ThreadPoolBuilder::new()
        .num_threads(threads)
        .build_global()
        .unwrap();

    let lower_kb = lower * 1024;
    let higher_kb = higher * 1024;

    let bar = ProgressBar::new(files.try_into().unwrap());

    if confirm {
        run_random(files, lower_kb, higher_kb, path, bar);
    } else if check_ok(&files.into()) {
        run_random(files, lower_kb, higher_kb, path, bar);
    } else {
        println!("Quitting")
    }
}

fn run_random(files: i32, lower: usize, higher: usize, path: PathBuf, bar: ProgressBar) {
    (0..files).into_par_iter().for_each(|x| {
        let file_size = rand::thread_rng().gen_range(lower..higher);
        let fake_string = file_size.fake::<String>();
        let file_name = format!("file{}.txt", x);
        let file_path = path.join(file_name);
        let mut file = File::create(file_path).unwrap();
        file.write_all(fake_string.as_bytes()).unwrap();
        bar.inc(1)
    })
}

fn main() {
    let cli = Cli::parse();

    match &cli.file_sizes {
        FileSizes::Static {
            files,
            size,
            threads,
        } => static_files(*files, *size, *threads, cli.path, cli.confirm),
        FileSizes::Random {
            files,
            lower,
            higher,
            threads,
        } => random_files(*files, *lower, *higher, *threads, cli.path, cli.confirm),
    }
}
