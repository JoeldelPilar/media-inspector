use std::process::Command;
use crate::models::VideoInfo;

pub fn get_video_info(filepath: &str) -> Result<VideoInfo, String> {
    let output = Command::new("ffprobe")
        .arg("-v")
        .arg("quiet")
        .arg("-print_format")
        .arg("json")
        .arg("-show_format")
        .arg("-show_streams")
        .arg(filepath)
        .output()
        .map_err(|e| format!("Failed to execute ffprobe: {}", e))?;
    
    if !output.status.success() {
        let error = String::from_utf8_lossy(&output.stderr);
        return Err(format!("FFprobe error: {}", error));
    }
    
    let json_output = String::from_utf8(output.stdout)
        .map_err(|e| format!("Invalid UTF-8: {}", e))?;
    
    if json_output.is_empty() {
        return Err("No output from FFprobe".to_string());
    }
    
    serde_json::from_str(&json_output)
        .map_err(|e| format!("JSON parse error: {}", e))
}