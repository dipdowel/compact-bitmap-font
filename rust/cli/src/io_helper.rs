use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

/// Saves the CBF data to a file and returns the path to where the file was written
pub fn write_cbf_to_file(
    dir: &str,
    file_name: &str,
    font_data: &[u8],
    path_buf: &mut PathBuf,
) -> std::io::Result<()> {
    let path = Path::new(dir);

    if !path.exists() {
        fs::create_dir_all(path)?;
    }

    let file_path = path.join(file_name);

    *path_buf = file_path.clone();

    let mut file = File::create(file_path)?;

    for num in font_data {
        file.write_all(&num.to_le_bytes())?; // Using little endian encoding
    }
    Ok(())
}


