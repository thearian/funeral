use std::env;

pub fn get_and_read_inputs() -> (String, String, bool) {
    let mut args = get_env_args();
    
    // get args that are not defined
    for arg_number in args.len()..=3 {
        match arg_number {
            1 => println!("Enter file location: "),
            2 => println!("Enter the password: "),
            3 => println!("Enter L to lock by encryption or U to unlock by decryption: "),
            _ => panic!("Source code has unfixed bug."),
        };
        args.push( get_from_user() );
    };

    let filename = args[1].to_owned();
    let password = args[2].to_owned();
    let lock_status = args[3] == "L";

    return (filename, password, lock_status);
}

fn get_env_args() -> Vec<String> {
    env::args().collect()
}

fn get_from_user() -> String {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("\nFailed to read user input");
    return input;
}