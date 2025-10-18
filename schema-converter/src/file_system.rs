use std::io::Write;

pub fn rust_to_file(bytes: &[u8], file_path_string: &str) -> anyhow::Result<()> {
    let file_path: std::path::PathBuf = std::path::Path::new(&file_path_string).to_path_buf();

    let mut file: std::fs::File = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(file_path)?;

    file.write_all(bytes)?;

    Ok(())
}

pub fn json_to_file(file_path_string: &str, data: serde_json::Value) -> anyhow::Result<()> {
    let file_path: std::path::PathBuf = std::path::Path::new(&file_path_string).to_path_buf();

    let file: std::fs::File = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(file_path)?;

    serde_json::to_writer_pretty(file, &data)?;

    Ok(())
}

pub fn output_folder() -> anyhow::Result<()> {
    let output_path = std::path::Path::new("output/request").to_path_buf();

    if output_path.exists() {
        return Ok(());
    }

    std::fs::DirBuilder::new()
        .recursive(true)
        .create(output_path)?;

    Ok(())
}
