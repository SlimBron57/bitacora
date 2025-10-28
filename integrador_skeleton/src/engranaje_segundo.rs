use clap::Parser;
use image::{DynamicImage, ImageBuffer, Rgba, RgbaImage};
use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Parser, Debug)]
#[command(name = "engranaje_segundo")]
#[command(about = "Procesador de imágenes que interpreta DSL")]
struct Args {
    #[arg(long)]
    input: String,
    
    #[arg(long)]
    output: String,
    
    #[arg(long)]
    base: u32,
    
    #[arg(long)]
    logic: String,
}

#[derive(Debug, Clone)]
struct Config {
    input_dir: String,
    output_dir: String,
    base: u32,
    mode: String,
    format: String,
    channels: String,
    alpha: String,
}

#[derive(Serialize, Debug)]
struct PixelData {
    r: String,
    g: String,
    b: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    a: Option<String>,
}

#[derive(Serialize, Debug)]
struct ImageResult {
    image: String,
    width: u32,
    height: u32,
    channels: String,
    base: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pixels: Option<Vec<Vec<PixelData>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    palette: Option<Vec<PaletteEntry>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    histogram: Option<HistogramData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<String>,
}

#[derive(Serialize, Debug)]
struct PaletteEntry {
    count: u32,
    color: PixelData,
}

#[derive(Serialize, Debug)]
struct HistogramData {
    r: Vec<String>,
    g: Vec<String>,
    b: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    a: Option<Vec<String>>,
}

fn parse_logic_file(logic_file: &str) -> Result<Config, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(logic_file)?;
    let mut config = HashMap::new();
    
    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        
        if let Some((key, value)) = line.split_once('=') {
            let clean_value = value.trim().split('#').next().unwrap_or("").trim();
            config.insert(key.trim().to_string(), clean_value.to_string());
        }
    }
    
    Ok(Config {
        input_dir: config.get("INPUT_DIR").cloned().unwrap_or_else(|| "./in".to_string()),
        output_dir: config.get("OUTPUT_DIR").cloned().unwrap_or_else(|| "./out".to_string()),
        base: config.get("BASE").and_then(|s| s.parse().ok()).unwrap_or(16),
        mode: config.get("MODE").cloned().unwrap_or_else(|| "per_pixel".to_string()),
        format: config.get("FORMAT").cloned().unwrap_or_else(|| "json".to_string()),
        channels: config.get("CHANNELS").cloned().unwrap_or_else(|| "RGBA".to_string()),
        alpha: config.get("ALPHA").cloned().unwrap_or_else(|| "keep".to_string()),
    })
}

fn format_value(value: u8, base: u32) -> String {
    match base {
        2 => format!("{:b}", value),
        8 => format!("{:o}", value),
        10 => format!("{}", value),
        16 => format!("{:02x}", value),
        _ => {
            if base <= 36 {
                let mut result = String::new();
                let mut val = value as u32;
                if val == 0 {
                    return "0".to_string();
                }
                
                while val > 0 {
                    let digit = (val % base) as usize;
                    let char = if digit < 10 {
                        (b'0' + digit as u8) as char
                    } else {
                        (b'a' + (digit - 10) as u8) as char
                    };
                    result.insert(0, char);
                    val /= base;
                }
                result
            } else {
                format!("{}", value) // Fallback a decimal
            }
        }
    }
}

fn get_image_files(input_dir: &str) -> Result<Vec<PathBuf>, Box<dyn std::error::Error>> {
    let mut image_files = Vec::new();
    let extensions = ["png", "jpg", "jpeg", "gif", "bmp", "tiff", "webp"];
    
    let dir = Path::new(input_dir);
    if !dir.exists() {
        return Ok(image_files);
    }
    
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        
        if path.is_file() {
            if let Some(ext) = path.extension() {
                if let Some(ext_str) = ext.to_str() {
                    if extensions.contains(&ext_str.to_lowercase().as_str()) {
                        image_files.push(path);
                    }
                }
            }
        }
    }
    
    Ok(image_files)
}

