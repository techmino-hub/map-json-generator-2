# Map.json Generator 2
This repo is a rewrite of [the first map.json generator](https://github.com/techmino-hub/map-json-generator/) in Rust.  
This repo contains the code to generate a map.json file for [Techmino Hub](https://techmino-hub.github.io/).  
The main reason I rewrote this in Rust is because fiddling around with Node.js was a pain and it just never seemed to easily work. Rust should be more reliable and cross-platform.

## Dependencies
- [`Lua`](https://www.lua.org/download.html)
- [`Cargo`](https://rustup.rs/)

## Running
After installing the dependencies, run `cargo run -r` in the root directory of this repo to generate the map.json file.

## About Extra Modes
The `/data/extra_modes.json` file exists because some of the modes in-game are not actually in the mode map.  
Reasons can include:
- It's a hidden mode (no I'm not spoiling how to access it; go find out yourself 😉)
- It's a mode which no longer exists in the latest version of the game
- It's an old version of some modes with different/unique mechanics

Note that the Y coordinate aren't the same as the Y coordinates on the normal map; they are shifted down 200 units under the bottommost "non-extra" mode.

## Output Format
The output will be generated in `./json/map.json` by default. The output will be a minified JSON containing an object.  
Here's a non-minified JSON for reference:
```json
{
    "modes": {
        "sprint_10l": {
            "name": "sprint_10l",
            "shape": 1,
            "unlock": ["sprint_20l"],
            "size": 40,
            "x": 0,
            "y": 0,
            "icon": "sprint1",
            "source": "https://github.com/26F-Studio/Techmino/tree/main/parts/modes/sprint_10l.lua"
        },
        "sprint_20l": {
            "name": "sprint_20l",
            "shape": 1,
            "unlock": [],
            "size": 50,
            "x": -200,
            "y": 200,
            "icon": "sprint1",
            "source": "https://github.com/26F-Studio/Techmino/tree/main/parts/modes/sprint_20l.lua"
        }
    },
    "starting_mode": "sprint_10l",
    "min_x": -200,
    "max_x": 0,
    "min_y": 0,
    "max_y": 200
}
```