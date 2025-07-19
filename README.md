### 📄 `README.md`

````markdown
# 🧩 Markdown to HTML Generator — Powered by Rust & Python

Convierte archivos Markdown (`.md`) en páginas HTML profesionales, modernas y visualmente atractivas, usando un motor de conversión rápido y extensible escrito en Rust con integración total en Python mediante PyO3.

---

## ✨ Características

✅ Conversión de Markdown a HTML enriquecido  
✅ Índice de contenidos automático (TOC) con enlaces internos  
✅ Visualización profesional de fragmentos de código (`<pre><code>`)  
✅ Temas claro/oscuro personalizables  
✅ Configuración avanzada vía archivo JSON  
✅ Selección de archivos mediante ventana gráfica (GUI) multiplataforma  
✅ Integración directa con Python — ideal para flujos de trabajo personalizados

---

## ⚙️ Requisitos

- 🦀 Rust (v1.70+)
- 🐍 Python (v3.8+)
- [Poetry](https://python-poetry.org/) o `maturin` para compilar el módulo
- Dependencias Rust:
  ```toml
  [dependencies]
  pyo3 = { version = "0.21", features = ["extension-module"] }
  serde = { version = "1", features = ["derive"] }
  serde_json = "1"
  pulldown-cmark = "0.9"
  rfd = "0.14"
````

---

## 🚀 Instalación

### Opción 1: Usando `maturin`

```bash
pip install maturin
maturin develop
```

Esto compilará y expondrá el módulo como `rust_html_gen` directamente en Python.

---

## 🧪 Ejemplo de uso

```python
import rust_html_gen

md_path = rust_html_gen.seleccionar_archivo_markdown()
config_path = rust_html_gen.seleccionar_config_json()
output_path = rust_html_gen.seleccionar_ruta_salida_html()

if md_path and config_path and output_path:
    with open(config_path, 'r', encoding='utf-8') as f:
        config = f.read()
    rust_html_gen.generar_html_desde_markdown(md_path, output_path, config)
    print("✅ HTML generado con éxito.")
```

---

## 📂 Estructura esperada

```
📁 proyecto/
├── entrada.md               # Archivo Markdown a convertir
├── config.json              # Configuración del diseño y opciones
├── salida.html              # Archivo generado (se crea automáticamente)
├── generador_html.py        # Script Python de ejemplo
└── src/lib.rs               # Código fuente en Rust
```

---

## 🧰 JSON de configuración (`config.json`)

Puedes personalizar la conversión con un archivo JSON como este:

```json
{
  "title": "Mi Documento",
  "include_toc": true,
  "theme": "dark",
  "toc_position": "left",
  "header": true,
  "custom_css": null,
  "lang": "es",
  "meta_description": "Descripción SEO para tu página HTML generada"
}
```

¿Quieres un ejemplo programático desde Python?

```python
print(rust_html_gen.ayuda_configuracion())
```

---

## 🎨 Ejemplo visual

Fragmentos de código aparecen así:

```rust
fn main() {
    println!("¡Hola desde Rust!");
}
```

Están renderizados con gradientes, sombras, esquinas redondeadas y colores legibles — tanto en modo claro como oscuro.

---

## 🛠 Funciones disponibles en Python

* `generar_html_desde_markdown(path_md, path_output, config_json)`
* `seleccionar_archivo_markdown()` 🗂
* `seleccionar_config_json()` ⚙️
* `seleccionar_ruta_salida_html()` 📤
* `ayuda_configuracion()` 📋

---

## 🧑‍💻 Autor

Proyecto desarrollado por **Angel A. Urbina**
📫 Contacto: [perfil de LinkedIn](https://www.linkedin.com/in/angelurbina/)

---

## 📄 Licencia

Este proyecto está licenciado bajo los términos de la **MIT License**.

---

## 🌟 Contribuciones

Las contribuciones son bienvenidas. Abre un *Pull Request* o crea una *Issue* si tienes ideas o mejoras.
