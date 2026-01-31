use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Write};
use std::path::Path;
use std::process::Command;

use quick_xml::Reader;
use quick_xml::events::Event;

/// Lit un fichier NFO et retourne un HashMap (API quick-xml fixée)
pub fn lire_nfo(nfo_path: &Path) -> Result<HashMap<String, String>, String> {
    let file = File::open(nfo_path).map_err(|e| e.to_string())?;
    let mut reader = Reader::from_reader(BufReader::new(file));
    let mut buf = Vec::new();
    let mut data: HashMap<String, String> = HashMap::new();
    let mut current_tag = String::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(e)) | Ok(Event::Empty(e)) => {
                current_tag = String::from_utf8_lossy(e.name().as_ref()).to_string();
            }
            Ok(Event::Text(e)) => {
                let text = e.decode().unwrap_or_default().trim().to_string();
                if !current_tag.is_empty() && !text.is_empty() {
                    data.insert(current_tag.clone(), text);
                }
            }
            Ok(Event::End(_)) => {
                current_tag.clear();
            }
            Ok(Event::Eof) => break,
            Err(e) => return Err(format!("Erreur lecture NFO: {}", e)),
            _ => {}
        }
        buf.clear();
    }
    Ok(data)
}

/// 1. Marquer une vidéo comme 'VU' (playcount +1, watched=true, commentaire=VU)
pub fn marquer_vu(mkv_path: &Path, nfo_path: &Path) -> Result<(), String> {
    let mut data = lire_nfo(nfo_path)?;
    let pc = data
        .get("playcount")
        .and_then(|v| v.parse::<u32>().ok())
        .unwrap_or(0)
        + 1;
    data.insert("playcount".to_string(), pc.to_string());
    data.insert("watched".to_string(), "true".to_string());

    let mut file = File::create(nfo_path).map_err(|e| e.to_string())?;
    writeln!(file, "<movie>").map_err(|e| e.to_string())?;
    for (k, v) in &data {
        writeln!(file, "  <{0}>{1}</{0}>", k, v).map_err(|e| e.to_string())?;
    }
    writeln!(file, "</movie>").map_err(|e| e.to_string())?;

    let status = Command::new("mkvpropedit")
        .args([
            mkv_path.to_string_lossy().to_string(),
            "--edit".to_string(),
            "info".to_string(),
            "--set".to_string(),
            format!("playcount={}", pc),
            "--set".to_string(),
            "comment=VU".to_string(),
        ])
        .status()
        .map_err(|e| e.to_string())?;

    if status.success() {
        Ok(())
    } else {
        Err("Erreur marquage VU".into())
    }
}

/// 2. Gestion des tags vidéo (Interactif) via mkvtoolnix
pub fn modifier_tag(mkv_path: &Path, tag: &str, valeur: &str) -> Result<(), String> {
    let status = Command::new("mkvpropedit")
        .args([
            mkv_path.to_string_lossy().to_string(),
            "--edit".to_string(),
            "info".to_string(),
            "--set".to_string(),
            format!("{}={}", tag, valeur),
        ])
        .status()
        .map_err(|e| e.to_string())?;
    if status.success() {
        Ok(())
    } else {
        Err("Erreur modification tag".into())
    }
}

/// 3. Injection de tags dans une vidéo depuis un NFO
pub fn appliquer_tags(mkv_path: &Path, nfo_path: &Path) -> Result<(), String> {
        let tags = lire_nfo(nfo_path)?;
    let mut args = vec![
        mkv_path.to_string_lossy().to_string(),
        "--edit".to_string(),
        "info".to_string(),
    ];
    for (k, v) in tags.iter() {
        args.push("--set".to_string());
        args.push(format!("{}={}", k, v));
    }
    let status = Command::new("mkvpropedit")
        .args(&args)
        .status()
        .map_err(|e| e.to_string())?;
    if status.success() {
        Ok(())
    } else {
        Err("Erreur mkvpropedit".into())
    }
}

/// 4. Injection de poster + ClearLogo + FanArt
pub fn ajouter_images_mkv(mkv_path: &Path) -> Result<(), String> {
    let parent = mkv_path.parent().ok_or("Dossier parent introuvable")?;
    let mut command = Command::new("mkvpropedit");
    command.arg(mkv_path);
    let cibles = ["poster", "clearlogo", "fanart"];
    let mut found = false;
    for entry in std::fs::read_dir(parent).map_err(|e| e.to_string())? {
        let path = entry.map_err(|e| e.to_string())?.path();
        if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
            if cibles.contains(&stem.to_lowercase().as_str()) {
                command.args([
                    "--attachment-name".to_string(),
                    path.file_name().unwrap().to_string_lossy().to_string(),
                    "--add-attachment".to_string(),
                    path.to_string_lossy().to_string(),
                ]);
                found = true;
            }
        }
    }
    if !found {
        return Ok(());
    }
    let status = command.status().map_err(|e| e.to_string())?;
    if status.success() {
        Ok(())
    } else {
        Err("Erreur ajout images".into())
    }
}

/// 5. Supprimer TOUS les tags d'une vidéo
pub fn supprimer_tous_tags(mkv_path: &Path) -> Result<(), String> {
    let status = Command::new("mkvpropedit")
        .args([
            mkv_path.to_string_lossy().to_string(),
            "--tags".to_string(),
            "all:delete".to_string(),
            "--delete-attachment".to_string(),
            "mime-type:image/jpeg".to_string(),
            "--delete-attachment".to_string(),
            "mime-type:image/png".to_string(),
        ])
        .status()
        .map_err(|e| e.to_string())?;
    if status.success() {
        Ok(())
    } else {
        Err("Erreur suppression tags".into())
    }
}