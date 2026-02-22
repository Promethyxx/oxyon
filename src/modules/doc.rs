#![allow(dead_code)]
use std::collections::BTreeMap;
use std::path::Path;
use std::sync::Arc;

use lopdf::content::{Content, Operation};
use lopdf::encryption::crypt_filters::{Aes128CryptFilter, CryptFilter};
use lopdf::encryption::{EncryptionState, EncryptionVersion, Permissions};
use lopdf::{dictionary, Document, Object, ObjectId, SaveOptions, Stream};

use crate::modules::binaries;

// ════════════════════════════════════════════════════════════════════════
//  ENUMS FORMATS PANDOC
// ════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone, Copy)]
pub enum FormatEntree {
    Docx, Csv, Dotx, Json, Log, Md, Odt, Typst, Yaml, Html, Tex, Rst, Pdf,
}

#[derive(Debug, Clone, Copy)]
pub enum FormatSortie {
    Docx, Html, Md, Odt, Tex, Plain, Pdf,
}

impl FormatEntree {
    pub fn to_pandoc_arg(&self) -> &str {
        match self {
            Self::Docx => "docx",
            Self::Csv  => "csv",
            Self::Dotx => "docx",
            Self::Json => "json",
            Self::Log  => "markdown",
            Self::Md   => "markdown",
            Self::Odt  => "odt",
            Self::Typst => "typst",
            Self::Yaml => "markdown",
            Self::Html => "html",
            Self::Tex  => "latex",
            Self::Rst  => "rst",
            Self::Pdf  => "pdf",
        }
    }
}

impl FormatSortie {
    pub fn to_pandoc_arg(&self) -> &str {
        match self {
            Self::Docx  => "docx",
            Self::Html  => "html",
            Self::Md    => "markdown",
            Self::Odt   => "odt",
            Self::Tex   => "latex",
            Self::Plain => "plain",
            Self::Pdf   => "pdf",
        }
    }
}

// ════════════════════════════════════════════════════════════════════════
//  CONVERSION PANDOC
// ════════════════════════════════════════════════════════════════════════

pub fn convertir(input: &Path, output: &str) -> bool {
    let pandoc = binaries::get_pandoc();
    crate::log_info(&format!("doc::convertir | pandoc={:?} | {:?} -> {}", pandoc, input, output));
    let result = binaries::silent_cmd(pandoc)
        .arg(input.to_str().unwrap())
        .arg("-o").arg(output)
        .output();
    match result {
        Ok(out) => {
            let success = out.status.success();
            if !success {
                let stderr = String::from_utf8_lossy(&out.stderr);
                crate::log_error(&format!("doc::convertir ÉCHEC | code={:?} | stderr={}", out.status.code(), stderr.trim()));
            }
            success
        },
        Err(e) => {
            crate::log_error(&format!("doc::convertir impossible de lancer pandoc : {}", e));
            false
        }
    }
}

pub fn extraire_texte(input: &Path, output: &str) -> bool {
    let pandoc = binaries::get_pandoc();
    crate::log_info(&format!("doc::extraire_texte | pandoc={:?} | {:?} -> {}", pandoc, input, output));
    let result = binaries::silent_cmd(pandoc)
        .arg(input.to_str().unwrap())
        .arg("-t").arg("plain")
        .arg("-o").arg(output)
        .output();
    match result {
        Ok(out) => {
            let success = out.status.success();
            if !success {
                crate::log_error(&format!("doc::extraire_texte ÉCHEC | stderr={}", String::from_utf8_lossy(&out.stderr).trim()));
            }
            success
        },
        Err(e) => {
            crate::log_error(&format!("doc::extraire_texte impossible de lancer pandoc : {}", e));
            false
        }
    }
}

pub fn convertir_avec_formats(
    input: &Path, output: &str,
    format_entree: Option<FormatEntree>,
    format_sortie: Option<FormatSortie>,
) -> bool {
    let pandoc = binaries::get_pandoc();
    crate::log_info(&format!(
        "doc::convertir_avec_formats | pandoc={:?} | entree={:?} sortie={:?} | {:?} -> {}",
        pandoc, format_entree, format_sortie, input, output
    ));
    // Vérifier que le fichier source existe
    if !input.exists() {
        crate::log_error(&format!("doc::convertir_avec_formats fichier source introuvable : {:?}", input));
        return false;
    }
    let mut cmd = binaries::silent_cmd(pandoc);
    if let Some(fmt) = format_entree { cmd.arg("-f").arg(fmt.to_pandoc_arg()); }
    cmd.arg(input.to_str().unwrap());
    if let Some(fmt) = format_sortie { cmd.arg("-t").arg(fmt.to_pandoc_arg()); }
    cmd.arg("-o").arg(output);
    match cmd.output() {
        Ok(out) => {
            let success = out.status.success();
            if !success {
                let stderr = String::from_utf8_lossy(&out.stderr);
                crate::log_error(&format!(
                    "doc::convertir_avec_formats ÉCHEC | code={:?} | pandoc stderr={}",
                    out.status.code(), stderr.trim()
                ));
            }
            success
        },
        Err(e) => {
            crate::log_error(&format!("doc::convertir_avec_formats impossible de lancer pandoc : {}", e));
            false
        }
    }
}

pub fn convertir_csv(input: &Path, output: &str, fmt: FormatSortie) -> bool {
    convertir_avec_formats(input, output, Some(FormatEntree::Csv), Some(fmt))
}

