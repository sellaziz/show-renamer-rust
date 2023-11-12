//! This library allows to rename file names when specifying the folder 
//! or the file path and matches the names with a TV show using TheMovieDB
//! and suggest a new standard name
mod file_search;
mod query;

use regex::Regex;
use std::env;
use std::error::Error;
use std::io;
use std::num::ParseIntError;
use std::path::{Path, PathBuf};
use std::{fs, vec};
extern crate tmdb;
use tmdb::themoviedb::*;
// use::show_renamer::{clean_name,get_info};
use ::std::collections::HashMap;

pub struct Config {
    pub filenames: Vec<PathBuf>,
    pub api_key: String,
    pub lang: String,
}

impl Config {
    pub fn new(filenames: Vec<PathBuf>, lang: String) -> Result<Config, &'static str> {
        let filenames = filenames.clone();
        let lang = lang.clone();

        let api_key = env!("TMDB_API_KEY").to_string();

        Ok(Config {
            filenames,
            api_key,
            lang,
        })
    }
}


/// Take a list of paths, suggest a new file name and ask to proceed
pub fn run(args: Config) -> Result<(), Box<dyn Error>> {
    let file_set = args.filenames;
    let cp_file_set = file_set.clone();
    let mut cleaned_set: Vec<String> = vec![];
    let mut extension_set: Vec<String> = vec![];
    let mut dir_set: Vec<PathBuf> = vec![];
    let mut info_set: Vec<[u32; 2]> = vec![];
    let mut unique_queries = HashMap::new();
    let mut counter: usize = 0;

    // Cluster cleaned name to minimize queries
    for file in file_set {
        let file_path = &file;
        let parent_dir = file_path.parent().unwrap();
        dir_set.push(PathBuf::from(parent_dir));
        let file_name = file_path
            .file_name()
            .unwrap()
            .to_os_string()
            .into_string()
            .unwrap();
        let cleaned = clean_name(&file_name);
        cleaned_set.push(cleaned.to_string());
        let info: [u32; 2] = match get_info(&file_name) {
            Some(value) => value,
            _ => {
                println!("For file {} type ep and season", file_name);
                ask_for_season().unwrap()
            }
        };
        info_set.push(info);
        let queries = unique_queries.entry(cleaned).or_insert(vec![]);
        queries.push(info);
        extension_set.push(
            Path::new(&file_name)
                .extension()
                .unwrap()
                .to_os_string()
                .into_string()
                .unwrap(),
        );
        counter = counter + 1;
    }
    // println!("{:?}", dir_set);
    // println!("{:?}", cleaned_set);
    // println!("{:?}", unique_queries);
    // println!("{:?}", info_set);
    // println!("{:?}", extension_set);

    // Using Config to set api_key doesn't work, I didn't find the way to set it like that
    /* let tmdb = TMDb { api_key: &args.api_key, language: &args.lang };
    let s: String = args.lang.to_owned();
    let lang: &str = &s[..];  // take a full slice of the string
    let s: String = args.api_key.to_owned();
    let api_key: &str = &s[..];  // take a full slice of the string
    let tmdb = TMDb { api_key: api_key, language: lang};
    */
    let tmdb = TMDb {
        api_key: env!("TMDB_API_KEY"),
        language: "fr",
    };

    let mut new_name: Vec<String> = vec![];
    let mut clean_show_name: Vec<String> = vec![];
    // Implementation to minimize queries, but mess up the order
    // and it doesn't allow to figure out which new name correspond to the old one
    /* for (show, s_e_query) in &unique_queries {
        println!("{} ", show);
        for s_e in s_e_query{
            let page = tmdb.search()
            .title(show)
            .execute_tv()
            .unwrap();
            let season = s_e[0] as u16;
            let episode = s_e[1] as usize;
            println!("{} : {:#?}", season, episode);

            let shows = page.results;
            let show = shows[0].fetch(&tmdb).unwrap();
            let season = show.fetch_season(&tmdb, season).unwrap();
            let episode = &season.episodes[episode];

            println!("Episodes: {:#?}", episode.name);
            new_name.push(episode.name.to_string());
            clean_show_name.push(show.name.to_string());
        }
    } */
    for i in 0..counter {
        let show = &cleaned_set[i];
        let s_e = info_set[i];
        println!("{} ", show);
        let page = tmdb.search().title(&show).execute_tv().unwrap();
        let season = s_e[0] as u16;
        let episode = s_e[1] as usize;
        println!("{} : {:#?}", season, episode);

        let shows = page.results;
        let show = shows[0].fetch(&tmdb).unwrap();
        let season = show.fetch_season(&tmdb, season).unwrap();
        let episode = &season.episodes[episode];

        // println!("Episodes: {:#?}", episode.name);
        new_name.push(episode.name.to_string());
        clean_show_name.push(show.name.to_string());
    }
    // println!("{:?}", new_name);
    // println!("{:?}", clean_show_name);

    let mut full_names: Vec<String> = vec!["".to_string()];
    println!(r"+{:-^123}+", "+");
    println!(r"|{:<60} | {:<60}|", "Original Name", "New Name");
    println!(r"+{:-^123}+", "+");
    for i in 0..counter {
        // Create the new name of the file and remove invalid char
        let mut full_name = format!(
            "{} - S{:02}E{:02} - {}.{}",
            clean_show_name[i],
            info_set[i][0],
            info_set[i][1],
            new_name[i],
            extension_set[i],
        );
        let illegal_char = [r"<", r"|", r">",  r":", r"/",  r"\",  r"|",  r"?",  r"*",  r"(",  r")"];
        for to_rm in illegal_char {
            full_name = full_name.replace(to_rm, "");
        }
        // println!("{}", full_name);
        full_name = format!(
            "{:?}/{:?}",
            dir_set[i],
            full_name
        );
        // println!(r"{:<50} | {:<50}", &cp_file_set[i], &full_name);
        let og_file_path = PathBuf::from(&cp_file_set[i]);
        let og_file_name = og_file_path
            .file_name()
            .unwrap()
            .to_os_string()
            .into_string()
            .unwrap();
        let file_path = PathBuf::from(&full_name);
        let file_name = file_path
            .file_name()
            .unwrap()
            .to_os_string()
            .into_string()
            .unwrap();
        println!(r"|{:<60} | {:<60}|", &og_file_name, &file_name);
        full_names.push(full_name);
    }
    full_names.remove(0);
    println!(r"+{:-^123}+", "+");
    // println!("{:?}", full_names);
    println!("Rename ? [y/n]");

    let mut answer = String::new();

    io::stdin()
        .read_line(&mut answer)
        .expect("Échec de la lecture de l'entrée utilisateur");

    let answer = answer.trim();
    if answer == "y" {
        for i in 0..counter {
            fs::rename(&cp_file_set[i], &full_names[i])?;
        }
    } else {
        println!("quit the program");
    }
    Ok(())
}


/// Cleans file basename and remove unecessary info (episode and season) for query
///
/// Given a file basename, it uses a set of regexes to remove unecessary informations.
/// Like file encoding, extension, etc. and extracts only the tv show name
///
/// # Examples
///
/// ```
/// use show_renamer::clean_name;
///
/// assert_eq!(clean_name(&"Breaking Bad x264 season 1 ep 4".to_string()), "breaking bad");
/// ```
// pub fn clean_name<'a>(text: &'a str) -> String { // faster implementation when using &str ?
pub fn clean_name(text: &String) -> String {
    /* lazy static implementation, but not working
    lazy_static! {
    s&tatic ref clean_has: HashMap<&'static str, &'static str> = HashMap::from([
        ("extension",   r"\.[^.]+$"),
        ("brackets",    r"\[[^]]*\]"),
        ("parenthesis", r"\([^)]*\)"),
        ("web",         r"www\.[^.]*\."),
        ("separators",  r"(\+|\.|\(|\)|\_)"),
        ("encoding",    r"[\w\.]{0,2}26\d"),  // (x264, x265, h264, h265,...)
        ("encoding2",   r"\d{1,2}.?bit[s]?"),  // 8-bits, 10-bit, 10-bits
        ("encoding3",   r"nvenc|hevc|bluray|(bd|hd|dvd|web)rip|xvid|hdlight"),
        ("lang",        r"\W(VOST\D{1,2}|VF\w{0,3})\W"),
        ("lang2",       r"\W(TRUE)?FRENCH|ENG\W"),
        ("subs",        r"\W(multi\w{0,3}|sub\w{0,3})\W"),
        ("qual",        r"(480|720|1080)\D"),
        ("audio_enc",   r"ac3|aac"),
        ("season_ep",   r"[Ss]([0-9]+)[][ ._-]*[Ee]([0-9]+)([^\\/]*)"),
        ("season_full", r"[Ss]eason\s*\d+"),
        ("ep",          r"\W\-?\s?\d{1,3}\W"),
        ("ep_short",    r"Ep\s*\d+"),
        ("episode",     r"episode[s]?\W\d{1,3}\W|\WEp\W")
    ]);
    static ref clean_set: regex::RegexSet = RegexSet::new(&[
        r"\.[^.]+$",                                                 // extension
        r"\[[^]]*\]",                                                // brackets
        r"\([^)]*\)",                                                // parenthesis
        r"www\.[^.]*\.",                                             // web
        r"(\+|\.|\(|\)|\_)",                                         // separators
        r"[\w\.]{0,2}26\d",  // (x264, x265, h264, h265,...)         // encoding
        r"\d{1,2}.?bit[s]?",  // 8-bits, 10-bit, 10-bits             // encoding2
        r"nvenc|hevc|bluray|(bd|hd|dvd|web)rip|xvid|hdlight",        // encoding3
        r"\W(VOST\D{1,2}|VF\w{0,3})\W",                              // lang
        r"\W(TRUE)?FRENCH|ENG\W",                                    // lang2
        r"\W(multi\w{0,3}|sub\w{0,3})\W",                            // subs
        r"(480|720|1080)\D",                                         // qual
        r"ac3|aac",                                                  // audio_enc
        r"[Ss]([0-9]+)[][ ._-]*[Ee]([0-9]+)([^\\/]*)",               // season_ep
        r"[Ss]eason\s*\d+",                                          // season_full
        r"\W\-?\s?\d{1,3}\W",                                        // ep
        r"Ep\s*\d+",                                                 // ep_short
        r"episode[s]?\W\d{1,3}\W|\WEp\W"                             // episode
    ]).unwrap();
    static ref clean_set: Vec<str> = vec![
    */
    let clean_set = vec![
        r"\.[^.]+$",                                          // extension
        r"\[[^]]*\]",                                         // brackets
        r"\([^)]*\)",                                         // parenthesis
        r"www\.[^.]*\.[^\s]+",                                // web
        r"(\+|\.|\(|\)|_)",                                   // separators
        r"[\w\.]{0,2}26\d",  // (x264, x265, h264, h265,...)         // encoding
        r"\d{1,2}.?bit[s]?", // 8-bits, 10-bit, 10-bits             // encoding2
        r"nvenc|hevc|bluray|(bd|hd|dvd|web)rip|xvid|hdlight", // encoding3
        r"\W(vost(\D{1,2})?|vf\w{0,3})\W?", // lang
        r"\W(true)?french|eng\W?", // lang2
        r"\W(multi\w{0,3}|sub\w{0,3})\W", // subs
        r"(480|720|1080)\D", // qual
        r"ac3|aac",          // audio_enc
        r"[Ss]([0-9]+)[ ._-]*[Ee]([0-9]+)([^\\/]*)", // season_ep
        r"[Ss]eason\s*\d+",  // season_full
        r"\W\-?\s?\d{1,3}\W", // ep
        r"\Wep?\s*\d*",         // ep_short
        r"episode[s]?(\W?\d{1,3}\W?)?", // episode
        r"\W\d+",            // nb alone
    ];
    let clean_reg: Vec<Regex> = clean_set
        .into_iter()
        .map(|x| Regex::new(x).unwrap())
        .collect();
    // println!("{:#?}", clean_reg);
    let replace_set = vec![":", "\n", "-", "_"];
    // }
    let mut temp_txt = String::from(text.clone().to_lowercase());
    for regex in clean_reg {
        // print output if needed
        // let cap_iter = regex.captures_iter(temp_txt.as_str()).count();
        // println!("{:?}", cap_iter);
        // // println!("{:?}", temp_txt);
        // if cap_iter>0 {
        //     for cap in regex.captures_iter(temp_txt.as_str()) {
        //         println!("capture : {}", &cap[0]);
        //     }
        // }
        temp_txt = regex.replace_all(temp_txt.as_str(), "").to_owned().to_string();
    }
    for to_rm in replace_set {
        temp_txt = temp_txt.replace(to_rm, "");
    }
    temp_txt.to_lowercase().trim().to_string()
    // text
}

fn _clean_vost<'a>(text: &'a str) -> String {
    let re = Regex::new(r"\W?(VOST\D{1,2}|VF\w{0,3})").unwrap();
    re.replace_all(&text, "")
        .to_owned()
        .to_string()
        .to_lowercase()
}