fn process_image_per_pixel(image_path: &Path, config: &Config) -> ImageResult {
    match image::open(image_path) {
        Ok(img) => {
            let rgba_img = img.to_rgba8();
            let (width, height) = rgba_img.dimensions();
            
            let mut pixels = Vec::new();
            
            for y in 0..height {
                let mut row = Vec::new();
                for x in 0..width {
                    let pixel = rgba_img.get_pixel(x, y);
                    let r = format_value(pixel[0], config.base);
                    let g = format_value(pixel[1], config.base);
                    let b = format_value(pixel[2], config.base);
                    
                    let pixel_data = if config.channels == "RGB" || config.alpha == "drop" {
                        PixelData { r, g, b, a: None }
                    } else {
                        let a = format_value(pixel[3], config.base);
                        PixelData { r, g, b, a: Some(a) }
                    };
                    
                    row.push(pixel_data);
                }
                pixels.push(row);
            }
            
            ImageResult {
                image: image_path.file_name().unwrap().to_string_lossy().to_string(),
                width,
                height,
                channels: config.channels.clone(),
                base: config.base,
                pixels: Some(pixels),
                palette: None,
                histogram: None,
                error: None,
            }
        }
        Err(e) => ImageResult {
            image: image_path.file_name().unwrap().to_string_lossy().to_string(),
            width: 0,
            height: 0,
            channels: config.channels.clone(),
            base: config.base,
            pixels: None,
            palette: None,
            histogram: None,
            error: Some(e.to_string()),
        }
    }
}

fn process_image_palette(image_path: &Path, config: &Config) -> ImageResult {
    match image::open(image_path) {
        Ok(img) => {
            let rgba_img = img.to_rgba8();
            let (width, height) = rgba_img.dimensions();
            
            let mut color_counts: HashMap<[u8; 4], u32> = HashMap::new();
            
            for pixel in rgba_img.pixels() {
                let color = [pixel[0], pixel[1], pixel[2], pixel[3]];
                *color_counts.entry(color).or_insert(0) += 1;
            }
            
            let mut palette = Vec::new();
            for (color, count) in color_counts {
                let r = format_value(color[0], config.base);
                let g = format_value(color[1], config.base);
                let b = format_value(color[2], config.base);
                
                let pixel_data = if config.channels == "RGB" || config.alpha == "drop" {
                    PixelData { r, g, b, a: None }
                } else {
                    let a = format_value(color[3], config.base);
                    PixelData { r, g, b, a: Some(a) }
                };
                
                palette.push(PaletteEntry {
                    count,
                    color: pixel_data,
                });
            }
            
            // Ordenar por frecuencia descendente
            palette.sort_by(|a, b| b.count.cmp(&a.count));
            
            ImageResult {
                image: image_path.file_name().unwrap().to_string_lossy().to_string(),
                width,
                height,
                channels: config.channels.clone(),
                base: config.base,
                pixels: None,
                palette: Some(palette),
                histogram: None,
                error: None,
            }
        }
        Err(e) => ImageResult {
            image: image_path.file_name().unwrap().to_string_lossy().to_string(),
            width: 0,
            height: 0,
            channels: config.channels.clone(),
            base: config.base,
            pixels: None,
            palette: None,
            histogram: None,
            error: Some(e.to_string()),
        }
    }
}

