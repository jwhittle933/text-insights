use anyhow::Context;
use ti_core::img::Image;

const HEBREW_IMAGES: &str = ".data/image/hebrew/aleph";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let entries = ti_core::fs::read_recursive(HEBREW_IMAGES)?;

    for entry in entries {
        println!("{entry}");
        let (label, fname) =
            read_file_label(HEBREW_IMAGES, &entry).context("failed to read file label")?;
        let f = std::fs::read(entry).context("failed to read image")?;
        let img = Image::try_from(f)?.transform();

        img.debug_write(&(label + "aleph"), &fname)?;
    }

    Ok(())
}

fn read_file_label(base_dir: &str, fpath: &str) -> anyhow::Result<(String, String)> {
    let labeled_path = fpath
        .split(base_dir)
        .last()
        .ok_or(anyhow::anyhow!("failed to get file label".to_string()))?
        .trim_start_matches("/")
        .to_string();

    let mut parts = labeled_path.split("/").collect::<Vec<_>>();
    let fname = parts
        .pop()
        .ok_or(anyhow::anyhow!("no filename".to_string()))?
        .to_string();

    Ok((parts.join("/"), fname))
}
