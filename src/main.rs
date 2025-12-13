use base64::{engine::general_purpose, Engine as _};
use data_encoding::BASE32;
use rand::RngCore;
use sha2::{Digest, Sha256};
use std::io::{self, Write};
use colored::Colorize;

fn base64_encode(s: &str) -> String {
    general_purpose::STANDARD.encode(s.as_bytes())
}

fn base64_decode(s: &str) -> Result<String, String> {
    general_purpose::STANDARD
        .decode(s)
        .map_err(|e| e.to_string())
        .and_then(|b| String::from_utf8(b).map_err(|e| e.to_string()))
}

fn base32_encode(s: &str) -> String {
    BASE32.encode(s.as_bytes())
}

fn base32_decode(s: &str) -> Result<String, String> {
    BASE32
        .decode(s.as_bytes())
        .map_err(|e| e.to_string())
        .and_then(|b| String::from_utf8(b).map_err(|e| e.to_string()))
}

fn hex_encode(s: &str) -> String {
    hex::encode(s.as_bytes())
}

fn hex_decode(s: &str) -> Result<String, String> {
    hex::decode(s)
        .map_err(|e| e.to_string())
        .and_then(|b| String::from_utf8(b).map_err(|e| e.to_string()))
}

fn bytes_to_hex_string(data: &[u8]) -> String {
    hex::encode(data)
}

fn scramble_1(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let chars: Vec<char> = s.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        if i + 1 < chars.len() {
            out.push(chars[i + 1]);
            out.push(chars[i]);
        } else {
            out.push(chars[i]);
        }
        i += 2;
    }
    out
}

fn unscramble_1(s: &str) -> String {
    scramble_1(s)
}

fn scramble_2(s: &str) -> Result<String, String> {
    let hex_encoded = hex_encode(s);
    let reversed_hex: String = hex_encoded.chars().rev().collect();
    let base64_encoded = base64_encode(&reversed_hex);
    Ok(scramble_1(&base64_encoded))
}

fn unscramble_2(s: &str) -> Result<String, String> {
    let unscrambled = unscramble_1(s);
    let base64_decoded = base64_decode(&unscrambled)?;
    let reversed_hex: String = base64_decoded.chars().rev().collect();
    hex_decode(&reversed_hex)
}

fn encode_method_1(s: &str) -> Result<String, String> {
    let byte_data = s.as_bytes();
    let hex_string = bytes_to_hex_string(byte_data);
    let pairs: Vec<&str> = hex_string
        .as_bytes()
        .chunks(2)
        .map(|c| std::str::from_utf8(c).unwrap_or(""))
        .collect();
    let reversed_string = pairs.into_iter().rev().collect::<String>();
    Ok(format!("{}!", reversed_string))
}

fn decode_method_1(s: &str) -> Result<String, String> {
    let mut s = s.to_string();
    if s.ends_with('!') {
        s.pop();
    }
    let pairs: Vec<&str> = s
        .as_bytes()
        .chunks(2)
        .map(|c| std::str::from_utf8(c).unwrap_or(""))
        .collect();
    let reversed: String = pairs.into_iter().rev().collect::<String>();
    hex_decode(&reversed)
}

fn encode_method_2(s: &str) -> Result<String, String> {
    let hex_string = hex::encode(s.as_bytes());
    let reversed_string: String = hex_string.chars().rev().collect();
    let base64_encoded = base64_encode(&reversed_string);
    Ok(format!("{}£", base64_encoded))
}

fn decode_method_2(s: &str) -> Result<String, String> {
    let mut s = s.to_string();
    if s.ends_with('£') {
        s.pop();
    }
    let missing_padding = s.len() % 4;
    if missing_padding != 0 {
        s.push_str(&"=".repeat(4 - missing_padding));
    }
    let base64_decoded = base64_decode(&s)?;
    let reversed_string: String = base64_decoded.chars().rev().collect();
    hex_decode(&reversed_string)
}

fn encode_method_11(s: &str) -> Result<String, String> {
    let bytes = s.as_bytes();
    let shifted: Vec<u8> = bytes.iter().map(|b| b.wrapping_add(3)).collect();
    let hex_str = hex::encode(&shifted);
    let reversed_hex: String = hex_str.chars().rev().collect();
    let b64 = base64_encode(&reversed_hex);
    Ok(format!("{}§", b64))
}

fn decode_method_11(s: &str) -> Result<String, String> {
    let mut s = s.to_string();
    if s.ends_with('§') {
        s.pop();
    }
    let missing_padding = s.len() % 4;
    if missing_padding != 0 {
        s.push_str(&"=".repeat(4 - missing_padding));
    }
    let reversed_hex = base64_decode(&s)?;
    let hex_str: String = reversed_hex.chars().rev().collect();
    let shifted_bytes = hex::decode(&hex_str).map_err(|e| e.to_string())?;
    let original: Vec<u8> = shifted_bytes.iter().map(|b| b.wrapping_sub(3)).collect();
    String::from_utf8(original).map_err(|e| e.to_string())
}

