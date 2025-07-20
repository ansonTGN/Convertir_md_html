### 🇪🇸 Sección en Español

### 🇬🇧 Sección en Inglés (debajo)

---

````markdown
# 🦀 rust_html_gen · Convertidor de Markdown a HTML profesional desde Python

`rust_html_gen` es una biblioteca escrita en **Rust** con bindings para **Python** usando [PyO3](https://pyo3.rs/), que permite transformar archivos Markdown (`.md`) en documentos HTML profesionales, visualmente atractivos y con soporte de configuración avanzada (temas, estilos personalizados, índice, etc.).

## 🚀 Características

- Conversión de Markdown a HTML con soporte para encabezados, listas, bloques de código y más.
- Generación automática de índice de contenidos (TOC), con anidamiento correcto.
- Temas claro u oscuro personalizables.
- CSS personalizado embebido o externo.
- Servidor local para visualizar el HTML generado (`http://localhost:3000` por defecto).
- Modo interactivo con selección de archivos gráficamente.
- Totalmente accesible desde Python.

---

## 🧪 Ejemplo de uso desde Python

```python
import rust_html_gen
import json

config = {
    "title": "Mi Documento",
    "include_toc": True,
    "theme": "dark",
    "toc_position": "left",
    "header": True,
    "custom_css": None,
    "lang": "es",
    "meta_description": "Ejemplo generado con rust_html_gen"
}

rust_html_gen.generar_html_desde_markdown(
    "entrada.md",
    "salida.html",
    json.dumps(config)
)

# Servir HTML generado en navegador
rust_html_gen.ver_html_local("salida.html", 3000)
````

---

## 🧰 Instalación

```bash
maturin develop
```

O puedes generar el `.whl` con:

```bash
maturin build
```

---

## 📝 Formato del archivo de configuración JSON

```json
{
  "title": "Título del documento",
  "include_toc": true,
  "theme": "light",
  "toc_position": "left",
  "header": true,
  "custom_css": null,
  "lang": "es",
  "meta_description": "Descripción para buscadores"
}
```

Puedes obtener esta plantilla directamente desde Python:

```python
print(rust_html_gen.ayuda_configuracion())
```

---

## 🧭 Funciones disponibles

| Función                            | Descripción                                                          |
| ---------------------------------- | -------------------------------------------------------------------- |
| `generar_html_desde_markdown(...)` | Convierte un archivo `.md` a `.html`.                                |
| `generar_html_interactivo()`       | Permite seleccionar archivos mediante ventanas gráficas.             |
| `ver_html_local(...)`              | Levanta un servidor en `localhost` para visualizar el HTML generado. |
| `ayuda_configuracion()`            | Devuelve una plantilla JSON de configuración válida.                 |

---

## 👤 Autor

Creado por **Ángel A. Urbina**.
Inspirado en la necesidad de generar documentación profesional y flexible de forma sencilla.

---

---

# 🦀 rust\_html\_gen · Professional Markdown to HTML Converter from Python

`rust_html_gen` is a **Rust** library with **Python** bindings via [PyO3](https://pyo3.rs/), that transforms Markdown (`.md`) files into beautiful, configurable HTML documents.

## 🚀 Features

* Markdown to HTML conversion with support for headings, lists, code blocks, etc.
* Automatic Table of Contents (TOC) generation with correct nesting.
* Light or dark theme support.
* Embedded or external custom CSS.
* Local server to preview the HTML (`http://localhost:3000` by default).
* Interactive file selector (graphical).
* Fully accessible from Python scripts.

---

## 🧪 Usage Example (Python)

```python
import rust_html_gen
import json

config = {
    "title": "My Document",
    "include_toc": True,
    "theme": "light",
    "toc_position": "left",
    "header": True,
    "custom_css": None,
    "lang": "en",
    "meta_description": "Example generated with rust_html_gen"
}

rust_html_gen.generar_html_desde_markdown(
    "input.md",
    "output.html",
    json.dumps(config)
)

# Serve locally
rust_html_gen.ver_html_local("output.html", 3000)
```

---

## 🧰 Installation

```bash
maturin develop
```

Or to generate the wheel:

```bash
maturin build
```

---

## 📝 JSON Configuration Format

```json
{
  "title": "Document title",
  "include_toc": true,
  "theme": "light",
  "toc_position": "left",
  "header": true,
  "custom_css": null,
  "lang": "en",
  "meta_description": "Meta description for SEO"
}
```

Get this template directly from Python:

```python
print(rust_html_gen.ayuda_configuracion())
```

---

## 🧭 Available Functions

| Function                           | Description                         |
| ---------------------------------- | ----------------------------------- |
| `generar_html_desde_markdown(...)` | Convert `.md` to `.html`.           |
| `generar_html_interactivo()`       | Select files via GUI dialog.        |
| `ver_html_local(...)`              | Launch local server to view result. |
| `ayuda_configuracion()`            | Returns a valid JSON template.      |

---

## 👤 Author

Created by **Ángel A. Urbina**.
Inspired by the need to produce clean and professional HTML documents from Markdown with ease.

```


