use clap::{ArgEnum, Parser};
use show_renamer::{run, Config};
use std::path::PathBuf;
use std::process;
use std::{env, fs};

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
    let mut filenames: Vec<String> = vec![String::from("init")];
    let mut lang: String = String::from("fr");
    if cli.file == None && cli.directory == None {
        eprintln!("Missing argument <FILE> or <FOLDER>");
        process::exit(1);
    } else if cli.file != None {
        let file = cli.file.unwrap();
        if file.exists() {
            println!("Renaming : {:#?}", file);
            filenames.push(
                fs::canonicalize(&file)
                    .unwrap()
                    .into_os_string()
                    .into_string()
                    .unwrap(),
            );
        } else {
            eprintln!("Error : {:#?} does not exist", file);
            process::exit(1);
        }
    } else {
        let directory = cli.directory.unwrap();
        if directory.exists() {
            let current_dir = directory.into_os_string().into_string().unwrap();
            // println!("Entering folder : {:#?}", &current_dir);
            // println!(
            //     "Entries modified in the last 24 hours in {:?}:",
            //     &current_dir
            // );

            for entry in fs::read_dir(&current_dir).unwrap() {
                let entry = entry.unwrap();
                let path = entry.path();

                let metadata = fs::metadata(&path).unwrap();
                if metadata.is_file() {
                    let subfile = path.clone();
                    // println!(
                    //     "Last modified: {:?} seconds, is read only: {:?}, size: {:?} bytes, filename: {:?}",
                    //     last_modified,
                    //     metadata.permissions().readonly(),
                    //     metadata.len(),
                    //     path.file_name().ok_or("No filename").unwrap()
                    // );
                    // println!("{:#?}", subfile);
                    filenames.push(
                        fs::canonicalize(&PathBuf::from(subfile))
                            .unwrap()
                            .into_os_string()
                            .into_string()
                            .unwrap(),
                    );
                }
            }
        } else {
            eprintln!("Error : {:#?} does not exist", directory);
            process::exit(1);
        }
    }
    match cli.lang {
        Some(Lang::Eng) => {
            lang = String::from("eng");
        }
        _ => {
            lang = String::from("fr");
        }
    }
    filenames.remove(0);
    // println!("{:#?}", filenames);
    // println!("{:#?}", lang);
    let config = Config::new(filenames, lang).unwrap();
    run(config);
}