/// Extract season and episode from file basename
///
/// Extract season and episode from file basename
///
/// # Examples
///
/// ```
/// use show_renamer::get_info;
///
/// assert_eq!(get_info(&"Breaking Bad x264 season 1 ep 4".to_string()).unwrap(), [1,4]);
/// ```
pub fn get_info(text: &String) -> Option<[u32; 2]> {
    let info_set = vec![
        r"[Ss]([0-9]+)[ ._-]*[Ee]([0-9]+)",    // S03E04
        r"s([eai]+son\W?)?(\d{1,2})",          // Season 1
        r"[\-\s]?\W(\d{1,3})[\W\.]?",          // - 13
        r"(episode[s]?\W|\Wep\W|\W)(\d{1,3})", // Episode 1, Ep 1
    ];
    let info_reg: Vec<Regex> = info_set
        .into_iter()
        .map(|x| Regex::new(x).unwrap())
        .collect();
    let mut temp_txt = String::from(text.clone().to_lowercase());
    if info_reg[0].is_match(&temp_txt) {
        // println!("First match");
        let caps = info_reg[0].captures(&temp_txt).unwrap();
        let season = caps.get(1).map_or("", |m| m.as_str()).parse().unwrap();
        let episode = caps.get(2).map_or("", |m| m.as_str()).parse().unwrap();
        // println!("{} {}", season, episode);
        return Some([season, episode]);
    } else if info_reg[1].is_match(&temp_txt) {
        let caps = info_reg[1].captures(&temp_txt).unwrap();
        let season: u32 = caps.get(2).map_or("", |m| m.as_str()).parse().unwrap();
        temp_txt = info_reg[1]
            .replace(temp_txt.as_str(), "")
            .to_owned()
            .to_string();
        // println!("2nd match");
        // println!("{}", season);
        if info_reg[2].is_match(&temp_txt) {
            let caps = info_reg[2].captures(&temp_txt).unwrap();
            let episode: u32 = caps.get(1).map_or("", |m| m.as_str()).parse().unwrap();
            temp_txt = info_reg[2]
                .replace(temp_txt.as_str(), "")
                .to_owned()
                .to_string();
            return Some([season, episode]);
        } else if info_reg[3].is_match(&temp_txt) {
            let caps = info_reg[3].captures(&temp_txt).unwrap();
            let episode: u32 = caps.get(0).map_or("", |m| m.as_str()).parse().unwrap();
            temp_txt = info_reg[3]
                .replace(temp_txt.as_str(), "")
                .to_owned()
                .to_string();
            return Some([season, episode]);
        } else {
            return Some([season, 0]);
        }
    } else if info_reg[2].is_match(&temp_txt) || info_reg[3].is_match(&temp_txt) {
        // println!("3rd match");
        if info_reg[2].is_match(&temp_txt) {
            // println!("1st reg");
            let caps = info_reg[2].captures(&temp_txt).unwrap();
            let episode: u32 = caps.get(1).map_or("", |m| m.as_str()).parse().unwrap();
            // println!("{}", episode);
            temp_txt = info_reg[2]
                .replace(temp_txt.as_str(), "")
                .to_owned()
                .to_string();
            return Some([1, episode]);
        } else if info_reg[3].is_match(&temp_txt) {
            // println!("2nd reg");
            let caps = info_reg[3].captures(&temp_txt).unwrap();
            let episode: u32 = caps.get(1).map_or("", |m| m.as_str()).parse().unwrap();
            // println!("{}", episode);
            temp_txt = info_reg[3]
                .replace(temp_txt.as_str(), "")
                .to_owned()
                .to_string();
            return Some([1, episode]);
        } else {
            return None;
        }
    } else {
        return None;
    }
}

