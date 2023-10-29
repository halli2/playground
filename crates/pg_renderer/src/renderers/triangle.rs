use std::path::PathBuf;

use eyre::Result;

use super::Renderer;
use crate::{
    resources::{
        pipeline_layout::PipelineLayoutDesc,
        render_pipeline::{GpuRenderPipelineHandle, RenderPipelineDesc},
        shader::ShaderDesc,
        RenderResourcePools,
    },
    RenderContext,
};

pub struct TriangleRenderer {
    render_pipeline: GpuRenderPipelineHandle,
}

impl Renderer for TriangleRenderer {
    fn new(ctx: &mut RenderContext, swapchain_format: wgpu::TextureFormat) -> Self {
        let pipeline_layout_handle = ctx.resources.pipeline_layouts.get_or_create(
            &ctx.device,
            &PipelineLayoutDesc {
                label: "triangle_layout".to_owned(),
            },
        );
        let shared_shader_desc = ShaderDesc {
            label: "triangle.wgsl".to_owned(),
            source: PathBuf::from("shaders/triangle.wgsl"),
        };
        let vertex_shader = ctx
            .resources
            .shaders
            .get_or_create(&ctx.device, &shared_shader_desc);
        let fragment_shader = ctx
            .resources
            .shaders
            .get_or_create(&ctx.device, &shared_shader_desc);
        let render_pipeline = ctx
            .resources
            .render_pipelines
            .get_or_create(
                &ctx.device,
                &RenderPipelineDesc {
                    label: "Wowza".to_owned(),
                    vertex_shader,
                    fragment_shader,
                    pipeline_layout: pipeline_layout_handle,
                    target_format: swapchain_format,
                },
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
