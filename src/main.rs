use clap::Parser;
use std::path::PathBuf;
use word_search::run;

/// Simple program to search frequency of each unique word in example.txt file
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// file path
    #[arg(short, long,default_value_t = String::from("data.txt"))]
    file: String,

    ///Word to search for
    #[arg(short, long)]
    word: Option<String>,
}

fn main() {
    let args = Args::parse();

    let file = PathBuf::from(args.file);
    let word = args.word;

    match run(file, word) {
        Ok(_) => (),
        Err(err) => println!("Error: {}", err),
    }
}
