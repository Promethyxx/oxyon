use std::path::PathBuf;
use std::sync::OnceLock;

// Cache global des chemins
static TOOLS_DIR: OnceLock<PathBuf> = OnceLock::new();

pub fn extraire_deps() -> Result<(), String> {
    let temp_dir = std::env::temp_dir().join("oxyon_tools");
    if !temp_dir.exists() {
        std::fs::create_dir_all(&temp_dir).map_err(|e| e.to_string())?;
    }

    // On extrait un par un au lieu de faire un gros tableau global
    let f = |name: &str, bytes: &[u8]| -> Result<(), String> {
        let path = temp_dir.join(name);
        if !path.exists() {
            std::fs::write(&path, bytes).map_err(|e| e.to_string())?;
        }
        Ok(())
    };

    f("ffmpeg.exe", include_bytes!("../../bin/ffmpeg.exe"))?;
    f("ffprobe.exe", include_bytes!("../../bin/ffprobe.exe"))?;
    f("mkvpropedit.exe", include_bytes!("../../bin/mkvpropedit.exe"))?;
    f("7za.exe", include_bytes!("../../bin/7za.exe"))?;
    f("pandoc.exe", include_bytes!("../../bin/pandoc.exe"))?;

    // Stocker le chemin du dossier temp
    TOOLS_DIR.set(temp_dir).ok();

    Ok(())
}

// Fonctions publiques pour récupérer les chemins
pub fn get_ffmpeg() -> PathBuf {
    TOOLS_DIR.get().expect("extraire_deps doit être appelé d'abord").join("ffmpeg.exe")
}

pub fn get_ffprobe() -> PathBuf {
    TOOLS_DIR.get().expect("extraire_deps doit être appelé d'abord").join("ffprobe.exe")
}

pub fn get_7z() -> PathBuf {
    TOOLS_DIR.get().expect("extraire_deps doit être appelé d'abord").join("7za.exe")
}

pub fn get_pandoc() -> PathBuf {
    TOOLS_DIR.get().expect("extraire_deps doit être appelé d'abord").join("pandoc.exe")
}

pub fn get_mkvpropedit() -> PathBuf {
    TOOLS_DIR.get().expect("extraire_deps doit être appelé d'abord").join("mkvpropedit.exe")
}

pub fn cleanup() {
    if let Some(dir) = TOOLS_DIR.get() {
        let _ = std::fs::remove_dir_all(dir);
    }
}