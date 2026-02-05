#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Version du package depuis Cargo.toml
const VERSION: &str = env!("CARGO_PKG_VERSION");

mod modules;
use eframe::egui;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::fs::OpenOptions;
use std::io::Write;

// --- Système de Log ---
fn log_error(message: &str) {
    let timestamp = chrono::Local::now().format("%Y-%m-%d %H:%M:%S");
    let log_line = format!("[{}] ERROR: {}\n", timestamp, message);

    if let Ok(mut file) = OpenOptions::new()
        .create(true)
        .append(true)
        .open("oxyon_errors.log") 
    {
        let _ = file.write_all(log_line.as_bytes());
    }
}

#[cfg(feature = "api")]
#[derive(Clone)]
struct ScrapeEntry {
    data: modules::scrap::ScrapeResult,
    texture: Option<egui::TextureHandle>,
}

#[derive(PartialEq, Clone, Copy, Debug)]
enum ModuleType {
    #[cfg(feature = "api")]
    Archive,
    #[cfg(feature = "api")]
    Audio,
    Doc,
    Image,
    #[cfg(feature = "api")]
    Scrapper,
    #[cfg(feature = "api")]
    Tag,
    #[cfg(feature = "api")]
    Video,
    Settings,
}

struct OxyonApp {
    #[cfg(feature = "api")]
    copie_flux: bool,
    current_files: Vec<PathBuf>,
    current_stem: String,
    format_choisi: String,
    module_actif: ModuleType,
    process: Option<std::process::Child>,
    ratio_img: u32,
    #[cfg(feature = "api")]
    results_ui: Arc<Mutex<Vec<ScrapeEntry>>>,
    status: Arc<Mutex<String>>,
    #[cfg(feature = "api")]
    taille_vol: String,
    deps_manquantes: Vec<String>,
    #[cfg(feature = "api")]
    tag_edit_val: String,
    current_theme: String,
    save_doc_format: bool,
    save_image_format: bool,
    #[cfg(feature = "api")]
    save_archive_format: bool,
    #[cfg(feature = "api")]
    save_audio_format: bool,
    #[cfg(feature = "api")]
    save_video_format: bool,
    // Options Image
    image_action: String, // "convert", "resize", "rotate", "crop"
    rotation_angle: u32,
    crop_x: u32,
    crop_y: u32,
    crop_width: u32,
    crop_height: u32,
    // Resize options
    resize_width: String,
    resize_height: String,
    resize_max_kb: String,
    // Options Doc
    doc_action: String, // "convert", "pdf_split", "pdf_merge", "pdf_rotate"
    pdf_rotation_angle: u16,
    pdf_pages_spec: String,
}

impl Default for OxyonApp {
    fn default() -> Self {
        Self {
            #[cfg(feature = "api")]
            copie_flux: false,
            current_files: Vec::new(),
            current_stem: String::new(),
            format_choisi: "mp4".into(),
            module_actif: ModuleType::Image,
            process: None,
            ratio_img: 2,
            #[cfg(feature = "api")]
            results_ui: Arc::new(Mutex::new(Vec::new())),
            status: Arc::new(Mutex::new("Déposez des fichiers".into())),
            #[cfg(feature = "api")]
            taille_vol: "".into(),
            deps_manquantes: Vec::new(),
            #[cfg(feature = "api")]
            tag_edit_val: String::new(),
            current_theme: "Dark".into(),
            save_doc_format: false,
            save_image_format: false,
            #[cfg(feature = "api")]
            save_archive_format: false,
            #[cfg(feature = "api")]
            save_audio_format: false,
            #[cfg(feature = "api")]
            save_video_format: false,
            image_action: "convert".into(),
            rotation_angle: 90,
            crop_x: 0,
            crop_y: 0,
            crop_width: 100,
            crop_height: 100,
            resize_width: String::new(),
            resize_height: String::new(),
            resize_max_kb: String::new(),
            doc_action: "convert".into(),
            pdf_rotation_angle: 90,
            pdf_pages_spec: "1-end".into(),
        }
    }
}

