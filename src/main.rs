
use std::sync::{Arc, Mutex};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{Device, Stream, StreamConfig, SupportedStreamConfig};
use anyhow::Result;


/// Инициализация аудиоустройства и конфигурации
fn init_audio_device() -> Result<(Device, SupportedStreamConfig)> {
    let host = cpal::default_host();
    let device = host
        .default_output_device()
        .ok_or_else(|| anyhow::anyhow!("Нет выходного аудиоустройства"))?;

    let config = device.default_output_config()?;
    Ok((device, config))
}

fn main() {

}
