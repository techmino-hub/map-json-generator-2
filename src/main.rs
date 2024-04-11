extern crate json;
extern crate reqwest;

use std::process::Command;
use std::fs;
use std::path::Path;
use std::error::Error;
use std::io::{self, Write};

use json::object;
use json::object::Object;
use json::JsonValue;

use reqwest::blocking::get;

const OUTPUT_STRPATH: &str = "./json/map.json";
const MODES_URL: &str = "https://raw.githubusercontent.com/26F-Studio/Techmino/main/parts/modes.lua";
const MODES_LUA_STRPATH: &str = "./lua/modes.lua";
const TABLE_TO_JSON_STRPATH: &str = "./lua/table_to_json.lua";
const EXTRA_MODES_STRPATH: &str = "./json/extra_modes.json";

fn main() {
    check_dependencies();

    {
        let mut download_modes_lua = true;

        if Path::new(MODES_LUA_STRPATH).exists() {
            download_modes_lua = ask_for_confirmation(
                "modes.lua already exists. Would you like to redownload/update it? [Y/N]: "
            );
        }

        if download_modes_lua {
            loop {
                let result = download_modes_lua_file();
                if let Err(e) = result {
                    println!("Error downloading modes.lua: {e}");
                    if ask_for_confirmation("Would you like to retry downloading it? [Y/N]: ") {
                        continue;
                    }
                } else {
                    println!("Downloaded modes.lua successfully");
                }
                break;
            }
        }

        if !Path::new(MODES_LUA_STRPATH).exists() {
            panic!("modes.lua is missing. The program cannot continue.");
        }
    }

    let mut map: JsonValue = object!{
        modes: {},
        starting_mode: "sprint_10l",
        min_x: 0, min_y: 0,
        max_x: 0, max_y: 0,
    };

    map["modes"] = get_base_modes_json();
}

fn check_dependencies() {
    // Check for Lua
    let result = Command::new("lua")
        .arg("-v")
        .output();

    if let Err(e) = result {
        panic!("Failed to run Lua: {}\nHave you installed Lua yet? Install it here if you haven't: https://www.lua.org/download.html", e);
    }

    // Check for table_to_json.lua
    let table_to_json_path = Path::new(TABLE_TO_JSON_STRPATH);
    if !table_to_json_path.exists() {
        panic!("Could not find Lua table to JSON converter at {}", TABLE_TO_JSON_STRPATH);
    }
}

fn download_modes_lua_file() -> Result<(), Box<dyn Error>> {
    let response = get(MODES_URL)?.bytes()?;
    fs::write(Path::new(MODES_LUA_STRPATH), response)?;

    return Ok(());
}

fn get_base_modes_json() -> JsonValue {
    let json_stdout = Command::new("lua")
        .arg(TABLE_TO_JSON_STRPATH)
        .output()
        .expect("Error while converting Lua table to JSON")
        .stdout;

    let json_str = std::str::from_utf8(&json_stdout)
        .expect("Error while converting Vec<u8> to String while processing modes.lua");

    return json::parse(json_str).expect("Error while parsing base JSON");
}

fn ask_for_confirmation(question: &str) -> bool {
    loop {
        print!("{question}");
        io::stdout().flush().expect("Failed to flush stdout");

        let mut answer = String::new();

        io::stdin()
            .read_line(&mut answer)
            .expect("Failed to read line");

        let answer = answer.trim().to_lowercase().bytes().next().unwrap_or(b'x');

        match answer {
            b'y' => { return true;  }
            b'n' => { return false; }
            _ => { continue; }
        }
    }
}