impl OxyonApp {
    fn load_config(&mut self) {
    // 1. Reset : pas de défaut pour les formats, juste vide
    match self.module_actif {
        ModuleType::Image => self.format_choisi = String::new(),
        ModuleType::Doc => self.format_choisi = String::new(),
        #[cfg(feature = "api")]
        ModuleType::Video => self.format_choisi = String::new(),
        #[cfg(feature = "api")]
        ModuleType::Audio => self.format_choisi = String::new(),
        #[cfg(feature = "api")]
        ModuleType::Archive => self.format_choisi = String::new(),
        _ => (),
    }

    // 2. Lecture du fichier
    if let Ok(c) = std::fs::read_to_string("config.toml") {
        if let Ok(parsed) = c.parse::<toml::Table>() {
            if let Some(theme) = parsed.get("display").and_then(|d| d.get("theme")).and_then(|t| t.as_str()) {
                self.current_theme = theme.to_string();
            }

            // Charger Doc
            if let Some(doc) = parsed.get("doc") {
                if let Some(fmt) = doc.get("format").and_then(|f| f.as_str()) {
                    if self.module_actif == ModuleType::Doc {
                        self.format_choisi = fmt.to_string();
                    }
                }
            }

            // Charger Image
            if let Some(img) = parsed.get("image") {
                if let Some(fmt) = img.get("format").and_then(|f| f.as_str()) {
                    if self.module_actif == ModuleType::Image {
                        self.format_choisi = fmt.to_string();
                    }
                }
                if let Some(ratio) = img.get("ratio_img").and_then(|r| r.as_integer()) {
                    self.ratio_img = ratio as u32;
                }
            }

            // Charger Archive
            #[cfg(feature = "api")]
            if let Some(arc) = parsed.get("archive") {
                if let Some(fmt) = arc.get("format").and_then(|f| f.as_str()) {
                    if self.module_actif == ModuleType::Archive {
                        self.format_choisi = fmt.to_string();
                    }
                }
            }

            // Charger Audio
            #[cfg(feature = "api")]
            if let Some(aud) = parsed.get("audio") {
                if let Some(fmt) = aud.get("format").and_then(|f| f.as_str()) {
                    if self.module_actif == ModuleType::Audio {
                        self.format_choisi = fmt.to_string();
                    }
                }
            }

            // Charger Video
            #[cfg(feature = "api")]
            if let Some(vid) = parsed.get("video") {
                if let Some(fmt) = vid.get("format").and_then(|f| f.as_str()) {
                    if self.module_actif == ModuleType::Video {
                        self.format_choisi = fmt.to_string();
                    }
                }
                if let Some(copie) = vid.get("copie_flux").and_then(|c| c.as_bool()) {
                    self.copie_flux = copie;
                }
            }
        }
    }
}

