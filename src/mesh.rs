use std::mem;
use gl::types::*;
use nalgebra_glm as glm;
use memoffset::offset_of;
use crate::shader;

#[repr(C, packed)]
#[derive(Debug)]
pub struct Vertex {
    // position
    pub Position: glm::Vec3,
    // normal
    pub Normal: glm::Vec3,
    // texCoords
    pub TexCoords: glm::Vec2,
}

impl Default for Vertex {
    fn default() -> Self {
        Vertex {
            Position: glm::zero(),
            Normal: glm::zero(),
            TexCoords: glm::zero(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Texture {
    pub id: GLuint,
    pub type_: String,
    pub path: String,
}

#[derive(Debug)]
pub struct Mesh {
    // mesh Data
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
    pub textures: Vec<Texture>,
    pub VAO: u32,

    // render data
    VBO: u32,
    EBO: u32,
}

impl Mesh {
    // constructor
    pub fn new(vertices: Vec<Vertex>, indices: Vec<u32>, textures: Vec<Texture>) -> Mesh {
        let mut mesh = Mesh { vertices, indices, textures, VAO: 0, VBO: 0, EBO: 0 };

        // now that we have all the required data, set the vertex buffers and its attribute pointers.
        mesh.setupMesh();

        mesh
    }

    // render the mesh
    pub fn Draw(&self, shader: &shader::Shader)
    {
        // bind appropriate textures
        let mut diffuseNr = 0;
        let mut specularNr = 0;
        let mut normalNr = 0;
        let mut heightNr = 0;

        unsafe {
            for (i, texture) in self.textures.iter().enumerate()
            {
                gl::ActiveTexture(gl::TEXTURE0 + i as GLenum); // active proper texture unit before binding
                // retrieve texture number (the N in diffuse_textureN)
                let name = &texture.type_;
                let number = match name.as_str() {
                    "texture_diffuse" => {
                        diffuseNr += 1;
                        diffuseNr
                    }
                    "texture_specular" => {
                        specularNr += 1;
                        specularNr
                    }
                    "texture_normal" => {
                        normalNr += 1;
                        normalNr
                    }
                    "texture_height" => {
                        heightNr += 1;
                        heightNr
                    }
                    _ => 0,
                };

                // now set the sampler to the correct texture unit
                shader.set_int(&format!("{name}{number}"), i as GLint);
                // and finally bind the texture
                gl::BindTexture(gl::TEXTURE_2D, texture.id);
            }

            // draw mesh
            gl::BindVertexArray(self.VAO);
            gl::DrawElements(gl::TRIANGLES, self.indices.len() as GLsizei, gl::UNSIGNED_INT, std::ptr::null());
            gl::BindVertexArray(0);

            // always good practice to set everything back to defaults once configured.
            gl::ActiveTexture(gl::TEXTURE0);
        }
    }

    // initializes all the buffer objects/arrays
    fn setupMesh(&mut self)
    {
        unsafe {
            // create buffers/arrays
            gl::GenVertexArrays(1, &mut self.VAO);
            gl::GenBuffers(1, &mut self.VBO);
            gl::GenBuffers(1, &mut self.EBO);

            gl::BindVertexArray(self.VAO);
            // load data into vertex buffers
            gl::BindBuffer(gl::ARRAY_BUFFER, self.VBO);
            // A great thing about structs is that their memory layout is sequential for all its items.
            // The effect is that we can simply pass a pointer to the struct and it translates perfectly to a gl::m::vec3/2 array which
            // again translates to 3/2 floats which translates to a byte array.
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (self.vertices.len() * mem::size_of::<Vertex>()) as GLsizeiptr,
                self.vertices.as_ptr() as *const GLvoid,
                gl::STATIC_DRAW);

            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.EBO);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (self.indices.len() * mem::size_of::<u32>()) as GLsizeiptr,
                self.indices.as_ptr() as *const GLvoid,
                gl::STATIC_DRAW);

            // set the vertex attribute pointers
            // vertex Positions
            gl::EnableVertexAttribArray(0);
            gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, mem::size_of::<Vertex>() as GLsizei, offset_of!(Vertex, Position) as *const GLvoid);
            // vertex normals
            gl::EnableVertexAttribArray(1);
            gl::VertexAttribPointer(1, 3, gl::FLOAT, gl::FALSE, mem::size_of::<Vertex>() as GLsizei, offset_of!(Vertex, Normal) as *const GLvoid);
            // vertex texture coords
            gl::EnableVertexAttribArray(2);
            gl::VertexAttribPointer(2, 2, gl::FLOAT, gl::FALSE, mem::size_of::<Vertex>() as GLsizei, offset_of!(Vertex, TexCoords) as *const GLvoid);

            gl::BindVertexArray(0);
        }
    }
}