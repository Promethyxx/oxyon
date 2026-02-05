#![allow(dead_code)]
use std::path::Path;
use std::process::Command;
use crate::modules::binaries;

/// Liste des formats supportés en entrée par Pandoc
#[derive(Debug, Clone, Copy)]
pub enum FormatEntree {
    Docx,
    Csv,
    Dotx,
    Json,
    Log,
    Md,
    Odt,
    Typst,
    Yaml,
    Html,
    Tex,
    Rst,
    Pdf,
}

/// Liste des formats supportés en sortie par Pandoc
#[derive(Debug, Clone, Copy)]
pub enum FormatSortie {
    Docx,
    Html,
    Md,
    Odt,
    Tex,
    Plain,
    Pdf,
}

impl FormatEntree {
    /// Convertit l'enum en argument Pandoc
    pub fn to_pandoc_arg(&self) -> &str {
        match self {
            Self::Docx => "docx",
            Self::Csv => "csv",
            Self::Dotx => "docx", // DOTX est traité comme DOCX par Pandoc
            Self::Json => "json",
            Self::Log => "markdown", // LOG traité comme du texte/markdown
            Self::Md => "markdown",
            Self::Odt => "odt",
            Self::Typst => "typst",
            Self::Yaml => "markdown", // YAML peut contenir du markdown
            Self::Html => "html",
            Self::Tex => "latex",
            Self::Rst => "rst",
            Self::Pdf => "pdf",
        }
    }
}

impl FormatSortie {
    /// Convertit l'enum en argument Pandoc
    pub fn to_pandoc_arg(&self) -> &str {
        match self {
            Self::Docx => "docx",
            Self::Html => "html",
            Self::Md => "markdown",
            Self::Odt => "odt",
            Self::Tex => "latex",
            Self::Plain => "plain",
            Self::Pdf => "pdf",
        }
    }
}

/// Convertit n'importe quoi vers n'importe quoi (ex: .docx -> .md)
/// **Fonction originale conservée pour compatibilité**
pub fn convertir(input: &Path, output: &str) -> bool {
    let status = Command::new(binaries::get_pandoc())
        .arg(input.to_str().unwrap())
        .arg("-o")
        .arg(output)
        .status();

    status.map(|s| s.success()).unwrap_or(false)
}

/// Extrait uniquement le texte brut (utile pour nettoyer un fichier)
/// **Fonction originale conservée pour compatibilité**
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

/// **NOUVELLE FONCTION** : Convertit avec spécification explicite des formats
pub fn convertir_avec_formats(
    input: &Path,
    output: &str,
    format_entree: Option<FormatEntree>,
    format_sortie: Option<FormatSortie>,
) -> bool {
    let mut cmd = Command::new(binaries::get_pandoc());
    
    // Spécifie le format d'entrée si fourni
    if let Some(fmt) = format_entree {
        cmd.arg("-f").arg(fmt.to_pandoc_arg());
    }
    
    cmd.arg(input.to_str().unwrap());
    
    // Spécifie le format de sortie si fourni
    if let Some(fmt) = format_sortie {
        cmd.arg("-t").arg(fmt.to_pandoc_arg());
    }
    
    cmd.arg("-o").arg(output);
    
    let status = cmd.status();
    status.map(|s| s.success()).unwrap_or(false)
}

/// **NOUVELLE FONCTION** : Convertit CSV vers d'autres formats
pub fn convertir_csv(input: &Path, output: &str, format_sortie: FormatSortie) -> bool {
    convertir_avec_formats(input, output, Some(FormatEntree::Csv), Some(format_sortie))
}

/// **NOUVELLE FONCTION** : Traite les fichiers LOG (extraction de texte)
pub fn traiter_log(input: &Path, output: &str) -> bool {
    // Les fichiers LOG sont généralement du texte brut
    // On peut les traiter comme du markdown ou du plain text
    let status = Command::new(binaries::get_pandoc())
        .arg(input.to_str().unwrap())
        .arg("-f")
        .arg("markdown")
        .arg("-t")
        .arg("plain")
        .arg("-o")
        .arg(output)
        .status();

    status.map(|s| s.success()).unwrap_or(false)
}

/// **NOUVELLE FONCTION** : Convertit YAML vers d'autres formats
pub fn convertir_yaml(input: &Path, output: &str, format_sortie: FormatSortie) -> bool {
    convertir_avec_formats(input, output, Some(FormatEntree::Yaml), Some(format_sortie))
}

/// **NOUVELLE FONCTION** : Convertit Typst vers d'autres formats
pub fn convertir_typst(input: &Path, output: &str, format_sortie: FormatSortie) -> bool {
    convertir_avec_formats(input, output, Some(FormatEntree::Typst), Some(format_sortie))
}

