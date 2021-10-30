use json::JsonValue;
use indicatif::{ProgressBar,ProgressStyle};
use console;
use std::time::{Duration, Instant};
use num;

mod concat_str;
use concat_str::concat_string_and_str;

mod inputs;
use inputs::{ get_and_read_inputs, get_env_args };

mod file;
use file::{write_file, read_file};

mod encryption;
use encryption::method_2::{
    lock_by_map,
    unlock_by_map
};
use encryption::generate_map::{
    new_map,
    gen_char_map
};
use encryption::hashing::{
    calculate_hash,
    calculate_hash_for_map
};

static WORD_LIMITS: (u8,u8) = (16, 128);
static WORD_COUNT: usize = 16;


fn main() {
    let (filepath, lock_status) = get_and_read_inputs();

    println!("{}",
        console::style("Reading your file").bold().green()
    );
    let content = read_file(&filepath);
    println!("Read successfully");
    
    match lock_status {
        true => locking_process(content, filepath),
        false => unlocking_process(content, filepath),
    };
}


fn locking_process(content: String, filepath: String) {
    // Generating a new map
    println!("{}",
        console::style("Generating Map").bold().green()
    );
    let map = new_map(WORD_LIMITS, WORD_COUNT, true);

    // Encrypt or data
    println!("{}",
        console::style("Encrypting").bold().green()
    );
    let new_content = lock_by_map(&content, &map, WORD_COUNT);

    // Saving files
    let filename = filepath[..filepath.len()-4]
        .to_owned();
    
    let will_destination = concat_string_and_str(&filename, "-locked.txt",);
    write_file(&will_destination, &new_content)
        .expect("\nFailed to save the locked will file.");
    
    let hash_map = &(json::stringify_pretty(
        calculate_hash_for_map(&map),
        4
    ));
    let map_destination = concat_string_and_str(
        &filename,
        "-hash-map.json"
    );
    write_file(&map_destination, hash_map)
        .expect("\nFailed to save the key-map json file.");
    
    // NOTE: Saving map key as json will be removed later for the will's security 
    let map_string =  &(json::stringify_pretty(map, 4));
    let map_destination = concat_string_and_str(
        &filename,
        "-map.json"
    );
    write_file(&map_destination, map_string)
        .expect("\nFailed to save the key-map json file.");

    let hash = &(
        json::stringify(
            calculate_hash(map_string)
        )
    );
    let hash_destination = concat_string_and_str(
        &filename,
        "-hash.txt"
    );
    write_file(&hash_destination, hash )
        .expect("\nFailed to save the map hash file");
    
    println!("Your will is locked at {}\nPlease keep this hash: {}",
        &will_destination,
        hash
    )
}


fn unlocking_process(content: String, filepath: String) {
    // Getting the hash from user
    // let hash = &get_env_args()
    //     [3]
    //     .parse::<u64>()
    //     .expect("Given hash is not type of Hash");
    let filename = filepath[..filepath.len()-4]
        .to_owned();
    let hash_map = json::parse(
        &read_file(
            &concat_string_and_str(
                &filename,
                "-hash-map.json"
            )
        )
    ).expect("Failed to parse hash map json");
    
    // Finding the map with the same hash
    let map = &gen_map_qual_to_hash_map(&hash_map);
        // &gen_map_qual_to_hash(&hash)

    println!("{}",
        console::style("Decrypting").bold().green()
    );
    let new_content = unlock_by_map(&content, &map);

    // Saving files
    let filename = filepath[..filepath.len()-4]
        .to_owned();
    
    let will_destination = concat_string_and_str(&filename, "-unlocked.txt",);
    write_file(&will_destination, &new_content)
        .expect("\nFailed to save the unlocked will file.");
        
    println!("Your will is unlocked at {}",
        &will_destination
    )

}


fn gen_map_qual_to_hash(hash: &u64) -> String {
    let mut map = new_map(WORD_LIMITS, WORD_COUNT, false);
    let mut map_string = json::stringify_pretty(map, 4);

    // Prints
    println!("{}\n\t{}",
        console::style("Chasing Hash")
            .bold()
            .green(),
        console::style("Cancel by entering: Ctrl c")
            .bold()
            .dim()
    );
    let pb = ProgressBar::new(500);
    let spinner_style = ProgressStyle::default_spinner()
        .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ ")
        .template("{prefix:.bold.dim} {spinner} {wide_msg}");
    pb.set_style(spinner_style.clone());
    pb.set_prefix(format!("[ HASH CHASE ]"));
    let mut i = 0;
    
    // Chasing
    while calculate_hash(&map_string) != *hash {
        i += 1;
        pb.set_message(
            format!("({}) {}: {}",
                i,
                hash,
                calculate_hash(&map_string),
            )
        );
        pb.inc(1);

        map = new_map((16,128), 4, false);
        map_string = json::stringify_pretty(map, 4);
    }
    map_string
}


fn gen_map_qual_to_hash_map(hash_map: &JsonValue) -> JsonValue {
    // Setting timer
    let start = Instant::now();
    
    // Setting map and memory for unduplicated check
    let mut map = JsonValue::new_object();
    let mut rand_memo: Vec<String> = Vec::new();

    // Prints
    println!("{}\n\t{}",
        console::style("Chasing Hash")
            .bold()
            .green(),
        console::style("Cancel by entering: Ctrl c")
            .bold()
            .dim()
    );
    let pb = ProgressBar::new(500);
    let spinner_style = ProgressStyle::default_spinner()
        .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ ")
        .template("{prefix:.bold.dim} {spinner} {wide_msg}");
    pb.set_style(spinner_style.clone());
    pb.set_prefix(format!("[ HASH CHASE ]"));
    let mut i = 0;

    // Chase
    for (letter, letter_hash_json) in hash_map.entries() {
        let mut volenteer = gen_char_map(
            WORD_LIMITS,
            WORD_COUNT,
            &mut rand_memo
        );
        let mut char_map_string = json::stringify(volenteer);
        let mut letter_hash: u64 = letter_hash_json
            .as_u64()
            .expect("Hash map is not made of numbers");

        while calculate_hash(&char_map_string) != letter_hash {
            // Timing
            i += 1;
            let max_states: u64 = (WORD_COUNT as u64) * num::pow(66, WORD_LIMITS.1 as usize);
            let duration = start.elapsed();
            let max_secs = duration.as_secs() * max_states / i;
            let max_duration = Duration::from_secs(max_secs);

            // Prints
            pb.set_message(
                format!("({}) {}/{} ({}%) ({}/{}): {} != {}",
                    letter,
                    i,
                    max_states,
                    ((i/max_states) * 100) as f32,
                    to_min(duration.as_secs()),
                    to_min(max_duration.as_secs()),
                    letter_hash,
                    calculate_hash(&char_map_string),
                )
            );
            pb.inc(1);

            volenteer = gen_char_map(
                WORD_LIMITS,
                WORD_COUNT,
                &mut rand_memo
            );
            char_map_string = json::stringify(volenteer);
            letter_hash = letter_hash_json
                .as_u64()
                .expect("Hash map is not made of numbers");
        }
        map[letter] = json::parse(&char_map_string)
            .expect("Failed to parse guess char map");
    }
    map
}


fn to_min(secs: u64) -> String {
    let seconds = secs % 60;
    let minuts = ((secs - seconds) / 60) % 60;
    let hours = (secs - (minuts * 60) - seconds) / 60;

    String::from(format!("{}:{}:{}",hours,minuts,seconds))
}