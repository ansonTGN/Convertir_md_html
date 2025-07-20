use pyo3::prelude::*;
use pyo3::exceptions::{PyIOError, PyValueError};
use pulldown_cmark::{html, Event, Options, Parser, Tag};
use serde::Deserialize;
use std::fs::{self, create_dir_all};
use std::path::{Path, PathBuf};
use rfd::FileDialog;
use std::net::SocketAddr;
use std::thread;
use std::time::Duration;
use std::net::TcpListener;
use warp;

#[derive(Deserialize, Clone)]
struct Config {
    title: String,
    include_toc: bool,
    theme: Option<String>,
    toc_position: Option<String>,
    header: bool,
    custom_css: Option<String>,
    lang: Option<String>,
    meta_description: Option<String>,
}

struct Heading {
    level: usize,
    text: String,
    id: String,
}

fn sanitize_id(text: &str) -> String {
    text.to_lowercase()
        .replace([' ', '.', ':', ',', '(', ')', '[', ']', '/', '\'', '"'], "-")
        .replace("--", "-")
        .trim_matches('-')
        .to_string()
}

fn extract_headings_and_generate_html(markdown: &str) -> (Vec<Heading>, String) {
    let mut headings = Vec::new();
    let mut html_out = String::new();
    let mut in_heading = false;
    let mut heading_text = String::new();
    let mut current_level = 0;

    let parser = Parser::new_ext(markdown, Options::all());

    let mapped = parser.filter_map(|event| {
        match event {
            Event::Start(Tag::Heading(level, _, _)) => {
                in_heading = true;
                current_level = level as usize;
                heading_text.clear();
                None
            }
            Event::Text(text) if in_heading => {
                heading_text.push_str(&text);
                Some(Event::Text(text))
            }
            Event::End(Tag::Heading(_, _, _)) => {
                in_heading = false;
                let id = sanitize_id(&heading_text);
                headings.push(Heading {
                    level: current_level,
                    text: heading_text.clone(),
                    id: id.clone(),
                });
                let heading_html = format!(
                    "<h{lvl} id=\"{id}\">{text}</h{lvl}>",
                    lvl = current_level,
                    id = id,
                    text = heading_text
                );
                Some(Event::Html(heading_html.into()))
            }
            other => Some(other),
        }
    });

    html::push_html(&mut html_out, mapped);
    (headings, html_out)
}

fn generate_toc_html(headings: &[Heading]) -> String {
    let mut toc = String::new();
    let mut last_level = 0;

    for heading in headings {
        let level = heading.level;

        if level > last_level {
            for _ in last_level..level {
                toc.push_str("<ul>");
            }
        } else if level < last_level {
            for _ in level..last_level {
                toc.push_str("</ul>");
            }
        }

        toc.push_str(&format!(
            "<li><a href=\"#{id}\">{text}</a></li>",
            id = heading.id,
            text = heading.text
        ));
        last_level = level;
    }

    for _ in 0..last_level {
        toc.push_str("</ul>");
    }

    toc
}

fn build_full_html(toc: &str, body: &str, cfg: &Config) -> String {
    let lang = cfg.lang.clone().unwrap_or_else(|| "es".to_string());
    let meta = cfg.meta_description.clone().unwrap_or_default();
    let toc_position = cfg.toc_position.clone().unwrap_or_else(|| "left".to_string());
    let custom_css = cfg.custom_css.clone().unwrap_or_default();
    let dark_theme = cfg.theme.as_deref() == Some("dark");

    let toc_style = if toc_position == "right" {
        "order: 2;"
    } else {
        "order: 0;"
    };

    let theme_class = if dark_theme { "dark" } else { "" };

    format!(
        r#"<!DOCTYPE html>
<html lang="{lang}" class="{theme_class}">
<head>
  <meta charset="UTF-8">
  <title>{title}</title>
  <meta name="description" content="{meta}">
  <style>
    body {{
      margin: 0;
      font-family: "Segoe UI", Roboto, Helvetica, Arial, sans-serif;
      display: flex;
      height: 100vh;
      background: {bg};
      color: {fg};
    }}
    nav {{
      width: 280px;
      padding: 2rem 1.5rem;
      background: {nav_bg};
      color: {nav_fg};
      overflow-y: auto;
      border-right: 1px solid #ccc;
      {toc_style}
    }}
    nav a {{
      color: {link};
      text-decoration: none;
    }}
    nav a:hover {{
      background: {hover};
      color: {hover_fg};
    }}
    main {{
      flex: 1;
      padding: 2rem 3rem;
      overflow-y: auto;
    }}
    {custom_css}
  </style>
</head>
<body>
  <nav>
    <h2>Índice</h2>
    {toc}
  </nav>
  <main>
    {header}
    <section>{body}</section>
  </main>
</body>
</html>"#,
        title = cfg.title,
        meta = meta,
        lang = lang,
        toc = toc,
        body = body,
        header = if cfg.header {
            format!("<header><h1>{}</h1></header>", cfg.title)
        } else {
            "".to_string()
        },
        custom_css = custom_css,
        toc_style = toc_style,
        theme_class = theme_class,
        bg = if dark_theme { "#111827" } else { "#fdfdfd" },
        fg = if dark_theme { "#f3f4f6" } else { "#1c1c1c" },
        nav_bg = if dark_theme { "#1f2937" } else { "#e5e7eb" },
        nav_fg = if dark_theme { "#f9fafb" } else { "#111827" },
        link = if dark_theme { "#93c5fd" } else { "#2563eb" },
        hover = if dark_theme { "#374151" } else { "#cbd5e1" },
        hover_fg = if dark_theme { "#ffffff" } else { "#1e293b" }
    )
}

