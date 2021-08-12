
mod pipeline;
mod renderpass;
mod pipeline_set;
mod per_pass_resources;

use crate::vk_renderer::{
    render_core::RenderCore,
    per_image_resources::{
        pipeline_set::PipelineSet,
        per_pass_resources::PerPassResources
    }
};
use defs::{
    SceneInfo,
    DrawingDescription
};

use ash::vk;

pub struct PerImageResources {
    resources: Vec<PerPassResources>,
    command_buffer: vk::CommandBuffer
}

impl PerImageResources {

    pub fn new(render_core: &RenderCore, swapchain_image_index: usize, description: &DrawingDescription, command_buffer: vk::CommandBuffer) -> Result<PerImageResources, String> {
        let resources = description.passes.iter()
            .map(|pass| PerPassResources::new(render_core, swapchain_image_index, pass).unwrap())
            .collect();
        Ok(PerImageResources {
            resources,
            command_buffer
        })
    }

    pub unsafe fn record_command_buffer(&self, render_core: &RenderCore, command_buffer: vk::CommandBuffer) -> Result<(), String> {
        for resources in self.resources.iter() {
            resources.renderpass_pipeline_set.record_command_buffer(render_core, &resources.renderpass, command_buffer)?;
        }
        Ok(())
    }

    pub fn destroy_resources(&mut self, render_core: &RenderCore) {
        for resources in self.resources.iter_mut() {
            resources.renderpass_pipeline_set.destroy_resources(render_core);
            resources.renderpass.destroy_resources(render_core);
        }
    }

    pub unsafe fn on_pre_render(&mut self, render_core: &mut RenderCore, scene_info: &dyn SceneInfo) {
        for resources in self.resources.iter_mut() {
            resources.renderpass_pipeline_set.update_uniform_buffer(render_core, scene_info).unwrap();
        }
    }

    pub fn get_command_buffer(&self) -> vk::CommandBuffer {
        self.command_buffer
    }
}
