# Random File

This is a tool to create a load of random files for testing purposes.

## Install

- Install Rust
- Clone this repo
- Run:

```
cargo install --path .
```

## Usage

### Base Commands

```
Usage: random-file.exe --path <PATH> <COMMAND>

Commands:
  static  static file size
  random  random file size
  help    Print this message or the help of the given subcommand(s)

Options:
  -p, --path <PATH>  File Path
  -h, --help         Print help
  -V, --version      Print version
```

### Static

```
Usage: random-file.exe --path <PATH> static [OPTIONS] --files <FILES> --size <SIZE>

Options:
  -f, --files <FILES>      Number of files to create
  -s, --size <SIZE>        Size of files to create in KB
  -t, --threads <THREADS>  Number of threads to use, defaults to number of cores [default: 20]
  -h, --help               Print help
```

### Random

```
Usage: random-file.exe --path <PATH> random [OPTIONS] --files <FILES> --lower <LOWER> --higher <HIGHER>

Options:
  -f, --files <FILES>      Number of files to create
      --lower <LOWER>      Lower bound of file size in KB
      --higher <HIGHER>    Upper bound of file size in KB
  -t, --threads <THREADS>  Number of threads to use, defaults to number of cores [default: 20]
  -h, --help               Print help
```

Static creates the same size of file, Random creates a random size of file between the lower and higher bounds.

The higher and lower bounds are in kilobytes.

## Threads

The tool uses Rayon to parallelize the file creation. The default number of threads is equal to the system logical processors, but this can be changed with the `--threads` flag.

Being Rust based the tool cannot use more threads than logical processors.

## Examples

### Static

```
random-file --path C:\Users\user\Desktop\test static --files 100 --size 1024
```

Creates 100 files of 1MB each.

### Random

```
random-file --path C:\Users\user\Desktop\test random --files 100 --lower 1 --higher 1024
```

Creates 100 files of random size between 1KB and 1MB each.

### Benchmark

Using hyperfine to benchmark the tool on a 12th Gen Intel Core i7-12700K @ 3.60GHz

1000 static files of 1KB each using 20 threads:

```
Benchmark 1: random-file -p . -c static --files 1000 --size 1
  Time (mean ± σ):      1.509 s ±  0.215 s    [User: 0.000 s, System: 0.000 s]
  Range (min … max):    1.140 s …  1.768 s    10 runs
```

1000 random files of 1KB - 1MB each using 20 threads:

```
Benchmark 1: random-file -p . -c random --files 1000 --lower 1 --higher 1024
  Time (mean ± σ):      1.992 s ±  0.698 s    [User: 0.000 s, System: 0.000 s]
  Range (min … max):    1.458 s …  3.583 s    10 runs
```
