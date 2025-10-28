use std::env;
use std::fs;
use std::io::{self, Read};
use std::path::Path;
use std::process::{Command, Stdio};

#[derive(Debug)]
struct Meta {
    palabra1: String,
    palabra2_enc: String,
}

fn load_kv_file<P: AsRef<Path>>(path: P) -> io::Result<Vec<(String, String)>> {
    let content = fs::read_to_string(path)?;
    let mut out = Vec::new();
    for line in content.lines() {
        let s = line.trim();
        if s.is_empty() || s.starts_with('#') { continue; }
        if let Some((k, v)) = s.split_once('=') {
            out.push((k.trim().to_string(), v.trim().to_string()));
        }
    }
    Ok(out)
}

fn load_meta<P: AsRef<Path>>(path: P) -> io::Result<Meta> {
    let kvs = load_kv_file(path)?;
    let mut palabra1 = None;
    let mut palabra2_enc = None;
    for (k, v) in kvs {
        match k {
            ref kk if kk.eq_ignore_ascii_case("PALABRA1") => palabra1 = Some(v),
            ref kk if kk.eq_ignore_ascii_case("PALABRA2_ENC") => palabra2_enc = Some(v.to_lowercase()),
            _ => {}
        }
    }
    let palabra1 = palabra1.ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Falta PALABRA1"))?;
    let palabra2_enc = palabra2_enc.ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Falta PALABRA2_ENC"))?;
    Ok(Meta { palabra1, palabra2_enc })
}

fn extract_tokens<P: AsRef<Path>>(path: P) -> io::Result<Vec<String>> {
    let mut f = fs::File::open(path)?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    let tokens = s
        .split(|c: char| c.is_whitespace() || c == ',' || c == ';' || c == '|' )
        .filter(|t| !t.is_empty())
        .map(|t| t.to_string())
        .collect::<Vec<_>>();
    Ok(tokens)
}

fn validar(tokens: &[String], meta: &Meta) -> bool {
    let has_p1 = tokens.iter().any(|t| t == &meta.palabra1);
    let has_p2 = {
        let p2 = meta.palabra2_enc.to_lowercase();
        tokens.iter().any(|t| t.to_lowercase() == p2)
    };
    has_p1 && has_p2
}

fn load_run_line<P: AsRef<Path>>(path: P) -> io::Result<String> {
    let content = fs::read_to_string(path)?;
    for line in content.lines() {
        let s = line.trim();
        if s.is_empty() || s.starts_with('#') { continue; }
        return Ok(s.to_string());
    }
    Err(io::Error::new(io::ErrorKind::InvalidData, "programa.run.txt no contiene comando"))
}

fn run_command_line(cmdline: &str) -> io::Result<i32> {
    let args = shell_words::split(cmdline)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, format!("No se pudo parsear comando: {}", e)))?;

    if args.is_empty() {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "Comando vacío"));
    }

    let program = &args[0];
    let program_args = &args[1..];

    let status = Command::new(program)
        .args(program_args)
        .stdin(Stdio::null())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()?;

    Ok(status.code().unwrap_or(-1))
}

fn main() -> io::Result<()> {
    // Uso: integrador <doc_usuario.txt> [programa.meta.txt] [programa.run.txt]
    let args: Vec<String> = env::args().collect();
    let exe = &args[0];
    if args.len() < 2 {
        eprintln!("Uso: {exe} <doc_usuario.txt> [programa.meta.txt] [programa.run.txt]");
        std::process::exit(2);
    }

    let user_doc = &args[1];
    let meta_path = args.get(2).cloned().unwrap_or_else(|| "programa.meta.txt".to_string());
    let run_path  = args.get(3).cloned().unwrap_or_else(|| "programa.run.txt".to_string());

    let meta = load_meta(&meta_path)?;
    let tokens = extract_tokens(&user_doc)?;

    if !validar(&tokens, &meta) {
        std::process::exit(1);
    }

    let cmdline = load_run_line(&run_path)?;
    let code = run_command_line(&cmdline)?;
    std::process::exit(code);
}