/// When get_info doesn't work, ask for user input
fn ask_for_season() -> Result<[u32; 2], ParseIntError> {
    println!("Season : ");
    let mut season = String::new();

    io::stdin()
        .read_line(&mut season)
        .expect("Échec de la lecture de l'entrée utilisateur");

    let season: u32 = match season.trim().parse() {
        Ok(nombre) => nombre,
        Err(err) => return Err(err),
    };

    println!("Episode : ");
    let mut episode = String::new();

    io::stdin()
        .read_line(&mut episode)
        .expect("Échec de la lecture de l'entrée utilisateur");

    let episode: u32 = match episode.trim().parse() {
        Ok(nombre) => nombre,
        Err(err) => return Err(err),
    };
    return Ok([season, episode]);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn cleans_vost() {
        let result = _clean_vost("Hello VOSTFR.mp4");
        assert_eq!(result, "Hello.mp4".to_lowercase());
    }

    #[test]
    // #[ignore]
    // Test if clean_name works well by generating file basename with stuff before and after show name
    fn cleans_well() {
        let webs = vec![
            "www.subs.net",
            "www.hey.net",
            "www.f0oB@rs.com",
            "www.eggs_and&_spams.net",
        ];
        let encodings = vec!["x264", "x265", "h264", "h265", "H264", "H265"];
        let encodings2 = vec!["8-bits", "10-bits"];
        let encodings3 = vec!["hevc", "nvenc", "BLURAY", "BDrip"];
        let audioencodings = vec!["ac3", "aac"];
        let subs = vec!["VOST", "VOSTFR", "VF", "ENG", "FRENCH"];
        let qualities = vec!["480p", "720p", "1080p"];
        let webs = webs.iter().map(|x| x.to_string()).collect::<Vec<String>>();
        let encodings = encodings
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>();
        let encodings2 = encodings2
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>();
        let encodings3 = encodings3
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>();
        let audioencodings = audioencodings
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>();
        let subs = subs.iter().map(|x| x.to_string()).collect::<Vec<String>>();
        let qualities = qualities
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>();
        let sets = vec![
            webs,
            encodings,
            encodings2,
            encodings3,
            audioencodings,
            subs,
            qualities,
        ];
        let base_example = "Foo";
        for my_set in sets {
            for word in my_set {
                // println!("Testing : {}", &[base_example," ",word.to_owned().as_str(),".mp4"].concat());
                assert_eq!(
                    clean_name(&[base_example, " ", word.to_owned().as_str(), ".mp4"].concat()),
                    base_example.to_lowercase()
                );
                // assert_eq!(clean_name(word+" "+base_example+".mp4"), base_example);
            }
        }
    }

    #[test]
    // #[ignore]
    // Test if get_info works well by generating file basename with info before and after show name
    fn get_ep_and_season() {
        let s_and_ep = vec!["S01E02", "S01E10", "S10E02", "S1E02", "S10E12"];
        let gt_s_and_ep = vec![[1, 2], [1, 10], [10, 2], [1, 2], [10, 12]];
        let ep = vec!["- 10", "- 1"];
        let gt_ep = vec![[1, 10], [1, 1]];
        let s_new_and_ep = vec!["(Season 5) - 1", "(Season 2) - 10"];
        let gt_s_new_and_ep = vec![[5, 1], [2, 10]];
        let episode = vec!["Episode 10", "Ep 10", "Episode 2", "Episode 10"];
        let gt_episode = vec![[1, 10], [1, 10], [1, 2], [1, 10]];
        let sets = vec![s_and_ep, ep, s_new_and_ep, episode];
        let gt_sets = vec![gt_s_and_ep, gt_ep, gt_s_new_and_ep, gt_episode];
        let base_example = "Foo";
        for i in 0..sets.len() {
            for j in 0..sets[i].len() {
                println!(
                    "Testing : {}",
                    &[base_example, " ", sets[i][j], ".mp4"].concat()
                );

                assert_eq!(
                    get_info(&[base_example, " ", sets[i][j], ".mp4"].concat()).unwrap(),
                    gt_sets[i][j]
                );
                assert_eq!(
                    get_info(&[sets[i][j], " ", base_example, ".mp4"].concat()).unwrap(),
                    gt_sets[i][j]
                );
                // assert_eq!(clean_name(word+" "+base_example+".mp4"), base_example);
            }
        }
    }
}
