use std::{collections::{HashMap, VecDeque}, io::BufReader, time::Duration, fs};
use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink, Source};
use serde_json;
use crate::tools;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state

pub struct TemplateApp {
    volume_slider_value: f32,
    song_progress_slider_value: f32,
    play_pause_button_text: String,

    queue: VecDeque<String>,

    #[serde(skip)]
    current_song_length: Duration,
    #[serde(skip)]
    _stream: OutputStream,
    #[serde(skip)]
    stream_handle: OutputStreamHandle,
    #[serde(skip)]
    prim_sink: rodio::Sink,
    #[serde(skip)]
    library_hashmap: HashMap<String, HashMap<String, HashMap<String, String>>>,
    #[serde(skip)]
    is_first_frame: bool,

    artistreq: String,
    albumreq: String,
    songreq: String
}

impl Default for TemplateApp {
    fn default() -> Self {
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let launchsink = Sink::try_new(&stream_handle).unwrap();
        launchsink.pause();

        let library_hashmap = serde_json::from_str(&std::fs::read_to_string("assets/library.json").unwrap()).unwrap();

        
        let file = BufReader::new(fs::File::open("D:/Coding/Music/underscores/fishmonger/underscores - fishmonger - 09 The fish song.wav").unwrap());

        let launchsource = Decoder::new(file).unwrap();
        let current_song_length = launchsource.total_duration().unwrap();

        launchsink.append(launchsource);

        Self {
            volume_slider_value: 1.0,
            song_progress_slider_value: 0.0,
            current_song_length: current_song_length,
            play_pause_button_text: "Play".to_string(),

            queue: VecDeque::new(),

            _stream: _stream,
            stream_handle: stream_handle,
            prim_sink: launchsink,
            library_hashmap:library_hashmap,
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
                tools::set_volume(&self.prim_sink, self.volume_slider_value);
            } 

            let play_pause_button = ui.add(egui::Button::new(&self.play_pause_button_text));
            //play/pause button
            if play_pause_button.clicked() {
                if self.prim_sink.is_paused() {
                    self.prim_sink.play();
                }
                else {
                    self.prim_sink.pause();
                }
            }

            let song_progress_slider = ui.add(egui::Slider::new(&mut self.song_progress_slider_value, 0.0..=(self.current_song_length).as_millis() as f32) 
            .trailing_fill(true)
            .text("Progress")
            .show_value(false)
        );

            if song_progress_slider.changed() {
                self.prim_sink.try_seek(Duration::from_millis(self.song_progress_slider_value as u64)).unwrap()
            } 

            self.song_progress_slider_value = self.prim_sink.get_pos().as_millis() as f32;

            if ui.button("printqueue").clicked() {
                println!("{:#?}", (self.queue))
            }

            ui.add(egui::TextEdit::singleline(&mut self.artistreq));
            ui.add(egui::TextEdit::singleline(&mut self.albumreq));
            ui.add(egui::TextEdit::singleline(&mut self.songreq));
            

            let append_button = ui.add(egui::Button::new("append to queue"));
            if append_button.clicked() {
                tools::append_to_queue(&mut self.queue, &self.artistreq, &self.albumreq, &self.songreq, &self.library_hashmap);
            }

            let skip_button = ui.add(egui::Button::new("skip"));
            if skip_button.clicked() {
                tools::skip(&self.prim_sink, &mut self.current_song_length, &mut self.queue);
            }

            ui.separator();

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                egui::warn_if_debug_build(ui)});
        });
        if self.is_first_frame == true {
            tools::first_frame_setup(&self.prim_sink, self.volume_slider_value);
            println!("wow");
            self.is_first_frame = false
        }
    }
}