pub fn traiter_log(input: &Path, output: &str) -> bool {
    binaries::silent_cmd(binaries::get_pandoc())
        .arg(input.to_str().unwrap())
        .arg("-f").arg("markdown").arg("-t").arg("plain")
        .arg("-o").arg(output)
        .status().map(|s| s.success()).unwrap_or(false)
}

pub fn convertir_yaml(input: &Path, output: &str, fmt: FormatSortie) -> bool {
    convertir_avec_formats(input, output, Some(FormatEntree::Yaml), Some(fmt))
}

pub fn convertir_typst(input: &Path, output: &str, fmt: FormatSortie) -> bool {
    convertir_avec_formats(input, output, Some(FormatEntree::Typst), Some(fmt))
}

pub fn convertir_pdf(input: &Path, output: &str, fmt: FormatSortie) -> bool {
    convertir_avec_formats(input, output, Some(FormatEntree::Pdf), Some(fmt))
}

pub fn convertir_vers_pdf(input: &Path, format_entree: Option<FormatEntree>) -> Result<String, String> {
    let output = input.with_extension("pdf");
    let output_str = output.to_str().ok_or("Chemin de sortie invalide")?;
    if convertir_avec_formats(input, output_str, format_entree, Some(FormatSortie::Pdf)) {
        Ok(output_str.to_string())
    } else {
        Err("Conversion vers PDF échouée".into())
    }
}

pub fn detecter_format_entree(path: &Path) -> Option<FormatEntree> {
    path.extension()?.to_str().and_then(|ext| match ext.to_lowercase().as_str() {
        "docx" => Some(FormatEntree::Docx),
        "dotx" => Some(FormatEntree::Dotx),
        "csv"  => Some(FormatEntree::Csv),
        "json" => Some(FormatEntree::Json),
        "log"  => Some(FormatEntree::Log),
        "md" | "markdown" => Some(FormatEntree::Md),
        "odt"  => Some(FormatEntree::Odt),
        "typst" | "typ" => Some(FormatEntree::Typst),
        "yaml" | "yml" => Some(FormatEntree::Yaml),
        "html" | "htm" => Some(FormatEntree::Html),
        "tex"  => Some(FormatEntree::Tex),
        "rst"  => Some(FormatEntree::Rst),
        "pdf"  => Some(FormatEntree::Pdf),
        _ => None,
    })
}

pub fn detecter_format_sortie(output: &str) -> Option<FormatSortie> {
    Path::new(output).extension()?.to_str().and_then(|ext| match ext.to_lowercase().as_str() {
        "docx" => Some(FormatSortie::Docx),
        "html" | "htm" => Some(FormatSortie::Html),
        "md" | "markdown" => Some(FormatSortie::Md),
        "odt"  => Some(FormatSortie::Odt),
        "tex"  => Some(FormatSortie::Tex),
        "txt"  => Some(FormatSortie::Plain),
        "pdf"  => Some(FormatSortie::Pdf),
        _ => None,
    })
}

// ════════════════════════════════════════════════════════════════════════
//  HELPERS — Conversion non-PDF → PDF → traitement → reconversion
// ════════════════════════════════════════════════════════════════════════

fn est_pdf(path: &Path) -> bool {
    path.extension()
        .and_then(|e| e.to_str())
        .map(|e| e.eq_ignore_ascii_case("pdf"))
        .unwrap_or(false)
}

/// Convertit un fichier non-PDF en PDF temporaire via pandoc.
/// Retourne le chemin du fichier PDF temporaire.
fn vers_pdf_temp(input: &Path) -> Result<String, String> {
    let tmp = std::env::temp_dir().join(format!(
        "oxyon_tmp_{}.pdf",
        input.file_stem().unwrap_or_default().to_string_lossy()
    ));
    let tmp_str = tmp.to_str().ok_or("Chemin temp invalide")?;
    let format_in = detecter_format_entree(input);
    crate::log_info(&format!("vers_pdf_temp | {:?} -> {} | format_in={:?}", input, tmp_str, format_in));
    if convertir_avec_formats(input, tmp_str, format_in, Some(FormatSortie::Pdf)) {
        Ok(tmp_str.to_string())
    } else {
        Err(format!("Conversion vers PDF temporaire échouée pour {}", input.display()))
    }
}

/// Reconvertit un PDF temporaire vers le format original du fichier.
fn depuis_pdf_temp(pdf_path: &str, output: &str) -> Result<(), String> {
    let fmt_out = detecter_format_sortie(output);
    if fmt_out.is_none() || matches!(fmt_out, Some(FormatSortie::Pdf)) {
        // Déjà PDF ou format inconnu : simple copie
        std::fs::copy(pdf_path, output)
            .map_err(|e| format!("Erreur copie {} → {} : {}", pdf_path, output, e))?;
        return Ok(());
    }
    if convertir_avec_formats(Path::new(pdf_path), output, Some(FormatEntree::Pdf), fmt_out) {
        Ok(())
    } else {
        Err(format!("Reconversion depuis PDF échouée pour {}", output))
    }
}

/// Nettoie un fichier temporaire (ignore les erreurs)
fn nettoyer_temp(path: &str) {
    let _ = std::fs::remove_file(path);
}

