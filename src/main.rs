#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
const VERSION: &str = env!("CARGO_PKG_VERSION");
pub mod modules;
#[cfg(test)]
#[path = "test.rs"]
#[cfg(test)]
#[path = "testmkvwebm.rs"]
mod test;
use eframe::egui;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::fs::OpenOptions;
use std::io::Write;

// ─────────────────────────────────────────────
//  SYSTÈME DE LOG
// ─────────────────────────────────────────────
fn log_entry(level: &str, message: &str) {
    let timestamp = chrono::Local::now().format("%Y-%m-%d %H:%M:%S");
    let thread = std::thread::current();
    let thread_name = thread.name().unwrap_or("worker");
    let log_line = format!("[{}] [{}] [{}] {}\n", timestamp, level, thread_name, message);
    if let Ok(mut file) = OpenOptions::new()
        .create(true)
        .append(true)
        .open("oxyon.log")
    {
        let _ = file.write_all(log_line.as_bytes());
    }
    // Aussi visible en console debug
    eprintln!("{}", log_line.trim());
}

fn log_info(msg: &str)  { log_entry("INFO",  msg); }
fn log_warn(msg: &str)  { log_entry("WARN",  msg); }
fn log_error(msg: &str) { log_entry("ERROR", msg); }

// ─────────────────────────────────────────────

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
        deps_manquantes: Vec<String>,
        #[cfg(feature = "api")]
        audio_action: String,
        #[cfg(feature = "api")]
        audio_formats_dispo: Vec<String>,
        #[cfg(feature = "api")]
        tag_edit_val: String,
        current_theme: String,
        tmdb_api_key: String,
        fanart_api_key: String,
        save_doc_format: bool,
        save_image_format: bool,
        #[cfg(feature = "api")]
        save_archive_format: bool,
        #[cfg(feature = "api")]
        archive_niveau: u32,
        #[cfg(feature = "api")]
        save_audio_format: bool,
        #[cfg(feature = "api")]
        audio_qualite: u32,
        #[cfg(feature = "api")]
        save_video_format: bool,
        #[cfg(feature = "api")]
        video_speed: u32,
        image_action: String,
        rotation_angle: u32,
        crop_x: u32,
        crop_y: u32,
        crop_width: u32,
        crop_height: u32,
        resize_width: String,
        resize_height: String,
        resize_max_kb: String,
        doc_action: String,
        pdf_rotation_angle: u16,
        pdf_pages_spec: String,
        pdf_crop_x: f64,
        pdf_crop_y: f64,
        pdf_crop_w: f64,
        pdf_crop_h: f64,
        pdf_num_debut: u32,
        pdf_num_position: String,
        pdf_num_taille: f64,
        pdf_owner_pass: String,
        pdf_user_pass: String,
        pdf_allow_print: bool,
        pdf_allow_copy: bool,
        pdf_unlock_pass: String,
        pdf_wm_texte: String,
        pdf_wm_taille: f64,
        pdf_wm_opacite: f64,
        pdf_nouvel_ordre: String,
        max_parallel_jobs: usize,
        active_jobs: Arc<Mutex<usize>>,
        completed_jobs: Arc<Mutex<usize>>,
        total_jobs: Arc<Mutex<usize>>,
        job_queue: Arc<Mutex<Vec<PathBuf>>>,
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
                status: Arc::new(Mutex::new("Drop files here".into())),
                deps_manquantes: Vec::new(),
                #[cfg(feature = "api")]
                audio_action: "Convert".into(),
                #[cfg(feature = "api")]
                audio_formats_dispo: vec!["aac","flac","m4a","mp3","ogg","opus","wav"].into_iter().map(String::from).collect(),
                #[cfg(feature = "api")]
                tag_edit_val: String::new(),
                current_theme: "Dark".into(),
                tmdb_api_key: String::new(),
                fanart_api_key: String::new(),
                save_doc_format: false,
                save_image_format: false,
                #[cfg(feature = "api")]
                save_archive_format: false,
                #[cfg(feature = "api")]
                archive_niveau: 6,
                #[cfg(feature = "api")]
                save_audio_format: false,
                #[cfg(feature = "api")]
                audio_qualite: 2,
                #[cfg(feature = "api")]
                save_video_format: false,
                #[cfg(feature = "api")]
                video_speed: 4,
                image_action: "Convert".into(),
                rotation_angle: 90,
                crop_x: 0,
                crop_y: 0,
                crop_width: 100,
                crop_height: 100,
                resize_width: String::new(),
                resize_height: String::new(),
                resize_max_kb: String::new(),
                doc_action: "Convert".into(),
                pdf_rotation_angle: 90,
                pdf_pages_spec: "1-end".into(),
                pdf_crop_x: 0.0,
                pdf_crop_y: 0.0,
                pdf_crop_w: 100.0,
                pdf_crop_h: 100.0,
                pdf_num_debut: 1,
                pdf_num_position: "BasCentre".into(),
                pdf_num_taille: 10.0,
                pdf_owner_pass: String::new(),
                pdf_user_pass: String::new(),
                pdf_allow_print: true,
                pdf_allow_copy: true,
                pdf_unlock_pass: String::new(),
                pdf_wm_texte: "CONFIDENTIEL".into(),
                pdf_wm_taille: 60.0,
                pdf_wm_opacite: 0.15,
                pdf_nouvel_ordre: String::new(),
                max_parallel_jobs: 4,
                active_jobs: Arc::new(Mutex::new(0)),
                completed_jobs: Arc::new(Mutex::new(0)),
                total_jobs: Arc::new(Mutex::new(0)),
                job_queue: Arc::new(Mutex::new(Vec::new())),
        }
    }
}
impl OxyonApp {
    fn load_config(&mut self) {
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
        if let Ok(c) = std::fs::read_to_string("config.toml") {
            if let Ok(parsed) = c.parse::<toml::Table>() {
                if let Some(theme) = parsed.get("display").and_then(|d| d.get("theme")).and_then(|t| t.as_str()) {
                    self.current_theme = theme.to_string();
                }
                if let Some(max_jobs) = parsed.get("performance").and_then(|p| p.get("max_parallel_jobs")).and_then(|j| j.as_integer()) {
                    self.max_parallel_jobs = max_jobs as usize;
                }
                if let Some(doc) = parsed.get("doc") {
                    if let Some(fmt) = doc.get("format").and_then(|f| f.as_str()) {
                        if self.module_actif == ModuleType::Doc {
                            self.format_choisi = fmt.to_string();
                        }
                    }
                }
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
                #[cfg(feature = "api")]
                if let Some(arc) = parsed.get("archive") {
                    if let Some(fmt) = arc.get("format").and_then(|f| f.as_str()) {
                        if self.module_actif == ModuleType::Archive {
                            self.format_choisi = fmt.to_string();
                        }
                    }
                    if let Some(n) = arc.get("niveau").and_then(|v| v.as_integer()) {
                        self.archive_niveau = n as u32;
                    }
                }
                #[cfg(feature = "api")]
                if let Some(aud) = parsed.get("audio") {
                    if let Some(fmt) = aud.get("format").and_then(|f| f.as_str()) {
                        if self.module_actif == ModuleType::Audio {
                            self.format_choisi = fmt.to_string();
                        }
                    }
                    if let Some(q) = aud.get("qualite").and_then(|v| v.as_integer()) {
                        self.audio_qualite = q as u32;
                    }
                }
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
                    if let Some(speed) = vid.get("speed").and_then(|s| s.as_integer()) {
                        self.video_speed = speed as u32;
                    }
                }
            }
        }
        dotenvy::dotenv().ok();
        if let Ok(k) = std::env::var("TMDB_API_KEY") { self.tmdb_api_key = k; }
        if let Ok(k) = std::env::var("FANART_API_KEY") { self.fanart_api_key = k; }
    }
    fn save_config(&self) {
        let mut parsed = if let Ok(c) = std::fs::read_to_string("config.toml") {
            c.parse::<toml::Table>().unwrap_or_else(|_| toml::Table::new())
        } else {
            toml::Table::new()
        };
        let display = parsed.entry("display").or_insert(toml::Value::Table(toml::Table::new()));
        if let Some(display_table) = display.as_table_mut() {
            display_table.insert("theme".to_string(), toml::Value::String(self.current_theme.clone()));
        }
        let perf = parsed.entry("performance").or_insert(toml::Value::Table(toml::Table::new()));
        if let Some(perf_table) = perf.as_table_mut() {
            perf_table.insert("max_parallel_jobs".to_string(), toml::Value::Integer(self.max_parallel_jobs as i64));
        }
        if self.save_doc_format && !self.format_choisi.is_empty() && self.module_actif == ModuleType::Doc {
            let doc = parsed.entry("doc").or_insert(toml::Value::Table(toml::Table::new()));
            if let Some(doc_table) = doc.as_table_mut() {
                doc_table.insert("format".to_string(), toml::Value::String(self.format_choisi.clone()));
            }
        } else if !self.save_doc_format && self.module_actif == ModuleType::Doc {
            if let Some(doc_table) = parsed.get_mut("doc").and_then(|v| v.as_table_mut()) {
                doc_table.remove("format");
            }
        }
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
        if self.module_actif == ModuleType::Image {
            let image = parsed.entry("image").or_insert(toml::Value::Table(toml::Table::new()));
            if let Some(img_table) = image.as_table_mut() {
                img_table.insert("ratio_img".to_string(), toml::Value::Integer(self.ratio_img as i64));
            }
        }
        #[cfg(feature = "api")]
        {
            if self.save_archive_format && !self.format_choisi.is_empty() && self.module_actif == ModuleType::Archive {
                let archive = parsed.entry("archive").or_insert(toml::Value::Table(toml::Table::new()));
                if let Some(arc_table) = archive.as_table_mut() {
                    arc_table.insert("format".to_string(), toml::Value::String(self.format_choisi.clone()));
                }
            } else if !self.save_archive_format && self.module_actif == ModuleType::Archive {
                if let Some(arc_table) = parsed.get_mut("archive").and_then(|v| v.as_table_mut()) {
                    arc_table.remove("format");
                }
            }
            if self.module_actif == ModuleType::Archive {
                let archive = parsed.entry("archive").or_insert(toml::Value::Table(toml::Table::new()));
                if let Some(arc_table) = archive.as_table_mut() {
                    arc_table.insert("niveau".to_string(), toml::Value::Integer(self.archive_niveau as i64));
                }
            }
            if self.save_audio_format && !self.format_choisi.is_empty() && self.module_actif == ModuleType::Audio {
                let audio = parsed.entry("audio").or_insert(toml::Value::Table(toml::Table::new()));
                if let Some(aud_table) = audio.as_table_mut() {
                    aud_table.insert("format".to_string(), toml::Value::String(self.format_choisi.clone()));
                }
            } else if !self.save_audio_format && self.module_actif == ModuleType::Audio {
                if let Some(aud_table) = parsed.get_mut("audio").and_then(|v| v.as_table_mut()) {
                    aud_table.remove("format");
                }
            }
            if self.module_actif == ModuleType::Audio {
                let audio = parsed.entry("audio").or_insert(toml::Value::Table(toml::Table::new()));
                if let Some(aud_table) = audio.as_table_mut() {
                    aud_table.insert("qualite".to_string(), toml::Value::Integer(self.audio_qualite as i64));
                }
            }
            if self.save_video_format && !self.format_choisi.is_empty() && self.module_actif == ModuleType::Video {
                let video = parsed.entry("video").or_insert(toml::Value::Table(toml::Table::new()));
                if let Some(vid_table) = video.as_table_mut() {
                    vid_table.insert("format".to_string(), toml::Value::String(self.format_choisi.clone()));
                }
            } else if !self.save_video_format && self.module_actif == ModuleType::Video {
                if let Some(vid_table) = parsed.get_mut("video").and_then(|v| v.as_table_mut()) {
                    vid_table.remove("format");
                }
            }
            if self.module_actif == ModuleType::Video {
                let video = parsed.entry("video").or_insert(toml::Value::Table(toml::Table::new()));
                if let Some(vid_table) = video.as_table_mut() {
                    vid_table.insert("copie_flux".to_string(), toml::Value::Boolean(self.copie_flux));
                    vid_table.insert("speed".to_string(), toml::Value::Integer(self.video_speed as i64));
                }
            }
        }
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
    fn lancer_batch(&mut self, ctx: egui::Context) {
        log_info(&format!(
            "=== BATCH START | {} fichier(s) | {} workers max | module={:?} | action={} ===",
            self.current_files.len(),
            self.max_parallel_jobs,
            self.module_actif,
            match self.module_actif {
                ModuleType::Doc   => self.doc_action.as_str(),
                ModuleType::Image => self.image_action.as_str(),
                _                 => "-",
            }
        ));
        for f in &self.current_files {
            log_info(&format!("  Fichier en queue: {:?}", f));
        }

        *self.completed_jobs.lock().unwrap() = 0;
        *self.total_jobs.lock().unwrap() = self.current_files.len();
        *self.active_jobs.lock().unwrap() = 0;
        let mut queue = self.job_queue.lock().unwrap();
        queue.clear();
        queue.extend(self.current_files.clone());
        drop(queue);
        *self.status.lock().unwrap() = format!("🚀 Starting {} tasks...", self.current_files.len());
        for _ in 0..self.max_parallel_jobs.min(self.current_files.len()) {
            self.spawn_worker(ctx.clone());
        }
    }
    fn spawn_worker(&mut self, ctx: egui::Context) {
        let queue = Arc::clone(&self.job_queue);
        let active = Arc::clone(&self.active_jobs);
        let completed = Arc::clone(&self.completed_jobs);
        let total = Arc::clone(&self.total_jobs);
        let status_arc = Arc::clone(&self.status);
        let module = self.module_actif;
        let fmt = self.format_choisi.clone();
        let ratio = self.ratio_img;
        #[cfg(feature = "api")]
        let copie = self.copie_flux;
        #[cfg(feature = "api")]
        let video_speed = self.video_speed;
        #[cfg(feature = "api")]
        let audio_action = self.audio_action.clone();
        #[cfg(feature = "api")]
        let audio_qualite = self.audio_qualite;
        #[cfg(feature = "api")]
        let archive_niveau = self.archive_niveau;
        let img_action = self.image_action.clone();
        let angle = self.rotation_angle;
        let crop_x = self.crop_x;
        let crop_y = self.crop_y;
        let crop_w = self.crop_width;
        let crop_h = self.crop_height;
        let resize_w = self.resize_width.parse::<u32>().unwrap_or(0);
        let resize_h = self.resize_height.parse::<u32>().unwrap_or(0);
        let resize_kb = self.resize_max_kb.parse::<u32>().unwrap_or(0);
        let doc_action = self.doc_action.clone();
        let pdf_angle = self.pdf_rotation_angle;
        let pdf_pages = self.pdf_pages_spec.clone();
        let pdf_merge_list = self.current_files.clone();
        let pdf_crop_x = self.pdf_crop_x;
        let pdf_crop_y = self.pdf_crop_y;
        let pdf_crop_w = self.pdf_crop_w;
        let pdf_crop_h = self.pdf_crop_h;
        let pdf_num_debut = self.pdf_num_debut;
        let pdf_num_position = self.pdf_num_position.clone();
        let pdf_num_taille = self.pdf_num_taille;
        let pdf_owner_pass = self.pdf_owner_pass.clone();
        let pdf_user_pass = self.pdf_user_pass.clone();
        let pdf_allow_print = self.pdf_allow_print;
        let pdf_allow_copy = self.pdf_allow_copy;
        let pdf_unlock_pass = self.pdf_unlock_pass.clone();
        let pdf_wm_texte = self.pdf_wm_texte.clone();
        let pdf_wm_taille = self.pdf_wm_taille;
        let pdf_wm_opacite = self.pdf_wm_opacite;
        let pdf_nouvel_ordre = self.pdf_nouvel_ordre.clone();
        std::thread::spawn(move || {
            loop {
                let job = {
                    let mut q = queue.lock().unwrap();
                    q.pop()
                };
                let input = match job {
                    Some(path) => path,
                    None => break,
                };
                *active.lock().unwrap() += 1;
                let effective_fmt = if module == ModuleType::Doc && doc_action != "Convert" {
                    "pdf".to_string()
                } else {
                    fmt.clone()
                };
                let output = input.parent().unwrap().join(format!(
                    "{}_oxyon.{}",
                    input.file_stem().unwrap_or_default().to_string_lossy(),
                    effective_fmt
                ));
                let out_str = output.to_str().unwrap().to_string();

                // ── Timing ──────────────────────────────────────────────
                let start = std::time::Instant::now();
                log_info(&format!(
                    "START | module={:?} | fichier={:?} | sortie={:?}",
                    module, input, out_str
                ));

                let current = *completed.lock().unwrap() + *active.lock().unwrap();
                let total_count = *total.lock().unwrap();
                *status_arc.lock().unwrap() = format!("⚙️ Processing {}/{} files...", current, total_count);
                ctx.request_repaint();

                // ── Exécution avec résultat détaillé ────────────────────
                let result: Result<(), String> = match module {
                    #[cfg(feature = "api")]
                    ModuleType::Archive => {
                        log_info(&format!("Archive: compression fmt={} niveau={} | {:?}", fmt, archive_niveau, input));
                        if modules::archive::compresser(&input, &out_str, &fmt, archive_niveau) {
                            Ok(())
                        } else {
                            Err(format!("compresser() returned false | fmt={} | file={:?}", fmt, input))
                        }
                    },
                    #[cfg(feature = "api")]
                    ModuleType::Audio => {
                        match audio_action.as_str() {
                            "extract" => {
                                log_info(&format!("Audio: extraction | {:?}", input));
                                let ext = modules::audio::detecter_extension(&input);
                                let extract_out = input.parent().unwrap().join(format!(
                                    "{}_oxyon.{}",
                                    input.file_stem().unwrap_or_default().to_string_lossy(),
                                    if ext.is_empty() { "mka".to_string() } else { ext }
                                ));
                                let extract_str = extract_out.to_str().unwrap().to_string();
                                match modules::audio::extraire(&input, &extract_str) {
                                    Ok(mut child) => {
                                        match child.wait() {
                                            Ok(status) if status.success() => Ok(()),
                                            Ok(status) => Err(format!("audio extraction process exited with code={:?}", status.code())),
                                            Err(e) => Err(format!("erreur wait() extraction audio: {}", e)),
                                        }
                                    },
                                    Err(e) => Err(format!("impossible de lancer ffmpeg extraction: {}", e)),
                                }
                            },
                            _ => {
                                log_info(&format!("Audio: conversion | {:?}", input));
                                match modules::audio::convertir(&input, &out_str, audio_qualite) {
                                    Ok(mut child) => {
                                        match child.wait() {
                                            Ok(status) if status.success() => Ok(()),
                                            Ok(status) => Err(format!("audio process exited with code={:?}", status.code())),
                                            Err(e) => Err(format!("erreur wait() audio: {}", e)),
                                        }
                                    },
                                    Err(e) => Err(format!("impossible de lancer ffmpeg audio: {}", e)),
                                }
                            },
                        }
                    },
                    #[cfg(feature = "api")]
                    ModuleType::Video => {
                        log_info(&format!("Video: copie_flux={} speed={} | {:?}", copie, video_speed, input));
                        match modules::video::traiter_video(&input, &out_str, copie, false, video_speed) {
                            Ok(mut child) => {
                                match child.wait() {
                                    Ok(status) if status.success() => Ok(()),
                                    Ok(status) => Err(format!("video process exited with code={:?}", status.code())),
                                    Err(e) => Err(format!("wait() error video: {}", e)),
                                }
                            },
                            Err(e) => Err(format!("failed to start ffmpeg video: {}", e)),
                        }
                    },
                    ModuleType::Doc => {
                        log_info(&format!("Doc: action={} | {:?}", doc_action, input));
                        match doc_action.as_str() {
                            "Convert" => {
                                let format_entree = modules::doc::detecter_format_entree(&input);
                                let format_sortie = modules::doc::detecter_format_sortie(&out_str);
                                log_info(&format!("Doc Convert: entree={:?} sortie={:?}", format_entree, format_sortie));
                                if modules::doc::convertir_avec_formats(&input, &out_str, format_entree, format_sortie) {
                                    Ok(())
                                } else {
                                    Err(format!("Convert_avec_formats failed | input={:?} output={:?} | file={:?}", format_entree, format_sortie, input))
                                }
                            },
                            "pdf_split" => {
                                let output_dir = input.parent().unwrap().join(format!(
                                    "{}_pages",
                                    input.file_stem().unwrap_or_default().to_string_lossy()
                                ));
                                std::fs::create_dir_all(&output_dir).ok();
                                log_info(&format!("Doc pdf_split: output_dir={:?}", output_dir));
                                modules::doc::pdf_split(&input, output_dir.to_str().unwrap())
                                    .map(|_| ())
                                    .map_err(|e| format!("pdf_split failed: {}", e))
                            },
                            "pdf_merge" => {
                                let paths: Vec<&Path> = pdf_merge_list.iter().map(|p| p.as_path()).collect();
                                let output_merge = input.parent().unwrap().join("merged_oxyon.pdf");
                                log_info(&format!("Doc pdf_merge: {} fichiers -> {:?}", paths.len(), output_merge));
                                modules::doc::pdf_merge(&paths, output_merge.to_str().unwrap())
                                    .map_err(|e| format!("pdf_merge failed: {}", e))
                            },
                            "pdf_rotate" => {
                                let pages_opt = parse_pages_spec(&pdf_pages);
                                log_info(&format!("Doc pdf_rotate: angle={} pages={:?}", pdf_angle, pages_opt));
                                modules::doc::pdf_rotate(&input, &out_str, pdf_angle, pages_opt.as_deref())
                                    .map_err(|e| format!("pdf_rotate failed: {}", e))
                            },
                            "pdf_compress" => {
                                log_info(&format!("Doc pdf_compress: {:?}", input));
                                modules::doc::pdf_compresser(&input, &out_str)
                                    .map(|_| ())
                                    .map_err(|e| format!("pdf_compress failed: {}", e))
                            },
                            "pdf_crop" => {
                                let pages_opt = parse_pages_spec(&pdf_pages);
                                log_info(&format!("Doc pdf_crop: x={} y={} w={} h={} pages={:?}", pdf_crop_x, pdf_crop_y, pdf_crop_w, pdf_crop_h, pages_opt));
                                modules::doc::pdf_crop(&input, &out_str, pdf_crop_x, pdf_crop_y, pdf_crop_w, pdf_crop_h, pages_opt.as_deref())
                                    .map_err(|e| format!("pdf_crop failed: {}", e))
                            },
                            "pdf_organize" => {
                                let ordre: Vec<u32> = pdf_nouvel_ordre.split(',')
                                    .filter_map(|s| s.trim().parse::<u32>().ok())
                                    .collect();
                                log_info(&format!("Doc pdf_organize: ordre={:?}", ordre));
                                if ordre.is_empty() {
                                    Err("pdf_organize: ordre vide ou invalide".to_string())
                                } else {
                                    modules::doc::pdf_organiser(&input, &out_str, &ordre)
                                        .map_err(|e| format!("pdf_organize failed: {}", e))
                                }
                            },
                            "pdf_delete_pages" => {
                                let pages_a_sup: Vec<u32> = pdf_pages.split(',')
                                    .filter_map(|s| s.trim().parse::<u32>().ok())
                                    .collect();
                                log_info(&format!("Doc pdf_delete_pages: pages={:?}", pages_a_sup));
                                if pages_a_sup.is_empty() {
                                    Err("pdf_delete_pages: liste de pages vide ou invalide".to_string())
                                } else {
                                    modules::doc::pdf_supprimer_pages(&input, &out_str, &pages_a_sup)
                                        .map_err(|e| format!("pdf_delete_pages failed: {}", e))
                                }
                            },
                            "pdf_numbers" => {
                                let position = match pdf_num_position.as_str() {
                                    "BasGauche"  => modules::doc::PositionNumero::BasGauche,
                                    "BasDroite"  => modules::doc::PositionNumero::BasDroite,
                                    "HautCentre" => modules::doc::PositionNumero::HautCentre,
                                    "HautGauche" => modules::doc::PositionNumero::HautGauche,
                                    "HautDroite" => modules::doc::PositionNumero::HautDroite,
                                    _            => modules::doc::PositionNumero::BasCentre,
                                };
                                log_info(&format!("Doc pdf_numbers: debut={} position={} taille={}", pdf_num_debut, pdf_num_position, pdf_num_taille));
                                modules::doc::pdf_numeroter(&input, &out_str, pdf_num_debut, position, pdf_num_taille)
                                    .map_err(|e| format!("pdf_number_pages failed: {}", e))
                            },
                            "pdf_protect" => {
                                log_info(&format!("Doc pdf_protect: print={} copy={}", pdf_allow_print, pdf_allow_copy));
                                modules::doc::pdf_proteger(&input, &out_str, &pdf_owner_pass, &pdf_user_pass, pdf_allow_print, pdf_allow_copy)
                                    .map_err(|e| format!("pdf_protect failed: {}", e))
                            },
                            "pdf_unlock" => {
                                log_info("Doc pdf_unlock");
                                modules::doc::pdf_dechiffrer(&input, &out_str, &pdf_unlock_pass)
                                    .map_err(|e| format!("pdf_unlock failed: {}", e))
                            },
                            "pdf_repair" => {
                                log_info(&format!("Doc pdf_repair: {:?}", input));
                                modules::doc::pdf_reparer(&input, &out_str)
                                    .map_err(|e| format!("pdf_repair failed: {}", e))
                            },
                            "pdf_watermark" => {
                                let pages_opt = parse_pages_spec(&pdf_pages);
                                log_info(&format!("Doc pdf_watermark: texte='{}' taille={} opacite={}", pdf_wm_texte, pdf_wm_taille, pdf_wm_opacite));
                                modules::doc::pdf_watermark(&input, &out_str, &pdf_wm_texte, pdf_wm_taille, pdf_wm_opacite, pages_opt.as_deref())
                                    .map_err(|e| format!("pdf_watermark failed: {}", e))
                            },
                            autre => {
                                log_warn(&format!("Doc: action inconnue '{}', fallback Convert()", autre));
                                if modules::doc::convertir(&input, &out_str) {
                                    Ok(())
                                } else {
                                    Err(format!("Convert() fallback failed for {:?}", input))
                                }
                            },
                        }
                    },
                    ModuleType::Image => {
                        log_info(&format!("Image: action={} fmt={} ratio={} | {:?}", img_action, fmt, ratio, input));
                        match img_action.as_str() {
                            "Convert" => {
                                if modules::pic::compresser(&input, &out_str, ratio) { Ok(()) }
                                else { Err(format!("pic::compresser failed | fmt={} ratio={} | {:?}", fmt, ratio, input)) }
                            },
                            "resize" => {
                                log_info(&format!("Image resize: w={} h={} kb={}", resize_w, resize_h, resize_kb));
                                if resize_w > 0 && resize_h > 0 {
                                    if resize_kb > 0 {
                                        let temp = format!("{}_temp.{}", out_str, fmt);
                                        if modules::pic::redimensionner_pixels(&input, &temp, resize_w, resize_h) {
                                            if modules::pic::redimensionner_poids(Path::new(&temp), &out_str, resize_kb) {
                                                Ok(())
                                            } else {
                                                Err(format!("resize by size failed | max_kb={} | file={:?}", resize_kb, input))
                                            }
                                        } else {
                                            Err(format!("resize by pixels failed | w={} h={} | file={:?}", resize_w, resize_h, input))
                                        }
                                    } else {
                                        if modules::pic::redimensionner_pixels(&input, &out_str, resize_w, resize_h) { Ok(()) }
                                        else { Err(format!("resize by pixels failed | w={} h={} | file={:?}", resize_w, resize_h, input)) }
                                    }
                                } else if resize_kb > 0 {
                                    if modules::pic::redimensionner_poids(&input, &out_str, resize_kb) { Ok(()) }
                                    else { Err(format!("resize by size only failed | max_kb={} | file={:?}", resize_kb, input)) }
                                } else {
                                    log_warn("Image resize: no w/h or kb specified, fallback to compress");
                                    if modules::pic::compresser(&input, &out_str, 1) { Ok(()) }
                                    else { Err(format!("pic::compresser fallback failed for {:?}", input)) }
                                }
                            },
                            "rotate" => {
                                log_info(&format!("Image rotate: angle={}", angle));
                                if modules::pic::pivoter(&input, &out_str, angle) { Ok(()) }
                                else { Err(format!("pic::rotate failed | angle={} | file={:?}", angle, input)) }
                            },
                            "crop" => {
                                log_info(&format!("Image crop: x={} y={} w={} h={}", crop_x, crop_y, crop_w, crop_h));
                                if modules::pic::recadrer(&input, &out_str, crop_x, crop_y, crop_w, crop_h) { Ok(()) }
                                else { Err(format!("pic::crop failed | x={} y={} w={} h={} | file={:?}", crop_x, crop_y, crop_w, crop_h, input)) }
                            },
                            autre => {
                                log_warn(&format!("Image: action inconnue '{}', fallback compresser", autre));
                                if modules::pic::compresser(&input, &out_str, ratio) { Ok(()) }
                                else { Err(format!("pic::compresser fallback failed for {:?}", input)) }
                            },
                        }
                    },
                    _ => Ok(()),
                };

                // ── Résultat + timing ────────────────────────────────────
                let elapsed = start.elapsed();
                match &result {
                    Ok(()) => {
                        if elapsed.as_secs() > 30 {
                            log_warn(&format!(
                                "OK mais LENT ({:.1}s) | module={:?} | {:?}",
                                elapsed.as_secs_f32(), module, input
                            ));
                        } else {
                            log_info(&format!(
                                "OK ({:.2}s) | module={:?} | {:?}",
                                elapsed.as_secs_f32(), module, input
                            ));
                        }
                    },
                    Err(raison) => {
                        log_error(&format!(
                            "FAILED ({:.2}s) | module={:?} | file={:?} | reason={}",
                            elapsed.as_secs_f32(), module, input, raison
                        ));
                    }
                }

                *active.lock().unwrap() -= 1;
                *completed.lock().unwrap() += 1;
                let done = *completed.lock().unwrap();
                let total_count = *total.lock().unwrap();
                if done >= total_count {
                    log_info(&format!("=== BATCH END | {}/{} files processed ===", done, total_count));
                    *status_arc.lock().unwrap() = format!("✅ Done: {}/{} files", done, total_count);
                } else {
                    *status_arc.lock().unwrap() = format!("⚙️ Processing {}/{} files...", done, total_count);
                }
                ctx.request_repaint();
            }
        });
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
                *self.status.lock().unwrap() = format!("📁 {} files loaded", self.current_files.len());
            }
        });
        if let Some(ref mut c) = self.process {
            if let Ok(Some(_)) = c.try_wait() {
                *self.status.lock().unwrap() = "✅ Done".into();
                self.process = None;
            }
            ctx.request_repaint();
        }
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| ui.heading(format!("OXYON v{}", VERSION)));
            if !self.deps_manquantes.is_empty() {
                ui.colored_label(egui::Color32::RED, format!("⚠️ Missing: {}", self.deps_manquantes.join(", ")));
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
                #[cfg(feature = "api")] mods.push((ModuleType::Video, "🎬 Video"));
                mods.push((ModuleType::Settings, "⚙ Settings"));
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
                    if ui.add(egui::Slider::new(&mut self.archive_niveau, 1..=9).text("Compression (1=fast, 9=quality")).changed() {
                        self.save_config();
                    }
                    if ui.checkbox(&mut self.save_archive_format, "💾 Save format").changed() {
                        self.save_config();
                    }
                },
                ModuleType::Doc => {
                    ui.horizontal(|ui| {
                        ui.label("Action :");
                        egui::ComboBox::from_id_salt("doc_action").selected_text(&self.doc_action).show_ui(ui, |ui| {
							ui.selectable_value(&mut self.doc_action, "Convert".into(), "Convert");
                            ui.selectable_value(&mut self.doc_action, "pdf_compress".into(), "PDF compress");
							ui.selectable_value(&mut self.doc_action, "pdf_crop".into(), "PDF crop");
                            ui.selectable_value(&mut self.doc_action, "pdf_delete_pages".into(), "PDF delete pages");
							ui.selectable_value(&mut self.doc_action, "pdf_merge".into(), "PDF merge");
                            ui.selectable_value(&mut self.doc_action, "pdf_numbers".into(), "PDF Number pages");
                            ui.selectable_value(&mut self.doc_action, "pdf_organize".into(), "PDF organize pages");
							ui.selectable_value(&mut self.doc_action, "pdf_protect".into(), "PDF protect");
                            ui.selectable_value(&mut self.doc_action, "pdf_repair".into(), "PDF repair");
							ui.selectable_value(&mut self.doc_action, "pdf_rotate".into(), "PDF rotate");
							ui.selectable_value(&mut self.doc_action, "pdf_split".into(), "PDF split");
							ui.selectable_value(&mut self.doc_action, "pdf_unlock".into(), "PDF unlock");
                            ui.selectable_value(&mut self.doc_action, "pdf_watermark".into(), "PDF watermark");
                        });
                    });
                    ui.separator();
                    match self.doc_action.as_str() {
                        "Convert" => {
                            ui.horizontal(|ui| {
                                ui.label("Format :");
                                egui::ComboBox::from_id_salt("dfmt").selected_text(&self.format_choisi).show_ui(ui, |ui| {
                                    for f in ["docx","epub","html","md","odt","pdf","rtf","txt"] {
                                        ui.selectable_value(&mut self.format_choisi, f.into(), f);
                                    }
                                });
                            });
                            if ui.checkbox(&mut self.save_doc_format, "💾 Save format").changed() {
                                self.save_config();
                            }
                        },
                        "pdf_split" => {
                            ui.label("✂️ Splits each PDF page into a separate file");
                            ui.label("💡 Creates a _pages/ folder next to the source file");
                        },
                        "pdf_merge" => {
                            ui.label("📎 Merges all loaded files into a single PDF");
                            ui.label("💡 Output will be merged_oxyon.pdf");
                        },
                        "pdf_rotate" => {
                            ui.horizontal(|ui| {
                                ui.label("Angle :");
                                egui::ComboBox::from_id_salt("pdf_rot").selected_text(format!("{}°", self.pdf_rotation_angle)).show_ui(ui, |ui| {
                                    ui.selectable_value(&mut self.pdf_rotation_angle, 90, "90°");
                                    ui.selectable_value(&mut self.pdf_rotation_angle, 180, "180°");
                                    ui.selectable_value(&mut self.pdf_rotation_angle, 270, "270°");
                                });
                            });
                            ui.horizontal(|ui| {
                                ui.label("Pages (ex: 1,3,5 ou vide = toutes) :");
                                ui.text_edit_singleline(&mut self.pdf_pages_spec);
                            });
                        },
                        "pdf_compress" => {
                            ui.label("🗜️ Reduces PDF size by recompressing");
                        },
                        "pdf_crop" => {
                            ui.label("Margins in % (0.0 - 100.0):");
                            ui.horizontal(|ui| {
                                ui.label("X:");
                                ui.add(egui::Slider::new(&mut self.pdf_crop_x, 0.0..=100.0).fixed_decimals(1));
                                ui.label("Y:");
                                ui.add(egui::Slider::new(&mut self.pdf_crop_y, 0.0..=100.0).fixed_decimals(1));
                            });
                            ui.horizontal(|ui| {
                                ui.label("W:");
                                ui.add(egui::Slider::new(&mut self.pdf_crop_w, 1.0..=100.0).fixed_decimals(1));
                                ui.label("H:");
                                ui.add(egui::Slider::new(&mut self.pdf_crop_h, 1.0..=100.0).fixed_decimals(1));
                            });
                            ui.horizontal(|ui| {
                                ui.label("Pages :");
                                ui.text_edit_singleline(&mut self.pdf_pages_spec);
                                ui.label("(ex: 1,3,5 ou vide = toutes)");
                            });
                        },
                        "pdf_organize" => {
                            ui.label("🔀 New page order (e.g. 3,1,2):");
                            ui.text_edit_singleline(&mut self.pdf_nouvel_ordre);
                        },
                        "pdf_delete_pages" => {
                            ui.label("🗑️ Pages to delete (e.g. 2,4,6):");
                            ui.text_edit_singleline(&mut self.pdf_pages_spec);
                        },
                        "pdf_numbers" => {
                            ui.horizontal(|ui| {
                                ui.label("Start:");
                                ui.add(egui::Slider::new(&mut self.pdf_num_debut, 1..=999));
                            });
                            ui.horizontal(|ui| {
                                ui.label("Position :");
                                egui::ComboBox::from_id_salt("pdf_numpos").selected_text(&self.pdf_num_position).show_ui(ui, |ui| {
                                    for pos in ["BasCentre","BasGauche","BasDroite","HautCentre","HautGauche","HautDroite"] {
                                        ui.selectable_value(&mut self.pdf_num_position, pos.into(), pos);
                                    }
                                });
                            });
                            ui.horizontal(|ui| {
                                ui.label("Taille :");
                                ui.add(egui::Slider::new(&mut self.pdf_num_taille, 6.0..=36.0).fixed_decimals(0));
                            });
                        },
                        "pdf_protect" => {
                            ui.horizontal(|ui| {
                                ui.label("Owner password:");
                                ui.add(egui::TextEdit::singleline(&mut self.pdf_owner_pass).password(true));
                            });
                            ui.horizontal(|ui| {
                                ui.label("User password:");
                                ui.add(egui::TextEdit::singleline(&mut self.pdf_user_pass).password(true));
                            });
                            ui.checkbox(&mut self.pdf_allow_print, "Allow printing");
                            ui.checkbox(&mut self.pdf_allow_copy, "Allow copying");
                        },
                        "pdf_unlock" => {
                            ui.horizontal(|ui| {
                                ui.label("Password:");
                                ui.add(egui::TextEdit::singleline(&mut self.pdf_unlock_pass).password(true));
                            });
                        },
                        "pdf_repair" => {
                            ui.label("🔧 Attempts to repair a corrupted document");
                            ui.label("Removes orphan objects, recompresses, renumbers");
                            ui.label("💡 Works on all Doc module formats");
                        },
                        "pdf_watermark" => {
                            ui.label("💧 Adds a diagonal text watermark");
                            ui.label("💡 Works on all Doc module formats");
                            ui.horizontal(|ui| {
                                ui.label("Texte :");
                                ui.text_edit_singleline(&mut self.pdf_wm_texte);
                            });
                            ui.horizontal(|ui| {
                                ui.label("Taille :");
                                ui.add(egui::Slider::new(&mut self.pdf_wm_taille, 12.0..=120.0).fixed_decimals(0));
                            });
                            ui.horizontal(|ui| {
                                ui.label("Opacity:");
                                ui.add(egui::Slider::new(&mut self.pdf_wm_opacite, 0.05..=1.0).fixed_decimals(2));
                            });
                            ui.horizontal(|ui| {
                                ui.label("Pages :");
                                ui.text_edit_singleline(&mut self.pdf_pages_spec);
                                ui.label("(ex: 1,3,5 ou vide = toutes)");
                            });
                        },
                        _ => {}
                    }
                },
                ModuleType::Image => {
                    ui.horizontal(|ui| {
                        ui.label("Action :");
                        egui::ComboBox::from_id_salt("img_action").selected_text(&self.image_action).show_ui(ui, |ui| {
                            ui.selectable_value(&mut self.image_action, "Convert".into(), "Convert");
							ui.selectable_value(&mut self.image_action, "crop".into(), "Crop");
                            ui.selectable_value(&mut self.image_action, "resize".into(), "Resize");
                            ui.selectable_value(&mut self.image_action, "rotate".into(), "Rotate");
                        });
                    });
                    ui.separator();
                    match self.image_action.as_str() {
                        "Convert" => {
                            ui.horizontal(|ui| {
                                ui.label("Format :");
                                egui::ComboBox::from_id_salt("ifmt").selected_text(&self.format_choisi).show_ui(ui, |ui| {
                                    for f in ["EXR","GIF","ICO","JPG","JXL","PNG","PSD","SVG","TIFF","WebP"] {
                                        ui.selectable_value(&mut self.format_choisi, f.into(), f);
                                    }
                                });
                            });
                            if ui.checkbox(&mut self.save_image_format, "💾 Save format").changed() {
                                self.save_config();
                            }
                            if ui.add(egui::Slider::new(&mut self.ratio_img, 1..=10).text("Quality (1=fast, 10=quality)")).changed() {
                                self.save_config();
                            }
                        },
                        "resize" => {
                            ui.horizontal(|ui| {
                                ui.label("Format :");
                                egui::ComboBox::from_id_salt("ifmt_resize").selected_text(&self.format_choisi).show_ui(ui, |ui| {
                                    for f in ["EXR","GIF","ICO","JPG","JXL","PNG","PSD","SVG","TIFF","WebP"] {
                                        ui.selectable_value(&mut self.format_choisi, f.into(), f);
                                    }
                                });
                            });
                            if ui.checkbox(&mut self.save_image_format, "💾 Save format").changed() {
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
                            ui.label("Coordinates in % (0-100):");
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
                ModuleType::Audio => {
                    ui.horizontal(|ui| {
                        ui.label("Action :");
                        egui::ComboBox::from_id_salt("audio_action").selected_text(&self.audio_action).show_ui(ui, |ui| {
                            ui.selectable_value(&mut self.audio_action, "extract".into(), "Audio extract");
							ui.selectable_value(&mut self.audio_action, "Convert".into(), "Convert");                            
                        });
                    });
                    ui.separator();
                    match self.audio_action.as_str() {
                        "Convert" => {
                            // Détection codec au chargement de fichiers
                            if let Some(f) = self.current_files.first() {
                                if ui.button("🔍 Detect codec").clicked() {
                                    let codec = modules::audio::detecter_extension(f);
                                    let fmts = modules::audio::formats_compatibles(&codec);
                                    self.audio_formats_dispo = fmts.iter().map(|s| s.to_string()).collect();
                                    crate::log_info(&format!("Audio: codec detected='{}' | formats compatibles={:?}", codec, self.audio_formats_dispo));
                                }
                            }
                            ui.horizontal(|ui| {
                                ui.label("Format :");
                                egui::ComboBox::from_id_salt("afmt").selected_text(&self.format_choisi).show_ui(ui, |ui| {
                                    for f in &self.audio_formats_dispo {
                                        ui.selectable_value(&mut self.format_choisi, f.clone(), f.as_str());
                                    }
                                });
                            });
                            if ui.add(egui::Slider::new(&mut self.audio_qualite, 0..=9).text("VBR (1=fast, 9=quality)")).changed() {
                                self.save_config();
                            }
                            if ui.checkbox(&mut self.save_audio_format, "💾 Save format").changed() {
                                self.save_config();
                            }
                        },
                        "extract" => {
                            ui.label("🎵 Extracts audio from a video (direct copy, no re-encoding)");
                            ui.label("💡 Output format is detected automatically");
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
                        if ui.checkbox(&mut self.copie_flux, "Stream copy").changed() { self.save_config(); }
                    });
                    if ui.add(egui::Slider::new(&mut self.video_speed, 0..=8).text("Quality (1=fast, 8=quality)")).changed() {
                        self.save_config();
                    }
                    if ui.checkbox(&mut self.save_video_format, "💾 Save format").changed() {
                        self.save_config();
                    }
                },
                #[cfg(feature = "api")]
                ModuleType::Scrapper => {
                    ui.horizontal(|ui| {
                        ui.label("TMDB API Key :");
                        ui.add(egui::TextEdit::singleline(&mut self.tmdb_api_key).password(true));
                    });
                    ui.horizontal(|ui| {
                        ui.label("Fanart API Key :");
                        ui.add(egui::TextEdit::singleline(&mut self.fanart_api_key).password(true));
                    });
                    if ui.button("💾 Save keys").clicked() {
                        let content = format!(
                            "TMDB_API_KEY={}\nFANART_API_KEY={}\n",
                            self.tmdb_api_key, self.fanart_api_key
                        );
                        let _ = std::fs::write(".env", content);
                    }
                    ui.separator();
                    ui.horizontal(|ui| {
                        let search = |is_series: bool, res_arc: Arc<Mutex<Vec<ScrapeEntry>>>, stem: String, ctx_c: egui::Context| {
                            res_arc.lock().unwrap().clear();
                            std::thread::spawn(move || {
                                if let Ok(results) = modules::scrap::search_tmdb(&stem, is_series) {
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
                        };
                        if ui.button("🎬 Movie").clicked() {
                            search(false, Arc::clone(&self.results_ui), self.current_stem.clone(), ctx.clone());
                        }
                        if ui.button("📺 Series").clicked() {
                            search(true, Arc::clone(&self.results_ui), self.current_stem.clone(), ctx.clone());
                        }
                    });
                    let entries = self.results_ui.lock().unwrap().clone();
                    for entry in entries {
                        ui.horizontal(|ui| {
                            if let Some(t) = &entry.texture { ui.image((t.id(), egui::vec2(50.0, 75.0))); }
                            ui.label(&entry.data.title);
                            if !self.current_files.is_empty() {
                                if ui.button("Choisir").clicked() { modules::scrap::save_metadata(self.current_files[0].clone(), entry.data.clone()); }
                            }
                        });
                    }
                },
                #[cfg(feature = "api")]
                ModuleType::Tag => {
                    let path_opt = self.current_files.get(0).cloned();
                    ui.vertical(|ui| {
                        if ui.button("✅ Mark as WATCHED").clicked() { if let Some(path) = &path_opt { let _ = modules::tag::marquer_vu(&path, &path.with_extension("nfo")); } }
                        if ui.button("📥 Inject tags from NFO").clicked() { if let Some(path) = &path_opt { let _ = modules::tag::appliquer_tags(&path, &path.with_extension("nfo")); } }
                        if ui.button("🖼️ Add poster / fanart").clicked() { if let Some(path) = &path_opt { let _ = modules::tag::ajouter_images_mkv(&path); } }
                        if ui.button("🗑️ Reset Tags").clicked() { if let Some(path) = &path_opt { let _ = modules::tag::supprimer_tous_tags(&path); } }
                        ui.horizontal(|ui| {
                            ui.text_edit_singleline(&mut self.tag_edit_val);
                            if ui.button("✏️ Edit Title").clicked() { if let Some(path) = &path_opt { let _ = modules::tag::modifier_tag(&path, "title", &self.tag_edit_val); } }
                        });
                    });
                },
                ModuleType::Settings => {
                    ui.vertical(|ui| {
                        ui.heading("Settings");
                        let old_theme = self.current_theme.clone();
                        ui.horizontal(|ui| {
                            ui.label("Theme:");
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
                        ui.separator();
                        ui.heading("Performance");
                        ui.horizontal(|ui| {
                            ui.label("Max parallel jobs:");
                            if ui.add(egui::Slider::new(&mut self.max_parallel_jobs, 1..=16).text("threads")).changed() {
                                self.save_config();
                            }
                        });
                        ui.label("💡 Higher = faster but more CPU load");
                    });
                },
            }
            let mut hide_exec = self.module_actif == ModuleType::Settings;
            #[cfg(feature = "api")]
            { hide_exec = hide_exec || self.module_actif == ModuleType::Scrapper || self.module_actif == ModuleType::Tag; }
            if !self.current_files.is_empty() && !hide_exec {
                ui.separator();
                if ui.button("🔥 RUN ALL").clicked() {
                    self.lancer_batch(ctx.clone());
                }
            }
            if self.current_files.is_empty() {
                ui.centered_and_justified(|ui| {
                    ui.vertical_centered(|ui| {
                        ui.label("📥 Drop your files here");
                        ui.add_space(5.0);
                        if ui.button("📂 Browse").clicked() {
                            if let Some(paths) = rfd::FileDialog::new().pick_files() {
                                self.current_files = paths;
                                if let Some(p) = self.current_files.first() {
                                    self.current_stem = p.file_stem().unwrap_or_default().to_string_lossy().to_string();
                                }
                                #[cfg(feature = "api")]
                                self.results_ui.lock().unwrap().clear();
                                *self.status.lock().unwrap() = format!("📁 {} files loaded", self.current_files.len());
                            }
                        }
                    });
                });
            }
            ui.add_space(10.0);
            ui.vertical_centered(|ui| {
                let completed = *self.completed_jobs.lock().unwrap();
                let total = *self.total_jobs.lock().unwrap();
                if total > 0 && completed < total {
                    let active = *self.active_jobs.lock().unwrap();
                    let pct = (completed as f32 / total as f32 * 100.0).round() as u32;
                    ui.heading(format!("⚙️ {}/{} files ({}%)", completed, total, pct));
                    ui.add(egui::ProgressBar::new(completed as f32 / total as f32).animate(true));
                    ui.small(format!("{} active · {} pending", active, self.job_queue.lock().unwrap().len()));
                } else if total > 0 && completed >= total {
                    ui.heading(format!("✅ Done - {} files processed", total));
                } else {
                    ui.heading(&*self.status.lock().unwrap());
                }
            });
            if !self.current_files.is_empty() { if ui.button("🗑️ Clear all").clicked() { self.current_files.clear(); } }
        });
    }
}
fn parse_pages_spec(spec: &str) -> Option<Vec<u32>> {
    let trimmed = spec.trim();
    if trimmed.is_empty() || trimmed == "1-end" {
        return None;
    }
    let pages: Vec<u32> = trimmed.split(',')
    .filter_map(|s| s.trim().parse::<u32>().ok())
    .collect();
    if pages.is_empty() { None } else { Some(pages) }
}
fn main() -> eframe::Result {
    log_info(&format!("=== OXYON v{} START ===", VERSION));
    let _ = modules::binaries::extraire_deps();
    let mut options = eframe::NativeOptions::default();
    #[cfg(target_os = "windows")]
    let icon_bytes: &[u8] = include_bytes!("../assets/Oxyon_icon.ico");
    #[cfg(not(target_os = "windows"))]
    let icon_bytes: &[u8] = include_bytes!("../assets/Oxyon_icon.png");
    if let Ok(icon_data) = image::load_from_memory(icon_bytes) {
        let icon_rgba = icon_data.to_rgba8();
        let (width, height) = icon_rgba.dimensions();
        options.viewport.icon = Some(std::sync::Arc::new(egui::IconData { rgba: icon_rgba.into_raw(), width, height }));
    }
    let result = eframe::run_native(
        &format!("oxyon v{}", VERSION),
        options,
        Box::new(|cc| {
            let mut app = OxyonApp::default();
            app.load_config();
            app.apply_theme(&cc.egui_ctx);
            Ok(Box::new(app))
        }),
    );
    log_info("=== OXYON FERMETURE ===");
    modules::binaries::cleanup();
    result
}