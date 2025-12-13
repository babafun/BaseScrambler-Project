use base64::{engine::general_purpose, Engine as _};
use data_encoding::BASE32;
use sha2::{Digest, Sha256};
use rand::RngCore;

pub fn base64_encode(s: &str) -> String {
    general_purpose::STANDARD.encode(s.as_bytes())
}

pub fn base64_decode(s: &str) -> Result<String, String> {
    general_purpose::STANDARD
        .decode(s)
        .map_err(|e| e.to_string())
        .and_then(|b| String::from_utf8(b).map_err(|e| e.to_string()))
}

pub fn base32_encode(s: &str) -> String {
    BASE32.encode(s.as_bytes())
}

pub fn base32_decode(s: &str) -> Result<String, String> {
    BASE32
        .decode(s.as_bytes())
        .map_err(|e| e.to_string())
        .and_then(|b| String::from_utf8(b).map_err(|e| e.to_string()))
}

pub fn hex_encode(s: &str) -> String {
    hex::encode(s.as_bytes())
}

pub fn hex_decode(s: &str) -> Result<String, String> {
    hex::decode(s)
        .map_err(|e| e.to_string())
        .and_then(|b| String::from_utf8(b).map_err(|e| e.to_string()))
}

pub fn bytes_to_hex_string(data: &[u8]) -> String {
    hex::encode(data)
}

pub fn scramble_1(s: &str) -> String {
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

pub fn unscramble_1(s: &str) -> String {
    scramble_1(s)
}

pub fn scramble_2(s: &str) -> Result<String, String> {
    let hex_encoded = hex_encode(s);
    let reversed_hex: String = hex_encoded.chars().rev().collect();
    let base64_encoded = base64_encode(&reversed_hex);
    Ok(scramble_1(&base64_encoded))
}

pub fn unscramble_2(s: &str) -> Result<String, String> {
    let unscrambled = unscramble_1(s);
    let base64_decoded = base64_decode(&unscrambled)?;
    let reversed_hex: String = base64_decoded.chars().rev().collect();
    hex_decode(&reversed_hex)
}

pub fn encode_method_1(s: &str) -> Result<String, String> {
    let byte_data = s.as_bytes();
    let hex_string = bytes_to_hex_string(byte_data);
    let pairs: Vec<&str> = hex_string.as_bytes()
        .chunks(2)
        .map(|c| std::str::from_utf8(c).unwrap_or(""))
        .collect();
    let reversed_string = pairs.into_iter().rev().collect::<String>();
    Ok(format!("{}!", reversed_string))
}

pub fn decode_method_1(s: &str) -> Result<String, String> {
    let mut s = s.to_string();
    if s.ends_with('!') {
        s.pop();
    }
    let pairs: Vec<&str> = s.as_bytes()
        .chunks(2)
        .map(|c| std::str::from_utf8(c).unwrap_or(""))
        .collect();
    let reversed: String = pairs.into_iter().rev().collect::<String>();
    hex_decode(&reversed)
}

pub fn encode_method_2(s: &str) -> Result<String, String> {
    let hex_string = hex::encode(s.as_bytes());
    let reversed_string: String = hex_string.chars().rev().collect();
    let base64_encoded = base64_encode(&reversed_string);
    Ok(format!("{}£", base64_encoded))
}

pub fn decode_method_2(s: &str) -> Result<String, String> {
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

pub fn encode_method_11(s: &str) -> Result<String, String> {
    let bytes = s.as_bytes();
    let shifted: Vec<u8> = bytes.iter().map(|b| b.wrapping_add(3)).collect();
    let hex_str = hex::encode(&shifted);
    let reversed_hex: String = hex_str.chars().rev().collect();
    let b64 = base64_encode(&reversed_hex);
    Ok(format!("{}§", b64))
}

pub fn decode_method_11(s: &str) -> Result<String, String> {
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

pub fn encode_string(s: &str) -> Result<String, String> {
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

pub fn decode_string(s: &str) -> Result<Option<String>, String> {
    if s.starts_with('~') {
        return base64_decode(&s[1..]).map(|v| Some(v));
    }
    let mut method = 0;
    let last = s.chars().last().unwrap_or('\0');
    match last {
        '#' => method = 1,
        '~' => method = 2,
        '!' => method = 3,
        '£' => method = 4,
        '$' => method = 5,
        '%' => method = 6,
        '^' => method = 7,
        '&' => method = 8,
        '¬' => method = 9,
        '@' => method = 10,
        '§' => method = 11,
        _ => return Ok(None),
    }
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
            // Expecting a Rust-like byte repr as in Python's repr(b"...")
            // We'll try to evaluate simple cases like [u8; N] style not supported, so return the raw string
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

pub fn hash_with_constant(prehashed_password: &[u8], constant: &[u8]) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(prehashed_password);
    hasher.update(constant);
    hasher.finalize().to_vec()
}

pub fn verify_password(stored_password: &[u8], input_password: &str, constant: &[u8]) -> bool {
    let mut hasher = Sha256::new();
    hasher.update(input_password.as_bytes());
    let prehashed = hasher.finalize_reset().to_vec();
    hasher.update(&prehashed);
    hasher.update(constant);
    let hashed_input = hasher.finalize().to_vec();
    hashed_input == stored_password
}

pub fn random_constant(len: usize) -> Vec<u8> {
    let mut buf = vec![0u8; len];
    rand::thread_rng().fill_bytes(&mut buf);
    buf
}
