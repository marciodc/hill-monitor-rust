fn main() {
    if std::env::var("CARGO_CFG_TARGET_OS").unwrap_or_default() == "windows" {
        let mut res = winres::WindowsResource::new();
        res.set_icon_with_id("Resource/app.ico", "IDI_ICON1");
        res.compile().unwrap();
    }

    // Gerar app.png a partir do app.ico no diretório de recursos (se existir app.ico)
    let ico_path = std::path::Path::new("Resource/app.ico");
    let png_path = std::path::Path::new("Resource/app.png");
    if ico_path.exists() && !png_path.exists() {
        if let Ok(img) = image::open(ico_path) {
            let _ = img.save(png_path);
        }
    }
}
