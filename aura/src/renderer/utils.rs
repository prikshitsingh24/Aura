extern crate gl;
use gl::types::*;
use std::{fs, ptr, ffi::CString};

pub struct VertexProcessor {}


impl VertexProcessor {
    pub fn create_vertex_buffer(vertices:Vec<f32>) {
        let mut vao: GLuint = 0;
        let mut vbo: GLuint = 0;
        unsafe {
            gl::GenVertexArrays(1,&mut vao);
            gl::BindVertexArray(vao);
            gl::GenBuffers(1,&mut vbo);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);

            gl::BufferData(gl::ARRAY_BUFFER, (vertices.len() * std::mem::size_of::<GLfloat>()) as GLsizeiptr,vertices.as_ptr() as *const _, gl::STATIC_DRAW);

            gl::VertexAttribPointer(
                0,                  // index of the attribute (location = 0 in vertex shader)
                3,                  // 3 components per vertex (x, y, z)
                gl::FLOAT,          // type
                gl::FALSE,          // normalized?
                3 * std::mem::size_of::<GLfloat>() as GLsizei, // stride
                std::ptr::null(),   // offset
            );
            gl::EnableVertexAttribArray(0);

        }   
    }
}


pub struct Shader {
    vertex_shader_path:String,
    fragment_shader_path:String
}


impl Shader {

    pub fn new(vertex_shader_path:String, fragment_shader_path:String) -> Self {
        Shader {
            vertex_shader_path,
            fragment_shader_path
        }
    }

    pub fn create_shader(&self) {
        // vertex shader
        let vertex_shader_source = CString::new(fs::read_to_string(&self.vertex_shader_path)
        .expect("Should have been able to read the file")).unwrap();
    
        let vertex_shader:GLuint;
        unsafe {
            vertex_shader = gl::CreateShader(gl::VERTEX_SHADER);
            gl::ShaderSource(vertex_shader,1,&vertex_shader_source.as_ptr(),ptr::null());
            gl::CompileShader(vertex_shader);
        }
 

        // fragment shader
        let fragment_shader_source = CString::new(fs::read_to_string(&self.fragment_shader_path)
        .expect("Should have been able to read the file")).unwrap();
        let fragment_shader:GLuint;
        unsafe {
            fragment_shader = gl::CreateShader(gl::FRAGMENT_SHADER);
            gl::ShaderSource(fragment_shader,1,&fragment_shader_source.as_ptr(),ptr::null());
            gl::CompileShader(fragment_shader);
        }

        // main shader program to link vertex and fragment shader
        unsafe {
            let shader_program:GLuint;
            shader_program = gl::CreateProgram();
            gl::AttachShader(shader_program,vertex_shader);
            gl::AttachShader(shader_program,fragment_shader);
            gl::LinkProgram(shader_program);
            gl::DeleteShader(vertex_shader);
            gl::DeleteShader(fragment_shader);
            gl::UseProgram(shader_program);
        }

       

    }
}