pub fn extraire_deps() -> Result<(), String> {
    let temp_dir = std::env::temp_dir().join("oxyon_tools");
    if !temp_dir.exists() {
        std::fs::create_dir_all(&temp_dir).map_err(|e| e.to_string())?;
    }

    // On extrait un par un au lieu de faire un gros tableau global
    let mut f = |name: &str, bytes: &[u8]| -> Result<(), String> {
        let path = temp_dir.join(name);
        if !path.exists() {
            std::fs::write(path, bytes).map_err(|e| e.to_string())?;
        }
        Ok(())
    };

    f("ffmpeg.exe", include_bytes!("../../bin/ffmpeg.exe"))?;
    f("ffplay.exe", include_bytes!("../../bin/ffplay.exe"))?;
    f("ffprobe.exe", include_bytes!("../../bin/ffprobe.exe"))?;
    f("mkvmerge.exe", include_bytes!("../../bin/mkvmerge.exe"))?;
    f(
        "mkvpropedit.exe",
        include_bytes!("../../bin/mkvpropedit.exe"),
    )?;
    f("7za.exe", include_bytes!("../../bin/7za.exe"))?;
    f("pandoc.exe", include_bytes!("../../bin/pandoc.exe"))?;
    f("exiftool.exe", include_bytes!("../../bin/exiftool.exe"))?;

    // ... reste du code pour le PATH ...
    Ok(())
}
