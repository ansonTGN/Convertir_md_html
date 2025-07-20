# 📝 rust_html_gen

A high-quality Markdown-to-HTML converter with support for:

✅ Table of contents  
✅ Light/Dark themes  
✅ Custom CSS styling  
✅ Live preview server on `http://localhost:<PORT>`  
✅ Full Python bindings via [PyO3](https://github.com/PyO3/pyo3)

Built in Rust. Designed to be used from Python with performance and flexibility in mind.

---

## 🚀 Features

- ⚡ Fast Markdown to HTML conversion
- 🧭 Automatic nested Table of Contents
- 🎨 Light/Dark themes (configurable)
- 🖼️ Custom CSS support
- 🌐 Local live preview via web server
- 🐍 Easy integration with Python (via `maturin`)

---

## 📦 Installation

Install the required tool:

```bash
pip install maturin
````

Then install the library locally:

```bash
maturin develop
```

This will compile the Rust code and make it importable from Python.

---

## 🧪 Example usage in Python

```python
import rust_html_gen

# JSON config string
config = """
{
  "title": "Demo Document",
  "include_toc": true,
  "theme": "dark",
  "toc_position": "left",
  "header": true,
  "custom_css": null,
  "lang": "en",
  "meta_description": "Generated from Python"
}
"""

# Convert Markdown to HTML
rust_html_gen.generar_html_desde_markdown("README.md", "output.html", config)

# Serve the HTML in a local live preview (defaults to port 3000)
rust_html_gen.vista_previa_html("README.md", config, 3000)
```

---

## ⚙️ JSON Configuration Schema

```json
{
  "title": "My Document",
  "include_toc": true,
  "theme": "light",
  "toc_position": "left",
  "header": true,
  "custom_css": null,
  "lang": "en",
  "meta_description": "Description for the page"
}
```

---

## 📚 API Overview

| Function                           | Description                               |
| ---------------------------------- | ----------------------------------------- |
| `generar_html_desde_markdown(...)` | Converts a `.md` file to HTML with config |
| `vista_previa_html(...)`           | Serves the HTML via local web server      |
| `ayuda_configuracion()`            | Returns an example of the JSON config     |

---

## 👤 Author

Developed by **Ángel A. Urbina**
Made with ❤️ for developers and content creators.

---

## 📄 License

MIT
