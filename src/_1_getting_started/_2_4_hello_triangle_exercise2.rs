extern crate gl;
extern crate glfw;

use std::ffi::CString;
use std::{mem, ptr};
use gl::types::*;
use glfw::{Action, Context, Key};

// settings
const SCR_WIDTH: u32 = 800;
const SCR_HEIGHT: u32 = 600;

const VERTEX_SHADER_SOURCE: &str = "\
#version 330 core
layout (location = 0) in vec3 aPos;
void main()
{
   gl_Position = vec4(aPos.x, aPos.y, aPos.z, 1.0);
}
";

const FRAGMENT_SHADER_SOURCE: &str = "\
#version 330 core
out vec4 FragColor;
void main()
{
   FragColor = vec4(1.0f, 0.5f, 0.2f, 1.0f);
}
";

pub fn main_1_2_4() {
    // glfw: initialize and configure
    // ------------------------------
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
    #[cfg(target_os = "macos")]
    glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));

    // glfw window creation
    // --------------------
    let (mut window, events) = glfw
        .create_window(SCR_WIDTH, SCR_HEIGHT, "LearnOpenGL", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window");
    window.make_current();
    window.set_key_polling(true);
    window.set_framebuffer_size_polling(true);

    // gl: load all OpenGL function pointers
    // ---------------------------------------
    gl::load_with(|s| glfw.get_proc_address_raw(s));

    // build and compile our shader program
    // ------------------------------------
    let mut success = gl::FALSE as GLint;
    let mut info_log: Vec<u8> = Vec::with_capacity(512);
    info_log.resize(512, 0);

    // vertex shader
    let vertex_shader = unsafe { gl::CreateShader(gl::VERTEX_SHADER) };
    unsafe {
        let vertex_shader_source = CString::new(VERTEX_SHADER_SOURCE.as_bytes()).unwrap();
        gl::ShaderSource(vertex_shader, 1, &vertex_shader_source.as_ptr(), ptr::null());
        gl::CompileShader(vertex_shader);

        // check for shader compile errors
        gl::GetShaderiv(vertex_shader, gl::COMPILE_STATUS, &mut success);
        if success != gl::TRUE as GLint {
            gl::GetShaderInfoLog(vertex_shader, 512, ptr::null_mut(), info_log.as_ptr() as *mut GLchar);
            eprintln!("ERROR::SHADER::VERTEX::COMPILATION_FAILED\n{}", String::from_utf8_lossy(&info_log));
        }
    }

    // fragment shader
    let fragment_shader = unsafe { gl::CreateShader(gl::FRAGMENT_SHADER) };
    unsafe {
        let fragment_shader_source = CString::new(FRAGMENT_SHADER_SOURCE.as_bytes()).unwrap();
        gl::ShaderSource(fragment_shader, 1, &fragment_shader_source.as_ptr(), std::ptr::null_mut());
        gl::CompileShader(fragment_shader);

        // check for shader compile errors
        gl::GetShaderiv(fragment_shader, gl::COMPILE_STATUS, &mut success);
        if success != gl::TRUE as GLint {
            gl::GetShaderInfoLog(fragment_shader, 512, ptr::null_mut(), info_log.as_ptr() as *mut GLchar);
            eprintln!("ERROR::SHADER::FRAGMENT::COMPILATION_FAILED\n{}", String::from_utf8_lossy(&info_log));
        }
    }

    // link shaders
    let shader_program = unsafe { gl::CreateProgram() };
    unsafe {
        gl::AttachShader(shader_program, vertex_shader);
        gl::AttachShader(shader_program, fragment_shader);
        gl::LinkProgram(shader_program);

        // check for linking errors
        gl::GetProgramiv(shader_program, gl::LINK_STATUS, &mut success);
        if success != gl::TRUE as GLint {
            gl::GetProgramInfoLog(shader_program, 512, std::ptr::null_mut(), info_log.as_ptr() as *mut GLchar);
            eprintln!("ERROR::SHADER::PROGRAM::LINKING_FAILED\n{}", String::from_utf8_lossy(&info_log));
        }
        gl::DeleteShader(vertex_shader);
        gl::DeleteShader(fragment_shader);
    }

    // set up vertex data (and buffer(s)) and configure vertex attributes
    // ------------------------------------------------------------------
    // Under macOS, the default type is 'f64', so we have to specific to 'f32'
    // add a new set of vertices to form a second triangle (a total of 6 vertices); the vertex attribute configuration remains the same (still one 3-float position vector per vertex)
    let first_triangle: [GLfloat; 9] = [
        // first triangle
        -0.9, -0.5, 0.0,  // left
        -0.0, -0.5, 0.0,  // right
        -0.45, 0.5, 0.0,  // top
    ];
    let second_triangle: [GLfloat; 9] = [
        // second triangle
        0.0, -0.5, 0.0,  // left
        0.9, -0.5, 0.0,  // right
        0.45, 0.5, 0.0   // top
    ];

    let mut vbos: [GLuint; 2] = [0; 2];
    let mut vaos: [GLuint; 2] = [0; 2];

    unsafe {
        gl::GenVertexArrays(2, vaos.as_mut_ptr());
        gl::GenBuffers(2, vbos.as_mut_ptr()); // we can also generate multiple VAOs or buffers at the same time

        // first triangle setup
        // --------------------
        gl::BindVertexArray(vaos[0]);

        gl::BindBuffer(gl::ARRAY_BUFFER, vbos[0]);
        gl::BufferData(gl::ARRAY_BUFFER,
                       (first_triangle.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                       first_triangle.as_ptr() as *const GLvoid,
                       gl::STATIC_DRAW);

        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, (3 * mem::size_of::<GLfloat>()) as GLsizei, 0 as *const GLvoid); // Vertex attributes stay the same
        gl::EnableVertexAttribArray(0);

        // gl::BindVertexArray(0); // no need to unbind at all as we directly bind a different VAO the next few lines

        // second triangle setup
        // ---------------------
        gl::BindVertexArray(vaos[1]); // note that we bind to a different VAO now

        gl::BindBuffer(gl::ARRAY_BUFFER, vbos[1]); // and a different VBO
        gl::BufferData(gl::ARRAY_BUFFER,
                       (second_triangle.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                       second_triangle.as_ptr() as *const GLvoid,
                       gl::STATIC_DRAW);

        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, (3 * mem::size_of::<GLfloat>()) as GLsizei, 0 as *const GLvoid); // because the vertex data is tightly packed we can also specify 0 as the vertex attribute's stride to let OpenGL figure it out
        gl::EnableVertexAttribArray(0);

        // gl::BindVertexArray(0); // not really necessary as well, but beware of calls that could affect VAOs while this one is bound (like binding element buffer objects, or enabling/disabling vertex attributes)

        // uncomment this call to draw in wireframe polygons.
        // gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
    }

    // render loop
    // -----------
    while !window.should_close() {
        unsafe {
            // render
            // ------
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            gl::UseProgram(shader_program);

            // draw first triangle using the data from the first VAO
            gl::BindVertexArray(vaos[0]);
            gl::DrawArrays(gl::TRIANGLES, 0, 3);

            // then we draw the second triangle using the data from the second VAO
            gl::BindVertexArray(vaos[1]);
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
        }

        // glfw: swap buffers and poll IO events (keys pressed/released, mouse moved etc.)
        // -------------------------------------------------------------------------------
        window.swap_buffers();
        glfw.poll_events();

        // events
        // ------
        for (_, event) in glfw::flush_messages(&events) {
            match event {
                glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    window.set_should_close(true);
                }
                glfw::WindowEvent::FramebufferSize(width, height) => {
                    // make sure the viewport matches the new window dimensions; note that width and
                    // height will be significantly larger than specified on retina displays.
                    unsafe {
                        gl::Viewport(0, 0, width, height);
                    }
                }
                _ => {}
            }
        }
    }

    // optional: de-allocate all resources once they've outlived their purpose:
    // ------------------------------------------------------------------------
    unsafe {
        gl::DeleteVertexArrays(2, vaos.as_mut_ptr());
        gl::DeleteBuffers(2, vbos.as_mut_ptr());
        gl::DeleteProgram(shader_program);
    }
}
