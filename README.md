# 🦀 rust_html_gen · Markdown ➜ HTML profesional desde Python

[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Language](https://img.shields.io/badge/language-Rust-orange.svg)](https://www.rust-lang.org/)
[![Python](https://img.shields.io/badge/python-3.8%2B-blue.svg)](https://www.python.org/)
[![Build](https://img.shields.io/badge/build-maturin-green.svg)](https://github.com/PyO3/maturin)

---

## 📌 Descripción (ES)

`rust_html_gen` es una biblioteca escrita en **Rust** con bindings para **Python** mediante [PyO3](https://pyo3.rs/), que convierte archivos Markdown (`.md`) en documentos HTML visualmente atractivos y configurables.

---

## 📚 Tabla de Contenidos

- [🚀 Características](#-características)
- [🧪 Ejemplo de uso](#-ejemplo-de-uso)
- [📦 Instalación](#-instalación)
- [🧰 Configuración JSON](#-configuración-json)
- [📁 Estructura del Proyecto](#-estructura-del-proyecto)
- [🧭 Funciones](#-funciones)
- [👤 Autor](#-autor)
- [🇬🇧 English Version](#-english-version)

---

## 🚀 Características

- Conversión Markdown → HTML con soporte para encabezados, listas, tablas y más.
- Índice automático (TOC), con anidamiento correcto.
- Temas claro/oscuro.
- CSS personalizado embebido.
- Vista previa en `localhost`.
- Modo interactivo con ventanas gráficas de selección de archivos.
- Fácilmente integrable en scripts Python.

---

## 🧪 Ejemplo de uso

```python
import rust_html_gen
import json

with open("config.json") as f:
    config = json.load(f)

rust_html_gen.generar_html_desde_markdown(
    "tests/ejemplo.md",
    "tests/salida.html",
    json.dumps(config)
)

rust_html_gen.ver_html_local("tests/salida.html", 3000)
````

---

## 📦 Instalación

```bash
maturin develop
# o para generar el wheel:
maturin build
```

---

## 🧰 Configuración JSON

```json
{
  "title": "Mi Documento",
  "include_toc": true,
  "theme": "light",
  "toc_position": "left",
  "header": true,
  "custom_css": "...",
  "lang": "es",
  "meta_description": "Descripción para buscadores"
}
```

Puedes obtener una plantilla directamente:

```python
print(rust_html_gen.ayuda_configuracion())
```

---

## 📁 Estructura del Proyecto

```
rust_html_gen/
├── src/
│   └── lib.rs
├── tests/
│   ├── ejemplo.md
│   └── config.json
├── test.py
├── Cargo.toml
├── pyproject.toml
└── README.md
```

---

## 🧭 Funciones disponibles

| Función                         | Descripción                                  |
| ------------------------------- | -------------------------------------------- |
| `generar_html_desde_markdown()` | Convierte `.md` a `.html` con configuración. |
| `generar_html_interactivo()`    | Modo con ventanas gráficas.                  |
| `ver_html_local()`              | Servidor en `localhost`.                     |
| `ayuda_configuracion()`         | Devuelve JSON base de configuración.         |

---

## 👤 Autor

Proyecto desarrollado por **Ángel A. Urbina**
📫 Contacto: [LinkedIn](https://www.linkedin.com) · [Github](https://github.com)

---

# 🇬🇧 English Version

## 🦀 rust\_html\_gen · Markdown ➜ Beautiful HTML from Python

[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Language](https://img.shields.io/badge/language-Rust-orange.svg)](https://www.rust-lang.org/)
[![Python](https://img.shields.io/badge/python-3.8%2B-blue.svg)](https://www.python.org/)
[![Build](https://img.shields.io/badge/build-maturin-green.svg)](https://github.com/PyO3/maturin)

---

## 📌 Description

`rust_html_gen` is a Rust library with Python bindings via [PyO3](https://pyo3.rs/) to transform Markdown (`.md`) into beautifully formatted and configurable HTML documents.

---

## 📚 Table of Contents

* [🚀 Features](#-features)
* [🧪 Example Usage](#-example-usage)
* [📦 Installation](#-installation)
* [🧰 JSON Config](#-json-config)
* [📁 Project Structure](#-project-structure)
* [🧭 Functions](#-functions)
* [👤 Author](#-author)

---

## 🚀 Features

* Markdown → HTML conversion with full feature support
* Auto-generated TOC with nesting
* Light/Dark theme support
* Inline or external CSS
* Local preview via browser
* Interactive GUI mode
* Python-friendly

---

## 🧪 Example Usage

```python
import rust_html_gen
import json

with open("config.json") as f:
    config = json.load(f)

rust_html_gen.generar_html_desde_markdown(
    "tests/ejemplo.md",
    "tests/salida.html",
    json.dumps(config)
)

rust_html_gen.ver_html_local("tests/salida.html", 3000)
```

---

## 📦 Installation

```bash
maturin develop
# or
maturin build
```

---

## 🧰 JSON Config

```json
{
  "title": "My Document",
  "include_toc": true,
  "theme": "light",
  "toc_position": "left",
  "header": true,
  "custom_css": "...",
  "lang": "en",
  "meta_description": "For search engines"
}
```

Generate template directly:

```python
print(rust_html_gen.ayuda_configuracion())
```

---

## 📁 Project Structure

```
rust_html_gen/
├── src/
│   └── lib.rs
├── tests/
│   ├── ejemplo.md
│   └── config.json
├── test.py
├── Cargo.toml
├── pyproject.toml
└── README.md
```

---

## 🧭 Available Functions

| Function                        | Description                            |
| ------------------------------- | -------------------------------------- |
| `generar_html_desde_markdown()` | Converts `.md` to `.html` with config. |
| `generar_html_interactivo()`    | Opens GUI file dialogs.                |
| `ver_html_local()`              | Launches preview server.               |
| `ayuda_configuracion()`         | Returns JSON config template.          |

---

## 👤 Author

Developed by **Ángel A. Urbina**
📫 Contact: [LinkedIn](https://www.linkedin.com/in/angelurbina/)


