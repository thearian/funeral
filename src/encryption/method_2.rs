use json::JsonValue;
use rand;
use rand::Rng;

pub fn lock_by_map(content: &String, map: &JsonValue) -> String {
    let mut rng = rand::thread_rng();
    let mut locked = String::new();
    for letter in content.chars() {
        let random_index: usize = rng.gen_range(0,16);
        let transalted = &map[
            String::from(letter)
        ][random_index]
            .as_str()
            .expect("Error code: 1389.");
        locked.push_str(transalted);
    }
    return locked;
}

pub fn unlock_by_map(content: &String, map: &JsonValue) -> String {
    let mut content_mut = String::from(content);
    for (letter, translations) in map.entries() {
        for translation in translations.members() {
            let tr = translation
                .as_str()
                .expect("Error code: 1249") ;
            if content_mut.contains(tr) {
                content_mut = content_mut.replace(tr, letter);
            }

        }
    }
    return content_mut;
}