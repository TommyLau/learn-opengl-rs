extern crate gl;
extern crate glfw;

use std::mem;
use gl::types::*;
use glfw::{Action, Context, Key};
use image;
use nalgebra_glm as glm;
use crate::shader::Shader;

// settings
const SCR_WIDTH: u32 = 800;
const SCR_HEIGHT: u32 = 600;

pub fn main_1_6_3() {
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
                                    "src/_1_getting_started/shaders/6.1.shader.vert",
                                    "src/_1_getting_started/shaders/5.1.shader.frag")
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
    let vertices: [f32; 180] = [
        -0.5, -0.5, -0.5,  0.0, 0.0,
         0.5, -0.5, -0.5,  1.0, 0.0,
         0.5,  0.5, -0.5,  1.0, 1.0,
         0.5,  0.5, -0.5,  1.0, 1.0,
        -0.5,  0.5, -0.5,  0.0, 1.0,
        -0.5, -0.5, -0.5,  0.0, 0.0,

        -0.5, -0.5,  0.5,  0.0, 0.0,
         0.5, -0.5,  0.5,  1.0, 0.0,
         0.5,  0.5,  0.5,  1.0, 1.0,
         0.5,  0.5,  0.5,  1.0, 1.0,
        -0.5,  0.5,  0.5,  0.0, 1.0,
        -0.5, -0.5,  0.5,  0.0, 0.0,

        -0.5,  0.5,  0.5,  1.0, 0.0,
        -0.5,  0.5, -0.5,  1.0, 1.0,
        -0.5, -0.5, -0.5,  0.0, 1.0,
        -0.5, -0.5, -0.5,  0.0, 1.0,
        -0.5, -0.5,  0.5,  0.0, 0.0,
        -0.5,  0.5,  0.5,  1.0, 0.0,

         0.5,  0.5,  0.5,  1.0, 0.0,
         0.5,  0.5, -0.5,  1.0, 1.0,
         0.5, -0.5, -0.5,  0.0, 1.0,
         0.5, -0.5, -0.5,  0.0, 1.0,
         0.5, -0.5,  0.5,  0.0, 0.0,
         0.5,  0.5,  0.5,  1.0, 0.0,

        -0.5, -0.5, -0.5,  0.0, 1.0,
         0.5, -0.5, -0.5,  1.0, 1.0,
         0.5, -0.5,  0.5,  1.0, 0.0,
         0.5, -0.5,  0.5,  1.0, 0.0,
        -0.5, -0.5,  0.5,  0.0, 0.0,
        -0.5, -0.5, -0.5,  0.0, 1.0,

        -0.5,  0.5, -0.5,  0.0, 1.0,
         0.5,  0.5, -0.5,  1.0, 1.0,
         0.5,  0.5,  0.5,  1.0, 0.0,
         0.5,  0.5,  0.5,  1.0, 0.0,
        -0.5,  0.5,  0.5,  0.0, 0.0,
        -0.5,  0.5, -0.5,  0.0, 1.0
    ];

    let cube_positions: [glm::Vec3; 10] = [
        glm::vec3(0.0, 0.0, 0.0),
        glm::vec3(2.0, 5.0, -15.0),
        glm::vec3(-1.5, -2.2, -2.5),
        glm::vec3(-3.8, -2.0, -12.3),
        glm::vec3(2.4, -0.4, -3.5),
        glm::vec3(-1.7, 3.0, -7.5),
        glm::vec3(1.3, -2.0, -2.5),
        glm::vec3(1.5, 2.0, -2.5),
        glm::vec3(1.5, 0.2, -1.5),
        glm::vec3(-1.3, 1.0, -1.5)
    ];

    let (mut vbo, mut vao): (GLuint, GLuint) = (0, 0);

    unsafe {
        // configure global opengl state
        // -----------------------------
        gl::Enable(gl::DEPTH_TEST);

        gl::GenVertexArrays(1, &mut vao);
        gl::GenBuffers(1, &mut vbo);

        gl::BindVertexArray(vao);

        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(gl::ARRAY_BUFFER,
                       (vertices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                       vertices.as_ptr() as *const GLvoid,
                       gl::STATIC_DRAW);

        // position attribute
        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, (5 * mem::size_of::<GLfloat>()) as GLsizei, 0 as *const GLvoid);
        gl::EnableVertexAttribArray(0);
        // texture coord attribute
        gl::VertexAttribPointer(1, 2, gl::FLOAT, gl::FALSE, (5 * mem::size_of::<GLfloat>()) as GLsizei, (3 * mem::size_of::<GLfloat>()) as *const GLvoid);
        gl::EnableVertexAttribArray(1);
    }

    // load and create a texture
    // -------------------------
    let (mut texture1, mut texture2): (GLuint, GLuint) = (0, 0);

    // texture 1
    // ---------
    unsafe {
        gl::GenTextures(1, &mut texture1);
        gl::BindTexture(gl::TEXTURE_2D, texture1);
        // set the texture wrapping parameters
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as GLint);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as GLint);
        // set texture filtering parameters
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as GLint);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as GLint);
    }
    // load image, create texture and generate mipmaps
    let img = image::open("resources/textures/container.jpg")
        .expect("Failed to load texture").flipv();
    let data = img.as_bytes();
    unsafe {
        gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGB as GLint, img.width() as GLsizei, img.height() as GLsizei,
                       0, gl::RGB, gl::UNSIGNED_BYTE, data.as_ptr() as *const GLvoid);
        gl::GenerateMipmap(gl::TEXTURE_2D);
    }

    // texture 2
    // ---------
    unsafe {
        gl::GenTextures(1, &mut texture2);
        gl::BindTexture(gl::TEXTURE_2D, texture2);
        // set the texture wrapping parameters
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as GLint);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as GLint);
        // set texture filtering parameters
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as GLint);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as GLint);
    }
    // load image, create texture and generate mipmaps
    let img = image::open("resources/textures/awesomeface.png")
        .expect("Failed to load texture").flipv();
    let data = img.as_bytes();
    unsafe {
        // note that the awesomeface.png has transparency and thus an alpha channel, so make sure to tell OpenGL the data type is of GL_RGBA
        gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGB as GLint, img.width() as GLsizei, img.height() as GLsizei,
                       0, gl::RGBA, gl::UNSIGNED_BYTE, data.as_ptr() as *const GLvoid);
        gl::GenerateMipmap(gl::TEXTURE_2D);
    }

    // tell opengl for each sampler to which texture unit it belongs to (only has to be done once)
    // -------------------------------------------------------------------------------------------
    shader.use_program();
    shader.set_int("texture1", 0);
    shader.set_int("texture2", 1);

    // render loop
    // -----------
    while !window.should_close() {
        unsafe {
            // render
            // ------
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT); // also clear the depth buffer now!

            // bind textures on corresponding texture units
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, texture1);
            gl::ActiveTexture(gl::TEXTURE1);
            gl::BindTexture(gl::TEXTURE_2D, texture2);

            // activate shader
            shader.use_program();

            // create transformations
            let view = glm::translate(&glm::identity(), &glm::vec3(0.0 as f32, 0.0, -3.0));
            let projection = glm::perspective(SCR_WIDTH as f32 / SCR_HEIGHT as f32, f32::to_radians(45.0), 0.1, 100.0);

            // pass transformation matrices to the shader
            shader.set_mat4("projection", &projection); // note: currently we set the projection matrix each frame, but since the projection matrix rarely changes
            shader.set_mat4("view", &view);

            // render boxes
            gl::BindVertexArray(vao);

            for i in 0..10 {
                let mut model = glm::translate(&glm::identity(), &cube_positions[i]);
                let angle = 20.0 * i as f32;
                model = glm::rotate(&model, angle.to_radians(), &glm::vec3(1.0, 0.3, 0.5));
                shader.set_mat4("model", &model);

                gl::DrawArrays(gl::TRIANGLES, 0, 36);
            }
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
    }
}
