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

fn main() {
    let (filename, password, lock_status) = get_and_read_inputs();
    let content = read_file(&filename);
    // let map_file = read_file(&String::from("map.json"));
    // let map = parse(&map_file)
        // .expect("Error code: json parse.");
    let map = new_map((16,128), 16);
    let (new_content, new_password) = match lock_status {
        true => lock_by_map(&content, &map),
        false => (unlock_by_map(&content, &map, password), String::new()),
    };
    let len = filename.len();
    let will_destination = concat_string_and_str(filename[..len-4].to_owned(),
        match lock_status {
            true => "-locked.txt",
            false => "-unlocked.txt",
        }
    );
    let map_destination = concat_string_and_str(filename[..len-4].to_owned(),"-map.json");
    write_file(&will_destination, &new_content)
        .expect("Error code: 0974");
    write_file(&map_destination, &(json::stringify_pretty(map, 4)))
        .expect("Error code: 0974");
    println!("Your will is {}",
        match lock_status {
            true => concat_str_and_string("locked with this password: ", new_password),
            false => concat_str_and_string("unlocked at ", will_destination)
        },
    )
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