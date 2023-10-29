use super::StaticResourcePool;

slotmap::new_key_type! {pub struct PipelineLayoutHandle;}
#[derive(Hash, Clone, PartialEq, Eq)]
pub struct PipelineLayoutDesc {
    pub label: String,
}

impl PipelineLayoutDesc {
    pub fn create_pipeline_layout(&self, device: &wgpu::Device) -> wgpu::PipelineLayout {
        device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some(&self.label),
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        })
    }
}

#[derive(Default)]
pub struct PipelineLayoutPool {
    pool: StaticResourcePool<PipelineLayoutHandle, PipelineLayoutDesc, wgpu::PipelineLayout>,
}

impl PipelineLayoutPool {
    pub fn get_or_create(
        &mut self,
        device: &wgpu::Device,
        desc: &PipelineLayoutDesc,
    ) -> PipelineLayoutHandle {
        self.pool
            .get_or_create(desc, |desc| desc.create_pipeline_layout(device))
    }
    pub fn get_resource(&self, handle: PipelineLayoutHandle) -> &wgpu::PipelineLayout {
        self.pool.resources.get(handle).unwrap()
    }
}