#[pyfunction]
fn generar_html_desde_markdown(path_md: String, path_output: String, config_json: String) -> PyResult<()> {
    if !Path::new(&path_md).exists() {
        return Err(PyIOError::new_err("Archivo Markdown no existe"));
    }

    let markdown_content = fs::read_to_string(&path_md)
        .map_err(|e| PyIOError::new_err(format!("Error leyendo archivo: {}", e)))?;

    let config: Config = serde_json::from_str(&config_json)
        .map_err(|e| PyValueError::new_err(format!("Error en config JSON: {}", e)))?;

    let (headings, html_body) = extract_headings_and_generate_html(&markdown_content);
    let toc_html = if config.include_toc {
        generate_toc_html(&headings)
    } else {
        "".to_string()
    };

    let final_html = build_full_html(&toc_html, &html_body, &config);

    if let Some(parent) = Path::new(&path_output).parent() {
        create_dir_all(parent).ok();
    }

    fs::write(&path_output, final_html)
        .map_err(|e| PyIOError::new_err(format!("Error escribiendo HTML: {}", e)))?;

    Ok(())
}

#[pyfunction]
fn generar_html_interactivo() -> PyResult<()> {
    let archivo_md = FileDialog::new().add_filter("Markdown", &["md"]).pick_file();
    let config_file = FileDialog::new().add_filter("JSON", &["json"]).pick_file();
    let salida = FileDialog::new().add_filter("HTML", &["html"]).set_file_name("output.html").save_file();

    if let (Some(md), Some(cfg), Some(out)) = (archivo_md, config_file, salida) {
        generar_html_desde_markdown(
            md.to_string_lossy().to_string(),
            out.to_string_lossy().to_string(),
            fs::read_to_string(cfg).map_err(|e| PyIOError::new_err(format!("Error leyendo JSON: {}", e)))?
        )?;
    }

    Ok(())
}

#[pyfunction]
fn ver_html_local(path_html: String, puerto: Option<u16>) -> PyResult<()> {
    let port = puerto.unwrap_or(3000);
    let path = PathBuf::from(&path_html);

    let dir = path.parent()
        .ok_or_else(|| PyValueError::new_err("No se puede determinar el directorio base del archivo"))?
        .to_path_buf();

    let addr = SocketAddr::from(([127, 0, 0, 1], port));

    if TcpListener::bind(addr).is_err() {
        return Err(PyIOError::new_err(format!("El puerto {} ya está en uso. Usa otro.", port)));
    }

    // Usar Tokio para ejecutar el futuro de warp
    thread::spawn(move || {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async move {
            println!("🌍 Sirviendo HTML desde http://localhost:{port}");
            let files = warp::fs::dir(dir);
            warp::serve(files).run(addr).await;
        });
    });

    thread::sleep(Duration::from_millis(500));
    Ok(())
}

#[pyfunction]
fn ayuda_configuracion() -> PyResult<String> {
    Ok(r#"
{
  "title": "Mi Documento",
  "include_toc": true,
  "theme": "light",
  "toc_position": "left",
  "header": true,
  "custom_css": null,
  "lang": "es",
  "meta_description": "Descripción del documento"
}
"#.to_string())
}

#[pymodule]
fn rust_html_gen(m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(generar_html_desde_markdown, m)?)?;
    m.add_function(wrap_pyfunction!(generar_html_interactivo, m)?)?;
    m.add_function(wrap_pyfunction!(ver_html_local, m)?)?;
    m.add_function(wrap_pyfunction!(ayuda_configuracion, m)?)?;
    Ok(())
}

