use std::collections::HashMap;

use slotmap::{Key, SlotMap};

use self::{
    pipeline_layout::PipelineLayoutPool, render_pipeline::GpuRenderPipelinePool, shader::ShaderPool,
};

pub(crate) mod pipeline_layout;
pub(crate) mod render_pipeline;
// pub(crate) mod shader;
pub mod shader;

pub struct StaticResourcePool<Handle: Key, Desc, Res> {
    resources: SlotMap<Handle, Res>,
    lookup: HashMap<Desc, Handle>,
}

impl<Handle, Desc, Res> StaticResourcePool<Handle, Desc, Res>
where
    Handle: Key,
    Desc: std::hash::Hash + Eq + Clone,
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

#[derive(Default)]
pub struct RenderResourcePools {
    pub pipeline_layouts: PipelineLayoutPool,
    pub render_pipelines: GpuRenderPipelinePool,
    pub shaders: ShaderPool,
}
