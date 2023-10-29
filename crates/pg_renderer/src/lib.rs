#![feature(lazy_cell)]

pub mod renderers;
pub mod resources;

use std::{
    borrow::Cow,
    path::{Path, PathBuf},
    sync::{Arc, LazyLock},
};

use resources::RenderResourcePools;

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

static FILE_SYSTEM: LazyLock<FileSystem> = LazyLock::new(|| FileSystem::new().unwrap());

pub struct FileSystem {
    asset_dir: PathBuf,
}

impl FileSystem {
    /// Finds the asset directory:
    /// - If executable next to asset dir
    /// - If ran in target dir
    /// - If current directory next to asset dir
    pub fn new() -> eyre::Result<Self> {
        let mut exe_dir = std::env::current_exe()?;
        let asset_dir = exe_dir.join("assets");
        if asset_dir.exists() {
            return Ok(Self { asset_dir });
        }

        while let Some(dir) = exe_dir.parent() {
            let asset_dir = dir.join("assets");
            if asset_dir.exists() {
                return Ok(Self { asset_dir });
            }
            exe_dir = dir.to_owned();
        }

        let asset_dir = std::env::current_dir()?.join("assets");
        if asset_dir.exists() {
            return Ok(Self { asset_dir });
        }

        panic!("Could not find asset directory!");
    }

    pub fn read_file(&self, file_path: &Path) -> eyre::Result<Cow<'static, str>> {
        let file = std::fs::read_to_string(self.asset_dir.join(file_path))?;
        Ok(file.into())
    }
}
