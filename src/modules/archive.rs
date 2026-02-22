#![allow(dead_code)]
use std::fs;
use std::io::{Write, Seek};
use std::path::Path;

// ════════════════════════════════════════════════════════════════════════
//  ZIP — via crate `zip`
// ════════════════════════════════════════════════════════════════════════

fn compresser_zip(input: &Path, output: &str) -> Result<(), String> {
    let file = fs::File::create(output)
        .map_err(|e| format!("Erreur creation zip : {}", e))?;
    let mut zip_writer = zip::ZipWriter::new(file);
    let options = zip::write::SimpleFileOptions::default()
        .compression_method(zip::CompressionMethod::Deflated);

    if input.is_dir() {
        ajouter_dossier_zip(&mut zip_writer, input, input, options)?;
    } else {
        let name = input.file_name().unwrap_or_default().to_string_lossy().to_string();
        zip_writer.start_file(&name, options)
            .map_err(|e| format!("Erreur zip start_file : {}", e))?;
        let data = fs::read(input).map_err(|e| format!("Erreur lecture : {}", e))?;
        zip_writer.write_all(&data).map_err(|e| format!("Erreur ecriture zip : {}", e))?;
    }

    zip_writer.finish().map_err(|e| format!("Erreur finalisation zip : {}", e))?;
    Ok(())
}

fn ajouter_dossier_zip<W: Write + Seek>(
    zip_writer: &mut zip::ZipWriter<W>,
    base: &Path,
    current: &Path,
    options: zip::write::SimpleFileOptions,
) -> Result<(), String> {
    for entry in fs::read_dir(current).map_err(|e| format!("Erreur lecture dossier : {}", e))? {
        let entry = entry.map_err(|e| format!("Erreur entree : {}", e))?;
        let path = entry.path();
        let relative = path.strip_prefix(base).unwrap_or(&path);
        let name = relative.to_string_lossy().replace('\\', "/");

        if path.is_dir() {
            zip_writer.add_directory(&format!("{}/", name), options)
                .map_err(|e| format!("Erreur zip add_directory : {}", e))?;
            ajouter_dossier_zip(zip_writer, base, &path, options)?;
        } else {
            zip_writer.start_file(&name, options)
                .map_err(|e| format!("Erreur zip start_file : {}", e))?;
            let data = fs::read(&path).map_err(|e| format!("Erreur lecture : {}", e))?;
            zip_writer.write_all(&data).map_err(|e| format!("Erreur ecriture : {}", e))?;
        }
    }
    Ok(())
}

fn extraire_zip(input: &Path, destination: &str) -> Result<(), String> {
    let file = fs::File::open(input).map_err(|e| format!("Erreur ouverture zip : {}", e))?;
    let mut archive = zip::ZipArchive::new(file)
        .map_err(|e| format!("Erreur lecture zip : {}", e))?;
    archive.extract(destination)
        .map_err(|e| format!("Erreur extraction zip : {}", e))?;
    Ok(())
}

// ════════════════════════════════════════════════════════════════════════
//  7Z — via crate `sevenz-rust2`
// ════════════════════════════════════════════════════════════════════════

fn compresser_7z(input: &Path, output: &str) -> Result<(), String> {
    sevenz_rust2::compress_to_path(input, output)
        .map_err(|e| format!("Erreur compression 7z : {}", e))
}

fn extraire_7z(input: &Path, destination: &str) -> Result<(), String> {
    sevenz_rust2::decompress_file(input, destination)
        .map_err(|e| format!("Erreur extraction 7z : {}", e))
}

// ════════════════════════════════════════════════════════════════════════
//  TAR.GZ — via crates `tar` + `flate2`
// ════════════════════════════════════════════════════════════════════════

fn compresser_tar(input: &Path, output: &str) -> Result<(), String> {
    let file = fs::File::create(output)
        .map_err(|e| format!("Erreur creation tar.gz : {}", e))?;
    let encoder = flate2::write::GzEncoder::new(file, flate2::Compression::default());
    let mut archive = tar::Builder::new(encoder);

    if input.is_dir() {
        let dir_name = input.file_name().unwrap_or_default().to_string_lossy().to_string();
        archive.append_dir_all(&dir_name, input)
            .map_err(|e| format!("Erreur ajout dossier tar : {}", e))?;
    } else {
        let name = input.file_name().unwrap_or_default().to_string_lossy().to_string();
        let mut f = fs::File::open(input).map_err(|e| format!("Erreur lecture : {}", e))?;
        archive.append_file(&name, &mut f)
            .map_err(|e| format!("Erreur ajout fichier tar : {}", e))?;
    }

    archive.finish().map_err(|e| format!("Erreur finalisation tar : {}", e))?;
    Ok(())
}

