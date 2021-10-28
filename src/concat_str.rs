pub fn concat_str_and_string(str1: &str, str2: &String) -> String {
    let mut owned = str1.to_owned();
    owned.push_str( str2.as_str() );
    return owned;
}

pub fn concat_string_and_str(str1: &String, str2: &str) -> String {
    let mut owned = str1.to_owned();
    owned.push_str( str2.to_owned().as_str() );
    return owned;
}