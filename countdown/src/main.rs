use ab_glyph::{FontRef, PxScale};
use image::{Rgb, RgbImage};
use imageproc::drawing::draw_text_mut;
use std::env;
use std::path::Path;
use std::process::Command;
use rayon::prelude::*;
use tempfile::tempdir;

fn create_frame(text :&str, frame_number:usize, temp_path:&Path) {
    let width = 512;
    let height = 256;
    let mut image = RgbImage::new(width, height);

    let font = FontRef::try_from_slice(include_bytes!("DejaVuSans.ttf")).unwrap();
    let scale = PxScale { x: 200.0, y: 200.0 }; // Larger text scale
    let text_width = (text.len() as f32 * scale.x) / 2.0;
    let x_pos = (width as f32 / 2.0 - text_width / 2.0) as i32;
    let y_pos = (height as f32 / 2.0 - scale.y / 2.0) as i32;
    let color = Rgb([255, 255, 255]); // Black digits
    
    draw_text_mut(&mut image,color, x_pos, y_pos, scale, &font, text);
    let filename = temp_path.join(Path::new(format!("frame_{:04}.png",frame_number).as_str()));
    image.save(filename).unwrap();
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let total_seconds: usize = if args.len() > 1 {
        args[1].parse().unwrap_or(60)
    } else {
        60
    };

    let temp_dir = tempdir().expect("Failed to create temporary directory");
    let temp_path = temp_dir.path().to_path_buf();
    
    println!("generate {} seconds",total_seconds);
    (0..total_seconds).into_par_iter().for_each(|i| {
        let minutes = (total_seconds - i) / 60;
        let seconds = (total_seconds - i) % 60;
        let text = format!("{:02}:{:02}", minutes, seconds);
        create_frame(&text, i, &temp_path);
    });

    
    println!("combine {} frames",total_seconds);
    let input_pattern = temp_path.join("frame_%04d.png");
    Command::new("ffmpeg")
        .args(["-framerate", "1", "-i", input_pattern.to_str().unwrap(), "-c:v", "libx264", "-pix_fmt", "yuv420p", "countdown.mp4"])
        .status()
        .expect("Failed to generate video with ffmpeg");
    println!("Countdown video created: countdown.mp4");
    
}
