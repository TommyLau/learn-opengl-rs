use glfw::{Action, Context, Key};
use nalgebra_glm as glm;
use crate::shader::Shader;
use crate::camera::Camera;
use crate::camera::CameraMovement;
use crate::model::Model;

// settings
const SCR_WIDTH: u32 = 800;
const SCR_HEIGHT: u32 = 600;

pub fn main_3_1() {
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

    // build and compile shaders
    // -------------------------
    let our_shader = match Shader::new("src/_3_model_loading/shaders/1.model_loading.vert",
                                       "src/_3_model_loading/shaders/1.model_loading.frag")
    {
        Ok(shader) => shader,
        Err(error) => {
            eprintln!("{}", error);
            std::process::exit(-1);
        }
    };

    // load models
    // -----------
    let our_model = Model::new("resources/objects/backpack/backpack.obj", false);

    // draw in wireframe
    //unsafe { gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE); }

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

            // be sure to activate shader when setting uniforms/drawing objects
            our_shader.use_program();

            // view/projection transformations
            let projection = glm::perspective(SCR_WIDTH as f32 / SCR_HEIGHT as f32, camera.zoom.to_radians(), 0.1, 100.0);
            let view = camera.get_view_matrix();
            our_shader.set_mat4("projection", &projection);
            our_shader.set_mat4("view", &view);

            // render the loaded model
            let mut model: glm::Mat4 = glm::identity();
            model = glm::translate(&model, &glm::vec3(0.0, 0.0, 0.0)); // translate it down so it's at the center of the scene
            model = glm::scale(&model, &glm::vec3(1.0, 1.0, 1.0)); // it's a bit too big for our scene, so scale it down
            our_shader.set_mat4("model", &model);
            our_model.draw(&our_shader);
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
}
