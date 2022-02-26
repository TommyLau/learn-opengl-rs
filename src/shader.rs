use std::ffi::CString;
use std::{fs, ptr};
use gl::{self, types::*};

pub struct Shader {
    pub id: GLuint,
}

impl Shader {
    // constructor generates the shader on the fly
    // ------------------------------------------------------------------------
    pub fn new(vertex_path: &str, fragment_path: &str) -> Result<Shader, String> {
        let mut shader = Shader { id: 0 };
        // 1. retrieve the vertex/fragment source code from filePath
        let vertex_code = match fs::read_to_string(vertex_path) {
            Ok(code) => code,
            Err(error) => return Err(format!("ERROR::SHADER::FILE_NOT_SUCCESFULLY_READ: {}", error)),
        };
        let fragment_code = match fs::read_to_string(fragment_path) {
            Ok(code) => code,
            Err(error) => return Err(format!("ERROR::SHADER::FILE_NOT_SUCCESFULLY_READ: {}", error)),
        };

        let v_shader_code = CString::new(&vertex_code[..]).unwrap();
        let f_shader_code = CString::new(&fragment_code[..]).unwrap();

        // 2. compile shaders
        // vertex shader
        let vertex = unsafe { gl::CreateShader(gl::VERTEX_SHADER) };
        unsafe {
            gl::ShaderSource(vertex, 1, &v_shader_code.as_ptr(), ptr::null());
            gl::CompileShader(vertex);
        }
        shader.check_compile_errors(vertex, "VERTEX")?;

        // fragment Shader
        let fragment = unsafe { gl::CreateShader(gl::FRAGMENT_SHADER) };
        unsafe {
            gl::ShaderSource(fragment, 1, &f_shader_code.as_ptr(), ptr::null());
            gl::CompileShader(fragment);
        }
        shader.check_compile_errors(fragment, "FRAGMENT")?;

        // shader Program
        shader.id = unsafe { gl::CreateProgram() };
        unsafe {
            gl::AttachShader(shader.id, vertex);
            gl::AttachShader(shader.id, fragment);
            gl::LinkProgram(shader.id);
        }
        shader.check_compile_errors(shader.id, "PROGRAM")?;

        // delete the shaders as they're linked into our program now and no longer necessary
        unsafe {
            gl::DeleteShader(vertex);
            gl::DeleteShader(fragment);
        }

        Ok(shader)
    }

    // activate the shader
    // ------------------------------------------------------------------------
    pub fn use_program(&self) {
        unsafe { gl::UseProgram(self.id); }
    }

    // utility uniform functions
    // ------------------------------------------------------------------------
    #[allow(dead_code)]
    pub fn set_bool(&self, name: &str, value: bool) {
        let name = CString::new(&name[..]).unwrap();
        unsafe { gl::Uniform1i(gl::GetUniformLocation(self.id, name.as_ptr()), value as GLint); }
    }
    // ------------------------------------------------------------------------
    pub fn set_int(&self, name: &str, value: GLint) {
        let name = CString::new(&name[..]).unwrap();
        unsafe { gl::Uniform1i(gl::GetUniformLocation(self.id, name.as_ptr()), value); }
    }
    // ------------------------------------------------------------------------
    pub fn set_float(&self, name: &str, value: GLfloat)
    {
        let name = CString::new(&name[..]).unwrap();
        unsafe { gl::Uniform1f(gl::GetUniformLocation(self.id, name.as_ptr()), value); }
    }

    // utility function for checking shader compilation/linking errors.
    // ------------------------------------------------------------------------
    fn check_compile_errors(&self, shader: GLuint, shader_type: &str) -> Result<(), String>
    {
        let mut success: GLint = gl::TRUE as GLint;

        match shader_type {
            "PROGRAM" => {
                unsafe { gl::GetProgramiv(shader, gl::LINK_STATUS, &mut success); }

                if success == gl::FALSE as GLint {
                    return Err(format!(
                        "ERROR::PROGRAM_LINKING_ERROR of type: {}\n{}\n -- --------------------------------------------------- -- ",
                        shader_type, self.get_shader_info_log(shader)));
                }
            }
            _ => {
                unsafe { gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success); }

                if success == gl::FALSE as GLint {
                    return Err(format!(
                        "ERROR::SHADER_COMPILATION_ERROR of type: {}\n{}\n -- --------------------------------------------------- -- ",
                        shader_type, self.get_shader_info_log(shader)));
                }
            }
        }

        Ok(())
    }

    fn get_shader_info_log(&self, shader: GLuint) -> String {
        let mut len: GLint = 0;

        unsafe {
            gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);
        }

        let mut info_log: Vec<u8> = Vec::new();
        info_log.resize(len as usize + 1, 0);

        unsafe {
            gl::GetShaderInfoLog(shader, len, ptr::null_mut(), info_log.as_ptr() as *mut GLchar);
        }

        String::from_utf8_lossy(&info_log).to_string()
    }
}
