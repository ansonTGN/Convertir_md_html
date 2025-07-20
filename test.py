import rust_html_gen
import json
import time
import webbrowser
from pathlib import Path

# Rutas relativas
md_path = "tests/Consideraciones.md"
html_path = "tests/salida.html"
config_path = "tests/config.json"

print("📄 Cargando configuración desde JSON...")
try:
    with open(config_path, "r", encoding="utf-8") as f:
        config = json.load(f)
except Exception as e:
    print("❌ Error al leer el archivo de configuración:", e)
    exit(1)

print("🧠 Generando HTML desde el Markdown...")
try:
    rust_html_gen.generar_html_desde_markdown(
        md_path,
        html_path,
        json.dumps(config)
    )
    print(f"✅ HTML generado en: {html_path}")
except Exception as e:
    print("❌ Error durante la generación:", e)
    exit(1)

# Esperamos un momento antes de levantar servidor
time.sleep(1)

print("🌍 Abriendo servidor local en el navegador...")
try:
    rust_html_gen.ver_html_local(html_path, 3000)
    webbrowser.open("http://localhost:3000/salida.html")
except Exception as e:
    print("❌ Error al abrir el servidor:", e)

