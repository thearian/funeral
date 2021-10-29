use json;
use indicatif::{ProgressBar,ProgressStyle};
use console;

mod concat_str;
use concat_str::{
    concat_string_and_str
};

mod inputs;
use inputs::{ get_and_read_inputs, get_env_args };

mod file;
use file::{write_file, read_file};

mod encryption;
use encryption::method_2::{lock_by_map, unlock_by_map};
use encryption::generate_map::new_map;
use encryption::hashing::calculate_hash;

static WORD_LIMITS: (u8,u8) = (16, 128);
static WORD_COUNT: usize = 16;

fn main() {
    let (filepath, lock_status) = get_and_read_inputs();
    let content = read_file(&filepath);
    match lock_status {
        true => locking_process(content, filepath),
        false => unlocking_process(content, filepath),
    };
}

fn locking_process(content: String, filepath: String) {
    let map = new_map(WORD_LIMITS, WORD_COUNT, true);
    // Encrypt or Decrypt data
    let new_content = lock_by_map(&content, &map, WORD_COUNT);
    // Saving files
    let filename = filepath[..filepath.len()-4]
        .to_owned();
    
    let will_destination = concat_string_and_str(&filename, "-locked.txt",);
    write_file(&will_destination, &new_content)
        .expect("\nFailed to save the locked will file.");
    
    // NOTE: Saving map key as json will be removed later for the will's security 
    let map_string =  &(json::stringify_pretty(map, 4));
    let map_destination = concat_string_and_str(
        &filename,
        "-map.json"
    );
    write_file(&map_destination, map_string)
        .expect("\nFailed to save the key-map json file.");
    
    let hash_destination = concat_string_and_str(
        &filename,
        "-hash.txt"
    );
    write_file(&hash_destination, &(json::stringify(calculate_hash(map_string))) )
        .expect("\nFailed to save the map hash file");
    
    println!("Your will is locked at {}",
        &will_destination
    )
}

fn unlocking_process(content: String, filepath: String) {
    let hash = &get_env_args()
        [3]
        .parse::<u64>()
        .expect("Given hash is not type of Hash");
    let map = json::parse(
        &gen_map_qual_to_hash(&hash)
    ).expect("Faild to parse");

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

    println!("Chasing Hash... \n\t{}",
        console::style("Cancel by entering: Ctrl c")
            .bold()
            .dim()
    );
    let pb = ProgressBar::new(500);
    let spinner_style = ProgressStyle::default_spinner()
        .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ ")
        .template("{prefix:.bold.dim} {spinner} {wide_msg}");
    pb.set_style(spinner_style.clone());
    pb.set_prefix(format!("[ HASH ]"));
    
    let mut i = 0;
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