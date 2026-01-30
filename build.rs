fn main() {
    if std::env::var("CARGO_CFG_WINDOWS").is_ok() {
        let mut res = winres::WindowsResource::new();
        res.set_icon("oxyon_icon.ico"); // Assurez-vous du nom du fichier
        // Récupère automatiquement la version du Cargo.toml
        res.set_language(0x040c); 
        res.compile().unwrap();
    }
}