# Simple TV Show Renamer in Rust

Simple CLI app that helps rename your TV Show files to a standard format by matching your filename to a TV Show and figuring out the episode/season to suggest the best Name. It aims to be similar to FileBot.

## Getting Started

### Executing program

You first need an API key from TMDB [account](https://www.themoviedb.org/documentation/api)

```bash
env TMDB_API_KEY=$MY_TMDB_API_KEY cargo run -- -f $YOUR_FILE
```

## License

This project is licensed under the MIT License - see the LICENSE.md file for details

## Acknowledgments

Inspiration, code snippets, etc.
* [Kodi-Regexes](https://kodi.wiki/view/Advancedsettings.xml#tvshowmatching)
* Crate [TMDB](https://gitlab.com/Cir0X/tmdb-rs) 