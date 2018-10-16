extern crate cpal;
extern crate simon;

fn main() {
    let freq_hz: f64 = simon::opt_required("f", "freq", "frequency in hertz", "HZ")
        .with_help_default()
        .parse_env_default_or_exit();

    let device = cpal::default_output_device().unwrap();
    let mut supported_formats_range = device.supported_output_formats().unwrap();
    let format = supported_formats_range
        .next()
        .unwrap()
        .with_max_sample_rate();

    let cpal::SampleRate(samples_per_second) = format.sample_rate;

    let event_loop = cpal::EventLoop::new();
    let stream_id = event_loop.build_output_stream(&device, &format).unwrap();
    event_loop.play_stream(stream_id);

    let index_multiplier = freq_hz * ::std::f64::consts::PI * 2. / (samples_per_second as f64);
    let sample_index_to_sample = |sample_index: usize| {
        ((sample_index as f64 * index_multiplier).sin() * (::std::i16::MAX as f64)) as i16
    };
    let mut sample_index = 0;

    event_loop.run(move |_stream_id, stream_data| match stream_data {
        cpal::StreamData::Output {
            buffer: cpal::UnknownTypeOutputBuffer::I16(mut buffer),
        } => for elem in buffer.iter_mut() {
            *elem = sample_index_to_sample(sample_index);
            sample_index += 1;
        },
        _ => (),
    });
}