fn encode_string(s: &str) -> Result<String, String> {
    if s.chars().any(|c| c as u32 > 127) {
        return Ok(format!("~{}", base64_encode(s)));
    }
    let mut rng = rand::thread_rng();
    let method = (rng.next_u32() % 11) + 1;
    match method {
        1 => Ok(format!("{}#", base32_encode(&scramble_1(&base64_encode(s)).as_str()))),
        2 => scramble_2(s).map(|r| format!("{}~", r)),
        3 => encode_method_1(s),
        4 => encode_method_2(s),
        5 => {
            let base64_encoded = base64_encode(s);
            let reversed_string: String = base64_encoded.chars().rev().collect();
            let scrambled = scramble_1(&reversed_string);
            Ok(format!("{}$", hex_encode(&scrambled)))
        }
        6 => {
            let hex_encoded = hex_encode(s);
            let reversed_string: String = hex_encoded.chars().rev().collect();
            let base64_encoded = base64_encode(&reversed_string);
            let scrambled = scramble_1(&base64_encoded);
            Ok(format!("{}%", scrambled))
        }
        7 => {
            let base32_encoded = base32_encode(s);
            let reversed_string: String = base32_encoded.chars().rev().collect();
            let hex_encoded = hex_encode(&reversed_string);
            let scrambled = scramble_1(&hex_encoded);
            Ok(format!("{}^", scrambled))
        }
        8 => {
            let base64_encoded = base64_encode(s);
            let scrambled = scramble_1(&base64_encoded);
            let base32_encoded = base32_encode(&scrambled);
            let reversed_string: String = base32_encoded.chars().rev().collect();
            Ok(format!("{}&", reversed_string))
        }
        9 => {
            let byte_data = s.as_bytes();
            let byte_string = format!("{:?}", byte_data);
            let reversed_string: String = byte_string.chars().rev().collect();
            let scrambled = scramble_1(&reversed_string);
            Ok(format!("{}¬", scrambled))
        }
        10 => {
            let byte_data = s.as_bytes();
            let hex_string = bytes_to_hex_string(byte_data);
            let reversed_string: String = hex_string.chars().rev().collect();
            let base64_encoded = base64_encode(&reversed_string);
            Ok(format!("{}@", base64_encoded))
        }
        11 => encode_method_11(s),
        _ => Ok(String::new()),
    }
}

fn decode_string(s: &str) -> Result<Option<String>, String> {
    if s.starts_with('~') {
        return base64_decode(&s[1..]).map(|v| Some(v));
    }
    let last = s.chars().last().unwrap_or('\0');
    let method = match last {
        '#' => 1,
        '~' => 2,
        '!' => 3,
        '£' => 4,
        '$' => 5,
        '%' => 6,
        '^' => 7,
        '&' => 8,
        '¬' => 9,
        '@' => 10,
        '§' => 11,
        _ => return Ok(None),
    };
    let trimmed = &s[..s.len() - 1];
    match method {
        1 => {
            let reversed_s: String = trimmed.chars().rev().collect();
            let base32_decoded = base32_decode(&reversed_s)?;
            let unscrambled = unscramble_1(&base32_decoded);
            base64_decode(&unscrambled).map(|v| Some(v))
        }
        2 => unscramble_2(trimmed).map(|v| Some(v)),
        3 => decode_method_1(trimmed).map(|v| Some(v)),
        4 => decode_method_2(trimmed).map(|v| Some(v)),
        5 => {
            let hex_decoded = hex_decode(trimmed)?;
            let unscrambled = scramble_1(&hex_decoded);
            let reversed_string: String = unscrambled.chars().rev().collect();
            base64_decode(&reversed_string).map(|v| Some(v))
        }
        6 => {
            let unscrambled = unscramble_1(trimmed);
            let base64_decoded = base64_decode(&unscrambled)?;
            let reversed_string: String = base64_decoded.chars().rev().collect();
            hex_decode(&reversed_string).map(|v| Some(v))
        }
        7 => {
            let unscrambled = unscramble_1(trimmed);
            let hex_decoded = hex_decode(&unscrambled)?;
            let reversed_string: String = hex_decoded.chars().rev().collect();
            base32_decode(&reversed_string).map(|v| Some(v))
        }
        8 => {
            let reversed_string: String = trimmed.chars().rev().collect();
            let base32_decoded = base32_decode(&reversed_string)?;
            let unscrambled = unscramble_1(&base32_decoded);
            base64_decode(&unscrambled).map(|v| Some(v))
        }
        9 => {
            let unscrambled = unscramble_1(trimmed);
            let reversed_string: String = unscrambled.chars().rev().collect();
            Ok(Some(reversed_string))
        }
        10 => {
            let base64_decoded = base64_decode(trimmed)?;
            let reversed_string: String = base64_decoded.chars().rev().collect();
            let bytes = hex::decode(&reversed_string).map_err(|e| e.to_string())?;
            String::from_utf8(bytes).map(|v| Some(v)).map_err(|e| e.to_string())
        }
        11 => decode_method_11(trimmed).map(|v| Some(v)),
        _ => Ok(None),
    }
}