/// Applique une opération PDF sur n'importe quel format compatible.
/// Pour les fichiers non-PDF, convertit d'abord en PDF, applique l'opération, puis reconvertit.
/// `op_pdf` reçoit (input_pdf, output_pdf) et fait le travail sur un vrai PDF.
fn appliquer_operation_doc<F>(input: &Path, output: &str, op_pdf: F) -> Result<(), String>
where
    F: FnOnce(&Path, &str) -> Result<(), String>,
{
    if est_pdf(input) {
        crate::log_info(&format!("appliquer_operation_doc | PDF direct | {:?} -> {}", input, output));
        return op_pdf(input, output);
    }

    // Entrée non-PDF : conversion → traitement → reconversion
    crate::log_info(&format!("appliquer_operation_doc | non-PDF, conversion intermédiaire | {:?}", input));
    let pdf_in = vers_pdf_temp(input)?;
    let pdf_out = format!("{}_out.pdf", pdf_in.trim_end_matches(".pdf"));

    let result = op_pdf(Path::new(&pdf_in), &pdf_out);
    nettoyer_temp(&pdf_in);

    if result.is_err() {
        nettoyer_temp(&pdf_out);
        return result;
    }

    let reconvert = depuis_pdf_temp(&pdf_out, output);
    nettoyer_temp(&pdf_out);
    reconvert
}

// ════════════════════════════════════════════════════════════════════════
//  HELPERS PDF INTERNES
// ════════════════════════════════════════════════════════════════════════

fn obtenir_pages_ordonnees(doc: &Document) -> Vec<ObjectId> {
    let pages = doc.get_pages();
    let mut page_list: Vec<(u32, ObjectId)> = pages.into_iter().collect();
    page_list.sort_by_key(|(num, _)| *num);
    page_list.into_iter().map(|(_, id)| id).collect()
}

fn obtenir_mediabox(doc: &Document, page_id: ObjectId) -> Option<[f64; 4]> {
    let page_dict = doc.get_dictionary(page_id).ok()?;

    let mediabox_obj = if let Ok(mb) = page_dict.get(b"MediaBox") {
        mb.clone()
    } else if let Ok(Object::Reference(parent_id)) = page_dict.get(b"Parent") {
        let parent_dict = doc.get_dictionary(*parent_id).ok()?;
        parent_dict.get(b"MediaBox").ok()?.clone()
    } else {
        return None;
    };

    if let Object::Array(ref arr) = mediabox_obj {
        if arr.len() == 4 {
            let vals: Vec<f64> = arr.iter().filter_map(|o| match o {
                Object::Integer(i) => Some(*i as f64),
                Object::Real(r) => Some((*r).into()),
                _ => None,
            }).collect();
            if vals.len() == 4 {
                return Some([vals[0], vals[1], vals[2], vals[3]]);
            }
        }
    }
    None
}

fn collecter_references(doc: &Document, obj: &Object, ids: &mut Vec<ObjectId>) {
    match obj {
        Object::Reference(id) => {
            if !ids.contains(id) {
                ids.push(*id);
                if let Ok(referenced) = doc.get_object(*id) {
                    collecter_references(doc, referenced, ids);
                }
            }
        }
        Object::Array(arr) => {
            for item in arr { collecter_references(doc, item, ids); }
        }
        Object::Dictionary(dict) => {
            for (_, val) in dict.iter() { collecter_references(doc, val, ids); }
        }
        Object::Stream(stream) => {
            for (_, val) in stream.dict.iter() { collecter_references(doc, val, ids); }
        }
        _ => {}
    }
}

/// Helper : ajouter un overlay (contenu + police + optionnel graphic state) à une page
fn ajouter_overlay_page(
    doc: &mut Document,
    page_id: ObjectId,
    content_bytes: Vec<u8>,
    font_name: &str,
    font_id: ObjectId,
    extra_gs: Option<(&str, ObjectId)>,
) -> Result<(), String> {
    let stream_id = doc.add_object(Stream::new(dictionary! {}, content_bytes));

    // Phase 1 : lire resources et contents (emprunt immutable via get_object)
    let (res_dict, font_ref_id, existing_contents) = {
        let page_obj = doc.get_object(page_id)
            .map_err(|e| format!("Page introuvable : {}", e))?;
        let dict = match page_obj {
            Object::Dictionary(d) => d,
            _ => return Err("L'objet page n'est pas un dictionnaire".into()),
        };

        let res = match dict.get(b"Resources") {
            Ok(Object::Dictionary(r)) => r.clone(),
            Ok(Object::Reference(_id)) => { lopdf::Dictionary::default() }  // marqueur : on résoudra après
            _ => lopdf::Dictionary::default(),
        };
        let res_ref = match dict.get(b"Resources") {
            Ok(Object::Reference(id)) => Some(*id),
            _ => None,
        };

        let contents = match dict.get(b"Contents") {
            Ok(Object::Array(arr)) => {
                let mut arr = arr.clone();
                arr.push(Object::Reference(stream_id));
                Object::Array(arr)
            }
            Ok(Object::Reference(ref_id)) => {
                Object::Array(vec![Object::Reference(*ref_id), Object::Reference(stream_id)])
            }
            _ => Object::Reference(stream_id),
        };

        (res, res_ref, contents)
    };

    // Résoudre les Resources si c'était une référence
    let mut resources = if let Some(ref_id) = font_ref_id {
        doc.get_dictionary(ref_id).cloned().unwrap_or(res_dict)
    } else {
        res_dict
    };

    // Résoudre Font
    let font_ref = match resources.get(b"Font") {
        Ok(Object::Reference(id)) => Some(*id),
        _ => None,
    };
    let mut fonts = match resources.get(b"Font") {
        Ok(Object::Dictionary(f)) => f.clone(),
        _ => lopdf::Dictionary::default(),
    };
    if let Some(fid) = font_ref {
        fonts = doc.get_dictionary(fid).cloned().unwrap_or(fonts);
    }
    fonts.set(font_name, Object::Reference(font_id));
    resources.set("Font", Object::Dictionary(fonts));

    // Résoudre ExtGState si nécessaire
    if let Some((gs_name, gs_id)) = extra_gs {
        let gs_ref = match resources.get(b"ExtGState") {
            Ok(Object::Reference(id)) => Some(*id),
            _ => None,
        };
        let mut ext_gstate = match resources.get(b"ExtGState") {
            Ok(Object::Dictionary(g)) => g.clone(),
            _ => lopdf::Dictionary::default(),
        };
        if let Some(gid) = gs_ref {
            ext_gstate = doc.get_dictionary(gid).cloned().unwrap_or(ext_gstate);
        }
        ext_gstate.set(gs_name, Object::Reference(gs_id));
        resources.set("ExtGState", Object::Dictionary(ext_gstate));
    }

    // Phase 2 : écrire (emprunt mutable)
    if let Ok(Object::Dictionary(dict)) = doc.get_object_mut(page_id) {
        dict.set("Resources", Object::Dictionary(resources));
        dict.set("Contents", existing_contents);
    }

    Ok(())
}

