use json::JsonValue;
use rand;
use rand::Rng;
use indicatif::ProgressBar;

pub fn lock_by_map(content: &String, map: &JsonValue) -> String {
    let mut rng = rand::thread_rng();
    let mut locked = String::new();
    let pb = ProgressBar::new(content.len() as u64);
    println!("Encryption is started:");
    for letter in content.chars() {
        pb.inc(1);
        let random_index: usize = rng.gen_range(0,16);
        let mut random_index_string = random_index.to_string();
        random_index_string.push_str("-");
        let password_unit =  random_index_string.as_str();
        let transalted = &map[
            String::from(letter)
        ][random_index]
            .as_str()
            .expect("\nMap json is not the type it needs to be.");
        locked.push_str(transalted);
    }
    pb.finish_with_message("Encryption is done");
    locked
}

pub fn unlock_by_map(content: &String, map: &JsonValue) -> String {
    let mut content_mut = String::from(content);
    for (letter, translations) in map.entries() {
        for translation in translations.members() {
            let tr = translation
                .as_str()
                .expect("\nMap json is not the type it needs to be.");
            if content_mut.contains(tr) {
                content_mut = content_mut.replace(tr, letter);
            }

        }
    }
    return content_mut;
}