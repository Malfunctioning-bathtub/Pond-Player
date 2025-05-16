use std::fs::File;
use std::io::BufReader;
use rodio::{source, Decoder, OutputStream, OutputStreamHandle, Sink};


/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state


pub struct TemplateApp {
    // Example stuff:
    volume: f32,

    #[serde(skip)]
    _stream: OutputStream,
    #[serde(skip)]
    stream_handle: OutputStreamHandle,
    #[serde(skip)]
    prim_sink: rodio::Sink,
}

impl Default for TemplateApp {
    fn default() -> Self {
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let launchsink = Sink::try_new(&stream_handle).unwrap();
        launchsink.pause();
        let file = BufReader::new(File::open("D:\\Coding\\Music\\Lena Raine\\Celeste  Farewell Original Soundtrack\\05 Crash.wav").unwrap());
        let launchsource = Decoder::new(file).unwrap();
        launchsink.append(launchsource);

        Self {
            volume: 1.0,
            _stream: _stream,
            stream_handle: stream_handle,
            prim_sink: launchsink,
        }
    }
}


impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for TemplateApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called every time the window is repainted
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        //Top menu bar containing quit option and theme toggle
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| { 
            egui::menu::bar(ui, |ui| {
                // NOTE: no File->Quit on web pages!
                let is_web = cfg!(target_arch = "wasm32");
                if !is_web {
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                    ui.add_space(16.0);
                }

                egui::widgets::global_theme_preference_buttons(ui);
            });
        });

        // The central panel the region left after adding TopPanels and SidePanels
        egui::CentralPanel::default().show(ctx, |ui| {
            //volume slider
            ui.add(egui::Slider::new(&mut self.volume, 0.0..=1.0)
                .text("Volume")
                .show_value(false));
            self.prim_sink.set_volume(self.volume);

            //play/pause button
            if ui.button("play/pause").clicked() {
                if self.prim_sink.is_paused() {
                    self.prim_sink.play();
                }
                else {
                    self.prim_sink.pause();
                }
            }
            ui.separator();

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                egui::warn_if_debug_build(ui);
            });
        });
    }
}