fn extraire_tar(input: &Path, destination: &str) -> Result<(), String> {
    let file = fs::File::open(input).map_err(|e| format!("Erreur ouverture tar.gz : {}", e))?;
    let decoder = flate2::read::GzDecoder::new(file);
    let mut archive = tar::Archive::new(decoder);
    archive.unpack(destination)
        .map_err(|e| format!("Erreur extraction tar : {}", e))?;
    Ok(())
}

// ════════════════════════════════════════════════════════════════════════
//  API PUBLIQUE — mêmes signatures que l'ancien module
// ════════════════════════════════════════════════════════════════════════

/// Compression : cree une archive (7z, zip, tar)
pub fn compresser(input: &Path, output: &str, format_archive: &str) -> bool {
    crate::log_info(&format!(
        "archive::compresser | format={} | {:?} -> {} | est_dossier={}",
        format_archive, input, output, input.is_dir()
    ));
    if input.is_dir() {
        // Compter les fichiers pour anticiper la durée
        let nb = std::fs::read_dir(input).map(|d| d.count()).unwrap_or(0);
        crate::log_info(&format!("archive::compresser | dossier avec ~{} entrées", nb));
    } else {
        let taille = std::fs::metadata(input).map(|m| m.len()).unwrap_or(0);
        crate::log_info(&format!("archive::compresser | fichier taille={} octets ({:.1} Mo)", taille, taille as f64 / 1_048_576.0));
    }
    let result = match format_archive.to_lowercase().as_str() {
        "zip" => compresser_zip(input, output),
        "7z"  => compresser_7z(input, output),
        "tar" | "tar.gz" | "tgz" => compresser_tar(input, output),
        _ => Err(format!("Format d'archive non supporte : {}", format_archive)),
    };
    match &result {
        Ok(()) => crate::log_info(&format!("archive::compresser OK | {}", output)),
        Err(e) => crate::log_error(&format!("archive::compresser ÉCHEC | format={} | {:?} | raison={}", format_archive, input, e)),
    }
    result.is_ok()
}

/// Extraction : decompresse une archive
pub fn extraire(input: &Path, destination: &str) -> bool {
    let ext = input.extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_lowercase();
    let taille = std::fs::metadata(input).map(|m| m.len()).unwrap_or(0);
    crate::log_info(&format!(
        "archive::extraire | ext={} | taille={:.1} Mo | {:?} -> {}",
        ext, taille as f64 / 1_048_576.0, input, destination
    ));

    let result = match ext.as_str() {
        "zip" => extraire_zip(input, destination),
        "7z"  => extraire_7z(input, destination),
        "gz" | "tgz" => extraire_tar(input, destination),
        "tar" => {
            // tar non compresse
            let file = fs::File::open(input).map_err(|e| format!("{}", e));
            match file {
                Ok(f) => {
                    let mut archive = tar::Archive::new(f);
                    archive.unpack(destination)
                        .map_err(|e| format!("Erreur extraction tar : {}", e))
                }
                Err(e) => Err(e),
            }
        }
        _ => Err(format!("Format d'archive non reconnu : .{}", ext)),
    };
    if let Err(e) = &result {
        crate::log_error(&format!("archive::extraire ÉCHEC | ext={} | {:?} | raison={}", ext, input, e));
    } else {
        crate::log_info(&format!("archive::extraire OK | {:?}", input));
    }
    result.is_ok()
}

/// Conversion : Change le format via dossier temporaire
pub fn convertir(input: &Path, format_cible: &str) -> bool {
    let temp_dir = std::env::temp_dir().join("oxyon_archive_conv");
    let temp_str = temp_dir.to_string_lossy().to_string();

    // 1. Extraire
    if !extraire(input, &temp_str) {
        return false;
    }
    // 2. Recompresser
    let nom_base = input.file_stem().unwrap_or_default().to_string_lossy();
    let sortie = format!("{}.{}", nom_base, format_cible);
    let success = compresser(&temp_dir, &sortie, format_cible);
    // 3. Nettoyer
    let _ = fs::remove_dir_all(&temp_dir);
    success
}