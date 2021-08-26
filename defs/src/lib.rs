
use model::factory::StaticVertex;

use raw_window_handle::HasRawWindowHandle;
use cgmath::Matrix4;
use std::collections::HashMap;

pub enum PresentResult {
    Ok,
    SwapchainOutOfDate
}

pub enum Shader {
    PlainPnt,        // Position-Normal-Texture, R8G8B8A8 texture, no lighting
    PlainPntClipped, // Position-Normal-Texture, R8G8B8A8 texture, no lighting, clip Y
    Text,            // Position-Normal-Texture, R8 texture, no lighting
    Cube,            // Position, cube texture, no lighting
    CubeClipped,     // Position, cube texture, no lighting, clip Y
    Water,           // Position-Normal-Texture, R8G8B8A8 texture, no lighting, projective texture coords
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum ImageUsage {
    TextureSampleOnly,
    DepthBuffer,
    OffscreenRenderSampleColorWriteDepth,
    Skybox
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum TexturePixelFormat {
    None,
    RGBA,
    Unorm16
}

pub enum VertexFormat {
    PositionNormalTexture
}

pub enum KeyCode {
    Unknown,
    Left,
    Up,
    Down,
    Right
}

pub enum InputState {
    Pressed,
    Released
}

pub trait Control {
    fn update(&mut self);
    fn process_keyboard_event(&mut self, keycode: KeyCode, state: InputState);
    fn get_dx(&self) -> f32;
    fn get_dy(&self) -> f32;
}

pub trait Camera {
    fn update_aspect(&mut self, aspect_ratio: f32);
    fn update(&mut self, time_step_millis: u64, controller: &dyn Control);
    fn get_view_matrix(&self) -> Matrix4<f32>;
    fn get_projection_matrix(&self) -> Matrix4<f32>;
}

pub trait RendererApi {
    fn new(window_owner: &dyn HasRawWindowHandle, resource_preloads: &ResourcePreloads, description: &DrawingDescription) -> Result<Self, String> where Self : Sized;
    fn draw_next_frame(&mut self, scene_info: &dyn SceneInfo) -> Result<PresentResult, String>;
    fn recreate_surface(&mut self, window_owner: &dyn HasRawWindowHandle, description: &DrawingDescription) -> Result<(), String>;
    fn recreate_scene_resources(&mut self, resource_preloads: &ResourcePreloads, description: &DrawingDescription) -> Result<(), String>;
    fn get_aspect_ratio(&self) -> f32;
}

pub enum FramebufferTarget {
    DefaultFramebuffer,
    Texture(FramebufferCreationData)
}

pub struct VboCreationData {
    pub vertex_format: VertexFormat,
    pub vertex_data: Vec<StaticVertex>,
    pub vertex_count: usize,
    pub draw_indexed: bool,
    pub index_data: Option<Vec<u16>>
}

pub struct TextureCreationData {
    pub layer_data: Option<Vec<Vec<u8>>>,
    pub width: u32,
    pub height: u32,
    pub format: TexturePixelFormat,
    pub usage: ImageUsage
}

pub struct FramebufferCreationData {
    pub color_texture_index: usize,
    pub depth_texture_index: Option<usize>,
    pub width: usize,
    pub height: usize,
    pub color_format: TexturePixelFormat,
    pub depth_format: TexturePixelFormat
}

pub struct DrawingStep {
    pub shader: Shader,
    pub vbo_index: usize,
    pub vbo_format: VertexFormat,
    pub draw_indexed: bool,
    pub texture_indices: Vec<usize>,
    pub depth_test: bool
}

pub struct DrawingPass {
    pub target: FramebufferTarget,
    pub steps: Vec<DrawingStep>
}

#[derive(Default)]
pub struct DrawingDescription {
    pub passes: Vec<DrawingPass>
}

pub struct ResourcePreloads {
    pub vbo_preloads: HashMap<usize, VboCreationData>,
    pub texture_preloads: HashMap<usize, TextureCreationData>
}

pub trait SceneManager {
    fn queue_scene(&self, new_scene: Box<dyn SceneInfo>);
}

pub trait SceneInfo {
    fn make_preloads(&self) -> ResourcePreloads;
    fn make_description(&self) -> DrawingDescription;
    fn update_aspect_ratio(&mut self, aspect_ratio: f32);
    fn update_camera(&mut self, time_step_millis: u64, controller: &dyn Control) -> Option<Box<dyn SceneInfo>>;
    unsafe fn get_ubo_data_ptr_and_size(&self, pass_index: usize, step_index: usize) -> (*const u8, usize);
}