fn process_image_histogram(image_path: &Path, config: &Config) -> ImageResult {
    match image::open(image_path) {
        Ok(img) => {
            let rgba_img = img.to_rgba8();
            let (width, height) = rgba_img.dimensions();
            
            let mut r_hist = [0u32; 256];
            let mut g_hist = [0u32; 256];
            let mut b_hist = [0u32; 256];
            let mut a_hist = [0u32; 256];
            
            for pixel in rgba_img.pixels() {
                r_hist[pixel[0] as usize] += 1;
                g_hist[pixel[1] as usize] += 1;
                b_hist[pixel[2] as usize] += 1;
                a_hist[pixel[3] as usize] += 1;
            }
            
            let r_data: Vec<String> = r_hist.iter().enumerate()
                .filter(|(_, &count)| count > 0)
                .map(|(i, count)| format!("{}:{}", format_value(i as u8, config.base), count))
                .collect();
            
            let g_data: Vec<String> = g_hist.iter().enumerate()
                .filter(|(_, &count)| count > 0)
                .map(|(i, count)| format!("{}:{}", format_value(i as u8, config.base), count))
                .collect();
            
            let b_data: Vec<String> = b_hist.iter().enumerate()
                .filter(|(_, &count)| count > 0)
                .map(|(i, count)| format!("{}:{}", format_value(i as u8, config.base), count))
                .collect();
            
            let histogram = if config.channels == "RGB" || config.alpha == "drop" {
                HistogramData {
                    r: r_data,
                    g: g_data,
                    b: b_data,
                    a: None,
                }
            } else {
                let a_data: Vec<String> = a_hist.iter().enumerate()
                    .filter(|(_, &count)| count > 0)
                    .map(|(i, count)| format!("{}:{}", format_value(i as u8, config.base), count))
                    .collect();
                
                HistogramData {
                    r: r_data,
                    g: g_data,
                    b: b_data,
                    a: Some(a_data),
                }
            };
            
            ImageResult {
                image: image_path.file_name().unwrap().to_string_lossy().to_string(),
                width,
                height,
                channels: config.channels.clone(),
                base: config.base,
                pixels: None,
                palette: None,
                histogram: Some(histogram),
                error: None,
            }
        }
        Err(e) => ImageResult {
            image: image_path.file_name().unwrap().to_string_lossy().to_string(),
            width: 0,
            height: 0,
            channels: config.channels.clone(),
            base: config.base,
            pixels: None,
            palette: None,
            histogram: None,
            error: Some(e.to_string()),
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    
    // Cargar configuración del archivo DSL
    let mut config = parse_logic_file(&args.logic)?;
    
    // Sobreescribir con argumentos de línea de comandos
    config.input_dir = args.input;
    config.output_dir = args.output;
    config.base = args.base;
    
    // Crear directorio de salida si no existe
    fs::create_dir_all(&config.output_dir)?;
    
    // Obtener archivos de imagen
    let image_files = get_image_files(&config.input_dir)?;
    
    if image_files.is_empty() {
        println!("No se encontraron archivos de imagen en {}", config.input_dir);
        return Ok(());
    }
    
    // Procesar cada imagen
    let mut results = Vec::new();
    
    for image_file in &image_files {
        println!("Procesando: {}", image_file.display());
        
        let result = match config.mode.as_str() {
            "per_pixel" => process_image_per_pixel(image_file, &config),
            "palette" => process_image_palette(image_file, &config),
            "histogram" => process_image_histogram(image_file, &config),
            _ => ImageResult {
                image: image_file.file_name().unwrap().to_string_lossy().to_string(),
                width: 0,
                height: 0,
                channels: config.channels.clone(),
                base: config.base,
                pixels: None,
                palette: None,
                histogram: None,
                error: Some(format!("Modo desconocido: {}", config.mode)),
            }
        };
        
        results.push(result);
    }
    
    // Guardar resultados
    if config.format == "json" {
        let output_file = Path::new(&config.output_dir).join("resultado.json");
        let json = serde_json::to_string_pretty(&results)?;
        fs::write(&output_file, json)?;
        println!("Resultados guardados en: {}", output_file.display());
    } else if config.format == "hex" {
        // Generar archivo hexadecimal plano
        let output_file = Path::new(&config.output_dir).join("resultado.hex");
        
        let mut hex_content = String::new();
        
        for result in &results {
            if let Some(error) = &result.error {
                hex_content.push_str(&format!("# Error procesando {}: {}\n", result.image, error));
                continue;
            }
            
            if let Some(pixels) = &result.pixels {
                for row in pixels.iter() {
                    for pixel in row.iter() {
                        // Solo el valor hexadecimal del píxel (RRGGBBAA o RRGGBB)
                        let hex_pixel = if let Some(a) = &pixel.a {
                            format!("{}{}{}{}",  pixel.r, pixel.g, pixel.b, a)
                        } else {
                            format!("{}{}{}", pixel.r, pixel.g, pixel.b)
                        };
                        hex_content.push_str(&format!("{}\n", hex_pixel));
                    }
                }
            } else if let Some(palette) = &result.palette {
                hex_content.push_str("# Paleta de colores (frecuencia:color)\n");
                for entry in palette {
                    let hex_color = if let Some(a) = &entry.color.a {
                        format!("{}{}{}{}", entry.color.r, entry.color.g, entry.color.b, a)
                    } else {
                        format!("{}{}{}", entry.color.r, entry.color.g, entry.color.b)
                    };
                    hex_content.push_str(&format!("{}:{}\n", entry.count, hex_color));
                }
            } else if let Some(histogram) = &result.histogram {
                hex_content.push_str("# Histograma por canal\n");
                hex_content.push_str("# Canal R:\n");
                for entry in &histogram.r {
                    hex_content.push_str(&format!("R:{}\n", entry));
                }
                hex_content.push_str("# Canal G:\n");
                for entry in &histogram.g {
                    hex_content.push_str(&format!("G:{}\n", entry));
                }
                hex_content.push_str("# Canal B:\n");
                for entry in &histogram.b {
                    hex_content.push_str(&format!("B:{}\n", entry));
                }
                if let Some(a_hist) = &histogram.a {
                    hex_content.push_str("# Canal A:\n");
                    for entry in a_hist {
                        hex_content.push_str(&format!("A:{}\n", entry));
                    }
                }
            }
            hex_content.push_str("\n");
        }
        
        fs::write(&output_file, hex_content)?;
        println!("Archivo hexadecimal guardado en: {}", output_file.display());
    }
    
    Ok(())
}