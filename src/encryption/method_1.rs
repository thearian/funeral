use rand::{Rng,SeedableRng};
use rand::rngs::StdRng;

pub struct MapItem {
    domain: char,
    translation: String,
}
pub type Map = Vec<MapItem>;

static ENCRYPT_DOMAIN: [char; 66] = [
    'q','w','e','r','t','y','u','i','o','p','a','s','d','f','g','h','j','k','l','z','x','c','v','b','n','m',' ','\n',
    '1','2','3','4','5','6','7','8','9','0',
    '~','!','@','#','$','%','^','&','*','(',')','_','+','-','=','{','}','|','[',']',':',';',',','.','/','<','>','?',
];

pub fn password_to_map(password: String) -> Map {
    match password .parse::<u64>() {
        Ok(seed) => seed_to_map(seed),
        Err(error) => panic!("\n\tError code: 5293 including {}", error)
    }
}

pub fn key_map(content: String, map: Map, lock_status: bool) -> String {
    let mut untranslated: Vec<u8> = content.into_bytes();
    let mut translated = String::new();
    for map_item in map {
        untranslated = translate_by_map_item(&mut translated, &untranslated, map_item);
    }
    return translated.to_string();
}

fn seed_to_map(seed: u64) -> Map {
    let mut map = Map::new();
    let mut rng = StdRng::seed_from_u64(seed);
    for domain in ENCRYPT_DOMAIN.iter() {
        map.push(
            MapItem{
                domain: *domain,
                translation: produce_word(&mut rng, 32)
            }
        );
    };
    return map;
}

fn produce_letter(rng: &mut StdRng) -> char {
    ENCRYPT_DOMAIN[
        rng.gen_range(0, ENCRYPT_DOMAIN.len())
    ]
}

fn produce_word(rng: &mut StdRng, count: usize) -> String {
    let mut word = String::new();
    for _ in 0..count {
        word.push( produce_letter(rng) )
    }
    return word;
}

fn translate_by_map_item(
    translated: &mut String,
    untranslated: &Vec<u8>,
    map_item: MapItem
) -> Vec<u8> {
    untranslated.iter().map(|letter| {
        if (*letter as char) == map_item.domain {
            translated.push_str(&map_item.translation);
            return 0;
        }
        else {
            return *letter
        }
    }).collect()
}

fn untranslate_by_map_item(
    translated: &mut String,
    untranslated: &Vec<u8>,
    map_item: MapItem
) -> String {
    for i in 0..(untranslated.len() / 32 as usize) {
        let period = i..(i+1)*32;
        let slice = &untranslated[period];
        if slice_to_string(slice) == map_item.translation {
            translated.push(map_item.domain);
        }
    }
    return translated.to_string();
}

fn slice_to_string(slice: &[u8]) -> String {
    let mut string = String::new();
    for letter in slice {
        string.push(*letter as char)
    }
    return string;
}