use std::{collections::{HashMap, VecDeque}, time::Duration, io::BufReader, fs::File};
use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink, Source};

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state

pub struct SinkHandler {
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
    
    queue: VecDeque<String>,
}

impl Default for SinkHandler {
    fn default() -> Self {
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let launchsink = Sink::try_new(&stream_handle).unwrap();
        launchsink.pause();

        let library_hashmap = serde_json::from_str(&std::fs::read_to_string("assets/library.json").unwrap()).unwrap();

        
        let file = BufReader::new(File::open("D:/Coding/Music/underscores/fishmonger/underscores - fishmonger - 09 The fish song.wav").unwrap());

        let launchsource = Decoder::new(file).unwrap();
        let current_song_length = launchsource.total_duration().unwrap();

        launchsink.append(launchsource);


        Self {
            current_song_length: current_song_length,
            _stream: _stream, 
            stream_handle: stream_handle, 
            prim_sink: launchsink, 
            library_hashmap: library_hashmap, 
            queue: VecDeque::new() 
        }
    }
}

impl SinkHandler {
    
    pub fn on_play_pause_button_clicked(&mut self) {
        if self.prim_sink.is_paused() {
            self.prim_sink.play();
        }
        else {
            self.prim_sink.pause();
        }
    }
    
    pub fn set_volume(&mut self, volume_slider_value: f32) {
        if volume_slider_value == -1.25{
            self.prim_sink.set_volume(0.0);
        }
        else {
            self.prim_sink.set_volume(7.0_f32.powf(volume_slider_value - 1.0));
        }
    }
    
    pub fn get_current_song_length(&mut self) -> Duration {
        return self.current_song_length;
    }
    
    pub fn handler_try_seek(&mut self, target: Duration) {
        self.prim_sink.try_seek(target).unwrap();
    }
    
    pub fn get_song_progress(&mut self) -> Duration {
        return self.prim_sink.get_pos();
    }
    
    pub fn get_queue(&mut self) -> &VecDeque<String> {
        return &self.queue;
    }
    
    pub fn append_to_queue(&mut self, file_path: String) {
        self.queue.push_back(file_path);
    }
    
    pub fn skip(&mut self) {
        let temp_song_file = BufReader::new(File::open(self.queue.front().unwrap()).unwrap());
        let temp_source = Decoder::new(temp_song_file).unwrap();
        self.current_song_length = temp_source.total_duration().unwrap();
        self.prim_sink.append(temp_source);
        self.prim_sink.skip_one();
        self.queue.pop_front();
    }

    pub fn song_end_handler(&mut self) {
        if self.prim_sink.empty() {
            let temp_song_file = BufReader::new(File::open(self.queue.front().unwrap()).unwrap());
            let temp_source = Decoder::new(temp_song_file).unwrap();
            self.current_song_length = temp_source.total_duration().unwrap();
            self.prim_sink.append(temp_source);
            self.queue.pop_front();
        }
    }
    
    pub fn debug_dump(&mut self) {
        println!("current song length: {}", self.current_song_length.as_micros());
        println!("queue: {:#?}", self.queue);
        println!("progress: {}", self.prim_sink.get_pos().as_micros());
        println!("paused: {}", self.prim_sink.is_paused());
        println!("{}", self.prim_sink.empty())
    }


}