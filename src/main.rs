use cpal::traits::DeviceTrait;
use cpal::traits::HostTrait;

fn main() {
    let host = cpal::default_host();
    println!("Host ID: {}", host.id().name());
    let device = host.default_output_device().unwrap();
    println!("Device: {}", device.name().unwrap());
    let mut supported_configs_range = device.supported_output_configs().unwrap();
    let supported_config = supported_configs_range
        .next()
        .unwrap()
        .with_max_sample_rate();
    println!("{:#?}", supported_config);
}
