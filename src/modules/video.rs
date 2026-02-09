#![allow(dead_code)]
use std::path::PathBuf;
use std::process::Child;
use crate::modules::binaries;

/// Convertit ou change le conteneur d'une vidéo
pub fn traiter_video(
    input: &PathBuf,
    output: &str,
    copie_flux: bool,
    est_audio_uniquement: bool,
) -> Result<Child, std::io::Error> {
    let mut args = vec!["-i", input.to_str().unwrap()];

    if copie_flux {
        if est_audio_uniquement {
            args.extend(vec!["-vn", "-c:a", "copy"]);
        } else {
            args.extend(vec!["-c", "copy"]);
        }
    } else if est_audio_uniquement {
        args.push("-vn");
    }

    args.extend(vec!["-y", output]);

    binaries::silent_cmd(binaries::get_ffmpeg()).args(&args).spawn()
}

/// Analyse le codec audio d'un fichier via ffprobe
pub fn extraire_nom_codec(input: &PathBuf) -> String {
    let out = binaries::silent_cmd(binaries::get_ffprobe())
        .args(&[
            "-v",
            "error",
            "-select_streams",
            "a:0",
            "-show_entries",
            "stream=codec_name",
            "-of",
            "default=noprint_wrappers=1:nokey=1",
            input.to_str().unwrap(),
        ])
        .output();

    if let Ok(o) = out {
        String::from_utf8_lossy(&o.stdout).trim().to_string()
    } else {
        String::new()
    }
}