/// **NOUVELLE FONCTION** : Convertit PDF vers d'autres formats (via conversion intermédiaire)
pub fn convertir_pdf(input: &Path, output: &str, format_sortie: FormatSortie) -> bool {
    // Pour PDF en entrée, on utilise pdftotext puis Pandoc
    // Ou directement Pandoc si disponible avec support PDF
    convertir_avec_formats(input, output, Some(FormatEntree::Pdf), Some(format_sortie))
}

/// **NOUVELLE FONCTION** : Convertit vers PDF (via LaTeX intermédiaire si nécessaire)
pub fn convertir_vers_pdf(input: &Path, format_entree: Option<FormatEntree>) -> Result<String, String> {
    let output = input.with_extension("pdf");
    let output_str = output.to_str().ok_or("Invalid output path")?;
    
    let success = convertir_avec_formats(
        input,
        output_str,
        format_entree,
        Some(FormatSortie::Pdf)
    );
    
    if success {
        Ok(output_str.to_string())
    } else {
        Err("Conversion vers PDF échouée".to_string())
    }
}

/// **NOUVELLE FONCTION** : Divise un PDF en pages individuelles
pub fn pdf_split(input: &Path, output_dir: &str) -> Result<Vec<String>, String> {
    use std::fs;
    
    // Utilise pdftk ou qpdf pour diviser
    let status = Command::new("pdftk")
        .arg(input.to_str().unwrap())
        .arg("burst")
        .arg("output")
        .arg(format!("{}/page_%04d.pdf", output_dir))
        .status();
    
    match status {
        Ok(s) if s.success() => {
            // Liste les fichiers créés
            let paths = fs::read_dir(output_dir)
                .map_err(|e| e.to_string())?
                .filter_map(|entry| {
                    entry.ok().and_then(|e| {
                        let path = e.path();
                        if path.extension()? == "pdf" {
                            Some(path.to_string_lossy().to_string())
                        } else {
                            None
                        }
                    })
                })
                .collect();
            Ok(paths)
        }
        _ => Err("Échec de la division du PDF".to_string())
    }
}

/// **NOUVELLE FONCTION** : Fusionne plusieurs PDFs en un seul
pub fn pdf_merge(inputs: &[&Path], output: &str) -> bool {
    let mut cmd = Command::new("pdftk");
    
    for input in inputs {
        cmd.arg(input.to_str().unwrap());
    }
    
    cmd.arg("cat")
        .arg("output")
        .arg(output);
    
    let status = cmd.status();
    status.map(|s| s.success()).unwrap_or(false)
}

/// **NOUVELLE FONCTION** : Pivote les pages d'un PDF
pub fn pdf_rotate(input: &Path, output: &str, rotation: u16, pages: Option<&str>) -> bool {
    // rotation: 90, 180, 270
    // pages: "1-end", "1-3", "even", "odd", etc.
    let pages_spec = pages.unwrap_or("1-end");
    let rotate_cmd = match rotation {
        90 => "east",
        180 => "south",
        270 => "west",
        _ => "north",
    };
    
    let status = Command::new("pdftk")
        .arg(input.to_str().unwrap())
        .arg("cat")
        .arg(format!("{}{}", pages_spec, rotate_cmd))
        .arg("output")
        .arg(output)
        .status();
    
    status.map(|s| s.success()).unwrap_or(false)
}

/// **NOUVELLE FONCTION** : Détecte automatiquement le format depuis l'extension
pub fn detecter_format_entree(path: &Path) -> Option<FormatEntree> {
    path.extension()?.to_str().and_then(|ext| match ext.to_lowercase().as_str() {
        "docx" => Some(FormatEntree::Docx),
        "dotx" => Some(FormatEntree::Dotx),
        "csv" => Some(FormatEntree::Csv),
        "json" => Some(FormatEntree::Json),
        "log" => Some(FormatEntree::Log),
        "md" | "markdown" => Some(FormatEntree::Md),
        "odt" => Some(FormatEntree::Odt),
        "typst" | "typ" => Some(FormatEntree::Typst),
        "yaml" | "yml" => Some(FormatEntree::Yaml),
        "html" | "htm" => Some(FormatEntree::Html),
        "tex" => Some(FormatEntree::Tex),
        "rst" => Some(FormatEntree::Rst),
        "pdf" => Some(FormatEntree::Pdf),
        _ => None,
    })
}

/// **NOUVELLE FONCTION** : Détecte automatiquement le format de sortie depuis l'extension
pub fn detecter_format_sortie(output: &str) -> Option<FormatSortie> {
    let path = Path::new(output);
    path.extension()?.to_str().and_then(|ext| match ext.to_lowercase().as_str() {
        "docx" => Some(FormatSortie::Docx),
        "html" | "htm" => Some(FormatSortie::Html),
        "md" | "markdown" => Some(FormatSortie::Md),
        "odt" => Some(FormatSortie::Odt),
        "tex" => Some(FormatSortie::Tex),
        "txt" => Some(FormatSortie::Plain),
        "pdf" => Some(FormatSortie::Pdf),
        _ => None,
    })
}