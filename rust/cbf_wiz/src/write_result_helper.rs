use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;

/// Creates a directory if it doesn't exist, then writes a file into it.
pub fn create_dir_and_write_file(dir: &str, file_name: &str, font_header: &[u16], font_body: &[u8]) -> std::io::Result<()> {

    let path = Path::new(dir);

    if !path.exists() {
        fs::create_dir_all(path)?;
    }

    let file_path = path.join(file_name);
    let mut file = File::create(file_path)?;


    for num in font_header {
        file.write_all(&num.to_le_bytes()).unwrap(); // Using little endian encoding
    }
    for num in font_body {
        file.write_all(&num.to_le_bytes()).unwrap(); // Using little endian encoding
    }
    Ok(())

}