/// Helper : doc.save() retourne Result<File, _> en 0.38, on le mappe en Result<(), _>
fn sauvegarder(doc: &mut Document, output: &str) -> Result<(), String> {
    doc.save(output).map(|_| ()).map_err(|e| format!("Erreur sauvegarde : {}", e))
}

// ════════════════════════════════════════════════════════════════════════
//  PDF SPLIT
// ════════════════════════════════════════════════════════════════════════

fn pdf_split_interne(input: &Path, output_dir: &str) -> Result<Vec<String>, String> {
    std::fs::create_dir_all(output_dir)
        .map_err(|e| format!("Impossible de créer le dossier : {}", e))?;

    let doc = Document::load(input)
        .map_err(|e| format!("Erreur chargement PDF : {}", e))?;
    let pages = obtenir_pages_ordonnees(&doc);

    if pages.is_empty() {
        return Err("Le PDF ne contient aucune page".into());
    }

    crate::log_info(&format!("pdf_split_interne | {:?} | {} pages -> {}", input, pages.len(), output_dir));

    let base_name = input.file_stem().unwrap_or_default().to_string_lossy();
    let mut fichiers = Vec::new();

    for (i, &page_id) in pages.iter().enumerate() {
        let mut new_doc = Document::with_version(&doc.version);

        let mut objets = vec![page_id];
        if let Ok(page_obj) = doc.get_object(page_id) {
            collecter_references(&doc, page_obj, &mut objets);
        }

        let mut id_map: BTreeMap<ObjectId, ObjectId> = BTreeMap::new();
        for &old_id in &objets {
            if !id_map.contains_key(&old_id) {
                if let Ok(obj) = doc.get_object(old_id) {
                    let new_id = new_doc.add_object(obj.clone());
                    id_map.insert(old_id, new_id);
                }
            }
        }

        let new_page_id = id_map.get(&page_id).copied().unwrap_or(page_id);
        let pages_id = new_doc.new_object_id();

        if let Ok(Object::Dictionary(dict)) = new_doc.get_object_mut(new_page_id) {
            dict.set("Parent", Object::Reference(pages_id));
        }

        let pages_dict = dictionary! {
            "Type" => "Pages",
            "Kids" => vec![Object::Reference(new_page_id)],
            "Count" => Object::Integer(1),
        };
        new_doc.objects.insert(pages_id, Object::Dictionary(pages_dict));

        let catalog_id = new_doc.add_object(dictionary! {
            "Type" => "Catalog",
            "Pages" => pages_id,
        });
        new_doc.trailer.set("Root", catalog_id);
        new_doc.compress();

        let output_path = format!("{}/{}_page_{:04}.pdf", output_dir, base_name, i + 1);
        sauvegarder(&mut new_doc, &output_path)?;
        fichiers.push(output_path);
    }

    Ok(fichiers)
}

/// Split : fonctionne sur PDF et autres formats (convertit d'abord en PDF)
pub fn pdf_split(input: &Path, output_dir: &str) -> Result<Vec<String>, String> {
    if est_pdf(input) {
        return pdf_split_interne(input, output_dir);
    }
    // Non-PDF : convertir d'abord
    let pdf_tmp = vers_pdf_temp(input)?;
    let result = pdf_split_interne(Path::new(&pdf_tmp), output_dir);
    nettoyer_temp(&pdf_tmp);
    result
}

// ════════════════════════════════════════════════════════════════════════
//  PDF MERGE
// ════════════════════════════════════════════════════════════════════════

