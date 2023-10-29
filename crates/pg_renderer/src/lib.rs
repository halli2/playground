pub mod renderers;

use eyre::Result;
use std::{borrow::Cow, collections::HashMap, hash::Hash, sync::Arc};

use slotmap::{Key, SlotMap};
pub struct StaticResourcePool<Handle: Key, Desc, Res> {
    resources: SlotMap<Handle, Res>,
    lookup: HashMap<Desc, Handle>,
}

impl<Handle, Desc, Res> StaticResourcePool<Handle, Desc, Res>
where
    Handle: Key,
    Desc: Hash + Eq + Clone,
{
    pub fn get_or_create<F: FnOnce(&Desc) -> Res>(&mut self, desc: &Desc, create: F) -> Handle {
        let handle = self.lookup.entry(desc.clone()).or_insert_with(|| {
            let resource = create(desc);
            self.resources.insert(resource)
        });
        *handle
    }
}
impl<Handle: Key, Desc, Res> Default for StaticResourcePool<Handle, Desc, Res> {
    fn default() -> Self {
        Self {
            resources: Default::default(),
            lookup: Default::default(),
        }
    }
}

#[derive(Default, Hash, Clone, PartialEq, Eq)]
pub struct ShaderDesc {
    pub label: String,
}
impl ShaderDesc {
    fn create_shader_module(&self, device: &wgpu::Device) -> Result<wgpu::ShaderModule> {
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some(&self.label),
            // TODO: Filesystem shit
            source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!(
                "../shaders/triangle.wgsl"
            ))),
        });
        Ok(shader)
    }
}

#[derive(Hash, Clone, PartialEq, Eq)]
pub struct RenderPipelineDesc {
    /// Debug label
    pub label: String,
    pub shader_desc: ShaderDesc,
    pub pipeline_layout_desc: PipelineLayoutDesc,
}

impl RenderPipelineDesc {
    fn create_render_pipeline(
        &self,
        device: &wgpu::Device,
        swapchain_format: wgpu::TextureFormat,
        shader_pool: &mut ShaderPool,
        pipeline_layout_pool: &mut PipelineLayoutPool,
    ) -> Result<wgpu::RenderPipeline> {
        // TODO: pipeline pool, shader pool
        let pipeline_layout_handle =
            pipeline_layout_pool.get_or_create(device, &self.pipeline_layout_desc);
        let pipeline_layout = pipeline_layout_pool.get_resource(pipeline_layout_handle);
        let shader_handle = shader_pool.get_or_create(device, &self.shader_desc);
        let shader = shader_pool.get_resource(shader_handle);

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some(&self.label),
            layout: Some(pipeline_layout),
            vertex: wgpu::VertexState {
                module: shader,
                entry_point: "vs_main",
                buffers: &[],
            },
            fragment: Some(wgpu::FragmentState {
                module: shader,
                entry_point: "fs_main",
                targets: &[Some(swapchain_format.into())],
            }),
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
        });

        Ok(render_pipeline)
    }
}

slotmap::new_key_type! {pub struct GpuRenderPipelineHandle;}
slotmap::new_key_type! {pub struct ShaderHandle;}
slotmap::new_key_type! {pub struct PipelineLayoutHandle;}

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

#[derive(Default)]
pub struct GpuRenderPipelinePool {
    pub pool: StaticResourcePool<GpuRenderPipelineHandle, RenderPipelineDesc, wgpu::RenderPipeline>,
}

impl GpuRenderPipelinePool {
    pub fn get_or_create(
        &mut self,
        device: &wgpu::Device,
        desc: &RenderPipelineDesc,
        swapchain_format: wgpu::TextureFormat,
        shader_pool: &mut ShaderPool,
        pipeline_layout_pool: &mut PipelineLayoutPool,
    ) -> Result<GpuRenderPipelineHandle> {
        // TODO: Get lol
        let handle = self.pool.get_or_create(desc, |desc| {
            desc.create_render_pipeline(device, swapchain_format, shader_pool, pipeline_layout_pool)
                .unwrap()
        });
        Ok(handle)
    }
    pub fn get_resource(&self, handle: GpuRenderPipelineHandle) -> &wgpu::RenderPipeline {
        self.pool.resources.get(handle).unwrap()
    }
}

#[derive(Hash, Clone, PartialEq, Eq)]
pub struct PipelineLayoutDesc {
    label: String,
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
#[derive(Default)]
pub struct RenderResourcePools {
    pub pipeline_layouts: PipelineLayoutPool,
    pub render_pipelines: GpuRenderPipelinePool,
    pub shaders: ShaderPool,
}

pub struct RenderContext {
    pub device: Arc<wgpu::Device>,
    pub queue: Arc<wgpu::Queue>,

    pub resources: RenderResourcePools,
}

impl RenderContext {
    pub fn new(
        _adapter: &wgpu::Adapter,
        device: Arc<wgpu::Device>,
        queue: Arc<wgpu::Queue>,
    ) -> Self {
        Self {
            device,
            queue,
            resources: RenderResourcePools::default(),
        }
    }
}
