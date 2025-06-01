use std::time::Duration;
use crate::tools;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state

pub struct TemplateApp {
    prim_sink_handler: tools::SinkHandler,
    volume_slider_value: f32,
    song_progress_slider_value: f32,
    play_pause_button_text: String,
    
    
    #[serde(skip)]
    is_first_frame: bool,
    
    artistreq: String,
    albumreq: String,
    songreq: String
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            volume_slider_value: 1.0,
            song_progress_slider_value: 0.0,
            prim_sink_handler: tools::SinkHandler::default(),
            play_pause_button_text: "Play".to_string(),


            is_first_frame: true,
            artistreq: "femtanyl".to_string(),
            albumreq: "REACTOR".to_string(),
            songreq: "M3 N MIN3".to_string()
        }
    }
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        match cc.storage {
            Some(storage) => eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default(),
            None => Default::default()
        }
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
                //file menu (idk why quit is in this but i dont have anywhere else to put it yet)
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });
                //Theme switch button
                egui::widgets::global_theme_preference_switch(ui);
            });
        });

        // The central panel the region left after adding TopPanels and SidePanels
        egui::CentralPanel::default().show(ctx, |ui| {
            //volume slider

            let volslider = ui.add(egui::Slider::new(&mut self.volume_slider_value, -1.25..=1.0)
            .text("Volume")
            .show_value(false));

            // conversion from logarithmic to to multiplicative units
            if volslider.changed() {
                self.prim_sink_handler.set_volume(self.volume_slider_value);
            } 

            let play_pause_button = ui.add(egui::Button::new(&self.play_pause_button_text));
            //play/pause button
            if play_pause_button.clicked() {
                self.prim_sink_handler.on_play_pause_button_clicked();
            }

            let song_progress_slider = ui.add(egui::Slider::new(&mut self.song_progress_slider_value, 0.0..=(self.prim_sink_handler.get_current_song_length()).as_millis() as f32) 
            .trailing_fill(true)
            .text("Progress")
            .show_value(false)
        );

            if song_progress_slider.drag_stopped() {
                self.prim_sink_handler.handler_try_seek(Duration::from_millis(self.song_progress_slider_value as u64))
            } 

            self.song_progress_slider_value = self.prim_sink_handler.get_song_progress().as_millis() as f32;

            if ui.button("printqueue").clicked() {
                println!("{:#?}", (self.prim_sink_handler.get_queue()))
            }

            ui.add(egui::TextEdit::singleline(&mut self.artistreq));
            ui.add(egui::TextEdit::singleline(&mut self.albumreq));
            ui.add(egui::TextEdit::singleline(&mut self.songreq));
            

            let append_button = ui.add(egui::Button::new("append to queue"));
            if append_button.clicked() {
                self.prim_sink_handler.append_to_queue(&self.artistreq, &self.albumreq, &self.songreq);
            }

            let skip_button = ui.add(egui::Button::new("skip"));
            if skip_button.clicked() {
                self.prim_sink_handler.skip();
            }

            ui.separator();
            
            if ui.button("debug").clicked(){
                self.prim_sink_handler.debug_dump();
            }
            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                egui::warn_if_debug_build(ui)});
            
            
        });
        
        if self.is_first_frame == true {
            tools::first_frame_setup(&mut self.prim_sink_handler, self.volume_slider_value);
            println!("wow");
            self.is_first_frame = false
        }

        self.prim_sink_handler.song_end_handler();

    }
}