    fn save_config(&self) {
        // Charger le fichier existant ou créer un nouveau toml::Table
        let mut parsed = if let Ok(c) = std::fs::read_to_string("config.toml") {
            c.parse::<toml::Table>().unwrap_or_else(|_| toml::Table::new())
        } else {
            toml::Table::new()
        };

        // Sauvegarder le thème
        let display = parsed.entry("display").or_insert(toml::Value::Table(toml::Table::new()));
        if let Some(display_table) = display.as_table_mut() {
            display_table.insert("theme".to_string(), toml::Value::String(self.current_theme.clone()));
        }

        // Sauvegarder Doc si coché
        if self.save_doc_format && !self.format_choisi.is_empty() && self.module_actif == ModuleType::Doc {
            let doc = parsed.entry("doc").or_insert(toml::Value::Table(toml::Table::new()));
            if let Some(doc_table) = doc.as_table_mut() {
                doc_table.insert("format".to_string(), toml::Value::String(self.format_choisi.clone()));
            }
        } else if !self.save_doc_format && self.module_actif == ModuleType::Doc {
            // Retirer la section doc si décoché
            if let Some(doc_table) = parsed.get_mut("doc").and_then(|v| v.as_table_mut()) {
                doc_table.remove("format");
            }
        }

        // Sauvegarder Image si coché
        if self.save_image_format && !self.format_choisi.is_empty() && self.module_actif == ModuleType::Image {
            let image = parsed.entry("image").or_insert(toml::Value::Table(toml::Table::new()));
            if let Some(img_table) = image.as_table_mut() {
                img_table.insert("format".to_string(), toml::Value::String(self.format_choisi.clone()));
                img_table.insert("ratio_img".to_string(), toml::Value::Integer(self.ratio_img as i64));
            }
        } else if !self.save_image_format && self.module_actif == ModuleType::Image {
            if let Some(img_table) = parsed.get_mut("image").and_then(|v| v.as_table_mut()) {
                img_table.remove("format");
            }
        }

        // Toujours sauvegarder ratio_img pour Image
        if self.module_actif == ModuleType::Image {
            let image = parsed.entry("image").or_insert(toml::Value::Table(toml::Table::new()));
            if let Some(img_table) = image.as_table_mut() {
                img_table.insert("ratio_img".to_string(), toml::Value::Integer(self.ratio_img as i64));
            }
        }

        // Sauvegarder Archive si coché
        #[cfg(feature = "api")]
        if self.save_archive_format && !self.format_choisi.is_empty() && self.module_actif == ModuleType::Archive {
            let archive = parsed.entry("archive").or_insert(toml::Value::Table(toml::Table::new()));
            if let Some(arc_table) = archive.as_table_mut() {
                arc_table.insert("format".to_string(), toml::Value::String(self.format_choisi.clone()));
            }
        } else if !self.save_archive_format && self.module_actif == ModuleType::Archive {
            #[cfg(feature = "api")]
            if let Some(arc_table) = parsed.get_mut("archive").and_then(|v| v.as_table_mut()) {
                arc_table.remove("format");
            }
        }

        // Sauvegarder Audio si coché
        #[cfg(feature = "api")]
        if self.save_audio_format && !self.format_choisi.is_empty() && self.module_actif == ModuleType::Audio {
            let audio = parsed.entry("audio").or_insert(toml::Value::Table(toml::Table::new()));
            if let Some(aud_table) = audio.as_table_mut() {
                aud_table.insert("format".to_string(), toml::Value::String(self.format_choisi.clone()));
            }
        } else if !self.save_audio_format && self.module_actif == ModuleType::Audio {
            #[cfg(feature = "api")]
            if let Some(aud_table) = parsed.get_mut("audio").and_then(|v| v.as_table_mut()) {
                aud_table.remove("format");
            }
        }

        // Sauvegarder Video si coché
        #[cfg(feature = "api")]
        if self.save_video_format && !self.format_choisi.is_empty() && self.module_actif == ModuleType::Video {
            let video = parsed.entry("video").or_insert(toml::Value::Table(toml::Table::new()));
            if let Some(vid_table) = video.as_table_mut() {
                vid_table.insert("format".to_string(), toml::Value::String(self.format_choisi.clone()));
                vid_table.insert("copie_flux".to_string(), toml::Value::Boolean(self.copie_flux));
            }
        } else if !self.save_video_format && self.module_actif == ModuleType::Video {
            #[cfg(feature = "api")]
            if let Some(vid_table) = parsed.get_mut("video").and_then(|v| v.as_table_mut()) {
                vid_table.remove("format");
            }
        }

        // Toujours sauvegarder copie_flux pour Video
        #[cfg(feature = "api")]
        if self.module_actif == ModuleType::Video {
            let video = parsed.entry("video").or_insert(toml::Value::Table(toml::Table::new()));
            if let Some(vid_table) = video.as_table_mut() {
                vid_table.insert("copie_flux".to_string(), toml::Value::Boolean(self.copie_flux));
            }
        }

        // Écrire le fichier
        let _ = std::fs::write("config.toml", toml::to_string(&parsed).unwrap_or_default());
    }

    fn apply_theme(&self, ctx: &egui::Context) {
        match self.current_theme.as_str() {
            "Light" => ctx.set_visuals(egui::Visuals::light()),
            "Dark" => ctx.set_visuals(egui::Visuals::dark()),
            _ => ctx.set_visuals(egui::Visuals::default()),
        }
    }

    fn verifier_deps(&mut self) {
        self.deps_manquantes = Vec::new();
    }