pub fn pdf_merge(inputs: &[&Path], output: &str) -> Result<(), String> {
    if inputs.is_empty() {
        return Err("Aucun fichier à fusionner".into());
    }
    crate::log_info(&format!("pdf_merge | {} fichier(s) -> {}", inputs.len(), output));
    for (i, p) in inputs.iter().enumerate() {
        crate::log_info(&format!("  [{}] {:?}", i+1, p));
    }

    // Convertir les non-PDF en PDF temporaire
    let mut documents: Vec<Document> = Vec::new();
    let mut temps: Vec<String> = Vec::new();
    for path in inputs {
        if est_pdf(path) {
            documents.push(
                Document::load(path)
                    .map_err(|e| format!("Erreur chargement {} : {}", path.display(), e))?
            );
        } else {
            let tmp = vers_pdf_temp(path)?;
            documents.push(
                Document::load(&tmp)
                    .map_err(|e| format!("Erreur chargement temp {} : {}", tmp, e))?
            );
            temps.push(tmp);
        }
    }

    let mut max_id = 1;
    let mut documents_pages = BTreeMap::new();
    let mut documents_objects = BTreeMap::new();
    let mut merged = Document::with_version("1.5");

    for mut doc in documents {
        doc.renumber_objects_with(max_id);
        max_id = doc.max_id + 1;

        documents_pages.extend(
            doc.get_pages().into_iter().map(|(_, object_id)| {
                (object_id, doc.get_object(object_id).unwrap().to_owned())
            }).collect::<BTreeMap<ObjectId, Object>>()
        );
        documents_objects.extend(doc.objects);
    }

    let mut catalog_object: Option<(ObjectId, Object)> = None;
    let mut pages_object: Option<(ObjectId, Object)> = None;

    for (object_id, object) in documents_objects.iter() {
        match object.type_name().unwrap_or(b"") {
            b"Catalog" => {
                catalog_object = Some((
                    catalog_object.map_or(*object_id, |(id, _)| id),
                    object.clone(),
                ));
            }
            b"Pages" => {
                if let Ok(dict) = object.as_dict() {
                    let mut dict = dict.clone();
                    if let Some((_, ref old)) = pages_object {
                        if let Ok(old_dict) = old.as_dict() {
                            dict.extend(old_dict);
                        }
                    }
                    pages_object = Some((
                        pages_object.map_or(*object_id, |(id, _)| id),
                        Object::Dictionary(dict),
                    ));
                }
            }
            b"Page" | b"Outlines" | b"Outline" => {}
            _ => { merged.objects.insert(*object_id, object.clone()); }
        }
    }

    let pages_obj = pages_object.ok_or("Nœud Pages introuvable")?;
    let catalog_obj = catalog_object.ok_or("Catalogue introuvable")?;

    for (object_id, object) in documents_pages.iter() {
        if let Ok(dict) = object.as_dict() {
            let mut dict = dict.clone();
            dict.set("Parent", pages_obj.0);
            merged.objects.insert(*object_id, Object::Dictionary(dict));
        }
    }

    if let Ok(dict) = pages_obj.1.as_dict() {
        let mut dict = dict.clone();
        dict.set("Count", documents_pages.len() as u32);
        dict.set("Kids", documents_pages.keys()
            .map(|id| Object::Reference(*id))
            .collect::<Vec<_>>());
        merged.objects.insert(pages_obj.0, Object::Dictionary(dict));
    }

    if let Ok(dict) = catalog_obj.1.as_dict() {
        let mut dict = dict.clone();
        dict.set("Pages", pages_obj.0);
        dict.remove(b"Outlines");
        merged.objects.insert(catalog_obj.0, Object::Dictionary(dict));
    }

    merged.trailer.set("Root", catalog_obj.0);
    merged.max_id = merged.objects.len() as u32;
    merged.renumber_objects();
    merged.compress();

    let result = sauvegarder(&mut merged, output);
    for t in &temps { nettoyer_temp(t); }
    result
}

// ════════════════════════════════════════════════════════════════════════
//  PDF ROTATE
// ════════════════════════════════════════════════════════════════════════

fn pdf_rotate_interne(input: &Path, output: &str, rotation: u16, pages_cibles: Option<&[u32]>) -> Result<(), String> {
    let mut doc = Document::load(input)
        .map_err(|e| format!("Erreur chargement PDF : {}", e))?;

    if !matches!(rotation, 90 | 180 | 270) {
        return Err(format!("Rotation invalide : {}. Utilisez 90, 180 ou 270.", rotation));
    }

    let pages = obtenir_pages_ordonnees(&doc);
    for (i, &page_id) in pages.iter().enumerate() {
        let page_num = (i + 1) as u32;
        if !pages_cibles.map_or(true, |c| c.contains(&page_num)) { continue; }

        if let Ok(Object::Dictionary(dict)) = doc.get_object_mut(page_id) {
            let actuel = dict.get(b"Rotate").ok()
                .and_then(|r| if let Object::Integer(i) = r { Some(*i) } else { None })
                .unwrap_or(0);
            dict.set("Rotate", Object::Integer((actuel + rotation as i64) % 360));
        }
    }

    sauvegarder(&mut doc, output)
}

pub fn pdf_rotate(input: &Path, output: &str, rotation: u16, pages_cibles: Option<&[u32]>) -> Result<(), String> {
    appliquer_operation_doc(input, output, |pdf_in, pdf_out| {
        pdf_rotate_interne(pdf_in, pdf_out, rotation, pages_cibles)
    })
}

// ════════════════════════════════════════════════════════════════════════
//  PDF COMPRESS
// ════════════════════════════════════════════════════════════════════════

