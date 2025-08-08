use std::{collections::VecDeque, time::Duration, io::BufReader, fs::File};
use rodio::{source::Empty, Decoder, OutputStream, OutputStreamHandle, Sink, Source};
use log::error;

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

    backqueue: VecDeque<Box<str>>,

    queue: VecDeque<Box<str>>,
}

impl Default for SinkHandler {
    fn default() -> Self {
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let launchsink = Sink::try_new(&stream_handle).unwrap();
        launchsink.pause();



        let launchsource: Empty<f32> = Empty::new();
        let current_song_length = launchsource.total_duration().unwrap();

        launchsink.append(launchsource);


        Self {
            current_song_length,
            _stream,
            stream_handle,
            prim_sink: launchsink,
            backqueue: VecDeque::new(),
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
        self.current_song_length
    }

    pub fn handler_try_seek(&mut self, target: Duration) {
        self.prim_sink.try_seek(target).unwrap_or(error!("Curiosity killed the cat."));
    }

    pub fn get_song_progress(&mut self) -> Duration {
        self.prim_sink.get_pos()
    }

    pub fn get_queue(&mut self) -> &VecDeque<Box<str>> {
        &self.queue
    }

    pub fn append_to_queue(&mut self, file_path: Box<str>) {
        self.queue.push_back(file_path);
    }

    pub fn back_skip(&mut self) {
        if self.backqueue.back().is_some() {
            let temp_song_file = BufReader::new(File::open(self.backqueue.back().unwrap() as &str).unwrap());
            let temp_source = Decoder::new(temp_song_file).unwrap();
            self.current_song_length = temp_source.total_duration().unwrap();
            self.prim_sink.append(temp_source);
            self.prim_sink.skip_one();
            self.queue.push_front(self.backqueue.pop_back().unwrap());
        }
        else {
            println!("you reached the end")
        }
    }

    pub fn skip(&mut self) {
        if self.queue.front().is_some() {
            let path = self.queue.front().unwrap() as &str;
            let temp_song_file = match File::open(path) {
                Ok(ok) => ok,
                Err(err) => {error!("Opening file `{path}` resulted in the following error:\n{err}"); return;}
            };
            let temp_source = Decoder::new(temp_song_file).unwrap();
            self.current_song_length = temp_source.total_duration().unwrap();
            self.prim_sink.append(temp_source);
            self.prim_sink.skip_one();
            self.queue.pop_front();
        }
        else {
            println!("you reached the end")
        }
    }

    pub fn clear_queue(&mut self) {
        self.queue.clear();
    }

    pub fn song_end_handler(&mut self) {
        if self.prim_sink.empty() {
            if self.queue.front().is_some() {
                let path = self.queue.front().unwrap() as &str;
                let temp_song_file = match File::open(path) {
                    Ok(ok) => BufReader::new(ok),
                    Err(err) => {error!("Opening file `{path}` resulted in the following error:\n{err}"); return;}
                };
                let temp_source = Decoder::new(temp_song_file).unwrap();
                self.current_song_length = temp_source.total_duration().unwrap();
                self.prim_sink.append(temp_source);
                self.queue.pop_front();
            }
            else {
                println!("you reached the end")
            }
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