fn hash_with_constant(prehashed_password: &[u8], constant: &[u8]) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(prehashed_password);
    hasher.update(constant);
    hasher.finalize().to_vec()
}

fn verify_password(stored_password: &[u8], input_password: &str, constant: &[u8]) -> bool {
    let mut hasher = Sha256::new();
    hasher.update(input_password.as_bytes());
    let prehashed = hasher.finalize_reset().to_vec();
    hasher.update(&prehashed);
    hasher.update(constant);
    let hashed_input = hasher.finalize().to_vec();
    hashed_input == stored_password
}

fn random_constant(len: usize) -> Vec<u8> {
    let mut buf = vec![0u8; len];
    rand::thread_rng().fill_bytes(&mut buf);
    buf
}

fn print_coloured(text: &str, colour: &str) {
    match colour {
        "bold_yellow" => println!("{}", text.yellow().bold()),
        "bold_cyan" => println!("{}", text.cyan().bold()),
        "bold_red" => println!("{}", text.red().bold()),
        "bold_green" => println!("{}", text.green().bold()),
        "white" => println!("{}", text.white()),
        "yellow" => println!("{}", text.yellow()),
        "cyan" => println!("{}", text.cyan()),
        "red" => println!("{}", text.red()),
        "green" => println!("{}", text.green()),
        _ => println!("{}", text),
    }
}

fn print_coloured_noln(text: &str, colour: &str) {
    match colour {
        "bold_yellow" => print!("{}", text.yellow().bold()),
        "bold_cyan" => print!("{}", text.cyan().bold()),
        "bold_red" => print!("{}", text.red().bold()),
        "bold_green" => print!("{}", text.green().bold()),
        "white" => print!("{}", text.white()),
        _ => print!("{}", text),
    }
    io::stdout().flush().ok();
}

fn prompt_and_read(prompt: &str, colour: &str) -> io::Result<String> {
    print_coloured_noln(prompt, colour);
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let prehashed_hex = "38de90475bb334fb3dea5d54f250500aba60fe2c6158115d342b06bcb46e39bf";
    let prehashed_bytes = hex::decode(prehashed_hex)?;
    let constant = random_constant(16);
    let stored_password = hash_with_constant(&prehashed_bytes, &constant);

    let mut attempts = 0;
    let mut decode_attempts = 0;

    print_coloured("BASE/Scramble", "bold_yellow");
    print_coloured("(version 2.1)", "white");
    while attempts < 3 {
        let user_password = prompt_and_read("Enter password: ", "bold_cyan")?;
        if verify_password(&stored_password, &user_password, &constant) {
            // mimic clearing previous input and showing green confirmation
            print!("\x1b[1A\x1b[2K");
            print_coloured(&user_password, "bold_green");
            print_coloured("Correct password!", "bold_green");
            loop {
                let action = prompt_and_read("Do you want to encode, decode, or exit? ", "white")?.to_lowercase();
                match action.as_str() {
                    "encode" => {
                        let text = prompt_and_read("Enter the text you want to encode: ", "bold_yellow")?;
                        match encode_string(&text) {
                            Ok(e) => print_coloured(&format!("Encoded: {}", e), "bold_cyan"),
                            Err(e) => print_coloured(&format!("Encode error: {}", e), "bold_red"),
                        }
                    }
                    "decode" => {
                        let text = prompt_and_read("Enter the text you want to decode: ", "bold_yellow")?;
                        match decode_string(&text) {
                            Ok(Some(d)) => { print_coloured(&format!("Decoded: {}", d), "bold_cyan"); decode_attempts = 0; }
                            Ok(None) => {
                                print_coloured("That's invalid.", "bold_red");
                                decode_attempts += 1;
                            }
                            Err(e) => {
                                print_coloured(&format!("Decode error: {}", e), "bold_red");
                                decode_attempts += 1;
                            }
                        }
                    }
                    "exit" => {
                        print_coloured("Exiting the program. Goodbye!", "bold_red");
                        return Ok(());
                    }
                    _ => print_coloured("Invalid option. Please choose 'encode', 'decode', or 'exit'.", "bold_red"),
                }
                if decode_attempts == 3 {
                    print_coloured("Maybe try a different approach?", "bold_yellow");
                } else if decode_attempts == 5 {
                    print_coloured("That's invalid... just like your life.", "bold_yellow");
                    decode_attempts = 0;
                }
            }
        } else {
            print!("\x1b[1A\x1b[2K");
            print_coloured("Incorrect password. Try again.", "bold_red");
            attempts += 1;
        }
    }
    print_coloured("Too many incorrect attempts. Exiting.", "bold_red");
    Ok(())
}
// End of program