fn pdf_compresser_interne(input: &Path, output: &str) -> Result<u64, String> {
    let taille_avant = std::fs::metadata(input).map(|m| m.len()).unwrap_or(0);

    let mut doc = Document::load(input)
        .map_err(|e| format!("Erreur chargement PDF : {}", e))?;

    doc.delete_zero_length_streams();
    doc.prune_objects();
    doc.renumber_objects();
    doc.compress();

    let options = SaveOptions::builder()
        .use_object_streams(true)
        .use_xref_streams(true)
        .compression_level(9)
        .build();

    let mut file = std::fs::File::create(output)
        .map_err(|e| format!("Erreur création fichier : {}", e))?;
    doc.save_with_options(&mut file, options)
        .map_err(|e| format!("Erreur sauvegarde : {}", e))?;

    let taille_apres = std::fs::metadata(output).map(|m| m.len()).unwrap_or(0);
    Ok(taille_avant.saturating_sub(taille_apres))
}

pub fn pdf_compresser(input: &Path, output: &str) -> Result<u64, String> {
    crate::log_info(&format!("pdf_compresser | {:?} -> {}", input, output));
    if est_pdf(input) {
        let result = pdf_compresser_interne(input, output);
        if let Ok(bytes_gagnés) = &result {
            crate::log_info(&format!("pdf_compresser OK | {} octets économisés", bytes_gagnés));
        }
        return result;
    }
    let pdf_tmp = vers_pdf_temp(input)?;
    let result = pdf_compresser_interne(Path::new(&pdf_tmp), output);
    nettoyer_temp(&pdf_tmp);
    result
}

// ════════════════════════════════════════════════════════════════════════
//  PDF CROP
// ════════════════════════════════════════════════════════════════════════

fn pdf_crop_interne(
    input: &Path, output: &str,
    x_pct: f64, y_pct: f64, w_pct: f64, h_pct: f64,
    pages_cibles: Option<&[u32]>,
) -> Result<(), String> {
    let mut doc = Document::load(input)
        .map_err(|e| format!("Erreur chargement PDF : {}", e))?;

    let pages = obtenir_pages_ordonnees(&doc);
    for (i, &page_id) in pages.iter().enumerate() {
        let page_num = (i + 1) as u32;
        if !pages_cibles.map_or(true, |c| c.contains(&page_num)) { continue; }

        let [mb_x, mb_y, mb_w, mb_h] = obtenir_mediabox(&doc, page_id)
            .ok_or_else(|| format!("MediaBox introuvable page {}", page_num))?;

        let largeur = mb_w - mb_x;
        let hauteur = mb_h - mb_y;

        let new_llx = mb_x + (largeur * x_pct / 100.0);
        let new_lly = mb_y + (hauteur * y_pct / 100.0);
        let new_urx = new_llx + (largeur * w_pct / 100.0);
        let new_ury = new_lly + (hauteur * h_pct / 100.0);

        if let Ok(Object::Dictionary(dict)) = doc.get_object_mut(page_id) {
            dict.set("CropBox", Object::Array(vec![
                Object::Real(format!("{:.2}", new_llx).parse().unwrap_or(0.0)),
                Object::Real(format!("{:.2}", new_lly).parse().unwrap_or(0.0)),
                Object::Real(format!("{:.2}", new_urx).parse().unwrap_or(0.0)),
                Object::Real(format!("{:.2}", new_ury).parse().unwrap_or(0.0)),
            ]));
        }
    }

    sauvegarder(&mut doc, output)
}

pub fn pdf_crop(
    input: &Path, output: &str,
    x_pct: f64, y_pct: f64, w_pct: f64, h_pct: f64,
    pages_cibles: Option<&[u32]>,
) -> Result<(), String> {
    appliquer_operation_doc(input, output, |pdf_in, pdf_out| {
        pdf_crop_interne(pdf_in, pdf_out, x_pct, y_pct, w_pct, h_pct, pages_cibles)
    })
}

// ════════════════════════════════════════════════════════════════════════
//  PDF ORGANIZE — Réorganise / supprime des pages
// ════════════════════════════════════════════════════════════════════════

fn pdf_organiser_interne(input: &Path, output: &str, nouvel_ordre: &[u32]) -> Result<(), String> {
    let mut doc = Document::load(input)
        .map_err(|e| format!("Erreur chargement PDF : {}", e))?;

    let pages = obtenir_pages_ordonnees(&doc);
    let total = pages.len() as u32;

    for &num in nouvel_ordre {
        if num == 0 || num > total {
            return Err(format!("Page invalide : {}. Le PDF a {} pages.", num, total));
        }
    }

    let new_kids: Vec<Object> = nouvel_ordre.iter()
        .map(|&n| Object::Reference(pages[n as usize - 1]))
        .collect();
    let new_count = new_kids.len() as i64;

    let catalog = doc.catalog()
        .map_err(|e| format!("Catalogue introuvable : {}", e))?;
    let pages_id = match catalog.get(b"Pages") {
        Ok(Object::Reference(id)) => *id,
        _ => return Err("Référence Pages introuvable".into()),
    };

    if let Ok(Object::Dictionary(dict)) = doc.get_object_mut(pages_id) {
        dict.set("Kids", Object::Array(new_kids));
        dict.set("Count", Object::Integer(new_count));
    }

    sauvegarder(&mut doc, output)
}

