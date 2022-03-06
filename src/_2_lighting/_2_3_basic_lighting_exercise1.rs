use std::mem;
use gl::types::*;
use glfw::{Action, Context, Key};
use nalgebra_glm as glm;
use crate::shader::Shader;
use crate::camera::Camera;
use crate::camera::CameraMovement;

// settings
const SCR_WIDTH: u32 = 800;
const SCR_HEIGHT: u32 = 600;

pub fn main_2_2_3() {
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
    window.set_cursor_pos_polling(true);
    window.set_scroll_polling(true);
    window.set_cursor_mode(glfw::CursorMode::Disabled);

    // gl: load all OpenGL function pointers
    // ---------------------------------------
    gl::load_with(|s| glfw.get_proc_address_raw(s));

    // configure global opengl state
    // -----------------------------
    unsafe { gl::Enable(gl::DEPTH_TEST) };

    // build and compile our shader program
    // ------------------------------------
    let lighting_shader = match Shader::new( // you can name your shader files however you like
                                             "src/_2_lighting/shaders/2.2.basic_lighting.vert",
                                             "src/_2_lighting/shaders/2.2.basic_lighting.frag")
    {
        Ok(shader) => shader,
        Err(error) => {
            eprintln!("{}", error);
            std::process::exit(-1);
        }
    };

    let light_cube_shader = match Shader::new( // you can name your shader files however you like
                                               "src/_2_lighting/shaders/1.light_cube.vert",
                                               "src/_2_lighting//shaders/1.light_cube.frag")
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
    let vertices: [f32; 216] = [
        -0.5, -0.5, -0.5,  0.0,  0.0, -1.0,
         0.5, -0.5, -0.5,  0.0,  0.0, -1.0,
         0.5,  0.5, -0.5,  0.0,  0.0, -1.0,
         0.5,  0.5, -0.5,  0.0,  0.0, -1.0,
        -0.5,  0.5, -0.5,  0.0,  0.0, -1.0,
        -0.5, -0.5, -0.5,  0.0,  0.0, -1.0,

        -0.5, -0.5,  0.5,  0.0,  0.0,  1.0,
         0.5, -0.5,  0.5,  0.0,  0.0,  1.0,
         0.5,  0.5,  0.5,  0.0,  0.0,  1.0,
         0.5,  0.5,  0.5,  0.0,  0.0,  1.0,
        -0.5,  0.5,  0.5,  0.0,  0.0,  1.0,
        -0.5, -0.5,  0.5,  0.0,  0.0,  1.0,

        -0.5,  0.5,  0.5, -1.0,  0.0,  0.0,
        -0.5,  0.5, -0.5, -1.0,  0.0,  0.0,
        -0.5, -0.5, -0.5, -1.0,  0.0,  0.0,
        -0.5, -0.5, -0.5, -1.0,  0.0,  0.0,
        -0.5, -0.5,  0.5, -1.0,  0.0,  0.0,
        -0.5,  0.5,  0.5, -1.0,  0.0,  0.0,

         0.5,  0.5,  0.5,  1.0,  0.0,  0.0,
         0.5,  0.5, -0.5,  1.0,  0.0,  0.0,
         0.5, -0.5, -0.5,  1.0,  0.0,  0.0,
         0.5, -0.5, -0.5,  1.0,  0.0,  0.0,
         0.5, -0.5,  0.5,  1.0,  0.0,  0.0,
         0.5,  0.5,  0.5,  1.0,  0.0,  0.0,

        -0.5, -0.5, -0.5,  0.0, -1.0,  0.0,
         0.5, -0.5, -0.5,  0.0, -1.0,  0.0,
         0.5, -0.5,  0.5,  0.0, -1.0,  0.0,
         0.5, -0.5,  0.5,  0.0, -1.0,  0.0,
        -0.5, -0.5,  0.5,  0.0, -1.0,  0.0,
        -0.5, -0.5, -0.5,  0.0, -1.0,  0.0,

        -0.5,  0.5, -0.5,  0.0,  1.0,  0.0,
         0.5,  0.5, -0.5,  0.0,  1.0,  0.0,
         0.5,  0.5,  0.5,  0.0,  1.0,  0.0,
         0.5,  0.5,  0.5,  0.0,  1.0,  0.0,
        -0.5,  0.5,  0.5,  0.0,  1.0,  0.0,
        -0.5,  0.5, -0.5,  0.0,  1.0,  0.0,
    ];

    let (mut vbo, mut cube_vao, mut light_cube_vao): (GLuint, GLuint, GLuint) = (0, 0, 0);

    unsafe {
        // first, configure the cube's VAO (and VBO)
        gl::GenVertexArrays(1, &mut cube_vao);
        gl::GenBuffers(1, &mut vbo);

        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(gl::ARRAY_BUFFER,
                       (vertices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                       vertices.as_ptr() as *const GLvoid,
                       gl::STATIC_DRAW);

        gl::BindVertexArray(cube_vao);

        // position attribute
        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, (6 * mem::size_of::<GLfloat>()) as GLsizei, std::ptr::null::<GLvoid>());
        gl::EnableVertexAttribArray(0);
        // normal attribute
        gl::VertexAttribPointer(1, 3, gl::FLOAT, gl::FALSE, (6 * mem::size_of::<GLfloat>()) as GLsizei, (3 * mem::size_of::<GLfloat>()) as *const GLvoid);
        gl::EnableVertexAttribArray(1);

        // second, configure the light's VAO (VBO stays the same; the vertices are the same for the light object which is also a 3D cube)
        gl::GenVertexArrays(1, &mut light_cube_vao);
        gl::BindVertexArray(light_cube_vao);

        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        // note that we update the lamp's position attribute's stride to reflect the updated buffer data
        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, (6 * mem::size_of::<GLfloat>()) as GLsizei, std::ptr::null::<GLvoid>());
        gl::EnableVertexAttribArray(0);
    }

    // camera
    let mut camera = Camera {
        position: glm::vec3(0.0, 0.0, 3.0),
        ..Camera::default()
    };
    let mut last_x = SCR_WIDTH as f32 / 2.0;
    let mut last_y = SCR_HEIGHT as f32 / 2.0;
    let mut first_mouse = true;

    // timing
    let mut delta_time; // time between current frame and last frame
    let mut last_frame = 0.0;

    // lighting
    let mut light_position = glm::vec3::<f32>(1.2, 1.0, 2.0);

    // render loop
    // -----------
    while !window.should_close() {
        // per-frame time logic
        // --------------------
        let current_frame = glfw.get_time() as f32;
        delta_time = current_frame - last_frame;
        last_frame = current_frame;

        unsafe {
            // render
            // ------
            gl::ClearColor(0.1, 0.1, 0.1, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            // change the light's position values over time (can be done anywhere in the render loop actually, but try to do it at least before using the light source positions)
            light_position.x = 1.0 + glfw.get_time().sin() as f32 * 2.0;
            light_position.y = (glfw.get_time() / 2.0).sin() as f32 * 1.0;

            // be sure to activate shader when setting uniforms/drawing objects
            lighting_shader.use_program();
            lighting_shader.set_vec3("objectColor", 1.0, 0.5, 0.31);
            lighting_shader.set_vec3("lightColor", 1.0, 1.0, 1.0);
            lighting_shader.set_vec3v("lightPos", &light_position);
            lighting_shader.set_vec3v("viewPos", &camera.position);

            // view/projection transformations
            let projection = glm::perspective(SCR_WIDTH as f32 / SCR_HEIGHT as f32, camera.zoom.to_radians(), 0.1, 100.0);
            let view = camera.get_view_matrix();
            lighting_shader.set_mat4("projection", &projection);
            lighting_shader.set_mat4("view", &view);

            // world transformation
            let mut model: glm::Mat4 = glm::identity();
            lighting_shader.set_mat4("model", &model);

            // render the cube
            gl::BindVertexArray(cube_vao);
            gl::DrawArrays(gl::TRIANGLES, 0, 36);

            // also draw the lamp object
            light_cube_shader.use_program();
            light_cube_shader.set_mat4("projection", &projection);
            light_cube_shader.set_mat4("view", &view);
            model = glm::translate(&glm::identity(), &light_position);
            model = glm::scale(&model, &glm::vec3(0.2, 0.2, 0.2));
            light_cube_shader.set_mat4("model", &model);

            gl::BindVertexArray(light_cube_vao);
            gl::DrawArrays(gl::TRIANGLES, 0, 36);
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
                glfw::WindowEvent::Key(Key::W, _, Action::Press | Action::Repeat, _) => {
                    camera.process_keyboard(CameraMovement::Forward, delta_time);
                }
                glfw::WindowEvent::Key(Key::S, _, Action::Press | Action::Repeat, _) => {
                    camera.process_keyboard(CameraMovement::Backward, delta_time);
                }
                glfw::WindowEvent::Key(Key::A, _, Action::Press | Action::Repeat, _) => {
                    camera.process_keyboard(CameraMovement::Left, delta_time);
                }
                glfw::WindowEvent::Key(Key::D, _, Action::Press | Action::Repeat, _) => {
                    camera.process_keyboard(CameraMovement::Right, delta_time);
                }
                glfw::WindowEvent::CursorPos(x, y) => {
                    let (x, y) = (x as f32, y as f32);

                    if first_mouse {
                        last_x = x;
                        last_y = y;
                        first_mouse = false;
                    }

                    let offset_x = x - last_x;
                    let offset_y = last_y - y; // reversed since y-coordinates go from bottom to top

                    last_x = x;
                    last_y = y;

                    camera.process_mouse_movement(offset_x, offset_y, true);
                }
                glfw::WindowEvent::Scroll(_offset_x, offset_y) => {
                    camera.process_mouse_scroll(offset_y as f32);
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
        gl::DeleteVertexArrays(1, &cube_vao);
        gl::DeleteVertexArrays(1, &light_cube_vao);
        gl::DeleteBuffers(1, &vbo);
    }
}
