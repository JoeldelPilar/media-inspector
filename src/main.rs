
use std::path::Path;
use std::fs;

mod models;
mod ffmpeg;

fn main() {
    println!("🎬 Media File Inspector");
    println!("===starting inspection...===");

    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        println!("❌ Usage: cargo run <video-file>");
        println!("   Example: cargo run video.mp4");
        return;
    }

    let video_file = &args[1];
    println!("📁 Inspecting: {}", video_file);

    if !Path::new(video_file).exists() {
        println!("❌ Error: File does not exist: {}", video_file);
        return;
    }

    println!("✅ File exists");

    let metadata = fs::metadata(video_file).unwrap();
    let file_size = metadata.len();
    let size_in_mb = file_size as f64 / 1_048_576.0;

    println!("📦 Size: {:.2} MB ({} bytes)", size_in_mb, file_size);

    println!("\n Fetching video-info...");

    let video_info = match ffmpeg::get_video_info(video_file) {
        Ok(info) => info,
        Err(e) => {
            println!("❌ Error: {}", e);
            return;
        }
    };

    println!("\n🎥 Video Information:");
    println!("==================");

    for stream in &video_info.streams {
        if let Some(codec_type) = &stream.codec_type {
            if codec_type == "video" {
                if let Some(codec) = &stream.codec_name {
                println!("🎬 Codec: {}", codec);
            }
            if let (Some(width), Some(height)) = (stream.width, stream.height) {
                println!("📐 Resolution: {}x{}", width, height);
            }
        }
    }
    }

    if let Some(duration) = &video_info.format.duration {
        let dur_secs: f64 = duration.parse().unwrap_or(0.0);
        let minutes = (dur_secs / 60.0) as u32;
        let seconds = (dur_secs % 60.0) as u32;
        println!("⏱️ Duration: {}m {}s", minutes, seconds);
    }
    
    if let Some(bitrate) = &video_info.format.bit_rate {
        let br: u64 = bitrate.parse().unwrap_or(0);
        let br_mbps = br as f64 / 1_000_000.0;
        println!("📊 Bitrate: {:.2} Mbps", br_mbps);
    }

    println!("\n=== Inspection complete ===");
}
