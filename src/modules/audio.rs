#![allow(dead_code)]
use std::path::Path;
use crate::modules::binaries;

/// Conversion & Compression
/// bitrate : ex "128k", "192k", "320k"
pub fn convertir(input: &Path, output: &str, bitrate: &str) -> std::io::Result<std::process::Child> {
    let ffmpeg = binaries::get_ffmpeg();
    crate::log_info(&format!("audio::convertir | ffmpeg={:?} | bitrate={} | {:?} -> {}", ffmpeg, bitrate, input, output));
    let child = binaries::silent_cmd(ffmpeg)
        .arg("-i")
        .arg(input)
        .arg("-b:a")
        .arg(bitrate)
        .arg(output)
        .arg("-y")
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn();
    if let Err(ref e) = child {
        crate::log_error(&format!("audio::convertir impossible de lancer ffmpeg : {}", e));
    }
    child
}

/// Détecter le codec via ffprobe
pub fn detecter_extension(input: &Path) -> String {
    let result = binaries::silent_cmd(binaries::get_ffprobe())
        .args([
            "-v", "error",
            "-select_streams", "a:0",
            "-show_entries", "stream=codec_name",
            "-of", "default=noprint_wrappers=1:nokey=1",
        ])
        .arg(input)
        .output();

    let codec = match result {
        Ok(output) => String::from_utf8_lossy(&output.stdout).trim().to_string(),
        Err(e) => {
            crate::log_warn(&format!("audio::detecter_extension ffprobe échoué : {}", e));
            return String::new();
        }
    };

    match codec.as_str() {
        "vorbis" => "ogg".into(),
        "opus" => "opus".into(),
        _ => codec,
    }
}

/// Retourne les formats de sortie compatibles selon le codec source
pub fn formats_compatibles(codec_source: &str) -> Vec<&'static str> {
    // ffmpeg gère la quasi-totalité des conversions, mais on filtre
    // les cas problématiques connus
    match codec_source {
        "pcm_s16le" | "pcm_s24le" | "pcm_s32le" | "pcm_f32le" | "flac" | "alac" =>
            vec!["mp3", "aac", "flac", "ogg", "opus", "wav", "m4a"],
        "vorbis" =>
            vec!["mp3", "aac", "flac", "ogg", "opus", "wav", "m4a"],
        "opus" =>
            vec!["mp3", "aac", "flac", "ogg", "opus", "wav", "m4a"],
        "aac" =>
            vec!["mp3", "aac", "flac", "ogg", "opus", "wav", "m4a"],
        "mp3" =>
            vec!["mp3", "aac", "flac", "ogg", "opus", "wav", "m4a"],
        // Codec inconnu ou vide → tout proposer, ffmpeg tentera
        _ =>
            vec!["mp3", "aac", "flac", "ogg", "opus", "wav", "m4a"],
    }
}

/// Extraction : Récupère l'audio d'une vidéo
pub fn extraire(input: &Path, output: &str) -> std::io::Result<std::process::Child> {
    crate::log_info(&format!("audio::extraire | {:?} -> {}", input, output));
    let child = binaries::silent_cmd(binaries::get_ffmpeg())
        .arg("-i")
        .arg(input)
        .arg("-vn")
        .arg("-c:a")
        .arg("copy")
        .arg("-y")
        .arg(output)
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn();
    if let Err(ref e) = child {
        crate::log_error(&format!("audio::extraire impossible de lancer ffmpeg : {}", e));
    }
    child
}