extern crate gl;
extern crate glfw;

use std::mem;
use gl::types::*;
use glfw::{Action, Context, Key};
use image;
use crate::shader::Shader;

// settings
const SCR_WIDTH: u32 = 800;
const SCR_HEIGHT: u32 = 600;

pub fn main_1_4_1() {
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
    let shader = match Shader::new( // you can name your shader files however you like
                                    "src/_1_getting_started/shaders/4.1.shader.vert",
                                    "src/_1_getting_started/shaders/4.1.shader.frag")
    {
        Ok(shader) => shader,
        Err(error) => {
            eprintln!("{}", error);
            std::process::exit(-1);
        }
    };

    // set up vertex data (and buffer(s)) and configure vertex attributes
    // ------------------------------------------------------------------
    // Under macOS, the default type is 'f64', so we have to specific to 'f32'
    let vertices: [GLfloat; 32] = [
        // positions      // colors       // texture coords
        0.5,   0.5, 0.0,  1.0, 0.0, 0.0,  1.0, 1.0,   // top right
        0.5,  -0.5, 0.0,  0.0, 1.0, 0.0,  1.0, 0.0,   // bottom right
        -0.5, -0.5, 0.0,  0.0, 0.0, 1.0,  0.0, 0.0,   // bottom left
        -0.5,  0.5, 0.0,  1.0, 1.0, 0.0,  0.0, 1.0    // top left
    ];
    let indices: [GLuint; 6] = [
        0, 1, 3,  // first Triangle
        1, 2, 3   // second Triangle
    ];

    let mut vbo: GLuint = 0;
    let mut vao: GLuint = 0;
    let mut ebo: GLuint = 0;

    unsafe {
        gl::GenVertexArrays(1, &mut vao);
        gl::GenBuffers(1, &mut vbo);
        gl::GenBuffers(1, &mut ebo);

        gl::BindVertexArray(vao);

        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(gl::ARRAY_BUFFER,
                       (vertices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                       vertices.as_ptr() as *const GLvoid,
                       gl::STATIC_DRAW);

        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
        gl::BufferData(gl::ELEMENT_ARRAY_BUFFER,
                       (indices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                       indices.as_ptr() as *const GLvoid,
                       gl::STATIC_DRAW);

        // position attribute
        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, (8 * mem::size_of::<GLfloat>()) as GLsizei, 0 as *const GLvoid);
        gl::EnableVertexAttribArray(0);
        // color attribute
        gl::VertexAttribPointer(1, 3, gl::FLOAT, gl::FALSE, (8 * mem::size_of::<GLfloat>()) as GLsizei, (3 * mem::size_of::<GLfloat>()) as *const GLvoid);
        gl::EnableVertexAttribArray(1);
        // texture attribute
        gl::VertexAttribPointer(2, 2, gl::FLOAT, gl::FALSE, (8 * mem::size_of::<GLfloat>()) as GLsizei, (6 * mem::size_of::<GLfloat>()) as *const GLvoid);
        gl::EnableVertexAttribArray(2);
    }

    // load and create a texture
    // -------------------------
    let mut texture: GLuint = 0;

    unsafe {
        gl::GenTextures(1, &mut texture);
        gl::BindTexture(gl::TEXTURE_2D, texture); // all upcoming GL_TEXTURE_2D operations now have effect on this texture object
        // set the texture wrapping parameters
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as GLint); // set texture wrapping to GL_REPEAT (default wrapping method)
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as GLint);
        // set texture filtering parameters
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as GLint);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as GLint);
    }
    // load image, create texture and generate mipmaps
    let img = image::open("resources/textures/container.jpg")
        .expect("Failed to load texture");
    let data = img.as_bytes();
    unsafe {
        gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGB as GLint, img.width() as GLsizei, img.height() as GLsizei,
                       0, gl::RGB, gl::UNSIGNED_BYTE, data.as_ptr() as *const GLvoid);
        gl::GenerateMipmap(gl::TEXTURE_2D);
    }

    // render loop
    // -----------
    while !window.should_close() {
        unsafe {
            // render
            // ------
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            // bind Texture
            gl::BindTexture(gl::TEXTURE_2D, texture);

            // render container
            shader.use_program();
            gl::BindVertexArray(vao);
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, 0 as *const GLvoid);
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
        gl::DeleteVertexArrays(1, &vao);
        gl::DeleteBuffers(1, &vbo);
        gl::DeleteBuffers(1, &ebo);
    }
}
