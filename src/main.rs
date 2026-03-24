use eframe::egui;
use egui::{FontData, FontDefinitions, FontFamily};

mod request_send;
use request_send::{send_get, send_post};

fn setup_custom_fonts(ctx: &egui::Context) {
    let mut fonts = FontDefinitions::default();

    fonts.font_data.insert(
        "my_chinese_font".to_owned(),
        FontData::from_static(include_bytes!("../assets/AaManHuaJia-2.ttf")).into(),
    );

    fonts
        .families
        .entry(FontFamily::Proportional)
        .or_default()
        .insert(0, "my_chinese_font".to_owned());

    fonts
        .families
        .entry(FontFamily::Monospace)
        .or_default()
        .push("my_chinese_font".to_owned());

    ctx.set_fonts(fonts);
}

fn main() -> eframe::Result<()> {

    // 默认窗口大小
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1400.0, 900.0])
            .with_min_inner_size([1000.0, 700.0]),
            centered: true,
        ..Default::default()
    };
    eframe::run_native(
        "GET/POST CTF GOD",
        
        options,
        Box::new(|cc| {
            setup_custom_fonts(&cc.egui_ctx);
            Ok(Box::new(MyApp::default()))
        }),
    )
}

#[derive(Default)]
struct MyApp {
    count: i32,
    base_url: String,
    param: String,
    url_preview: String,
    post_body: String,
    headers_text: String,
    result: String,
    error_msg: String,
}

impl MyApp {
    fn parse_headers(&self) -> Result<Vec<(String, String)>, String> {
        let mut headers = Vec::new();

        for (idx, line) in self.headers_text.lines().enumerate() {
            let line = line.trim();

            if line.is_empty() {
                continue;
            }

            let Some((key, value)) = line.split_once(':') else {
                return Err(format!("第 {} 行请求头格式错误，应为 Key: Value", idx + 1));
            };

            headers.push((key.trim().to_string(), value.trim().to_string()));
        }

        Ok(headers)
    }

    fn fill_browser_headers(&mut self) {
        self.headers_text = r#"User-Agent: Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:148.0) Gecko/20100101 Firefox/148.0
Accept: text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8
Accept-Language: zh-CN,zh;q=0.9,en;q=0.8
Connection: keep-alive
Upgrade-Insecure-Requests: 1
Cache-Control: no-cache
Pragma: no-cache"#
            .to_string();
    }

    fn clear_all(&mut self) {
        self.base_url.clear();
        self.param.clear();
        self.url_preview.clear();
        self.post_body.clear();
        self.headers_text.clear();
        self.result.clear();
        self.error_msg.clear();
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let request_url = if self.param.trim().is_empty() {
            self.base_url.trim().to_string()
        } else {
            format!("{}{}", self.base_url.trim(), self.param.trim())
        };
        self.url_preview = request_url;

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("GET/POST CTF GOD");
            ui.separator();

            ui.columns(2, |columns| {
                let [left, right] = columns else {
                    return;
                };

                left.set_min_width(420.0);
                right.set_min_width(720.0);

                left.heading("Request");

                left.horizontal(|ui| {
                    ui.label("URL:");
                    ui.text_edit_singleline(&mut self.base_url);
                });

                left.horizontal(|ui| {
                    ui.label("Param:");
                    ui.text_edit_singleline(&mut self.param);
                });

                if !self.url_preview.is_empty() {
                    left.label(format!("Request URL: {}", self.url_preview));
                }

                left.separator();
                left.label("Custom Headers (每行一个，格式：Key: Value):");

                egui::ScrollArea::vertical()
                    .id_salt("headers_scroll_area")
                    .max_height(150.0)
                    .show(left, |ui| {
                        ui.add(
                            egui::TextEdit::multiline(&mut self.headers_text)
                                .desired_width(f32::INFINITY)
                                .desired_rows(8),
                        );
                    });

                left.horizontal(|ui| {
                    if ui.button("填充浏览器头").clicked() {
                        self.fill_browser_headers();
                    }

                    if ui.button("清空请求头").clicked() {
                        self.headers_text.clear();
                    }
                });

                left.separator();
                left.label("POST Body:");

                egui::ScrollArea::vertical()
                    .id_salt("post_body_scroll_area")
                    .max_height(220.0)
                    .show(left, |ui| {
                        ui.add(
                            egui::TextEdit::multiline(&mut self.post_body)
                                .desired_width(f32::INFINITY)
                                .desired_rows(12),
                        );
                    });

                left.separator();

                left.horizontal(|ui| {
                    if ui.button("EXECUTE GET").clicked() {
                        self.count += 1;
                        self.error_msg.clear();
                        self.result.clear();

                        if self.url_preview.trim().is_empty() {
                            self.error_msg = "URL 不能为空".to_string();
                        } else {
                            match self.parse_headers() {
                                Ok(headers) => match send_get(&self.url_preview, &headers) {
                                    Ok(response) => {
                                        self.result = response;
                                    }
                                    Err(e) => {
                                        self.error_msg = format!("GET Error: {}", e);
                                    }
                                },
                                Err(e) => {
                                    self.error_msg = e;
                                }
                            }
                        }
                    }

                    if ui.button("EXECUTE POST").clicked() {
                        self.count += 1;
                        self.error_msg.clear();
                        self.result.clear();

                        if self.url_preview.trim().is_empty() {
                            self.error_msg = "URL 不能为空".to_string();
                        } else {
                            match self.parse_headers() {
                                Ok(headers) => {
                                    match send_post(&self.url_preview, &self.post_body, &headers) {
                                        Ok(response) => {
                                            self.result = response;
                                        }
                                        Err(e) => {
                                            self.error_msg = format!("POST Error: {}", e);
                                        }
                                    }
                                }
                                Err(e) => {
                                    self.error_msg = e;
                                }
                            }
                        }
                    }

                    if ui.button("CLEAR ALL").clicked() {
                        self.clear_all();
                    }
                });

                left.separator();
                left.label(format!("Request Count: {}", self.count));

                if !self.error_msg.is_empty() {
                    left.separator();
                    left.label(format!("Error: {}", self.error_msg));
                }

                right.heading("Response");
                right.separator();

                egui::Frame::group(right.style()).show(right, |ui| {
                    ui.set_min_height(700.0);

                    if self.result.is_empty() {
                        ui.label("暂无响应数据");
                    } else {
                        egui::ScrollArea::both()
                            .id_salt("response_result_scroll_area")
                            .auto_shrink([false, false])
                            .show(ui, |ui| {
                                ui.add(
                                    egui::TextEdit::multiline(&mut self.result)
                                        .desired_width(f32::INFINITY)
                                        .desired_rows(40),
                                );
                            });
                    }
                });
            });
        });
    }
}