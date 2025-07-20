# 📝 rust_html_gen

Conversor de archivos Markdown a HTML profesional, con soporte para índice, temas claro/oscuro, estilos personalizados y vista previa en servidor local. Hecho en Rust y accesible desde Python gracias a [PyO3](https://github.com/PyO3/pyo3).

---

## 🚀 Características

✅ Conversión Markdown → HTML  
✅ Índice automático de contenidos  
✅ Tema claro/oscuro configurable  
✅ Vista previa en vivo en navegador (`http://localhost:PUERTO`)  
✅ Uso desde Python mediante `maturin`  
✅ Totalmente personalizable vía JSON  

---

## 📦 Instalación

```bash
pip install maturin
maturin develop
````

---

## 🧪 Uso en Python

```python
import rust_html_gen

# Config JSON como string
config = """
{
  "title": "Demo",
  "include_toc": true,
  "theme": "dark",
  "toc_position": "left",
  "header": true,
  "custom_css": null,
  "lang": "es",
  "meta_description": "Ejemplo desde Python"
}
"""

# Generar archivo HTML
rust_html_gen.generar_html_desde_markdown("README.md", "salida.html", config)

# Vista previa en servidor (puerto 3000 por defecto)
rust_html_gen.vista_previa_html("README.md", config, 3000)
```

---

## 🧰 Configuración JSON de ejemplo

```json
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
```

---

## 💡 Autor

Desarrollado por **Ángel A. Urbina**
❤️ Código abierto para la comunidad.

---

## 📜 Licencia

MIT


