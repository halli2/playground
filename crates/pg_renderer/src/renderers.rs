mod triangle;
use eyre::Result;
pub use triangle::TriangleRenderer;

use crate::{RenderContext, RenderResourcePools};

pub trait Renderer {
    fn new(render_context: &mut RenderContext, format: wgpu::TextureFormat) -> Self;

    fn draw<'a>(
        &self,
        render_pass: &mut wgpu::RenderPass<'a>,
        pools: &'a RenderResourcePools,
    ) -> Result<()>;
}
