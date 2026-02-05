#![allow(dead_code)]
use image::imageops::FilterType;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

/// Liste des formats supportés
#[derive(Debug, Clone, Copy)]
pub enum ImageFormat {
    // Formats de base
    Gif,
    Jpg,
    Png,
    WebP,
    // Formats avec features
    Avif,
    Heic, // Même que AVIF
    Exr,
    Jxl,
    Tiff,
    Ico,
    Hdr,
    // Formats avec crates externes
    Svg,
    Raw,
    Dng,
    Psd,
}

impl ImageFormat {
    pub fn extension(&self) -> &str {
        match self {
            Self::Gif => "gif",
            Self::Jpg => "jpg",
            Self::Png => "png",
            Self::WebP => "webp",
            Self::Avif => "avif",
            Self::Heic => "heic",
            Self::Exr => "exr",
            Self::Jxl => "jxl",
            Self::Tiff => "tiff",
            Self::Ico => "ico",
            Self::Hdr => "hdr",
            Self::Svg => "svg",
            Self::Raw => "raw",
            Self::Dng => "dng",
            Self::Psd => "psd",
        }
    }
}

/// Redimensionne l'image (Compression par dimension)
pub fn compresser(input: &Path, output: &str, ratio: u32) -> bool {
    // Détection du format d'entrée
    if let Some(ext) = input.extension().and_then(|e| e.to_str()) {
        match ext.to_lowercase().as_str() {
            "svg" => return compresser_svg(input, output, ratio),
            "psd" => return compresser_psd(input, output, ratio),
            "dng" | "cr2" | "nef" | "arw" | "orf" | "rw2" => return compresser_raw(input, output, ratio),
            _ => {}
        }
    }

    // Format standard supporté par image crate
    if let Ok(img) = image::open(input) {
        let (w, h) = (img.width(), img.height());
        // Ratio de 2 = divise par 2 la largeur et hauteur
        let scaled = img.resize(w / ratio, h / ratio, FilterType::Lanczos3);
        scaled.save(output).is_ok()
    } else {
        false
    }
}

/// Conversion de format (ex: PNG -> JPG, WEBP -> PNG)
pub fn convertir(input: &Path, output: &str) -> bool {
    // Détection du format d'entrée
    if let Some(ext) = input.extension().and_then(|e| e.to_str()) {
        match ext.to_lowercase().as_str() {
            "svg" => return convertir_svg(input, output),
            "psd" => return convertir_psd(input, output),
            "dng" | "cr2" | "nef" | "arw" | "orf" | "rw2" => return convertir_raw(input, output),
            _ => {}
        }
    }

    // Format standard
    match image::open(input) {
        Ok(img) => img.save(output).is_ok(),
        Err(_) => false,
    }
}

/// Lecture des métadonnées EXIF (GPS, Appareil, Date)
pub fn lire_exif(input: &Path) -> Vec<String> {
    let mut resultats = Vec::new();
    if let Ok(file) = File::open(input) {
        let mut reader = BufReader::new(file);
        let exifreader = exif::Reader::new();
        if let Ok(exdata) = exifreader.read_from_container(&mut reader) {
            for f in exdata.fields() {
                resultats.push(format!(
                    "{}: {}",
                    f.tag,
                    f.display_value().with_unit(&exdata)
                ));
            }
        }
    }
    resultats
}

/// Supprime l'EXIF en réenregistrant les pixels nus
pub fn supprimer_exif(input: &Path, output: &str) -> bool {
    if let Ok(img) = image::open(input) {
        // Enregistre uniquement les données de pixels, ignore les métadonnées sources
        img.save(output).is_ok()
    } else {
        false
    }
}

/// Rotation simple (90, 180, 270)
pub fn pivoter(input: &Path, output: &str, angle: u32) -> bool {
    if let Ok(img) = image::open(input) {
        let rotated = match angle {
            90 => img.rotate90(),
            180 => img.rotate180(),
            270 => img.rotate270(),
            _ => img,
        };
        rotated.save(output).is_ok()
    } else {
        false
    }
}

