pub mod device;

use anyhow::Result;
use cpal::traits::HostTrait;
use device::{AudioDevice, DeviceType};

pub struct AudioSystem {
    host: cpal::Host,
}

impl AudioSystem {
    pub fn new() -> Result<Self> {
        let host = cpal::default_host();
        Ok(Self { host })
    }

    pub fn get_available_devices(&self) -> (Vec<AudioDevice>, Vec<AudioDevice>) {
        let input_devices = self.host
            .input_devices()
            .map(|devices| {
                devices
                    .filter_map(|d| AudioDevice::new(d, DeviceType::Input).ok())
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();

        let output_devices = self.host
            .output_devices()
            .map(|devices| {
                devices
                    .filter_map(|d| AudioDevice::new(d, DeviceType::Output).ok())
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();

        (input_devices, output_devices)
    }

    pub fn get_default_devices(&self) -> (Option<AudioDevice>, Option<AudioDevice>) {
        let default_input = self.host
            .default_input_device()
            .and_then(|d| AudioDevice::new(d, DeviceType::Input).ok());

        let default_output = self.host
            .default_output_device()
            .and_then(|d| AudioDevice::new(d, DeviceType::Output).ok());

        (default_input, default_output)
    }

    pub fn get_supported_configs(&self, device: &AudioDevice) -> Vec<SupportedConfig> {
        device.get_supported_configs()
    }
}

#[derive(Debug, Clone)]
pub struct SupportedConfig {
    pub sample_rate: u32,
    pub buffer_size: Option<u32>,
}

