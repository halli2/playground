use std::path::PathBuf;

use super::StaticResourcePool;
use crate::FILE_SYSTEM;

slotmap::new_key_type! {pub struct ShaderHandle;}

#[derive(Default)]
pub struct ShaderPool {
    pub pool: StaticResourcePool<ShaderHandle, ShaderDesc, wgpu::ShaderModule>,
}

impl ShaderPool {
    pub fn get_or_create(&mut self, device: &wgpu::Device, desc: &ShaderDesc) -> ShaderHandle {
        self.pool
            .get_or_create(desc, |desc| desc.create_shader_module(device).unwrap())
    }
    pub fn get_resource(&self, handle: ShaderHandle) -> &wgpu::ShaderModule {
        self.pool.resources.get(handle).unwrap()
    }
}

#[derive(Default, Hash, Clone, PartialEq, Eq)]
pub struct ShaderDesc {
    pub label: String,
    pub source: PathBuf,
}
impl ShaderDesc {
    pub fn new(label: String, source: PathBuf) -> Self {
        Self { label, source }
    }

    fn create_shader_module(&self, device: &wgpu::Device) -> eyre::Result<wgpu::ShaderModule> {
        let source = FILE_SYSTEM.read_file(&self.source)?;
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some(&self.label),
            source: wgpu::ShaderSource::Wgsl(source),
        });
        Ok(shader)
    }
}