    fn lancer_action(&mut self, input: PathBuf) {
        let output = input.parent().unwrap().join(format!(
            "{}_oxyon.{}",
            input.file_stem().unwrap_or_default().to_string_lossy(),
            self.format_choisi
        ));
        let out_str = output.to_str().unwrap().to_string();
        let module = self.module_actif;
        let fmt = self.format_choisi.clone();
        let ratio = self.ratio_img;
        #[cfg(feature = "api")]
        let copie = self.copie_flux;
        
        // Copier les paramètres Image
        let img_action = self.image_action.clone();
        let angle = self.rotation_angle;
        let crop_x = self.crop_x;
        let crop_y = self.crop_y;
        let crop_w = self.crop_width;
        let crop_h = self.crop_height;
        let resize_w = self.resize_width.parse::<u32>().unwrap_or(0);
        let resize_h = self.resize_height.parse::<u32>().unwrap_or(0);
        let resize_kb = self.resize_max_kb.parse::<u32>().unwrap_or(0);
        
        // Copier les paramètres Doc
        let doc_action = self.doc_action.clone();
        let pdf_angle = self.pdf_rotation_angle;
        let pdf_pages = self.pdf_pages_spec.clone();
        let pdf_merge_list = self.current_files.clone();
        
        let status_arc = Arc::clone(&self.status);
        *self.status.lock().unwrap() = "🚀 Action en cours...".into();

        match module {
            #[cfg(feature = "api")]
            ModuleType::Video => {
                match modules::video::traiter_video(&input, &out_str, copie, false) {
                    Ok(c) => self.process = Some(c),
                    Err(e) => {
                        log_error(&format!("Vidéo: {:?} sur {:?}", e, input));
                        *self.status.lock().unwrap() = "❌ Erreur Vidéo (voir log)".into();
                    }
                }
            }
            #[cfg(feature = "api")]
            ModuleType::Audio => {
                match modules::audio::convertir(&input, &out_str, "192k") {
                    Ok(c) => self.process = Some(c),
                    Err(e) => {
                        log_error(&format!("Audio: {:?} sur {:?}", e, input));
                        *self.status.lock().unwrap() = "❌ Erreur Audio (voir log)".into();
                    }
                }
            }
            _ => {
                std::thread::spawn(move || {
                    let success = match module {
                        #[cfg(feature = "api")]
                        ModuleType::Archive => modules::archive::compresser(&input, &out_str, &fmt),
                        ModuleType::Doc => {
                            match doc_action.as_str() {
                                "convert" => {
                                    let format_entree = modules::doc::detecter_format_entree(&input);
                                    let format_sortie = modules::doc::detecter_format_sortie(&out_str);
                                    modules::doc::convertir_avec_formats(&input, &out_str, format_entree, format_sortie)
                                },
                                "pdf_split" => {
                                    let output_dir = input.parent().unwrap().join(format!(
                                        "{}_pages",
                                        input.file_stem().unwrap_or_default().to_string_lossy()
                                    ));
                                    std::fs::create_dir_all(&output_dir).ok();
                                    modules::doc::pdf_split(&input, output_dir.to_str().unwrap()).is_ok()
                                },
                                "pdf_merge" => {
                                    let paths: Vec<&Path> = pdf_merge_list.iter().map(|p| p.as_path()).collect();
                                    let output = input.parent().unwrap().join("merged_oxyon.pdf");
                                    modules::doc::pdf_merge(&paths, output.to_str().unwrap())
                                },
                                "pdf_rotate" => {
                                    let pages_opt = if pdf_pages == "1-end" { None } else { Some(pdf_pages.as_str()) };
                                    modules::doc::pdf_rotate(&input, &out_str, pdf_angle, pages_opt)
                                },
                                _ => modules::doc::convertir(&input, &out_str),
                            }
                        },
                        ModuleType::Image => {
                            match img_action.as_str() {
                                "convert" => modules::pic::compresser(&input, &out_str, ratio),
                                "resize" => {
                                    // Priorité : Si pixels ET poids, faire pixels puis poids
                                    if resize_w > 0 && resize_h > 0 {
                                        // Resize par pixels
                                        if resize_kb > 0 {
                                            // Puis compresser par poids
                                            let temp = format!("{}_temp.{}", out_str, fmt);
                                            if modules::pic::redimensionner_pixels(&input, &temp, resize_w, resize_h) {
                                                modules::pic::redimensionner_poids(Path::new(&temp), &out_str, resize_kb)
                                            } else {
                                                false
                                            }
                                        } else {
                                            modules::pic::redimensionner_pixels(&input, &out_str, resize_w, resize_h)
                                        }
                                    } else if resize_kb > 0 {
                                        // Seulement poids
                                        modules::pic::redimensionner_poids(&input, &out_str, resize_kb)
                                    } else {
                                        // Rien de spécifié, conversion simple
                                        modules::pic::compresser(&input, &out_str, 1)
                                    }
                                },
                                "rotate" => modules::pic::pivoter(&input, &out_str, angle),
                                "crop" => modules::pic::recadrer(&input, &out_str, crop_x, crop_y, crop_w, crop_h),
                                _ => modules::pic::compresser(&input, &out_str, ratio),
                            }
                        },
                        _ => true,
                    };

                    if !success {
                        log_error(&format!("Erreur Module {:?} sur {:?}", module, input));
                        *status_arc.lock().unwrap() = "❌ Erreur (voir log)".into();
                    } else {
                        *status_arc.lock().unwrap() = "✅ Terminé".into();
                    }
                });
            }
        }
    }
}

