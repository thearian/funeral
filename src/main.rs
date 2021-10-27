use json::{parse};

mod inputs;
use inputs::{ get_and_read_inputs };

mod file;
use file::{write_file, read_file};

mod encryption;
use encryption::method_2::{lock_by_map, unlock_by_map};

fn main() {
    let (filename, password, lock_status) = get_and_read_inputs();
    let content = read_file(&filename);
    let map_file = read_file(&String::from("map-small.json"));
    let map = parse(&map_file)
        .expect("Error code: json parse.");
    let new_content = match lock_status {
        true => lock_by_map(&content, &map),
        false => unlock_by_map(&content, &map),
    };
    let len = filename.len();
    let mut destination = filename[..len-4].to_owned();
    destination.push_str(match lock_status {
        true => "-locked.txt",
        false => "-unlocked.txt",
    });
    let file = write_file(&destination, &new_content);
    match file {
        Ok(_) => println!("Your will is translated at {}.will!", filename),
        Err(_) => println!("Error code: 0974."),
    }
}

// fn main() {
    // let (filename, password, lock_status) = get_and_read_inputs();
    // let content = read_file(&filename);
    // let map: Map = password_to_map(password);
    // let new_content = key_map(content, map, lock_status);
    // let destination = filename.to_owned() + "-locked.txt";
    // let file = write_file(&destination, &new_content);
    // match file {
    //     Ok(_) => println!("Your will is translated at {}.will!", filename),
    //     Err(_) => println!("Error code: 0974."),
    // }
// }