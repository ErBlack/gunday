use std::error::Error;
use std::fs::File;
use std::path::{Path, PathBuf};

const MAX_TEXTURE_DIMENSION: usize = 8192;
const LEVEL_DIR: &str = "page/assets/levels";

fn main() {
    if let Err(err) = run() {
        panic!("build script failed: {err}");
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let level_count = split_image("level")?;
    let foreground_count = split_image("foreground")?;

    println!("cargo:rustc-env=LEVEL_CHUNK_COUNT={level_count}");
    println!("cargo:rustc-env=FOREGROUND_CHUNK_COUNT={foreground_count}");
    Ok(())
}

fn split_image(basename: &str) -> Result<usize, Box<dyn Error>> {
    let source_path = Path::new(LEVEL_DIR).join(format!("{basename}.png"));
    println!("cargo:rerun-if-changed={}", source_path.display());

    let decoder = png::Decoder::new(File::open(&source_path)?);
    let mut reader = decoder.read_info()?;
    let palette = reader.info().palette.clone();
    let trns = reader.info().trns.clone();
    let mut buffer = vec![0; reader.output_buffer_size()];
    let info = reader.next_frame(&mut buffer)?;
    let data = &buffer[..info.buffer_size()];

    let bytes_per_pixel = info.line_size / info.width as usize;
    let width = info.width as usize;
    let height = info.height as usize;

    let mut offsets = 0;
    let mut chunk_index = 0;
    let mut chunk_count = 0;
    while offsets < width {
        let remaining = width - offsets;
        let chunk_width = remaining.min(MAX_TEXTURE_DIMENSION);
        let mut chunk_data = Vec::with_capacity(chunk_width * height * bytes_per_pixel);

        for row in 0..height {
            let start = row * info.line_size + offsets * bytes_per_pixel;
            let end = start + chunk_width * bytes_per_pixel;
            chunk_data.extend_from_slice(&data[start..end]);
        }

        chunk_index += 1;
        chunk_count += 1;
        let output_path = chunk_path(basename, chunk_index);
        write_png(
            &output_path,
            chunk_width as u32,
            height as u32,
            info.color_type,
            info.bit_depth,
            palette.as_deref(),
            trns.as_deref(),
            &chunk_data,
        )?;
        offsets += chunk_width;
    }

    // Remove any stale chunks beyond the count we just produced.
    let mut stale_index = chunk_count + 1;
    while chunk_path(basename, stale_index).exists() {
        std::fs::remove_file(chunk_path(basename, stale_index))?;
        stale_index += 1;
    }

    Ok(chunk_count)
}

fn chunk_path(basename: &str, index: usize) -> PathBuf {
    Path::new(LEVEL_DIR).join(format!("{basename}_part{index}.png"))
}

fn write_png(
    path: &Path,
    width: u32,
    height: u32,
    color_type: png::ColorType,
    bit_depth: png::BitDepth,
    palette: Option<&[u8]>,
    trns: Option<&[u8]>,
    data: &[u8],
) -> Result<(), Box<dyn Error>> {
    let file = File::create(path)?;
    let mut encoder = png::Encoder::new(file, width, height);
    encoder.set_color(color_type);
    encoder.set_depth(bit_depth);
    if let Some(palette) = palette {
        encoder.set_palette(palette.to_vec());
    }
    if let Some(trns) = trns {
        encoder.set_trns(trns.to_vec());
    }
    let mut writer = encoder.write_header()?;
    writer.write_image_data(data)?;
    Ok(())
}
