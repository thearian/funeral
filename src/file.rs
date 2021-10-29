use std::fs;
use std::io::Write;


pub fn read_file(filepath: &String) -> String {
    fs::read_to_string(filepath.to_owned())
        .expect("\nFaild to read the file")
}


pub fn write_file(destination: &String, content: &String) -> std::io::Result<()> {
    let mut file = fs::File::create(destination.to_owned())?;
    file.write_all(content.as_bytes() as &[u8])?;
    Ok(())
}
