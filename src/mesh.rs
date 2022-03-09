use std::mem;
use gl::types::*;
use nalgebra_glm as glm;
use memoffset::offset_of;
use crate::shader;

#[repr(C)]
pub struct Vertex {
    // position
    pub position: glm::Vec3,
    // normal
    pub normal: glm::Vec3,
    // texCoords
    pub tex_coords: glm::Vec2,
}

impl Default for Vertex {
    fn default() -> Self {
        Vertex {
            position: glm::zero(),
            normal: glm::zero(),
            tex_coords: glm::zero(),
        }
    }
}

#[derive(Clone)]
pub struct Texture {
    pub id: GLuint,
    pub type_name: String,
    pub path: String,
}

pub struct Mesh {
    // mesh Data
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
    pub textures: Vec<Texture>,
    pub vao: u32,

    // render data
    vbo: u32,
    ebo: u32,
}

impl Mesh {
    // constructor
    pub fn new(vertices: Vec<Vertex>, indices: Vec<u32>, textures: Vec<Texture>) -> Mesh {
        let mut mesh = Mesh { vertices, indices, textures, vao: 0, vbo: 0, ebo: 0 };

        // now that we have all the required data, set the vertex buffers and its attribute pointers.
        mesh.setup_mesh();

        mesh
    }

    // render the mesh
    pub fn draw(&self, shader: &shader::Shader)
    {
        // bind appropriate textures
        let mut diffuse_count = 0;
        let mut specular_count = 0;
        let mut normal_count = 0;
        let mut height_count = 0;

        unsafe {
            for (i, texture) in self.textures.iter().enumerate()
            {
                gl::ActiveTexture(gl::TEXTURE0 + i as GLenum); // active proper texture unit before binding
                // retrieve texture number (the N in diffuse_textureN)
                let name = &texture.type_name;
                let number = match name.as_str() {
                    "texture_diffuse" => {
                        diffuse_count += 1;
                        diffuse_count
                    }
                    "texture_specular" => {
                        specular_count += 1;
                        specular_count
                    }
                    "texture_normal" => {
                        normal_count += 1;
                        normal_count
                    }
                    "texture_height" => {
                        height_count += 1;
                        height_count
                    }
                    _ => 0,
                };

                // now set the sampler to the correct texture unit
                shader.set_int(&format!("{name}{number}"), i as GLint);
                // and finally bind the texture
                gl::BindTexture(gl::TEXTURE_2D, texture.id);
            }

            // draw mesh
            gl::BindVertexArray(self.vao);
            gl::DrawElements(gl::TRIANGLES, self.indices.len() as GLsizei, gl::UNSIGNED_INT, std::ptr::null());
            gl::BindVertexArray(0);

            // always good practice to set everything back to defaults once configured.
            gl::ActiveTexture(gl::TEXTURE0);
        }
    }

    // initializes all the buffer objects/arrays
    fn setup_mesh(&mut self)
    {
        unsafe {
            // create buffers/arrays
            gl::GenVertexArrays(1, &mut self.vao);
            gl::GenBuffers(1, &mut self.vbo);
            gl::GenBuffers(1, &mut self.ebo);

            gl::BindVertexArray(self.vao);
            // load data into vertex buffers
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
            // A great thing about structs is that their memory layout is sequential for all its items.
            // The effect is that we can simply pass a pointer to the struct and it translates perfectly to a gl::m::vec3/2 array which
            // again translates to 3/2 floats which translates to a byte array.
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (self.vertices.len() * mem::size_of::<Vertex>()) as GLsizeiptr,
                self.vertices.as_ptr() as *const GLvoid,
                gl::STATIC_DRAW);

            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.ebo);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (self.indices.len() * mem::size_of::<u32>()) as GLsizeiptr,
                self.indices.as_ptr() as *const GLvoid,
                gl::STATIC_DRAW);

            // set the vertex attribute pointers
            // vertex Positions
            gl::EnableVertexAttribArray(0);
            gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, mem::size_of::<Vertex>() as GLsizei, offset_of!(Vertex, position) as *const GLvoid);
            // vertex normals
            gl::EnableVertexAttribArray(1);
            gl::VertexAttribPointer(1, 3, gl::FLOAT, gl::FALSE, mem::size_of::<Vertex>() as GLsizei, offset_of!(Vertex, normal) as *const GLvoid);
            // vertex texture coords
            gl::EnableVertexAttribArray(2);
            gl::VertexAttribPointer(2, 2, gl::FLOAT, gl::FALSE, mem::size_of::<Vertex>() as GLsizei, offset_of!(Vertex, tex_coords) as *const GLvoid);

            gl::BindVertexArray(0);
        }
    }
}