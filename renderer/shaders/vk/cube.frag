#version 400
#extension GL_ARB_separate_shader_objects : enable
#extension GL_ARB_shading_language_420pack : enable

layout (location = 0) in vec3 o_tex_coord;

layout (set = 0, binding = 1) uniform samplerCube textureSampler;

layout (location = 0) out vec4 uFragColor;

void main() {
    uFragColor = texture(textureSampler, o_tex_coord);
}