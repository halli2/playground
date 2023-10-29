use eyre::Result;

use crate::{
    GpuRenderPipelineHandle, PipelineLayoutDesc, RenderContext, RenderPipelineDesc,
    RenderResourcePools, ShaderDesc,
};

use super::Renderer;

pub struct TriangleRenderer {
    render_pipeline: GpuRenderPipelineHandle,
}

impl Renderer for TriangleRenderer {
    fn new(ctx: &mut RenderContext, swapchain_format: wgpu::TextureFormat) -> Self {
        let render_pipeline = ctx
            .resources
            .render_pipelines
            .get_or_create(
                &ctx.device,
                &RenderPipelineDesc {
                    label: "Wowza".to_owned(),
                    shader_desc: ShaderDesc {
                        label: "triangle_shader".to_owned(),
                    },
                    pipeline_layout_desc: PipelineLayoutDesc {
                        label: "PIPE IT UP".to_owned(),
                    },
                },
                swapchain_format,
                &mut ctx.resources.shaders,
                &mut ctx.resources.pipeline_layouts,
            )
            .unwrap();
        Self { render_pipeline }
    }

    fn draw<'a>(
        &self,
        pass: &mut wgpu::RenderPass<'a>,
        pools: &'a RenderResourcePools,
    ) -> Result<()> {
        let pipeline = pools.render_pipelines.get_resource(self.render_pipeline);
        pass.set_pipeline(pipeline);
        pass.draw(0..3, 0..1);
        Ok(())
    }
}
