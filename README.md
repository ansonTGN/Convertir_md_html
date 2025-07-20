# 🦀 rust_html_gen · Markdown a HTML Profesional con Rust y Python

[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Language](https://img.shields.io/badge/language-Rust-orange.svg)](https://www.rust-lang.org/)
[![Python](https://img.shields.io/badge/python-3.8%2B-blue.svg)](https://www.python.org/)
[![Build](https://img.shields.io/badge/build-maturin-green.svg)](https://github.com/PyO3/maturin)

---

## 📌 Descripción

`rust_html_gen` es una potente biblioteca, escrita en **Rust**, que utiliza [PyO3](https://pyo3.rs/) para ofrecer *bindings* de alto rendimiento en **Python**. Convierte tus archivos Markdown (`.md`) en documentos HTML visualmente atractivos, personalizables y listos para la web.

---

## ✨ Características Principales

*   **Conversión Markdown a HTML:** Soporte completo para encabezados, listas, tablas, código y mucho más.
*   **Índice Automático (TOC):** Genera un índice jerárquico automáticamente para facilitar la navegación.
*   **Temas:** Elige entre temas claro y oscuro, o personaliza tu propio estilo.
*   **CSS Personalizado:** Incorpora estilos CSS personalizados para un control total sobre la apariencia.
*   **Vista Previa Local:** Previsualiza tus documentos HTML en un servidor local con recarga automática.
*   **Interfaz Interactiva:** Utiliza ventanas gráficas para seleccionar archivos y configurar opciones.
*   **Integración Sencilla:** Llama a las funciones de `rust_html_gen` directamente desde tus scripts de Python.

---

## ⚙️ Ejemplo de Uso

```python
import rust_html_gen
import json

# Cargar la configuración desde un archivo JSON
with open("config.json") as f:
    config = json.load(f)

# Generar el archivo HTML desde Markdown
rust_html_gen.generar_html_desde_markdown(
    "tests/ejemplo.md",  # Archivo Markdown de entrada
    "tests/salida.html",   # Archivo HTML de salida
    json.dumps(config)    # Configuración en formato JSON
)

# Iniciar un servidor local para ver el resultado
rust_html_gen.ver_html_local("tests/salida.html", 3000)
```

---

## 🛠️ Instalación

1.  **Requisitos:** Asegúrate de tener Rust y Python instalados.
2.  **Instalar con Maturin:**

    ```bash
    maturin develop  # Para desarrollo (se instala en el entorno virtual)
    # o
    maturin build    # Para generar un wheel distribuible
    ```

---

## 🧰 Configuración (JSON)

Personaliza la apariencia y el comportamiento de `rust_html_gen` con un archivo JSON:

```json
{
  "title": "Título del Documento",
  "include_toc": true,
  "theme": "light",
  "toc_position": "left",
  "header": true,
  "custom_css": "/* Aquí tu CSS personalizado */",
  "lang": "es",
  "meta_description": "Descripción para motores de búsqueda"
}
```

Obtén una plantilla de configuración básica directamente desde Python:

```python
print(rust_html_gen.ayuda_configuracion())
```

---

## 📂 Estructura del Proyecto

```
rust_html_gen/
├── src/            # Código fuente de Rust
│   └── lib.rs
├── tests/          # Archivos de prueba
│   ├── ejemplo.md
│   └── config.json
├── test.py         # Script de prueba en Python
├── Cargo.toml      # Configuración de Rust
├── pyproject.toml  # Configuración de Python
└── README.md       # Este archivo
```

---

## ⚙️ Funciones Disponibles

| Función                         | Descripción                                                                 |
| :------------------------------ | :-------------------------------------------------------------------------- |
| `generar_html_desde_markdown()` | Convierte un archivo Markdown a HTML utilizando la configuración JSON.     |
| `generar_html_interactivo()`    | Abre una interfaz gráfica para seleccionar archivos y generar el HTML.       |
| `ver_html_local()`              | Inicia un servidor local para previsualizar el archivo HTML.                |
| `ayuda_configuracion()`         | Devuelve una cadena JSON con la estructura básica del archivo de configuración. |

---

## 👤 Autor

Desarrollado por **Ángel A. Urbina**

*   📫 Contacto: [LinkedIn](https://www.linkedin.com) · [GitHub](https://github.com/ansonTGN)

---

## 🌐 English Version

## 🦀 rust\_html\_gen · Markdown to Beautiful HTML with Rust and Python

[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Language](https://img.shields.io/badge/language-Rust-orange.svg)](https://www.rust-lang.org/)
[![Python](https://img.shields.io/badge/python-3.8%2B-blue.svg)](https://www.python.org/)
[![Build](https://img.shields.io/badge/build-maturin-green.svg)](https://github.com/PyO3/maturin)

---

## 📌 Description

`rust_html_gen` is a powerful library, written in **Rust**, that uses [PyO3](https://pyo3.rs/) to provide high-performance Python bindings. It converts your Markdown (`.md`) files into visually appealing, customizable, and web-ready HTML documents.

---

## ✨ Key Features

*   **Markdown to HTML Conversion:** Full support for headings, lists, tables, code, and more.
*   **Automatic Table of Contents (TOC):** Generates a hierarchical table of contents automatically for easy navigation.
*   **Themes:** Choose between light and dark themes, or customize your own style.
*   **Custom CSS:** Incorporate custom CSS styles for total control over appearance.
*   **Local Preview:** Preview your HTML documents on a local server with automatic reloading.
*   **Interactive Interface:** Use graphical windows to select files and configure options.
*   **Easy Integration:** Call `rust_html_gen` functions directly from your Python scripts.

---

## ⚙️ Example Usage

```python
import rust_html_gen
import json

# Load configuration from a JSON file
with open("config.json") as f:
    config = json.load(f)

# Generate the HTML file from Markdown
rust_html_gen.generar_html_desde_markdown(
    "tests/ejemplo.md",  # Input Markdown file
    "tests/salida.html",   # Output HTML file
    json.dumps(config)    # Configuration in JSON format
)

# Start a local server to view the result
rust_html_gen.ver_html_local("tests/salida.html", 3000)
```

---

## 🛠️ Installation

1.  **Prerequisites:** Make sure you have Rust and Python installed.
2.  **Install with Maturin:**

    ```bash
    maturin develop  # For development (installs in the virtual environment)
    # or
    maturin build    # To generate a distributable wheel
    ```

---

## 🧰 Configuration (JSON)

Customize the appearance and behavior of `rust_html_gen` with a JSON file:

```json
{
  "title": "Document Title",
  "include_toc": true,
  "theme": "light",
  "toc_position": "left",
  "header": true,
  "custom_css": "/* Your custom CSS here */",
  "lang": "en",
  "meta_description": "Description for search engines"
}
```

Get a basic configuration template directly from Python:

```python
print(rust_html_gen.ayuda_configuracion())
```

---

## 📂 Project Structure

```
rust_html_gen/
├── src/            # Rust source code
│   └── lib.rs
├── tests/          # Test files
│   ├── ejemplo.md
│   └── config.json
├── test.py         # Python test script
├── Cargo.toml      # Rust configuration
├── pyproject.toml  # Python configuration
└── README.md       # This file
```

---

## ⚙️ Available Functions

| Function                        | Description                                                                |
| :------------------------------ | :------------------------------------------------------------------------- |
| `generar_html_desde_markdown()` | Converts a Markdown file to HTML using the JSON configuration.            |
| `generar_html_interactivo()`    | Opens a graphical interface to select files and generate the HTML.          |
| `ver_html_local()`              | Starts a local server to preview the HTML file.                             |
| `ayuda_configuracion()`         | Returns a JSON string with the basic structure of the configuration file. |

---

## 👤 Author

Developed by **Ángel A. Urbina**

*   📫 Contact: [LinkedIn](https://www.linkedin.com) · [GitHub](https://github.com/ansonTGN)
