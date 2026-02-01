#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod modules;
use eframe::egui;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
struct ScrapeEntry {
    data: modules::scrap::ScrapeResult,
    texture: Option<egui::TextureHandle>,
}

#[derive(PartialEq, Clone, Copy)]
enum ModuleType {
    Archive,
    Audio,
    Doc,
    Image,
    Scrapper,
    Tag,
    Video,
    Settings,
}

struct OxyonApp {
    copie_flux: bool,
    current_files: Vec<PathBuf>,
    current_stem: String,
    format_choisi: String,
    module_actif: ModuleType,
    process: Option<std::process::Child>,
    ratio_img: u32,
    results_ui: Arc<Mutex<Vec<ScrapeEntry>>>,
    status: Arc<Mutex<String>>,
    taille_vol: String,
    deps_manquantes: Vec<String>,
    tag_edit_val: String,
    current_theme: String,
}

impl Default for OxyonApp {
    fn default() -> Self {
        Self {
            copie_flux: false,
            current_files: Vec::new(),
            current_stem: String::new(),
            format_choisi: "mp4".into(),
            module_actif: ModuleType::Video,
            process: None,
            ratio_img: 2,
            results_ui: Arc::new(Mutex::new(Vec::new())),
            status: Arc::new(Mutex::new("Déposez des fichiers".into())),
            taille_vol: "".into(),
            deps_manquantes: Vec::new(),
            tag_edit_val: String::new(),
            current_theme: "Auto".into(),
        }
    }
}

impl OxyonApp {
    fn load_config(&mut self) {
        if let Ok(c) = std::fs::read_to_string("config.toml") {
            if let Ok(parsed) = c.parse::<toml::Table>() {
                if let Some(theme) = parsed.get("display").and_then(|d| d.get("theme")).and_then(|t| t.as_str()) {
                    self.current_theme = theme.to_string();
                }
            }
        }
    }

    fn save_config(&self) {
        let content = format!("[display]\ntheme = \"{}\"\n", self.current_theme);
        let _ = std::fs::write("config.toml", content);
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
        let copie = self.copie_flux;
        *self.status.lock().unwrap() = "🚀 Action en cours...".into();

        match module {
            ModuleType::Video => {
                if let Ok(c) = modules::video::traiter_video(&input, &out_str, copie, false) {
                    self.process = Some(c);
                }
            }
            ModuleType::Audio => {
                if let Ok(c) = modules::audio::convertir(&input, &out_str, "192k") {
                    self.process = Some(c);
                }
            }
            _ => {
                std::thread::spawn(move || match module {
                    ModuleType::Archive => { let _ = modules::archive::compresser(&input, &out_str, &fmt); }
                    ModuleType::Doc => { let _ = modules::doc::convertir(&input, &out_str); }
                    ModuleType::Image => { let _ = modules::image::compresser(&input, &out_str, ratio); }
                    _ => {}
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
            ui.vertical_centered(|ui| ui.heading("🚀 OXYON ALL-IN-ONE"));

            if !self.deps_manquantes.is_empty() {
                ui.colored_label(egui::Color32::RED, format!("⚠️ Manquant : {}", self.deps_manquantes.join(", ")));
            }

            ui.separator();

            ui.horizontal_wrapped(|ui| {
                let mods = vec![
                    (ModuleType::Archive, "📦 Archive"),
                    (ModuleType::Audio, "🎵 Audio"),
                    (ModuleType::Doc, "📄 Doc"),
                    (ModuleType::Image, "🖼️ Image"),
                    (ModuleType::Scrapper, "🔍 Scrapper"),
                    (ModuleType::Tag, "🏷️ Tag"),
                    (ModuleType::Video, "🎬 Vidéo"),
                    (ModuleType::Settings, "⚙ Paramètres"),
                ];
                for (m, txt) in mods {
                    if ui.selectable_value(&mut self.module_actif, m, txt).clicked() {
                        self.format_choisi = match m {
                            ModuleType::Archive => "7z".into(),
                            ModuleType::Audio => "aac".into(),
                            ModuleType::Doc => "csv".into(),
                            ModuleType::Image => "GIF".into(),
                            ModuleType::Scrapper => "nfo".into(),
                            ModuleType::Tag => "mkv".into(),
                            ModuleType::Video => "avi".into(),
                            _ => self.format_choisi.clone(),
                        };
                    }
                }
            });

            ui.separator();

            match self.module_actif {
                ModuleType::Archive => {
                    ui.horizontal(|ui| {
                        ui.label("Format :");
                        egui::ComboBox::from_id_salt("arfmt").selected_text(&self.format_choisi).show_ui(ui, |ui| {
                            for f in ["7z", "tar", "zip"] { ui.selectable_value(&mut self.format_choisi, f.into(), f); }
                        });
                    });
                    ui.horizontal(|ui| { ui.label("Découpage (Mo) :"); ui.text_edit_singleline(&mut self.taille_vol); });
                },
                ModuleType::Audio => {
                    ui.horizontal(|ui| {
                        ui.label("Format :");
                        egui::ComboBox::from_id_salt("afmt").selected_text(&self.format_choisi).show_ui(ui, |ui| {
                            for f in ["aac","flac","mp3","ogg","wav"] { ui.selectable_value(&mut self.format_choisi, f.into(), f); }
                        });
                    });
                    if ui.button("🎵 Extraire Original (Auto)").clicked() {
                        for p in self.current_files.clone() {
                            let ext = modules::audio::detecter_extension(&p);
                            let out = p.with_extension(format!("extracted.{}", ext));
                            if let Ok(c) = modules::audio::extraire(&p, out.to_str().unwrap()) { self.process = Some(c); }
                        }
                    }
                },
                ModuleType::Doc => {
                    ui.horizontal(|ui| {
                        ui.label("Format :");
                        egui::ComboBox::from_id_salt("dfmt").selected_text(&self.format_choisi).show_ui(ui, |ui| {
                            for f in ["pdf","docx","txt"] { ui.selectable_value(&mut self.format_choisi, f.into(), f); }
                        });
                    });
                },
                ModuleType::Image => {
                    ui.horizontal(|ui| {
                        ui.label("Format :");
                        egui::ComboBox::from_id_salt("ifmt").selected_text(&self.format_choisi).show_ui(ui, |ui| {
                            for f in ["GIF","JPG","PNG","WebP"] { ui.selectable_value(&mut self.format_choisi, f.into(), f); }
                        });
                    });
                    ui.add(egui::Slider::new(&mut self.ratio_img, 1..=10).text("Qualité"));
                },
                ModuleType::Video => {
                    ui.horizontal(|ui| {
                        egui::ComboBox::from_id_salt("vfmt").selected_text(&self.format_choisi).show_ui(ui, |ui| {
                            for f in ["mkv","mp4","webm"] { ui.selectable_value(&mut self.format_choisi, f.into(), f); }
                        });
                        ui.checkbox(&mut self.copie_flux, "Copie flux");
                    });
                },
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

            if !self.current_files.is_empty() && self.module_actif != ModuleType::Scrapper && self.module_actif != ModuleType::Tag && self.module_actif != ModuleType::Settings {
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
        "OXYON Multi-Tool",
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