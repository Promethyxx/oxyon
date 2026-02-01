#![allow(dead_code)]
use std::path::Path;
use std::process::Command;
use crate::modules::binaries;

/// Convertit n'importe quoi vers n'importe quoi (ex: .docx -> .md)
pub fn convertir(input: &Path, output: &str) -> bool {
    let status = Command::new(binaries::get_pandoc())
        .arg(input.to_str().unwrap())
        .arg("-o")
        .arg(output)
        .status();

    status.map(|s| s.success()).unwrap_or(false)
}

/// Extrait uniquement le texte brut (utile pour nettoyer un fichier)
pub fn extraire_texte(input: &Path, output: &str) -> bool {
    let status = Command::new(binaries::get_pandoc())
        .arg(input.to_str().unwrap())
        .arg("-t")
        .arg("plain")
        .arg("-o")
        .arg(output)
        .status();

    status.map(|s| s.success()).unwrap_or(false)
}
