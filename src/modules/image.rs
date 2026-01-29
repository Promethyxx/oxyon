#![allow(dead_code)]
use image::imageops::FilterType;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

/// Redimensionne l'image (Compression par dimension)
pub fn compresser(input: &Path, output: &str, ratio: u32) -> bool {
    if let Ok(img) = image::open(input) {
        let (w, h) = (img.width(), img.height());
        // Ratio de 2 = divise par 2 la largeur et hauteur
        let scaled = img.resize(w / ratio, h / ratio, FilterType::Lanczos3);
        scaled.save(output).is_ok()
    } else {
        false
    }
}

/// Conversion de format (ex: PNG -> JPG, WEBP -> PNG)
pub fn convertir(input: &Path, output: &str) -> bool {
    match image::open(input) {
        Ok(img) => img.save(output).is_ok(),
        Err(_) => false,
    }
}

/// Lecture des métadonnées EXIF (GPS, Appareil, Date)
pub fn lire_exif(input: &Path) -> Vec<String> {
    let mut resultats = Vec::new();
    if let Ok(file) = File::open(input) {
        let mut reader = BufReader::new(file);
        let exifreader = exif::Reader::new();
        if let Ok(exdata) = exifreader.read_from_container(&mut reader) {
            for f in exdata.fields() {
                resultats.push(format!(
                    "{}: {}",
                    f.tag,
                    f.display_value().with_unit(&exdata)
                ));
            }
        }
    }
    resultats
}

/// Supprime l'EXIF en réenregistrant les pixels nus
pub fn supprimer_exif(input: &Path, output: &str) -> bool {
    if let Ok(img) = image::open(input) {
        // Enregistre uniquement les données de pixels, ignore les métadonnées sources
        img.save(output).is_ok()
    } else {
        false
    }
}

/// Rotation simple (90, 180, 270)
pub fn pivoter(input: &Path, output: &str, angle: u32) -> bool {
    if let Ok(img) = image::open(input) {
        let rotated = match angle {
            90 => img.rotate90(),
            180 => img.rotate180(),
            270 => img.rotate270(),
            _ => img,
        };
        rotated.save(output).is_ok()
    } else {
        false
    }
}
