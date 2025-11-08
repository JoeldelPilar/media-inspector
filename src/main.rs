use std::path::Path;
use std::fs;
fn main() {
    println!("🎬 Media File Inspector");
    println!("========================");

    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        println!("Usage: cargo run <video-file>");
        println!("Example: cargo run video.mp4");
        return;
    }

    let video_file = &args[1];
    println!("📁 Inspekterar: {}", video_file);

    if !Path::new(video_file).exists() {
        println!("🚨 Error: File does not exist: {}", video_file);
        return;
    }

    println!("✅ File exists");

    let metadata = fs::metadata(video_file).unwrap();
    let file_size = metadata.len();
    let size_in_mb = file_size as f64 / 1_048_576.0;

    println!("📦 Size: {:.2} MB ({} bytes)", size_in_mb, file_size);

        
}
