use std::path::PathBuf;
use std::str::FromStr;

use super::AuraShapes::Shapes;
extern crate gl;
extern crate glfw;
use super::utils::{self, VertexProcessor, Shader};
use gl::types::*;

pub struct Renderer {}


impl Renderer {
    pub fn initialize(window: &mut glfw::PWindow) {
        gl::load_with(|s| window.get_proc_address(s) as *const _);
    }

    pub fn fill(red:f32,blue:f32,green:f32,alpha:f32) {
        unsafe {
            gl::ClearColor(red,blue,green,alpha);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
    }
    
    pub fn draw(shape:Shapes) {
       
        let vertex_data = match shape {
            Shapes::Pyramid => Shapes::pyramid() 
        };

        VertexProcessor::create_vertex_buffer(vertex_data);
        
        let base_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let vertex_shader_path = base_path.join("assets\\shaders\\pyramid_vertex.glsl");
        let fragment_shader_path = base_path.join("assets\\shaders\\pyramid_fragment.glsl");
        
        let shader = Shader::new(vertex_shader_path.to_str().unwrap().to_string(),fragment_shader_path.to_str().unwrap().to_string());
        shader.create_shader();

        unsafe {

            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            gl::DrawArrays(gl::TRIANGLES, 0, 18);

        }
    }
    
}







