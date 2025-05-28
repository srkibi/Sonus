use anyhow::{Result, Context};
use cpal::{self, traits::DeviceTrait};
use std::fmt;

use super::{SupportedConfig};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeviceType {
    Input,
    Output,
}

pub struct AudioDevice {
    device: cpal::Device,
    name: String,
    device_type: DeviceType,
}

impl AudioDevice {
    pub fn new(device: cpal::Device, device_type: DeviceType) -> Result<Self> {
        let name = device.name().context("Failed to get device name")?;
        Ok(Self {
            device,
            name,
            device_type,
        })
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn device_type(&self) -> &DeviceType {
        &self.device_type
    }

    pub fn get_supported_configs(&self) -> Vec<SupportedConfig> {
        match self.device.supported_output_configs() {
            Ok(configs) => configs
                .filter_map(|config| {
                    let sample_rate = config.max_sample_rate().0;
                    let buffer_size = match config.buffer_size() {
                        cpal::SupportedBufferSize::Range { min: _, max } => Some(*max),
                        cpal::SupportedBufferSize::Unknown => None,
                    };
                    
                    Some(SupportedConfig {
                        sample_rate,
                        buffer_size,
                    })
                })
                .collect(),
            Err(_) => Vec::new().into_iter().collect(),
        }
    }
}

impl DeviceType {
    pub fn as_str(&self) -> &str {
        match self {
            DeviceType::Input => "Input",
            DeviceType::Output => "Output",
        }
    }
}

impl fmt::Debug for AudioDevice {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AudioDevice")
            .field("name", &self.name)
            .field("device_type", &self.device_type)
            .finish()
    }
}

impl fmt::Display for AudioDevice {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

