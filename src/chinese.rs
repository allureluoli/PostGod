use eframe::egui;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();

    eframe::run_native(
        "egui 中文测试",
        options,
        Box::new(|cc| {
            setup_custom_fonts(&cc.egui_ctx);
            Ok(Box::new(MyApp::default()))
        }),
    )
}

#[derive(Default)]
struct MyApp;

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("你好，egui");
            ui.label("这是一段中文文本。");
            ui.label("如果你能看到这句话，说明中文字体加载成功。");
        });
    }
}

fn setup_custom_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();

    fonts.font_data.insert(
        "my_chinese_font".to_owned(),
        std::sync::Arc::new(egui::FontData::from_owned(
            std::fs::read("/usr/share/fonts/opentype/noto/NotoSansCJK-Regular.ttc")
                .expect("failed to read Chinese font"),
        )),
    );

    fonts
        .families
        .entry(egui::FontFamily::Proportional)
        .or_default()
        .insert(0, "my_chinese_font".to_owned());

    fonts
        .families
        .entry(egui::FontFamily::Monospace)
        .or_default()
        .push("my_chinese_font".to_owned());

    ctx.set_fonts(fonts);
}