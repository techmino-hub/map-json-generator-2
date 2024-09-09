use crate::OUTPUT_STRPATH;

use std::fs;
use json;

/// Performs checks and outputs a list of warnings.
/// If a warning starts with [!?!], it is a fatal error and further checking cannot be done.
/// If a warning starts with [!!], it is a critical error which may mean that the JSON file is malformed.
/// If a warning starts with [!], it usually means incorrect or inconsistent data was detected, but the JSON file is still technically valid.
pub fn perform_checks() -> Vec<String> {
    let mut warnings: Vec<String> = Vec::new();

    let json_str = fs::read_to_string(OUTPUT_STRPATH);

    if let Err(e) = json_str {
        warnings.push(format!("[!?!] Failed to read JSON file: {}", e));
        return warnings;
    }

    let json_str = json_str.unwrap();

    let json = json::parse(&json_str);

    if let Err(e) = json {
        warnings.push(format!("[!?!] Failed to parse JSON: {}", e));
        return warnings;
    }

    let json = json.unwrap();

    if !json.is_object() {
        warnings.push("[!?!] JSON is not an object".to_string());
        return warnings;
    }

    let expected_root_keys = ["modes", "starting_mode", "min_x", "min_y", "max_x", "max_y"];

    for key in expected_root_keys.iter() {
        if !json.has_key(key) {
            warnings.push(format!("[!!] JSON is missing key: {}", key));
        }
    }

    let modes = &json["modes"];

    if !modes.is_object() {
        warnings.push("[!!] modes is not an object".to_string());
        return warnings;
    } else {
        let mode_warnings = check_modes(modes);

        for warning in mode_warnings.iter() {
            warnings.push(warning.to_string());
        }

        let boundary_warnings = check_boundaries(&json, mode_warnings.is_empty());

        for warning in boundary_warnings.iter() {
            warnings.push(warning.to_string());
        }
    }

    return warnings;
}

fn check_modes(modes: &json::JsonValue) -> Vec<String> {
    let mut warnings: Vec<String> = Vec::new();

    for (key, value) in modes.entries() {
        if !value.is_object() {
            warnings.push(format!("[!!] mode {} is not an object", key));
            continue;
        } 

        let expected_mode_keys = ["name", "x", "y", "size", "shape", "icon", "unlock", "source"];

        for mode_key in expected_mode_keys.iter() {
            if !value.has_key(mode_key) {
                warnings.push(format!("[!] mode {} is missing key: {}", key, mode_key));
            }
        }

        if value.has_key("shape") {
            let shape = &value["shape"];

            if !shape.is_number() {
                warnings.push(format!("[!] mode {} shape is not a number", key));
            } else {
                let shape = shape.as_i32();

                if let Some(shape) = shape {
                    if shape < 0 || shape > 3 {
                        warnings.push(format!("[!] mode {} shape is out of range", key));
                    }
                } else {
                    warnings.push(format!("[!] mode {} shape is not a valid integer", key));
                }
            }
        }

        if value.has_key("icon") {
            let icon = &value["icon"];

            if !icon.is_string() {
                warnings.push(format!("[!] mode {} icon is not a string", key));
            }
        }

        if value.has_key("x") {
            let x = &value["x"];

            if !x.is_number() {
                warnings.push(format!("[!] mode {} x is not a number", key));
            }
        }

        if value.has_key("y") {
            let y = &value["y"];

            if !y.is_number() {
                warnings.push(format!("[!] mode {} y is not a number", key));
            }
        }

        if value.has_key("size") {
            let size = &value["size"];

            if !size.is_number() {
                warnings.push(format!("[!] mode {} size is not a number", key));
            }
        }

        if value.has_key("unlock") {
            let unlock = &value["unlock"];

            if !unlock.is_array() {
                warnings.push(format!("[!] mode {} unlock is not an array", key));
            } else {
                for (index, unlock) in unlock.members().enumerate() {
                    if !unlock.is_string() {
                        warnings.push(format!("[!] mode {} unlock #{} is not a string", key, index));
                    }

                    let referenced_mode = unlock.as_str().unwrap();

                    if !modes.has_key(referenced_mode) {
                        warnings.push(format!("[!] mode {} references non-existent mode {}", key, referenced_mode));
                    }
                }
            }
        }

        if value.has_key("source") {
            let source = &value["source"];

            if !source.is_string() {
                warnings.push(format!("[!] mode {} source is not a string", key));
            }
        }
    }

    return warnings;
}

fn check_boundaries(root: &json::JsonValue, check_modes: bool) -> Vec<String> {
    let mut warnings: Vec<String> = Vec::new();

    let mut return_soon = false;

    let keys_to_check = ["min_x", "max_x", "min_y", "max_y"];

    for key in keys_to_check.iter() {
        if !root.has_key(key) {
            warnings.push(format!("[!!] JSON is missing key: {}", key));
            return_soon = true;
            continue;
        }

        if !root[*key].is_number() {
            warnings.push(format!("[!] JSON key {} is not of type number", key));
            return_soon = true;
            continue;
        }

        let value = root[*key].as_f64().unwrap();

        if value.is_nan() {
            warnings.push(format!("[!] JSON key {} is NaN", key));
            return_soon = true;
            continue;
        }
    }

    if return_soon {
        return warnings;
    }

    let min_x = root["min_x"].as_f64().unwrap();
    let max_x = root["max_x"].as_f64().unwrap();
    let min_y = root["min_y"].as_f64().unwrap();
    let max_y = root["max_y"].as_f64().unwrap();

    if min_x > max_x {
        warnings.push("[!] min_x is greater than max_x".to_string());
    }

    if min_y > max_y {
        warnings.push("[!] min_y is greater than max_y".to_string());
    }

    if check_modes {
        let modes = &root["modes"];

        for (_, mode) in modes.entries() {
            let x = mode["x"].as_f64().unwrap();
            let y = mode["y"].as_f64().unwrap();
            let r = mode["size"].as_f64().unwrap();

            if x - r < min_x {
                warnings.push(format!("[!] mode {} x - r is less than min_x", mode["name"].as_str().unwrap()));
            }

            if x + r > max_x {
                warnings.push(format!("[!] mode {} x + r is greater than max_x", mode["name"].as_str().unwrap()));
            }

            if y - r < min_y {
                warnings.push(format!("[!] mode {} y - r is less than min_y", mode["name"].as_str().unwrap()));
            }

            if y + r > max_y {
                warnings.push(format!("[!] mode {} y + r is greater than max_y", mode["name"].as_str().unwrap()));
            }
        }
    }

    return warnings;
}