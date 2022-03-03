use std::env;
use std::fs;
use std::io;
use std::io::BufWriter;
use std::path::PathBuf;
use std::time;

pub fn write_image(pixels: &[u8], height: u32, width: u32) -> Result<PathBuf, io::Error> {
    let file_path = get_output_file();
    let output = file_path.clone();
    let file = fs::File::create(file_path)?;
    let ref mut w = BufWriter::new(file);
    let mut encoder = png::Encoder::new(w, width, height);
    encoder.set_color(png::ColorType::Rgb);
    encoder.set_depth(png::BitDepth::Eight);

    let mut writer = encoder.write_header().unwrap();
    writer.write_image_data(&pixels).unwrap();

    Ok(output)
}

fn get_output_file() -> PathBuf {
    let file_name = format!(
        "{:?}.png",
        time::SystemTime::now()
            .duration_since(time::UNIX_EPOCH)
            .unwrap()
            .as_millis()
    );

    env::current_dir()
        .expect("Couldn't get current directory.")
        .join(file_name)
}
