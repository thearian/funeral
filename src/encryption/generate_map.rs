use json::JsonValue;
use rand::{self, Rng};
use indicatif::ProgressBar;

static ENCRYPT_DOMAIN: [char; 66] = [
    'q','w','e','r','t','y','u','i','o','p',
    'a','s','d','f','g','h','j','k','l',
    'z','x','c','v','b','n','m',' ','\n',
    '1','2','3','4','5','6','7','8','9','0',
    '~','!','@','#','$','%','^','&','*','(',')','_','+','-','=',
    '{','}','|','[',']',':',';',',','.','/','<','>','?',
];

type WordLimit = (u8, u8);


pub fn new_map(
    word_limits: WordLimit,
    char_map_count: usize,
    include_progress: bool
) -> JsonValue {
    let mut map = JsonValue::new_object();
    let mut rand_memo: Vec<String> = Vec::new();

    if include_progress {
        println!("Map is being genereated:");
    }
    let pb = ProgressBar::new(66); // ENCRYPT_DOMAIN len

    ENCRYPT_DOMAIN.iter()
        .for_each(|letter| {
            if include_progress {
                pb.inc(1);
            }
            map[String::from(*letter)] = gen_char_map(
                word_limits,
                char_map_count,
                &mut rand_memo
            );
        });
    
    if include_progress {
        pb.finish_with_message("Map is made");
    }
    map
}


pub fn gen_char_map(
    word_limits: WordLimit,
    count: usize,
    rand_memo: &mut Vec<String>
) -> JsonValue {
    let mut char_map = JsonValue::new_array();

    for _ in 0..count {
        let (mut word,mut is_old) = gen_new_word(word_limits, rand_memo);
        while is_old {
            let (new_word, new_is_old) = gen_new_word(word_limits, rand_memo);
            word = new_word;
            is_old = new_is_old;
        };
        rand_memo.push(word.to_owned());
        char_map.push(word)
            .expect("Source code has unfixed bugs");
    }

    char_map
}


fn gen_new_word(
    word_limits: WordLimit,
    rand_memo: &mut Vec<String>
) -> (String, bool) {
    let word = gen_word(
        rand_range(word_limits.0 as usize, word_limits.1 as usize)
    );
    
    let check_if_already_set: Vec<&String> = rand_memo.iter()
        .filter( |already_set_value| {
                *word == **already_set_value
            }
        )
        .collect();
    
    (word, check_if_already_set.len() > 0)
}

fn gen_word(count: usize) -> String {
    let mut word = String::new();
    for _ in 0..count {
        word.push(gen_char());
    }
    word
}

fn gen_char() -> char {
    ENCRYPT_DOMAIN[
        rand_range(0,ENCRYPT_DOMAIN.len())
    ]
}

fn rand_range(start: usize, end: usize) -> usize {
    rand::thread_rng()
        .gen_range(start,end)
}