impl eframe::App for OxyonApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if ctx.cumulative_pass_nr() == 0 {
            self.load_config();
            self.apply_theme(ctx);
            self.verifier_deps();
        }

        ctx.input(|i| {
            if !i.raw.dropped_files.is_empty() {
                self.current_files = i.raw.dropped_files.iter().filter_map(|f| f.path.clone()).collect();
                if let Some(p) = self.current_files.first() {
                    self.current_stem = p.file_stem().unwrap_or_default().to_string_lossy().to_string();
                }
                #[cfg(feature = "api")]
                self.results_ui.lock().unwrap().clear();
                *self.status.lock().unwrap() = format!("📁 {} fichiers chargés", self.current_files.len());
            }
        });

        if let Some(ref mut c) = self.process {
            if let Ok(Some(_)) = c.try_wait() {
                *self.status.lock().unwrap() = "✅ Terminé".into();
                self.process = None;
            }
            ctx.request_repaint();
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| ui.heading(format!("OXYON v{}", VERSION)));

            if !self.deps_manquantes.is_empty() {
                ui.colored_label(egui::Color32::RED, format!("⚠️ Manquant : {}", self.deps_manquantes.join(", ")));
            }

            ui.separator();

            ui.horizontal_wrapped(|ui| {
                let mut mods = vec![];
                #[cfg(feature = "api")] mods.push((ModuleType::Archive, "📦 Archive"));
                #[cfg(feature = "api")] mods.push((ModuleType::Audio, "🎵 Audio"));
                mods.push((ModuleType::Doc, "📄 Doc"));
                mods.push((ModuleType::Image, "🖼️ Image"));
                #[cfg(feature = "api")] mods.push((ModuleType::Scrapper, "🔍 Scrapper"));
                #[cfg(feature = "api")] mods.push((ModuleType::Tag, "🏷️ Tag"));
                #[cfg(feature = "api")] mods.push((ModuleType::Video, "🎬 Vidéo"));
                mods.push((ModuleType::Settings, "⚙ Paramètres"));
                
                for (m, txt) in mods {
                    if ui.selectable_value(&mut self.module_actif, m, txt).clicked() {
                        self.load_config();
                    }
                }
            });

            ui.separator();

            match self.module_actif {
                #[cfg(feature = "api")]
                ModuleType::Archive => {
                    ui.horizontal(|ui| {
                        ui.label("Format :");
                        egui::ComboBox::from_id_salt("arfmt").selected_text(&self.format_choisi).show_ui(ui, |ui| {
                            for f in ["7z", "tar", "zip"] { 
                                ui.selectable_value(&mut self.format_choisi, f.into(), f);
                            }
                        });
                    });
                    if ui.checkbox(&mut self.save_archive_format, "💾 Sauvegarder ce format").changed() {
                        self.save_config();
                    }
                    ui.horizontal(|ui| { ui.label("Découpage (Mo) :"); ui.text_edit_singleline(&mut self.taille_vol); });
                },
                #[cfg(feature = "api")]
                ModuleType::Audio => {
                    ui.horizontal(|ui| {
                        ui.label("Format :");
                        egui::ComboBox::from_id_salt("afmt").selected_text(&self.format_choisi).show_ui(ui, |ui| {
                            for f in ["aac","flac","mp3","ogg","wav"] { 
                                ui.selectable_value(&mut self.format_choisi, f.into(), f);
                            }
                        });
                    });
                    if ui.checkbox(&mut self.save_audio_format, "💾 Sauvegarder ce format").changed() {
                        self.save_config();
                    }
                    if ui.button("🎵 Extraire Original (Auto)").clicked() {
                        for p in self.current_files.clone() {
                            let ext = modules::audio::detecter_extension(&p);
                            let out = p.with_extension(format!("extracted.{}", ext));
                            if let Ok(c) = modules::audio::extraire(&p, out.to_str().unwrap()) { self.process = Some(c); }
                        }
                    }
                },
                ModuleType::Doc => {
                    // Sélecteur d'action
                    ui.horizontal(|ui| {
                        ui.label("Action :");
                        egui::ComboBox::from_id_salt("doc_action").selected_text(&self.doc_action).show_ui(ui, |ui| {
                            ui.selectable_value(&mut self.doc_action, "convert".into(), "Convert");
                            ui.selectable_value(&mut self.doc_action, "pdf_split".into(), "PDF Split");
                            ui.selectable_value(&mut self.doc_action, "pdf_merge".into(), "PDF Merge");
                            ui.selectable_value(&mut self.doc_action, "pdf_rotate".into(), "PDF Rotate");
                        });
                    });
                    
                    ui.separator();
                    
                    // Options selon l'action choisie
                    match self.doc_action.as_str() {
                        "convert" => {
                            ui.horizontal(|ui| {
                                ui.label("Format :");
                                egui::ComboBox::from_id_salt("dfmt").selected_text(&self.format_choisi).show_ui(ui, |ui| {
                                    for f in ["docx","html","md","odt","tex","txt","pdf"] { 
                                        ui.selectable_value(&mut self.format_choisi, f.into(), f);
                                    }
                                });
                            });
                            if ui.checkbox(&mut self.save_doc_format, "💾 Sauvegarder ce format").changed() {
                                self.save_config();
                            }
                        },
                        "pdf_split" => {
                            ui.label("📄 Divise le PDF en pages individuelles");
                            ui.label("Les pages seront créées dans un sous-dossier");
                        },
                        "pdf_merge" => {
                            ui.label("📦 Fusionne plusieurs PDFs");
                            ui.label("Ajoutez plusieurs fichiers PDF via drag & drop");
                            if !self.current_files.is_empty() {
                                ui.label(format!("Fichiers à fusionner : {}", self.current_files.len()));
                            }
                        },
                        "pdf_rotate" => {
                            ui.horizontal(|ui| {
                                ui.label("Angle :");
                                egui::ComboBox::from_id_salt("pdf_rot_angle").selected_text(format!("{}°", self.pdf_rotation_angle)).show_ui(ui, |ui| {
                                    ui.selectable_value(&mut self.pdf_rotation_angle, 90, "90°");
                                    ui.selectable_value(&mut self.pdf_rotation_angle, 180, "180°");
                                    ui.selectable_value(&mut self.pdf_rotation_angle, 270, "270°");
                                });
                            });
                            ui.horizontal(|ui| {
                                ui.label("Pages :");
                                ui.text_edit_singleline(&mut self.pdf_pages_spec);
                                ui.label("(ex: 1-end, 1-3, even, odd)");
                            });
                        },
                        _ => {}
                    }
                },
                ModuleType::Image => {
                    // Sélecteur d'action
                    ui.horizontal(|ui| {
                        ui.label("Action :");
                        egui::ComboBox::from_id_salt("img_action").selected_text(&self.image_action).show_ui(ui, |ui| {
                            ui.selectable_value(&mut self.image_action, "convert".into(), "Convert");
                            ui.selectable_value(&mut self.image_action, "resize".into(), "Resize");
                            ui.selectable_value(&mut self.image_action, "rotate".into(), "Rotate");
                            ui.selectable_value(&mut self.image_action, "crop".into(), "Crop");
                        });
                    });
                    
                    ui.separator();
                    
                    // Options selon l'action choisie
                    match self.image_action.as_str() {
                        "convert" => {
                            ui.horizontal(|ui| {
                                ui.label("Format :");
                                egui::ComboBox::from_id_salt("ifmt").selected_text(&self.format_choisi).show_ui(ui, |ui| {
                                    for f in ["AVIF","DNG","EXR","GIF","HDR","HEIC","ICO","JPG","JXL","PNG","PSD","RAW","SVG","TIFF","WebP"] { 
                                        ui.selectable_value(&mut self.format_choisi, f.into(), f);
                                    }
                                });
                            });
                            if ui.checkbox(&mut self.save_image_format, "💾 Sauvegarder ce format").changed() {
                                self.save_config();
                            }
                            if ui.add(egui::Slider::new(&mut self.ratio_img, 1..=10).text("Qualité/Ratio")).changed() { 
                                self.save_config(); 
                            }
                        },
                        "resize" => {
                            ui.horizontal(|ui| {
                                ui.label("Format :");
                                egui::ComboBox::from_id_salt("ifmt_resize").selected_text(&self.format_choisi).show_ui(ui, |ui| {
                                    for f in ["AVIF","DNG","EXR","GIF","HDR","HEIC","ICO","JPG","JXL","PNG","PSD","RAW","SVG","TIFF","WebP"] { 
                                        ui.selectable_value(&mut self.format_choisi, f.into(), f);
                                    }
                                });
                            });
                            if ui.checkbox(&mut self.save_image_format, "💾 Sauvegarder ce format").changed() {
                                self.save_config();
                            }
                            ui.separator();
                            ui.label("Redimensionner par taille (pixels) :");
                            ui.horizontal(|ui| {
                                ui.label("Largeur :");
                                ui.text_edit_singleline(&mut self.resize_width);
                                ui.label("Hauteur :");
                                ui.text_edit_singleline(&mut self.resize_height);
                            });
                            ui.label("ET/OU");
                            ui.horizontal(|ui| {
                                ui.label("Poids max (Ko) :");
                                ui.text_edit_singleline(&mut self.resize_max_kb);
                            });
                        },
                        "rotate" => {
                            ui.horizontal(|ui| {
                                ui.label("Angle :");
                                egui::ComboBox::from_id_salt("rot_angle").selected_text(format!("{}°", self.rotation_angle)).show_ui(ui, |ui| {
                                    ui.selectable_value(&mut self.rotation_angle, 90, "90°");
                                    ui.selectable_value(&mut self.rotation_angle, 180, "180°");
                                    ui.selectable_value(&mut self.rotation_angle, 270, "270°");
                                });
                            });
                        },
                        "crop" => {
                            ui.label("Coordonnées en % (0-100) :");
                            ui.horizontal(|ui| {
                                ui.label("X:");
                                ui.add(egui::Slider::new(&mut self.crop_x, 0..=100));
                                ui.label("Y:");
                                ui.add(egui::Slider::new(&mut self.crop_y, 0..=100));
                            });
                            ui.horizontal(|ui| {
                                ui.label("Largeur:");
                                ui.add(egui::Slider::new(&mut self.crop_width, 1..=100));
                                ui.label("Hauteur:");
                                ui.add(egui::Slider::new(&mut self.crop_height, 1..=100));
                            });
                        },
                        _ => {}
                    }
                },
                #[cfg(feature = "api")]
                ModuleType::Video => {
                    ui.horizontal(|ui| {
                        egui::ComboBox::from_id_salt("vfmt").selected_text(&self.format_choisi).show_ui(ui, |ui| {
                            for f in ["mkv","mp4","webm"] { 
                                ui.selectable_value(&mut self.format_choisi, f.into(), f);
                            }
                        });
                        if ui.checkbox(&mut self.copie_flux, "Copie flux").changed() { self.save_config(); }
                    });
                    if ui.checkbox(&mut self.save_video_format, "💾 Sauvegarder ce format").changed() {
                        self.save_config();
                    }
                },
                #[cfg(feature = "api")]
                ModuleType::Scrapper => {
                    ui.horizontal(|ui| {
                        if ui.button("🎬 Film").clicked() {
                            let res_arc = Arc::clone(&self.results_ui);
                            let stem = self.current_stem.clone();
                            let ctx_c = ctx.clone();
                            res_arc.lock().unwrap().clear();
                            std::thread::spawn(move || {
                                if let Ok(results) = modules::scrap::search_tmdb(&stem, false) {
                                    for r in results {
                                        let tex = r.poster_path.as_ref()
                                            .and_then(|p| modules::scrap::download_image_bytes(p))
                                            .and_then(|b| image::load_from_memory(&b).ok())
                                            .map(|img| {
                                                let ci = egui::ColorImage::from_rgba_unmultiplied(
                                                    [img.width() as usize, img.height() as usize],
                                                    img.to_rgba8().as_flat_samples().as_slice()
                                                );
                                                ctx_c.load_texture(format!("p_{}", r.id), ci, Default::default())
                                            });
                                        res_arc.lock().unwrap().push(ScrapeEntry { data: r, texture: tex });
                                        ctx_c.request_repaint();
                                    }
                                }
                            });
                        }
                    });
                    let entries = self.results_ui.lock().unwrap().clone();
                    for entry in entries {
                        ui.horizontal(|ui| {
                            if let Some(t) = &entry.texture { ui.image((t.id(), egui::vec2(50.0, 75.0))); }
                            ui.label(&entry.data.title);
                            if !self.current_files.is_empty() {
                                if ui.button("Choisir").clicked() { modules::scrap::save_metadata(self.current_files[0].clone(), entry.data.clone(), true); }
                            }
                        });
                    }
                },
                #[cfg(feature = "api")]
                ModuleType::Tag => {
                    let path_opt = self.current_files.get(0).cloned();
                    ui.vertical(|ui| {
                        if ui.button("✅ Marquer VU").clicked() { if let Some(path) = &path_opt { let _ = modules::tag::marquer_vu(&path, &path.with_extension("nfo")); } }
                        if ui.button("📥 Injecter tags depuis NFO").clicked() { if let Some(path) = &path_opt { let _ = modules::tag::appliquer_tags(&path, &path.with_extension("nfo")); } }
                        if ui.button("🖼️ Ajouter poster / fanart").clicked() { if let Some(path) = &path_opt { let _ = modules::tag::ajouter_images_mkv(&path); } }
                        if ui.button("🗑️ Reset Tags").clicked() { if let Some(path) = &path_opt { let _ = modules::tag::supprimer_tous_tags(&path); } }
                        ui.horizontal(|ui| {
                            ui.text_edit_singleline(&mut self.tag_edit_val);
                            if ui.button("✏️ Modifier Titre").clicked() { if let Some(path) = &path_opt { let _ = modules::tag::modifier_tag(&path, "title", &self.tag_edit_val); } }
                        });
                    });
                },
                ModuleType::Settings => {
                    ui.vertical(|ui| {
                        ui.heading("Paramètres");
                        let old_theme = self.current_theme.clone();
                        ui.horizontal(|ui| {
                            ui.label("Thème :");
                            egui::ComboBox::from_id_salt("theme_sel").selected_text(&self.current_theme).show_ui(ui, |ui| {
                                ui.selectable_value(&mut self.current_theme, "Auto".into(), "Auto");
                                ui.selectable_value(&mut self.current_theme, "Light".into(), "Light");
                                ui.selectable_value(&mut self.current_theme, "Dark".into(), "Dark");
                            });
                        });
                        if self.current_theme != old_theme {
                            self.apply_theme(ctx);
                            self.save_config();
                        }
                    });
                },
            }

            let mut hide_exec = self.module_actif == ModuleType::Settings;
            #[cfg(feature = "api")]
            { hide_exec = hide_exec || self.module_actif == ModuleType::Scrapper || self.module_actif == ModuleType::Tag; }

            if !self.current_files.is_empty() && !hide_exec {
                ui.separator();
                if ui.button("🔥 EXÉCUTER TOUT").clicked() { for p in self.current_files.clone() { self.lancer_action(p); } }
            }

            if self.current_files.is_empty() { ui.centered_and_justified(|ui| { ui.label("📥 Glissez vos fichiers ici"); }); }
            ui.add_space(10.0);
            ui.vertical_centered(|ui| { ui.heading(&*self.status.lock().unwrap()); });
            if !self.current_files.is_empty() { if ui.button("🗑️ Tout effacer").clicked() { self.current_files.clear(); } }
        });
    }
}

fn main() -> eframe::Result {
    let _ = modules::binaries::extraire_deps();
    let mut options = eframe::NativeOptions::default();
    let icon_bytes = include_bytes!("../assets/oxyon_icon.ico");
    if let Ok(icon_data) = image::load_from_memory(icon_bytes) {
        let icon_rgba = icon_data.to_rgba8();
        let (width, height) = icon_rgba.dimensions();
        options.viewport.icon = Some(std::sync::Arc::new(egui::IconData { rgba: icon_rgba.into_raw(), width, height }));
    }
    
    let result = eframe::run_native(
        &format!("OXYON v{}", VERSION),
        options,
        Box::new(|cc| {
            let mut app = OxyonApp::default();
            app.load_config();
            app.apply_theme(&cc.egui_ctx);
            Ok(Box::new(app))
        }),
    );
    
    modules::binaries::cleanup();
    result
}