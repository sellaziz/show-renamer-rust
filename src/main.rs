use clap::{ArgEnum, Parser};
use show_renamer::{run, Config};
use std::path::PathBuf;
use std::process;
use std::{env, fs};

mod file_search;
#[derive(Parser)]
// #[clap(author = None)]
#[clap(version = "0.1")]
#[clap(about = "Simple TV Show Renamer", long_about = None)]
struct Cli {
    /// File of TV Show To Rename
    #[clap(short, long, value_parser, value_name = "FILE")]
    file: Option<PathBuf>,

    /// Folder for TV Show To Rename
    #[clap(short, long, value_parser, value_name = "FOLDER")]
    directory: Option<PathBuf>,

    /// Language
    #[clap(arg_enum, value_parser)]
    lang: Option<Lang>,

    /// Turn debugging information on
    #[clap(long, action = clap::ArgAction::Count)]
    debug: u8,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ArgEnum)]
enum Lang {
    Fr,
    Eng,
}
fn main() {
    let cli = Cli::parse();
    let mut filenames: Vec<PathBuf> = vec![];
    let mut lang: String = String::from("fr");

    if cli.file == None && cli.directory == None {
        eprintln!("Missing argument <FILE> or <FOLDER>");
        process::exit(1);
    } else if cli.file != None {
        filenames = file_search::SrrFileSet::from_filepath(cli.file.unwrap())
            .unwrap()
            .file_set;
    } else {
        filenames = file_search::SrrFileSet::from_directory(cli.directory.unwrap())
            .unwrap()
            .file_set;
    }
    match cli.lang {
        Some(Lang::Eng) => {
            lang = String::from("eng");
        }
        _ => {
            lang = String::from("fr");
        }
    }
    // println!("file_set : {:#?}", filenames);
    // println!("{:#?}", lang);
    let config = Config::new(filenames, lang).unwrap();
    run(config);
}
