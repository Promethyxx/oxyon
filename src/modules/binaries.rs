use std::path::PathBuf;
use std::process::Command;
use std::sync::OnceLock;

static TOOLS_DIR: OnceLock<PathBuf> = OnceLock::new();

#[cfg(target_os = "windows")]
const EXT: &str = ".exe";
#[cfg(not(target_os = "windows"))]
const EXT: &str = "";

// ════════════════════════════════════════════════════════════════════════
//  BINAIRES EMBARQUÉS PAR PLATEFORME
// ════════════════════════════════════════════════════════════════════════

#[cfg(all(target_os = "windows", target_arch = "x86_64"))]
mod embedded {
    pub const FFMPEG:      &[u8] = include_bytes!("../../bin/ffmpeg.exe");
    pub const FFPROBE:     &[u8] = include_bytes!("../../bin/ffprobe.exe");
    pub const MKVPROPEDIT: &[u8] = include_bytes!("../../bin/mkvpropedit.exe");
    pub const PANDOC:      &[u8] = include_bytes!("../../bin/pandoc.exe");
}

#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
mod embedded {
    pub const FFMPEG:      &[u8] = include_bytes!("../../bin-linux-x64/ffmpeg");
    pub const FFPROBE:     &[u8] = include_bytes!("../../bin-linux-x64/ffprobe");
    pub const MKVPROPEDIT: &[u8] = include_bytes!("../../bin-linux-x64/mkvpropedit");
    pub const PANDOC:      &[u8] = include_bytes!("../../bin-linux-x64/pandoc");
}

#[cfg(all(target_os = "linux", target_arch = "aarch64"))]
mod embedded {
    pub const FFMPEG:      &[u8] = include_bytes!("../../bin-linux-arm/ffmpeg");
    pub const FFPROBE:     &[u8] = include_bytes!("../../bin-linux-arm/ffprobe");
    pub const MKVPROPEDIT: &[u8] = include_bytes!("../../bin-linux-arm/mkvpropedit");
    pub const PANDOC:      &[u8] = include_bytes!("../../bin-linux-arm/pandoc");
}

#[cfg(all(target_os = "macos", target_arch = "aarch64"))]
mod embedded {
    pub const FFMPEG:      &[u8] = include_bytes!("../../bin-mac-arm/ffmpeg");
    pub const FFPROBE:     &[u8] = include_bytes!("../../bin-mac-arm/ffprobe");
    pub const MKVPROPEDIT: &[u8] = include_bytes!("../../bin-mac-arm/mkvpropedit");
    pub const PANDOC:      &[u8] = include_bytes!("../../bin-mac-arm/pandoc");
}

// ════════════════════════════════════════════════════════════════════════
//  EXTRACTION
// ════════════════════════════════════════════════════════════════════════

pub fn extraire_deps() -> Result<(), String> {
    let temp_dir = std::env::temp_dir().join("oxyon_tools");
    if !temp_dir.exists() {
        std::fs::create_dir_all(&temp_dir).map_err(|e| e.to_string())?;
    }

    let f = |name: &str, bytes: &[u8]| -> Result<(), String> {
        let path = temp_dir.join(name);
        if !path.exists() {
            std::fs::write(&path, bytes).map_err(|e| e.to_string())?;
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o755))
                    .map_err(|e| e.to_string())?;
            }
        }
        Ok(())
    };

    f(&format!("ffmpeg{}", EXT),      embedded::FFMPEG)?;
    f(&format!("ffprobe{}", EXT),     embedded::FFPROBE)?;
    f(&format!("mkvpropedit{}", EXT), embedded::MKVPROPEDIT)?;
    f(&format!("pandoc{}", EXT),      embedded::PANDOC)?;

    TOOLS_DIR.set(temp_dir).ok();
    Ok(())
}

// ════════════════════════════════════════════════════════════════════════
//  HELPERS
// ════════════════════════════════════════════════════════════════════════

/// Crée une Command silencieuse (pas de fenêtre CMD sur Windows)
pub fn silent_cmd(program: PathBuf) -> Command {
    let cmd = Command::new(program);
    #[cfg(target_os = "windows")]
    {
        use std::os::windows::process::CommandExt;
        const CREATE_NO_WINDOW: u32 = 0x08000000;
        cmd.creation_flags(CREATE_NO_WINDOW);
    }
    cmd
}

pub fn get_ffmpeg() -> PathBuf {
    TOOLS_DIR.get().expect("extraire_deps doit être appelé d'abord").join(format!("ffmpeg{}", EXT))
}

pub fn get_ffprobe() -> PathBuf {
    TOOLS_DIR.get().expect("extraire_deps doit être appelé d'abord").join(format!("ffprobe{}", EXT))
}

pub fn get_pandoc() -> PathBuf {
    TOOLS_DIR.get().expect("extraire_deps doit être appelé d'abord").join(format!("pandoc{}", EXT))
}

pub fn get_mkvpropedit() -> PathBuf {
    TOOLS_DIR.get().expect("extraire_deps doit être appelé d'abord").join(format!("mkvpropedit{}", EXT))
}

pub fn cleanup() {
    if let Some(dir) = TOOLS_DIR.get() {
        let _ = std::fs::remove_dir_all(dir);
    }
}
