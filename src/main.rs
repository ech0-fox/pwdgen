use rand::Rng;
mod pgarg;

// Global setup
const DEFAULT_LENGTH: isize = 2;
const DEFAULT_MODE: i8 = 0b1111;

const CHARS: [&str; 4] = [
    "0123456789",
    "abcdefghijklmnopqrstuvwxyz",
    "ABCDEFGHIJKLMNOPQRSTUVWXYZ",
    "!@#$%^&*,.`~+-/?<>="
];

const MODE_TAGS: [&str; 4] = [
    "nums",
    "alpha_lower",
    "alpha_upper",
    "special_chars"
];
// ------------

// Obtain password length setting based on -length parameter
fn get_len(length_in: &str, bypass: bool) -> isize {
    if bypass {
        if let Ok(length_out) = length_in.parse::<isize>() {
            return length_out;
        } else {
            eprintln!("Invalid entry for --length: \"{}\" (--bplc ON)", length_in);
            return -1;
        }
    } else {
        let int_length: isize = match length_in {
            "0" | "ss" | "super_short" => 0,
            "1" | "s" | "short" => 1,
            "2" | "m" | "medium" => 2,
            "3" | "l" | "long" => 3,
            "4" | "ll" | "super_long" => 4,
            _ => {
                eprintln!("Invalid entry for --length: \"{}\"", length_in);
                return -1;
            }
        };
        return int_length;
    }
}

// Obtain mode number from -mode
fn get_mode_num(mode_in: &str) -> i8 {
    // Obtain unsigned 4-bit integer from mode_in
    match mode_in {
        // If mode_in is written in decimal
        "1" => 1,
        "2" => 2,
        "3" => 3,
        "4" => 4,
        "5" => 5,
        "6" => 6,
        "7" => 7,
        "8" => 8,
        "9" => 9,
        "10" => 10,
        "11" => 11,
        "12" => 12,
        "13" => 13,
        "14" => 14,
        "15" => 15,
        _ => {
            // If mode_in is written in binary
            let binary = i8::from_str_radix(mode_in, 2);
            match binary {
                Ok(value) if value <= 15 && value > 0 => value,
                _ => {
                    eprintln!("Invalid entry for --mode: \"{}\"", mode_in);
                    -1
                }
            }
        }
    }
}

// Generate mode flags with mode number
fn get_mode_flags(mode_num: i8) -> [bool; 4] {
    // Bitwise checks on mode_num for boolean array
    let mut result: [bool; 4] = [false; 4];
    for i in 0..4 {
        result[3 - i] = (mode_num & (1 << i)) != 0; // This might be the most brainfucky turing-complete bullshit shenanigan I've pulled in a script to date
    }
    return result;
}

// Make password length
fn calc_pwd_len<R: Rng>(rng: &mut R, len: isize) -> isize {
    let scaling_factor: isize = 4;
    let base_len: isize = (scaling_factor * len) + 4;
    return base_len + rng.gen_range(0..scaling_factor);
}

// Generate password
fn pwd_gen<R: Rng>(rng: &mut R, length: isize, chars: String) -> String {
    let mut password = String::new();
    let char_range: usize = chars.len();

    let mut bc: u8;
    let mut ichar: String;

    for _i in 0..length {
        bc = chars.as_bytes()[rng.gen_range(0..char_range)];
        ichar = (bc as char).to_string();
        password.push_str(&ichar);
    }

    return password;
}

// Main
fn main () {

    // --- Application level setup ---

    // Get command line logic from pgarg.rs
    let matches = pgarg::cmds();

    // Initialize runtime settings
    let length_setting: isize;
    let mode_num: i8;
    let mut chars = String::new();

    // --- Get settings from arguments ---
    if let Some(length_in) = matches.get_one::<String>("length") {
        length_setting = get_len(length_in, matches.get_flag("bypass_primary_length_check"));
    } else {
        length_setting = DEFAULT_LENGTH;
    }

    if let Some(mode_in) = matches.get_one::<String>("mode") {
        mode_num = get_mode_num(mode_in);
    } else {
        mode_num = DEFAULT_MODE;
    }
    let flags: [bool; 4] = get_mode_flags(mode_num);

    // Secondary information
    let mut mode_abstr = String::new();

    let length_tag: &str = match length_setting {
        0 => "super_short",
        1 => "short",
        2 => "medium",
        3 => "long",
        4 => "super_long",
        _ => "Eggs Benedict"
    };

    // Load character classes using -mode
    for i in 0..4 {
        if flags[i] {
            chars.push_str(CHARS[i]);
            mode_abstr.push_str(MODE_TAGS[i]);
            mode_abstr.push_str(";");
        }
    }
    // ------------

    // Initialize a random number geneator
    let mut rng = rand::thread_rng();

    // Make password length
    let password_length: isize = calc_pwd_len(&mut rng, length_setting);

    // Generate password
    let password: String = pwd_gen(&mut rng, password_length, chars);

    // Print the password and its relevant information to the console
    println!("Password: {}", password);
    println!("length = {} characters ({}), mode = {} ({})", password_length, length_tag, mode_num, mode_abstr);
}