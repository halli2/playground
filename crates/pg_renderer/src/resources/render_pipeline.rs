use super::{
    pipeline_layout::{PipelineLayoutHandle, PipelineLayoutPool},
    shader::{ShaderHandle, ShaderPool},
    StaticResourcePool,
};

slotmap::new_key_type! {pub struct GpuRenderPipelineHandle;}

#[derive(Default)]
pub struct GpuRenderPipelinePool {
    pub pool: StaticResourcePool<GpuRenderPipelineHandle, RenderPipelineDesc, wgpu::RenderPipeline>,
}

impl GpuRenderPipelinePool {
    pub fn get_or_create(
        &mut self,
        device: &wgpu::Device,
        desc: &RenderPipelineDesc,
        shader_pool: &mut ShaderPool,
        pipeline_layout_pool: &mut PipelineLayoutPool,
    ) -> eyre::Result<GpuRenderPipelineHandle> {
        // TODO: Get lol
        let handle = self.pool.get_or_create(desc, |desc| {
            desc.create_render_pipeline(device, shader_pool, pipeline_layout_pool)
                .unwrap()
        });
        Ok(handle)
    }
    pub fn get_resource(&self, handle: GpuRenderPipelineHandle) -> &wgpu::RenderPipeline {
        self.pool.resources.get(handle).unwrap()
    }
}

#[derive(Hash, Clone, PartialEq, Eq)]
pub struct RenderPipelineDesc {
    /// Debug label
    pub label: String,
    pub vertex_shader: ShaderHandle,
    pub fragment_shader: ShaderHandle,
    pub pipeline_layout: PipelineLayoutHandle,
    pub target_format: wgpu::TextureFormat,
}

impl RenderPipelineDesc {
    fn create_render_pipeline(
        &self,
        device: &wgpu::Device,
        shader_pool: &mut ShaderPool,
        pipeline_layout_pool: &mut PipelineLayoutPool,
    ) -> eyre::Result<wgpu::RenderPipeline> {
        let pipeline_layout = pipeline_layout_pool.get_resource(self.pipeline_layout);
        let vertex_shader = shader_pool.get_resource(self.vertex_shader);
        let fragment_shader = shader_pool.get_resource(self.fragment_shader);

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some(&self.label),
            layout: Some(pipeline_layout),
            vertex: wgpu::VertexState {
                module: vertex_shader,
                entry_point: "vs_main",
                buffers: &[],
            },
            fragment: Some(wgpu::FragmentState {
                module: fragment_shader,
                entry_point: "fs_main",
                targets: &[Some(self.target_format.into())],
            }),
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
        });

        Ok(render_pipeline)
    }
}