pub fn pdf_organiser(input: &Path, output: &str, nouvel_ordre: &[u32]) -> Result<(), String> {
    let ordre = nouvel_ordre.to_vec();
    appliquer_operation_doc(input, output, |pdf_in, pdf_out| {
        pdf_organiser_interne(pdf_in, pdf_out, &ordre)
    })
}

pub fn pdf_supprimer_pages(input: &Path, output: &str, pages_a_supprimer: &[u32]) -> Result<(), String> {
    // On a besoin du nombre total de pages → charger d'abord
    let total = if est_pdf(input) {
        let doc = Document::load(input).map_err(|e| format!("Erreur : {}", e))?;
        doc.get_pages().len() as u32
    } else {
        let pdf_tmp = vers_pdf_temp(input)?;
        let doc = Document::load(&pdf_tmp).map_err(|e| format!("Erreur : {}", e))?;
        let total = doc.get_pages().len() as u32;
        nettoyer_temp(&pdf_tmp);
        total
    };

    let pages_a_garder: Vec<u32> = (1..=total)
        .filter(|n| !pages_a_supprimer.contains(n))
        .collect();

    if pages_a_garder.is_empty() {
        return Err("Impossible de supprimer toutes les pages".into());
    }

    pdf_organiser(input, output, &pages_a_garder)
}

// ════════════════════════════════════════════════════════════════════════
//  PDF PAGE NUMBERS
// ════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone, Copy)]
pub enum PositionNumero {
    BasCentre, BasGauche, BasDroite,
    HautCentre, HautGauche, HautDroite,
}

fn pdf_numeroter_interne(
    input: &Path, output: &str,
    debut: u32,
    position: PositionNumero,
    taille_police: f64,
) -> Result<(), String> {
    let mut doc = Document::load(input)
        .map_err(|e| format!("Erreur chargement PDF : {}", e))?;

    let font_dict = dictionary! {
        "Type" => "Font",
        "Subtype" => "Type1",
        "BaseFont" => "Helvetica",
    };
    let font_id = doc.add_object(font_dict);

    let pages = obtenir_pages_ordonnees(&doc);

    for (i, &page_id) in pages.iter().enumerate() {
        let numero = format!("{}", debut + i as u32);
        let mediabox = obtenir_mediabox(&doc, page_id).unwrap_or([0.0, 0.0, 595.0, 842.0]);
        let largeur = mediabox[2] - mediabox[0];
        let hauteur = mediabox[3] - mediabox[1];

        let (x, y) = match position {
            PositionNumero::BasCentre  => (largeur / 2.0 - 10.0, 30.0),
            PositionNumero::BasGauche  => (40.0, 30.0),
            PositionNumero::BasDroite  => (largeur - 60.0, 30.0),
            PositionNumero::HautCentre => (largeur / 2.0 - 10.0, hauteur - 30.0),
            PositionNumero::HautGauche => (40.0, hauteur - 30.0),
            PositionNumero::HautDroite => (largeur - 60.0, hauteur - 30.0),
        };

        let content = Content {
            operations: vec![
                Operation::new("BT", vec![]),
                Operation::new("Tf", vec!["Fnum".into(), taille_police.into()]),
                Operation::new("Td", vec![x.into(), y.into()]),
                Operation::new("Tj", vec![Object::string_literal(numero)]),
                Operation::new("ET", vec![]),
            ],
        };
        let content_bytes = content.encode()
            .map_err(|e| format!("Erreur encodage contenu : {}", e))?;

        ajouter_overlay_page(&mut doc, page_id, content_bytes, "Fnum", font_id, None)?;
    }

    sauvegarder(&mut doc, output)
}

pub fn pdf_numeroter(
    input: &Path, output: &str,
    debut: u32,
    position: PositionNumero,
    taille_police: f64,
) -> Result<(), String> {
    appliquer_operation_doc(input, output, |pdf_in, pdf_out| {
        pdf_numeroter_interne(pdf_in, pdf_out, debut, position, taille_police)
    })
}

// ════════════════════════════════════════════════════════════════════════
//  PDF PROTECT — AES-128, V4
// ════════════════════════════════════════════════════════════════════════

fn pdf_proteger_interne(
    input: &Path, output: &str,
    mot_de_passe_owner: &str,
    mot_de_passe_user: &str,
    autoriser_impression: bool,
    autoriser_copie: bool,
) -> Result<(), String> {
    let mut doc = Document::load(input)
        .map_err(|e| format!("Erreur chargement PDF : {}", e))?;

    let mut perms = Permissions::empty();
    if autoriser_impression {
        perms |= Permissions::PRINTABLE;
        perms |= Permissions::PRINTABLE_IN_HIGH_QUALITY;
    }
    if autoriser_copie {
        perms |= Permissions::COPYABLE;
        perms |= Permissions::COPYABLE_FOR_ACCESSIBILITY;
    }

    let crypt_filter: Arc<dyn CryptFilter> = Arc::new(Aes128CryptFilter);
    let version = EncryptionVersion::V4 {
        document: &doc,
        encrypt_metadata: true,
        crypt_filters: BTreeMap::from([(b"StdCF".to_vec(), crypt_filter)]),
        stream_filter: b"StdCF".to_vec(),
        string_filter: b"StdCF".to_vec(),
        owner_password: mot_de_passe_owner,
        user_password: mot_de_passe_user,
        permissions: perms,
    };

    let state = EncryptionState::try_from(version)
        .map_err(|e| format!("Erreur création chiffrement : {}", e))?;

    doc.encrypt(&state)
        .map_err(|e| format!("Erreur chiffrement : {}", e))?;

    sauvegarder(&mut doc, output)
}

