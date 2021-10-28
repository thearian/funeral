use json::{parse};

mod concat_str;
use concat_str::{
    concat_str_and_string,
    concat_string_and_str
};

mod inputs;
use inputs::{ get_and_read_inputs };

mod file;
use file::{write_file, read_file};

mod encryption;
use encryption::method_2::{lock_by_map, unlock_by_map};
use encryption::generate_map::new_map;
use encryption::hashing::calculate_hash;

fn main() {
    let (filepath, lock_status) = get_and_read_inputs();
    let content = read_file(&filepath);
    let map = new_map((16,128), 256);
    // Encrypt or Decrypt data
    let new_content = match lock_status {
        true => lock_by_map(&content, &map),
        false => unlock_by_map(&content, &map),
    };
    // Saving files
    let filename = filepath[..filepath.len()-4]
        .to_owned();
    
    let will_destination = concat_string_and_str(&filename,
        match lock_status {
            true => "-locked.txt",
            false => "-unlocked.txt",
        }
    );
    write_file(&will_destination, &new_content)
        .expect("\nFailed to save the translated will file.");
    
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
    
    println!("Your will is {} {}",
        match lock_status {
            true => "locked at",
            false => "unlocked at",
        },
        &will_destination
    )
}