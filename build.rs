use std::{fs, path::Path, error::Error};

fn main() -> Result<(), Box<dyn Error>> {
    let ui_dir = Path::new("ui");
    if ui_dir.exists() {
        for e in fs::read_dir(ui_dir)? {
            let path = e?.path();
            if path.extension().and_then(|s| s.to_str()) == Some("slint") {
                println!("cargo:rerun-if-changed={}", path.display());
                slint_build::compile(&path)?;
            }
        }
    }

    Ok(())
}