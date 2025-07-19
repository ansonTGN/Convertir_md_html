import rust_html_gen

md = rust_html_gen.seleccionar_archivo_markdown()
cfg = rust_html_gen.seleccionar_config_json()
out = rust_html_gen.seleccionar_ruta_salida_html()

if md and cfg and out:
    with open(cfg, "r", encoding="utf-8") as f:
        config = f.read()
    rust_html_gen.generar_html_desde_markdown(md, out, config)
    print(f"✅ HTML generado: {out}")
