use indicatif::ProgressIterator;
use std::env;
use std::fs;
use std::io;
use std::io::Write;
use std::time;

use crate::color::Color;

pub fn write_image(width: i32, height: i32) -> Result<(), io::Error> {
    let file_name = format!(
        "{:?}.ppm",
        time::SystemTime::now()
            .duration_since(time::UNIX_EPOCH)
            .unwrap()
            .as_millis()
    );

    let mut file_path = env::current_dir().expect("Couldn't get current directory.");
    file_path.push(file_name);
    println!("Writing file {}", file_path.display());

    let file = fs::File::create(file_path)?;
    let mut file = io::LineWriter::new(file);

    file.write(format!("P3\n{} {}\n255\n", width, height).as_bytes())?;
    for h in (0..height).progress() {
        for w in 0..width {
            let (r, g, b) = Color::new(
                (w as f64) / (width as f64),
                (h as f64) / (height as f64),
                0.25,
            )
            .to_rgb();

            file.write(format!("{} {} {}\n", r, g, b).as_bytes())?;
        }
    }

    Ok(())
}
