 #![allow(dead_code)]
use std::fs;
use std::path::Path;
use std::process::Command;

use crate::modules::binaries;

/// Compression : Crée une archive (7z, zip, tar)
pub fn compresser(input: &Path, output: &str, format_archive: &str) -> bool {
    let t_arg = format!("-t{}", format_archive); // ex: -t7z ou -tzip
    let status = Command::new(binaries::get_7z()) // ✅ Utilise le binaire embarqué
        .arg("a") // "a" pour Add
        .arg(t_arg)
        .arg(output)
        .arg(input.to_str().unwrap())
        .arg("-y") // Réponse "Oui" automatique
        .status();
    status.map(|s| s.success()).unwrap_or(false)
}

/// Extraction : Décompresse une archive
pub fn extraire(input: &Path, destination: &str) -> bool {
    let o_arg = format!("-o{}", destination); // -oPath
    let status = Command::new(binaries::get_7z()) // ✅ Utilise le binaire embarqué
        .arg("x") // "x" pour eXtract avec dossiers
        .arg(input.to_str().unwrap())
        .arg(o_arg)
        .arg("-y")
        .status();
    status.map(|s| s.success()).unwrap_or(false)
}

/// Division : Coupe une archive en volumes (ex: "10m" pour 10Mo)
pub fn diviser(input: &Path, output: &str, taille_volume: &str) -> bool {
    let v_arg = format!("-v{}", taille_volume);
    let status = Command::new(binaries::get_7z()) // ✅ Utilise le binaire embarqué
        .arg("a")
        .arg(v_arg)
        .arg(output)
        .arg(input.to_str().unwrap())
        .arg("-y")
        .status();
    status.map(|s| s.success()).unwrap_or(false)
}

/// Fusion : Reconstruit une archive à partir du premier volume (.001)
pub fn fusionner(premier_volume: &Path, destination: &str) -> bool {
    // Sous Windows/7z, extraire le .001 fusionne automatiquement
    extraire(premier_volume, destination)
}

/// Conversion : Change le format (ex: 7z vers zip) via un dossier temporaire
pub fn convertir(input: &Path, format_cible: &str) -> bool {
    let temp_dir = "temp_oxyon_conv";
    // 1. Extraire l'original
    if !extraire(input, temp_dir) {
        return false;
    }
    // 2. Créer la nouvelle archive depuis le dossier temp
    let nom_base = input.file_stem().unwrap().to_str().unwrap();
    let sortie = format!("{}.{}", nom_base, format_cible);
    let success = compresser(Path::new(temp_dir), &sortie, format_cible);
    // 3. Nettoyer le dossier temporaire
    let _ = fs::remove_dir_all(temp_dir);
    success
}

/// Édition : Supprime un fichier à l'intérieur d'une archive existante
pub fn supprimer_fichier(archive: &Path, fichier_a_supprimer: &str) -> bool {
    let status = Command::new(binaries::get_7z()) // ✅ Utilise le binaire embarqué
        .arg("d") // "d" pour Delete
        .arg(archive.to_str().unwrap())
        .arg(fichier_a_supprimer)
        .arg("-y")
        .status();
    status.map(|s| s.success()).unwrap_or(false)
}

/// Commentaire : Non supporté par le binaire 7z.exe (Windows)
pub fn ajouter_commentaire(_archive: &Path, _commentaire: &str) -> bool {
    // Cette fonction reste pour respecter ton cahier de fonctions,
    // mais 7z.exe ne possède pas d'argument simple pour injecter un texte.
    false
}