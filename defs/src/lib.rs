
pub mod render;
pub mod control;
pub mod ubo;

use cgmath::Matrix4;
use image::ImageError;
use winit::error::OsError;

/// Camera trait
/// Abstraction for an entity that has a position and a projection view, such as a player-controlled
/// camera or an automatic flying camera.
pub trait Camera {
    fn update_aspect(&mut self, aspect_ratio: f32);
    fn update(&mut self, time_step_millis: u64, controller: &dyn control::Control);
    fn get_view_matrix(&self) -> Matrix4<f32>;
    fn get_projection_matrix(&self) -> Matrix4<f32>;
}

/// SceneManager trait
/// Consumer of scenes.
pub trait SceneManager {
    fn queue_scene(&self, new_scene: Box<dyn Scene>);
}

/// SceneInfo trait
/// Factory for scene descriptions. Can produce descriptions not only for what the scene does, but
/// also what resources the scene needs that can be specified ahead of time.
pub trait SceneInfo {

    /// Return everything needed to initialise the resources required for this scene type
    fn make_preloads(&self) -> render::ResourcePreloads;

    /// Make the description for how to render this scene
    fn make_description(&self) -> render::DrawingDescription;

    /// Get a pointer to the uniform data, and the data size in bytes, ready for upload into the
    /// renderer implementation
    /// # Safety - should ensure size covers the actual data
    unsafe fn get_ubo_data_ptr_and_size(
        &self,
        pass_index: usize,
        step_index: usize) -> (*const u8, usize);
}

pub trait SceneUpdates {

    /// Notify this implementation of a changed in the client area aspect ratio
    fn update_aspect_ratio(&mut self, aspect_ratio: f32);

    /// Instruct this implementation to perform time-dependent operations, given the supplied time
    /// slice and controller. This may include moving the camera or other physical processes.
    /// If this determines that a new scene should be transitioned to it, return it here.
    fn on_time_elapsed(
        &mut self,
        time_step_millis: u64,
        controller: &dyn control::Control
    ) -> Option<Box<dyn Scene>>;

    /// Set up the current scene for rendering. If on_time_elapsed caused a screen transition,
    /// this function will be called on the new scene.
    fn on_pre_render(&mut self);
}

pub trait Scene: SceneInfo + SceneUpdates {}

/// EngineError enum
/// Error types used throughout the engine.
#[derive(Debug)]
pub enum EngineError {
    GeneralError(String),
    DecodeError(String),
    RenderError(String)
}

impl From<image::ImageError> for EngineError {
    fn from(e: ImageError) -> Self {
        EngineError::DecodeError(format!("Decode error: {:?}", e))
    }
}

impl From<OsError> for EngineError {
    fn from(e: OsError) -> Self {
        EngineError::GeneralError(format!("OS error: {:?}", e))
    }
}
