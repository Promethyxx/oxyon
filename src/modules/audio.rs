use std::path::Path;
use std::process::Command;

/// Conversion & Compression
/// bitrate : ex "128k", "192k", "320k"
pub fn convertir(input: &Path, output: &str, bitrate: &str) -> std::io::Result<std::process::Child> {
    std::process::Command::new("ffmpeg")
        .arg("-i")
        .arg(input)
        .arg("-b:a")
        .arg(bitrate)
        .arg(output)
        .arg("-y")
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .spawn()
}

///Détecter le codec
pub fn detecter_extension(input: &Path) -> String {
    let output = std::process::Command::new("ffprobe")
        .args([
            "-v", "error",
            "-select_streams", "a:0",
            "-show_entries", "stream=codec_name",
            "-of", "default=noprint_wrappers=1:nokey=1",
        ])
        .arg(input)
        .output()
        .expect("Échec ffprobe");

    let codec = String::from_utf8_lossy(&output.stdout).trim().to_string();
    
    // Correction pour les extensions communes
    match codec.as_str() {
        "vorbis" => "ogg".into(),
        "opus" => "opus".into(),
        _ => codec,
    }
}

/// Extraction : Récupère l'audio d'une vidéo
pub fn extraire(input: &Path, output: &str) -> std::io::Result<std::process::Child> {
    std::process::Command::new("ffmpeg")
        .arg("-i")
        .arg(input)
        .arg("-vn")
        .arg("-c:a") // Plus moderne que -acodec
        .arg("copy")
        .arg("-y")
        .arg(output)
        .spawn() 
}