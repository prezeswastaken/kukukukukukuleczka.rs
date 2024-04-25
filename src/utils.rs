use anyhow::anyhow;

pub fn clean_storage(storage_path: String) -> anyhow::Result<()> {
    let paths = std::fs::read_dir(&storage_path)?;
    for path in paths {
        let path = path?.path();
        if path.is_file() {
            let file_name = path
                .file_name()
                .ok_or_else(|| anyhow!("Couldn't get file name"))?
                .to_str()
                .ok_or_else(|| anyhow!("Couldn't convert file name to string"))?;
            if !file_name.ends_with(".pdf") {
                std::fs::remove_file(&path)?;
            }
        }
    }

    Ok(())
}
