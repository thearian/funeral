use json::JsonValue;
use rand;
use rand::Rng;

pub fn lock_by_map(content: &String, map: &JsonValue) -> (String, String) {
    let mut rng = rand::thread_rng();
    let mut locked = String::new();
    let mut password = String::new();
    for letter in content.chars() {
        let random_index: usize = rng.gen_range(0,16);
        let mut random_index_string = random_index.to_string();
        random_index_string.push_str("-");
        let password_unit =  random_index_string.as_str();
        password.push_str( password_unit );
        let transalted = &map[
            String::from(letter)
        ][random_index]
            .as_str()
            .expect("Error code: 1389.");
        locked.push_str(transalted);
    }
    (locked, password)
}

pub fn unlock_by_map(content: &String, map: &JsonValue, password: String) -> String {
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