
use crate::vk_renderer::{
    RenderCore,
    renderpass::RenderpassWrapper,
    pipeline::PipelineWrapper,
    pipeline_config
};

use ash::{
    vk,
    version::DeviceV1_0
};
use cgmath::Matrix4;

pub struct PipelineSet {
    pipelines: Vec<PipelineWrapper>,
    command_buffers: Vec<vk::CommandBuffer>
}

impl PipelineSet {

    pub fn new(render_core: &RenderCore, renderpass_wrapper: &RenderpassWrapper) -> Result<PipelineSet, String> {

        let configs = vec![
            pipeline_config::PipelineConfig {
                shader: pipeline_config::Shader::TextureFlat,
                model: pipeline_config::Model::MenuScene,
                texture: pipeline_config::Texture::Jpeg(pipeline_config::TextureSource::Terrain)
            },
            pipeline_config::PipelineConfig {
                shader: pipeline_config::Shader::TextureFlat,
                model: pipeline_config::Model::Grimace,
                texture: pipeline_config::Texture::Png(pipeline_config::TextureSource::MusicaFont)
            }
        ];
        let pipelines = configs
            .into_iter()
            .map(|config| PipelineWrapper::new(config, render_core, renderpass_wrapper).unwrap())
            .collect();

        let mut pipeline_set = PipelineSet {
            pipelines,
            command_buffers: vec![]
        };
        unsafe { pipeline_set.create_resources(render_core, renderpass_wrapper)?; }

        Ok(pipeline_set)
    }

    pub fn destroy_resources(&mut self, render_core: &RenderCore) {
        for pipeline in self.pipelines.iter_mut() {
            pipeline.destroy_resources(render_core);
        }
    }

    pub unsafe fn create_resources(&mut self, render_core: &RenderCore, renderpass_wrapper: &RenderpassWrapper) -> Result<(), String> {

        for pipeline in self.pipelines.iter_mut() {
            pipeline.create_resources(render_core, renderpass_wrapper)?;
        }

        // Allocate and record command buffers
        let command_buffer_count = render_core.image_views.len() as u32;
        let command_buffer_allocate_info = vk::CommandBufferAllocateInfo::builder()
            .command_pool(render_core.graphics_command_buffer_pool)
            .command_buffer_count(command_buffer_count);
        let command_buffers = render_core.device
            .allocate_command_buffers(&command_buffer_allocate_info)
            .map_err(|e| format!("{:?}", e))?;
        for (index, &command_buffer) in command_buffers.iter().enumerate() {
            let begin_info = vk::CommandBufferBeginInfo::builder();
            let clear_values = [
                vk::ClearValue {
                    color: vk::ClearColorValue {
                        float32: [0.0, 0.0, 0.0, 1.0]
                    }
                },
                vk::ClearValue {
                    depth_stencil: vk::ClearDepthStencilValue {
                        depth: 1.0,
                        stencil: 0
                    }
                }
            ];
            let renderpass_begin_info = vk::RenderPassBeginInfo::builder()
                .render_pass(renderpass_wrapper.renderpass)
                .framebuffer(renderpass_wrapper.framebuffers[index])
                .render_area(vk::Rect2D {
                    offset: vk::Offset2D { x: 0, y: 0 },
                    extent: render_core.get_extent()?
                })
                .clear_values(&clear_values);

            render_core.device.begin_command_buffer(command_buffer, &begin_info)
                .map_err(|e| format!("{:?}", e))?;
            render_core.device.cmd_begin_render_pass(command_buffer, &renderpass_begin_info, vk::SubpassContents::INLINE);

            self.pipelines[0].record_commands(index, command_buffer, render_core).unwrap();
            self.pipelines[1].record_commands(index, command_buffer, render_core).unwrap();

            render_core.device.cmd_end_render_pass(command_buffer);
            render_core.device.end_command_buffer(command_buffer)
                .map_err(|e| format!("{:?}", e))?;
        }

        self.command_buffers.clear();
        for command_buffer in command_buffers.iter() {
            self.command_buffers.push(*command_buffer);
        }

        Ok(())
    }

    pub fn get_command_buffer(&self, image_index: usize) -> vk::CommandBuffer {
        self.command_buffers[image_index]
    }

    pub unsafe fn update_camera_matrix(&mut self, render_core: &mut RenderCore, camera_matrix: Matrix4<f32>) -> Result<(), String> {
        for pipeline in self.pipelines.iter_mut() {
            pipeline.update_camera_matrix(render_core, camera_matrix)?;
        }
        Ok(())
    }
}
