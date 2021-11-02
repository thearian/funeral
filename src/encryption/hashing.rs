use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use json::JsonValue;

pub fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}


pub fn calculate_hash_for_map(map: &JsonValue) -> JsonValue {
    let mut hash_map = JsonValue::new_object();
    for (letter, translations) in map.entries() {
        let translations_vec: Vec<u64> = translations
            .members()
            .map(|tr| {
                calculate_hash(&tr.as_str()
                    .expect("")
                    .to_owned()
                )
            })
            .collect();
        hash_map[
            String::from(letter)
        ] = json::from(translations_vec);
    }
    hash_map
}