/// Recadrage d'image (crop) - coordonnées en pourcentage de l'image
/// x, y = coin supérieur gauche (0-100)
/// width, height = dimensions du crop (0-100)
pub fn recadrer(input: &Path, output: &str, x_pct: u32, y_pct: u32, width_pct: u32, height_pct: u32) -> bool {
    if let Ok(img) = image::open(input) {
        let (img_w, img_h) = (img.width(), img.height());
        
        // Convertir pourcentages en pixels
        let x = (img_w * x_pct) / 100;
        let y = (img_h * y_pct) / 100;
        let width = (img_w * width_pct) / 100;
        let height = (img_h * height_pct) / 100;
        
        // Vérifier les limites
        if x + width > img_w || y + height > img_h {
            return false;
        }
        
        let cropped = img.crop_imm(x, y, width, height);
        cropped.save(output).is_ok()
    } else {
        false
    }
}

// === FONCTIONS POUR FORMATS SPÉCIAUX ===

/// Conversion SVG vers format raster
fn convertir_svg(input: &Path, output: &str) -> bool {
    let mut file = match File::open(input) {
        Ok(f) => f,
        Err(_) => return false,
    };
    
    let mut svg_data = Vec::new();
    if file.read_to_end(&mut svg_data).is_err() {
        return false;
    }

    let opt = usvg::Options::default();
    let tree = match usvg::Tree::from_data(&svg_data, &opt) {
        Ok(t) => t,
        Err(_) => return false,
    };

    let size = tree.size();
    let mut pixmap = match tiny_skia::Pixmap::new(size.width() as u32, size.height() as u32) {
        Some(p) => p,
        None => return false,
    };
    
    resvg::render(&tree, tiny_skia::Transform::default(), &mut pixmap.as_mut());

    // Convertir en image::RgbaImage
    let img = match image::RgbaImage::from_raw(
        pixmap.width(),
        pixmap.height(),
        pixmap.data().to_vec(),
    ) {
        Some(i) => i,
        None => return false,
    };

    image::DynamicImage::ImageRgba8(img).save(output).is_ok()
}

/// Compression SVG (rasterise puis redimensionne)
fn compresser_svg(input: &Path, output: &str, ratio: u32) -> bool {
    let mut file = match File::open(input) {
        Ok(f) => f,
        Err(_) => return false,
    };
    
    let mut svg_data = Vec::new();
    if file.read_to_end(&mut svg_data).is_err() {
        return false;
    }

    let opt = usvg::Options::default();
    let tree = match usvg::Tree::from_data(&svg_data, &opt) {
        Ok(t) => t,
        Err(_) => return false,
    };

    let size = tree.size();
    let mut pixmap = match tiny_skia::Pixmap::new(size.width() as u32, size.height() as u32) {
        Some(p) => p,
        None => return false,
    };
    
    resvg::render(&tree, tiny_skia::Transform::default(), &mut pixmap.as_mut());

    let img = match image::RgbaImage::from_raw(
        pixmap.width(),
        pixmap.height(),
        pixmap.data().to_vec(),
    ) {
        Some(i) => i,
        None => return false,
    };

    let dynamic_img = image::DynamicImage::ImageRgba8(img);
    let (w, h) = (dynamic_img.width(), dynamic_img.height());
    let scaled = dynamic_img.resize(w / ratio, h / ratio, FilterType::Lanczos3);
    scaled.save(output).is_ok()
}

