use eframe::{App, Frame};
use egui::{menu, CentralPanel, Context, TopBottomPanel, Window};
use rfd::MessageDialog;
use std::path::PathBuf;
use std::{fs, process};

fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions {
        initial_window_size: Some([1280.0, 720.0].into()),
        min_window_size: Some([640.0, 480.0].into()),
        ..Default::default()
    };
    eframe::run_native(
        "SimpleEdit",
        native_options,
        Box::new(|cc| Box::new(SimpleEdit::new(cc))),
    )
}

struct SimpleEdit {
    buffer: String,
    current: Option<PathBuf>,
    replace_window_shown: bool,

    replace_old: String,
    replace_new: String,

    search_window_shown: bool,
    statistics_window_shown: bool
}

impl SimpleEdit {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            buffer: String::from(""),
            current: None,
            replace_window_shown: false,

            replace_old: String::from(""),
            replace_new: String::from(""),

            search_window_shown: false,
            statistics_window_shown: false
        }
    }

    fn render_text_edit(&mut self, ctx: &Context) {
        CentralPanel::default().show(ctx, |ui| {
            ui.add_sized(
                ui.available_size(),
                egui::TextEdit::multiline(&mut self.buffer),
            );
        });
    }

    fn render_menu(&mut self, ctx: &Context) {
        TopBottomPanel::top("Menu Bar").show(ctx, |ui| {
            menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("New").clicked() {
                        self.handle_new();
                    }

                    if ui.button("Open...").clicked() {
                        self.handle_open();
                    }

                    if ui.button("Save").clicked() {
                        self.handle_save();
                    }

                    if ui.button("Save as...").clicked() {
                        self.handle_save_as(None);
                    }

                    if ui.button("Exit").clicked() {
                        process::exit(0);
                    }
                });

                ui.menu_button("Tools", |ui| {
                    if ui.button("Replace...").clicked() {
                        self.replace_window_shown = true;
                    }

                    if ui.button("Search...").clicked() {
                        self.search_window_shown = true;
                    }

                    if ui.button("Statistics...").clicked() {
                        self.statistics_window_shown = true;
                    }
                });

                ui.menu_button("About", |ui| {
                    if ui.button("About simple_edit").clicked() {
                        let mut dialog = MessageDialog::new()
                            .set_title("Simple edit")
                            .set_description("Simple edit is a simple editor using egui by JZISONTHEWAY");

                        dialog.show();
                    }
                });
            });
        });
    }

    fn handle_new(&mut self) {
        self.buffer.clear();
        self.current = None;
    }

    fn handle_open(&mut self) {
        let path = std::env::current_dir().unwrap();

        let res = rfd::FileDialog::new()
            .add_filter("Text files", &["txt"])
            .set_directory(&path)
            .pick_file();

        if let Some(res) = res {
            self.buffer = fs::read_to_string(&res).unwrap();

            self.current = Some(res)
        }
    }

    fn handle_save(&mut self) {
        self.handle_save_as(self.current.clone());
    }

    fn handle_save_as(&mut self, mut res: Option<PathBuf>) {
        let path = std::env::current_dir().unwrap();

        if res.is_none() {
            res = rfd::FileDialog::new()
                .add_filter("Text files", &["txt"])
                .set_directory(&path)
                .save_file();
        }

        if let Some(res) = res {
            fs::write(res.clone(), &mut self.buffer).unwrap();

            self.current = Some(res)
        }
    }

    fn show_replace_window(&mut self, ctx: &Context) {
        if !self.replace_window_shown {
            return;
        }

        Window::new("Replace").show(ctx, |ui| {
            ui.label("This tool replaces string a with string b");

            ui.horizontal(|ui| {
                ui.label("Old: ");
                ui.text_edit_singleline(&mut self.replace_old);
            });

            ui.horizontal(|ui| {
                ui.label("New: ");
                ui.text_edit_singleline(&mut self.replace_new);
            });

            ui.separator();

            ui.horizontal(|ui| {
                if ui.button("Replace").clicked() {
                    self.buffer = self.buffer.replace(&self.replace_old, &self.replace_new);

                    self.replace_window_shown = false;
                    self.replace_old.clear();
                    self.replace_new.clear();
                }

                if ui.button("Skip replacing").clicked() {
                    self.replace_window_shown = false;
                    self.replace_old.clear();
                    self.replace_new.clear();
                }
            });
        });
    }

    fn show_search_window(&mut self, ctx: &Context) {
        if !self.search_window_shown {
            return;
        }

        Window::new("Search").show(ctx, |ui| {
            ui.label("This tool finds strings in this editor");

            ui.horizontal(|ui| {
                ui.label("Search: ");
                ui.text_edit_singleline(&mut self.replace_old);
            });

            ui.separator();

            ui.label(format!("Result: {}", self.replace_new));

            ui.horizontal(|ui| {
                if ui.button("Search").clicked() {
                    self.replace_new = self
                        .buffer
                        .match_indices(&self.replace_old)
                        .map(|(a, _)| a.to_string())
                        .collect::<Vec<_>>()
                        .join(", ");
                }

                if ui.button("Abort").clicked() {
                    self.replace_old.clear();
                    self.replace_new.clear();

                    self.search_window_shown = false;
                }
            });
        });
    }

    fn show_statistics_window(&mut self, ctx: &Context) {
        if !self.statistics_window_shown {
            return
        }

        Window::new("Statistics").show(ctx, |ui| {
            ui.label("This tool shows the statistics");
            ui.label(format!("Currently opened: {:?}", self.current));
            ui.label(format!("Number of chars: {}", self.buffer.len()));

            ui.separator();

            ui.horizontal(|ui| {
                if ui.button("Close").clicked() {
                    self.statistics_window_shown = false;
                }
            })
        });
    }
}

impl App for SimpleEdit {
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        self.render_menu(ctx);
        self.render_text_edit(ctx);

        self.show_replace_window(ctx);
        self.show_search_window(ctx);
        self.show_statistics_window(ctx);
    }
}
