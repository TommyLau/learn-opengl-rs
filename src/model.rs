use image::DynamicImage;
use russimp::scene::{PostProcess, Scene};
use crate::mesh::{Mesh, Texture, Vertex};
use nalgebra_glm as glm;
use russimp::texture::TextureType;
use gl::types::*;
use crate::shader::Shader;

#[derive(Default)]
pub struct Model {
    // model data
    // stores all the textures loaded so far, optimization to make sure textures aren't loaded more than once.
    pub textures_loaded: Vec<Texture>,
    pub meshes: Vec<Mesh>,
    pub directory: String,
    pub gammaCorrection: bool,
}

impl Model {
    // constructor, expects a filepath to a 3D model.
    pub fn new(path: &str, gamma: bool) -> Model
    {
        let mut model = Model {
            gammaCorrection: gamma,
            ..Model::default()
        };
        model.loadModel(path);
        model
    }

    // draws the model, and thus all its meshes
    pub fn Draw(&self, shader: &Shader)
    {
        for mesh in &self.meshes {
            mesh.Draw(shader);
        }
    }

    // loads a model with supported ASSIMP extensions from file and stores the resulting meshes in the meshes vector.
    fn loadModel(&mut self, path: &str)
    {
        // read file via ASSIMP
        let scene = match Scene::from_file(path,
                                           vec![
                                               PostProcess::Triangulate,
                                               PostProcess::GenerateSmoothNormals,
                                               //PostProcess::FlipUVs,
                                               PostProcess::CalculateTangentSpace],
        ) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("ERROR::ASSIMP:: {}", e);
                return;
            }
        };

        // retrieve the directory path of the filepath
        self.directory = path[..path.rfind('/').unwrap()].to_owned();

        for mesh in scene.meshes.iter() {
            println!("name: {}", mesh.name);
            // walk through each of the mesh's vertices
            let mut vertices: Vec<Vertex> = Vec::with_capacity(mesh.vertices.len());

            for i in 0..mesh.vertices.len() {
                let mut vertex = Vertex::default();

                // positions
                vertex.Position = glm::vec3(mesh.vertices[i].x, mesh.vertices[i].y, mesh.vertices[i].z);
                // normals
                vertex.Normal = glm::vec3(mesh.normals[i].x, mesh.normals[i].y, mesh.normals[i].z);
                // texture coordinates
                if let Some(texture_coord) = &mesh.texture_coords[0] {
                    vertex.TexCoords = glm::vec2(texture_coord[i].x, texture_coord[i].y);
                } else {
                    vertex.TexCoords = glm::zero();
                }

                vertices.push(vertex);
            }

            // now wak through each of the mesh's faces (a face is a mesh its triangle) and retrieve the corresponding vertex indices.
            let mut indices: Vec<u32> = Vec::with_capacity(mesh.faces.len() * 3);
            for face in mesh.faces.iter() {
                // retrieve all indices of the face and store them in the indices vector
                indices.extend(&face.0);
            }

            // process materials
            let material = scene.materials.get(mesh.material_index as usize).unwrap();

            // we assume a convention for sampler names in the shaders. Each diffuse texture should be named
            // as 'texture_diffuseN' where N is a sequential number ranging from 1 to MAX_SAMPLER_NUMBER.
            // Same applies to other texture as the following list summarizes:
            // diffuse: texture_diffuseN
            // specular: texture_specularN
            // normal: texture_normalN
            let mut textures = Vec::new();
            for (texture_type, texture) in &material.textures {
                let typeName = match texture_type {
                    // 1. diffuse maps
                    TextureType::Diffuse => "texture_diffuse",
                    // 2. specular maps
                    TextureType::Specular => "texture_specular",
                    // 3. normal maps
                    TextureType::Normals => "texture_normal",
                    // 4. height maps
                    TextureType::Height => "texture_height",
                    // 5. ambient maps
                    TextureType::Ambient => "texture_ambient",
                    // Unknown
                    _ => "texture_unknown",
                };
                textures.push(self.loadMaterialTexture(typeName, &texture[0].path));
            }

            self.meshes.push(Mesh::new(vertices, indices, textures));
        }
        //println!("{:#?}", self.meshes);
    }

    fn loadMaterialTexture(&mut self, typeName: &str, path: &str) -> Texture {
        print!("{typeName} - {path} : ");
        if let Some(texture) = self.textures_loaded.iter().find(|&x| x.path == path)
        {
            println!("[OLD] - {}", texture.id);
            return texture.clone();
        }

        let texture = Texture {
            id: TextureFromFile(path, &self.directory, self.gammaCorrection),
            type_: typeName.to_string(),
            path: path.to_string(),
        };
        println!("[New] - {}", texture.id);
        self.textures_loaded.push(texture.clone());
        texture
    }
}

fn TextureFromFile(path: &str, directory: &str, gamma: bool) -> GLuint
{
    let filename = format!("{directory}/{path}");
    let image = image::open(filename).expect("Texture failed to load at path: {path}");

    let mut texture_id: GLuint = 0;
    unsafe { gl::GenTextures(1, &mut texture_id) };

    let format = match image {
        DynamicImage::ImageLuma8(_) => gl::RED,
        DynamicImage::ImageRgb8(_) => gl::RGB,
        DynamicImage::ImageRgba8(_) => gl::RGBA,
        _ => gl::RGB,
    };

    let data = image.as_bytes();

    unsafe {
        gl::GenTextures(1, &mut texture_id);
        gl::BindTexture(gl::TEXTURE_2D, texture_id);
        gl::TexImage2D(gl::TEXTURE_2D, 0, format as GLint, image.width() as GLsizei, image.height() as GLsizei,
                       0, format, gl::UNSIGNED_BYTE, data.as_ptr() as *const GLvoid);
        gl::GenerateMipmap(gl::TEXTURE_2D);

        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as GLint);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as GLint);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR_MIPMAP_LINEAR as GLint);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as GLint);
    }

    texture_id
}