pub fn pdf_proteger(
    input: &Path, output: &str,
    mot_de_passe_owner: &str,
    mot_de_passe_user: &str,
    autoriser_impression: bool,
    autoriser_copie: bool,
) -> Result<(), String> {
    appliquer_operation_doc(input, output, |pdf_in, pdf_out| {
        pdf_proteger_interne(pdf_in, pdf_out, mot_de_passe_owner, mot_de_passe_user, autoriser_impression, autoriser_copie)
    })
}

// ════════════════════════════════════════════════════════════════════════
//  PDF UNLOCK
// ════════════════════════════════════════════════════════════════════════

pub fn pdf_dechiffrer(input: &Path, output: &str, mot_de_passe: &str) -> Result<(), String> {
    let mut doc = Document::load(input)
        .map_err(|e| format!("Erreur chargement PDF : {}", e))?;

    if !doc.is_encrypted() {
        return sauvegarder(&mut doc, output);
    }

    doc.decrypt(mot_de_passe)
        .map_err(|e| format!("Mot de passe incorrect ou erreur déchiffrement : {}", e))?;

    doc.trailer.remove(b"Encrypt");

    sauvegarder(&mut doc, output)
}

// ════════════════════════════════════════════════════════════════════════
//  PDF REPAIR
// ════════════════════════════════════════════════════════════════════════

fn pdf_reparer_interne(input: &Path, output: &str) -> Result<(), String> {
    let mut doc = Document::load(input)
        .map_err(|e| format!("Erreur chargement PDF (fichier trop corrompu ?) : {}", e))?;

    doc.delete_zero_length_streams();
    doc.prune_objects();
    doc.renumber_objects();
    doc.compress();

    sauvegarder(&mut doc, output)
}

pub fn pdf_reparer(input: &Path, output: &str) -> Result<(), String> {
    appliquer_operation_doc(input, output, |pdf_in, pdf_out| {
        pdf_reparer_interne(pdf_in, pdf_out)
    })
}

// ════════════════════════════════════════════════════════════════════════
//  PDF WATERMARK
// ════════════════════════════════════════════════════════════════════════

fn pdf_watermark_interne(
    input: &Path, output: &str,
    texte: &str,
    taille_police: f64,
    opacite: f64,
    pages_cibles: Option<&[u32]>,
) -> Result<(), String> {
    let mut doc = Document::load(input)
        .map_err(|e| format!("Erreur chargement PDF : {}", e))?;

    let gs_dict = dictionary! {
        "Type" => "ExtGState",
        "CA"  => opacite,
        "ca"  => opacite,
    };
    let gs_id = doc.add_object(gs_dict);

    let font_dict = dictionary! {
        "Type" => "Font",
        "Subtype" => "Type1",
        "BaseFont" => "Helvetica-Bold",
    };
    let font_id = doc.add_object(font_dict);

    let pages = obtenir_pages_ordonnees(&doc);

    for (i, &page_id) in pages.iter().enumerate() {
        let page_num = (i + 1) as u32;
        if !pages_cibles.map_or(true, |c| c.contains(&page_num)) { continue; }

        let mediabox = obtenir_mediabox(&doc, page_id).unwrap_or([0.0, 0.0, 595.0, 842.0]);
        let cx = (mediabox[2] - mediabox[0]) / 2.0;
        let cy = (mediabox[3] - mediabox[1]) / 2.0;

        let angle: f64 = 45.0_f64.to_radians();
        let cos_a = angle.cos();
        let sin_a = angle.sin();

        let content = Content {
            operations: vec![
                Operation::new("q", vec![]),
                Operation::new("gs", vec!["GSwm".into()]),
                Operation::new("BT", vec![]),
                Operation::new("Tf", vec!["Fwm".into(), taille_police.into()]),
                Operation::new("rg", vec![0.7.into(), 0.7.into(), 0.7.into()]),
                Operation::new("Tm", vec![
                    cos_a.into(), sin_a.into(),
                    (-sin_a).into(), cos_a.into(),
                    cx.into(), cy.into(),
                ]),
                Operation::new("Tj", vec![Object::string_literal(texte.to_string())]),
                Operation::new("ET", vec![]),
                Operation::new("Q", vec![]),
            ],
        };
        let content_bytes = content.encode()
            .map_err(|e| format!("Erreur encodage watermark : {}", e))?;

        ajouter_overlay_page(
            &mut doc, page_id, content_bytes,
            "Fwm", font_id,
            Some(("GSwm", gs_id)),
        )?;
    }

    sauvegarder(&mut doc, output)
}

pub fn pdf_watermark(
    input: &Path, output: &str,
    texte: &str,
    taille_police: f64,
    opacite: f64,
    pages_cibles: Option<&[u32]>,
) -> Result<(), String> {
    let texte = texte.to_string();
    let opacite = opacite;
    let taille = taille_police;
    let pages = pages_cibles.map(|p| p.to_vec());
    appliquer_operation_doc(input, output, move |pdf_in, pdf_out| {
        pdf_watermark_interne(pdf_in, pdf_out, &texte, taille, opacite, pages.as_deref())
    })
}