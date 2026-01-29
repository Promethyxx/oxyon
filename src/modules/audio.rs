use std::path::Path;
use std::process::Command;

/// Conversion & Compression
/// bitrate : ex "128k", "192k", "320k"
pub fn convertir(input: &Path, output: &str, bitrate: &str) -> bool {
    let status = Command::new("ffmpeg")
        .arg("-i")
        .arg(input.to_str().unwrap())
        .arg("-b:a")
        .arg(bitrate)
        .arg(output)
        .arg("-y")
        .status();

    status.map(|s| s.success()).unwrap_or(false)
}

/// Division : Coupe un extrait (Format "00:00:00")
pub fn diviser(input: &Path, output: &str, debut: &str, duree: &str) -> bool {
    let status = Command::new("ffmpeg")
        .arg("-ss")
        .arg(debut)
        .arg("-t")
        .arg(duree)
        .arg("-i")
        .arg(input.to_str().unwrap())
        .arg("-acodec")
        .arg("copy")
        .arg(output)
        .arg("-y")
        .status();

    status.map(|s| s.success()).unwrap_or(false)
}

/// Extraction : Récupère l'audio d'une vidéo
pub fn extraire(input: &Path, output: &str) -> bool {
    let status = Command::new("ffmpeg")
        .arg("-i")
        .arg(input.to_str().unwrap())
        .arg("-vn")
        .arg("-acodec")
        .arg("copy")
        .arg(output)
        .arg("-y")
        .status();

    status.map(|s| s.success()).unwrap_or(false)
}

/// Fusion : Combine plusieurs fichiers via un fichier liste.txt
pub fn fusionner(liste_txt: &str, output: &str) -> bool {
    let status = Command::new("ffmpeg")
        .arg("-f")
        .arg("concat")
        .arg("-safe")
        .arg("0")
        .arg("-i")
        .arg(liste_txt)
        .arg("-c")
        .arg("copy")
        .arg(output)
        .arg("-y")
        .status();

    status.map(|s| s.success()).unwrap_or(false)
}

/// Tag : Ajoute une pochette (Poster)
pub fn ajouter_poster(musique: &Path, image: &Path, output: &str) -> bool {
    let status = Command::new("ffmpeg")
        .arg("-i")
        .arg(musique.to_str().unwrap())
        .arg("-i")
        .arg(image.to_str().unwrap())
        .arg("-map")
        .arg("0:0")
        .arg("-map")
        .arg("1:0")
        .arg("-c")
        .arg("copy")
        .arg("-id3v2_version")
        .arg("3")
        .arg(output)
        .arg("-y")
        .status();

    status.map(|s| s.success()).unwrap_or(false)
}

/// Tag : Modifie Titre, Artiste, Album
pub fn modifier_tags(input: &Path, output: &str, artiste: &str, titre: &str, album: &str) -> bool {
    let status = Command::new("ffmpeg")
        .arg("-i")
        .arg(input.to_str().unwrap())
        .arg("-metadata")
        .arg(format!("artist={}", artiste))
        .arg("-metadata")
        .arg(format!("title={}", titre))
        .arg("-metadata")
        .arg(format!("album={}", album))
        .arg("-c")
        .arg("copy")
        .arg(output)
        .arg("-y")
        .status();

    status.map(|s| s.success()).unwrap_or(false)
}

/// Tag : Supprime toutes les métadonnées
pub fn supprimer_tags(input: &Path, output: &str) -> bool {
    let status = Command::new("ffmpeg")
        .arg("-i")
        .arg(input.to_str().unwrap())
        .arg("-map_metadata")
        .arg("-1")
        .arg("-c")
        .arg("copy")
        .arg(output)
        .arg("-y")
        .status();

    status.map(|s| s.success()).unwrap_or(false)
}

/// Réparation : Tente de reconstruire un flux corrompu
pub fn reparer(input: &Path, output: &str) -> bool {
    let status = Command::new("ffmpeg")
        .arg("-i")
        .arg(input.to_str().unwrap())
        .arg("-map")
        .arg("0")
        .arg("-c")
        .arg("copy")
        .arg(output)
        .arg("-y")
        .status();

    status.map(|s| s.success()).unwrap_or(false)
}
