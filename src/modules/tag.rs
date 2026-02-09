use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Write};
use std::path::Path;
use quick_xml::Reader;
use quick_xml::events::Event;
use crate::modules::binaries;

/// Lit un fichier NFO et retourne un HashMap
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

/// Crée le fichier XML temporaire pour mkvpropedit
fn creer_xml_tags(tags: &HashMap<String, String>) -> String {
    let mut xml = String::from("<?xml version=\"1.0\"?>\n<Tags>\n  <Tag>\n    <Targets><TargetTypeValue>50</TargetTypeValue></Targets>\n");
    for (cle, valeur) in tags {
        if !valeur.is_empty() {
            xml.push_str("    <Simple>\n");
            xml.push_str(&format!("      <Name>{}</Name>\n", cle.to_uppercase()));
            xml.push_str(&format!("      <String>{}</String>\n", valeur.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;")));
            xml.push_str("    </Simple>\n");
        }
    }
    xml.push_str("  </Tag>\n</Tags>");
    xml
}

/// 1. Marquer une vidéo comme 'VU'
pub fn marquer_vu(mkv_path: &Path, nfo_path: &Path) -> Result<(), String> {
    let mut data = lire_nfo(nfo_path)?;

    let pc = data.get("playcount").and_then(|v| v.parse::<u32>().ok()).unwrap_or(0) + 1;

    data.insert("playcount".to_string(), pc.to_string());
    data.insert("watched".to_string(), "true".to_string());
    data.insert("KEYWORDS".to_string(), "WATCHED".to_string());

    let mut file = File::create(nfo_path).map_err(|e| e.to_string())?;
    writeln!(file, "<movie>").map_err(|e| e.to_string())?;
    for (k, v) in &data {
        writeln!(file, "  <{0}>{1}</{0}>", k, v).map_err(|e| e.to_string())?;
    }
    writeln!(file, "</movie>").map_err(|e| e.to_string())?;

    let xml_content = creer_xml_tags(&data);
    let temp_xml = "temp_vu.xml";
    std::fs::write(temp_xml, xml_content).map_err(|e| e.to_string())?;

    let status = binaries::silent_cmd(binaries::get_mkvpropedit())
        .args([mkv_path.to_str().unwrap(), "--tags", &format!("global:{}", temp_xml)])
        .status().map_err(|e| e.to_string())?;

    let _ = std::fs::remove_file(temp_xml);
    if status.success() { Ok(()) } else { Err("Erreur marquage VU complet".into()) }
}

/// 2. Modification directe
pub fn modifier_tag(mkv_path: &Path, tag: &str, valeur: &str) -> Result<(), String> {
    let status = binaries::silent_cmd(binaries::get_mkvpropedit())
        .args([
            mkv_path.to_str().unwrap(),
            "--edit", "info",
            "--set", &format!("{}={}", tag, valeur),
        ])
        .status().map_err(|e| e.to_string())?;
    if status.success() { Ok(()) } else { Err("Erreur modification tag".into()) }
}

/// 3. Injection complète depuis NFO
pub fn appliquer_tags(mkv_path: &Path, nfo_path: &Path) -> Result<(), String> {
    let mut tags = lire_nfo(nfo_path)?;

    // On supprime le statut de lecture pour ne pas l'écraser
    tags.remove("playcount");
    tags.remove("watched");
    tags.remove("watchedstatus");
    tags.remove("KEYWORDS"); // Statut VU géré séparément

    // Rating dans COMMENT
    if let Some(rating) = tags.get("value").cloned() {
        if let Ok(r) = rating.parse::<f64>() {
            let rounded = (r * 10.0).round() / 10.0;
            tags.insert("COMMENT".to_string(), format!("{} / 10", rounded));
        }
    }

    let xml_content = creer_xml_tags(&tags);
    let temp_xml = "temp_meta.xml";
    std::fs::write(temp_xml, xml_content).map_err(|e| e.to_string())?;

    let status = binaries::silent_cmd(binaries::get_mkvpropedit())
        .args([mkv_path.to_str().unwrap(), "--tags", &format!("global:{}", temp_xml)])
        .status().map_err(|e| e.to_string())?;

    let _ = std::fs::remove_file(temp_xml);
    if status.success() { Ok(()) } else { Err("Erreur injection métadonnées".into()) }
}

/// 4. Injection Poster / Fanart / Logo
pub fn ajouter_images_mkv(mkv_path: &Path) -> Result<(), String> {
    let parent = mkv_path.parent().ok_or("Dossier parent introuvable")?;
    let stem_mkv = mkv_path.file_stem().unwrap().to_string_lossy().to_lowercase();
    let mut command = binaries::silent_cmd(binaries::get_mkvpropedit());
    command.arg(mkv_path);

    let mut found = false;
    for entry in std::fs::read_dir(parent).map_err(|e| e.to_string())? {
        let path = entry.map_err(|e| e.to_string())?.path();
        let name = path.file_name().unwrap().to_string_lossy().to_lowercase();

        if name.contains(&stem_mkv) && (name.contains("poster") || name.contains("fanart") || name.contains("clearlogo")) {
            let attachment_name = if name.contains("poster") { "cover" }
                                 else if name.contains("fanart") { "fanart" }
                                 else { "clearlogo" };
            let mime = if name.ends_with(".png") { "image/png" } else { "image/jpeg" };

            command.args(["--attachment-name", attachment_name, "--attachment-mime-type", mime, "--add-attachment", path.to_str().unwrap()]);
            found = true;
        }
    }

    if found {
        let status = command.status().map_err(|e| e.to_string())?;
        if status.success() { return Ok(()); }
    }
    Ok(())
}

/// 5. Supprimer TOUS les tags et TOUTES les pièces jointes (Reset total)
pub fn supprimer_tous_tags(mkv_path: &Path) -> Result<(), String> {
    let xml_vide = "<?xml version=\"1.0\"?>\n<Tags>\n</Tags>";
    let temp_xml = "temp_reset.xml";
    std::fs::write(temp_xml, xml_vide).map_err(|e| e.to_string())?;

    let status = binaries::silent_cmd(binaries::get_mkvpropedit())
        .args([
            mkv_path.to_str().unwrap(),
            "--tags", &format!("global:{}", temp_xml),
            "--edit", "info", "--set", "title=",
            "--delete-attachment", "mime-type:image/jpeg",
            "--delete-attachment", "mime-type:image/png",
        ])
        .status()
        .map_err(|e| e.to_string())?;

    let _ = std::fs::remove_file(temp_xml);

    if status.success() { Ok(()) } else { Err("Erreur reset".into()) }
}