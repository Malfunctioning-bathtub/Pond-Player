use crate::sinkhandler;

pub fn first_frame_setup(sink_handler: &mut sinkhandler::SinkHandler, volume_slider_value: f32) {
    sink_handler.set_volume(volume_slider_value);
}