/// Conversion RAW vers format standard
fn convertir_raw(input: &Path, output: &str) -> bool {
    let mut file = match File::open(input) {
        Ok(f) => f,
        Err(_) => return false,
    };

    let decoded = match rawloader::decode(&mut file) {
        Ok(d) => d,
        Err(_) => return false,
    };

    let width = decoded.width;
    let height = decoded.height;
    
    // Extraire les données selon le type
    let image_data: Vec<u8> = match decoded.data {
        rawloader::RawImageData::Integer(ref data) => {
            data.iter().map(|&val| (val >> 8) as u8).collect()
        },
        rawloader::RawImageData::Float(ref data) => {
            data.iter().map(|&val| (val.clamp(0.0, 1.0) * 255.0) as u8).collect()
        },
    };

    let img = match image::RgbImage::from_raw(width as u32, height as u32, image_data) {
        Some(i) => i,
        None => return false,
    };

    image::DynamicImage::ImageRgb8(img).save(output).is_ok()
}

/// Compression RAW (décode puis redimensionne)
fn compresser_raw(input: &Path, output: &str, ratio: u32) -> bool {
    let mut file = match File::open(input) {
        Ok(f) => f,
        Err(_) => return false,
    };

    let decoded = match rawloader::decode(&mut file) {
        Ok(d) => d,
        Err(_) => return false,
    };

    let width = decoded.width;
    let height = decoded.height;
    
    let image_data: Vec<u8> = match decoded.data {
        rawloader::RawImageData::Integer(ref data) => {
            data.iter().map(|&val| (val >> 8) as u8).collect()
        },
        rawloader::RawImageData::Float(ref data) => {
            data.iter().map(|&val| (val.clamp(0.0, 1.0) * 255.0) as u8).collect()
        },
    };

    let img = match image::RgbImage::from_raw(width as u32, height as u32, image_data) {
        Some(i) => i,
        None => return false,
    };

    let dynamic_img = image::DynamicImage::ImageRgb8(img);
    let (w, h) = (dynamic_img.width(), dynamic_img.height());
    let scaled = dynamic_img.resize(w / ratio, h / ratio, FilterType::Lanczos3);
    scaled.save(output).is_ok()
}

/// Conversion PSD vers format standard
fn convertir_psd(input: &Path, output: &str) -> bool {
    let mut file = match File::open(input) {
        Ok(f) => f,
        Err(_) => return false,
    };

    let mut psd_data = Vec::new();
    if file.read_to_end(&mut psd_data).is_err() {
        return false;
    }

    let psd_file = match psd::Psd::from_bytes(&psd_data) {
        Ok(p) => p,
        Err(_) => return false,
    };

    // Récupérer l'image composite (flatten)
    let rgba = match psd_file.flatten_layers_rgba(&|_| true) {
        Ok(r) => r,
        Err(_) => return false,
    };

    let img = match image::RgbaImage::from_raw(
        psd_file.width(),
        psd_file.height(),
        rgba,
    ) {
        Some(i) => i,
        None => return false,
    };

    image::DynamicImage::ImageRgba8(img).save(output).is_ok()
}

/// Compression PSD (décode puis redimensionne)
fn compresser_psd(input: &Path, output: &str, ratio: u32) -> bool {
    let mut file = match File::open(input) {
        Ok(f) => f,
        Err(_) => return false,
    };

    let mut psd_data = Vec::new();
    if file.read_to_end(&mut psd_data).is_err() {
        return false;
    }

    let psd_file = match psd::Psd::from_bytes(&psd_data) {
        Ok(p) => p,
        Err(_) => return false,
    };

    let rgba = match psd_file.flatten_layers_rgba(&|_| true) {
        Ok(r) => r,
        Err(_) => return false,
    };

    let img = match image::RgbaImage::from_raw(
        psd_file.width(),
        psd_file.height(),
        rgba,
    ) {
        Some(i) => i,
        None => return false,
    };

    let dynamic_img = image::DynamicImage::ImageRgba8(img);
    let (w, h) = (dynamic_img.width(), dynamic_img.height());
    let scaled = dynamic_img.resize(w / ratio, h / ratio, FilterType::Lanczos3);
    scaled.save(output).is_ok()
}