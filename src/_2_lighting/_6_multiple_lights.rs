use std::mem;
use gl::types::*;
use glfw::{Action, Context, Key};
use image::DynamicImage;
use nalgebra_glm as glm;
use crate::shader::Shader;
use crate::camera::Camera;
use crate::camera::CameraMovement;

// settings
const SCR_WIDTH: u32 = 800;
const SCR_HEIGHT: u32 = 600;

pub fn main_2_6() {
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
                                             "src/_2_lighting/shaders/4.1.lighting_maps.vert",
                                             "src/_2_lighting/shaders/6.multiple_lights.frag")
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
    let vertices:[f32;288] = [
        // positions       // normals        // texture coords
        -0.5, -0.5, -0.5,  0.0,  0.0, -1.0,  0.0,  0.0,
         0.5, -0.5, -0.5,  0.0,  0.0, -1.0,  1.0,  0.0,
         0.5,  0.5, -0.5,  0.0,  0.0, -1.0,  1.0,  1.0,
         0.5,  0.5, -0.5,  0.0,  0.0, -1.0,  1.0,  1.0,
        -0.5,  0.5, -0.5,  0.0,  0.0, -1.0,  0.0,  1.0,
        -0.5, -0.5, -0.5,  0.0,  0.0, -1.0,  0.0,  0.0,

        -0.5, -0.5,  0.5,  0.0,  0.0,  1.0,  0.0,  0.0,
         0.5, -0.5,  0.5,  0.0,  0.0,  1.0,  1.0,  0.0,
         0.5,  0.5,  0.5,  0.0,  0.0,  1.0,  1.0,  1.0,
         0.5,  0.5,  0.5,  0.0,  0.0,  1.0,  1.0,  1.0,
        -0.5,  0.5,  0.5,  0.0,  0.0,  1.0,  0.0,  1.0,
        -0.5, -0.5,  0.5,  0.0,  0.0,  1.0,  0.0,  0.0,

        -0.5,  0.5,  0.5, -1.0,  0.0,  0.0,  1.0,  0.0,
        -0.5,  0.5, -0.5, -1.0,  0.0,  0.0,  1.0,  1.0,
        -0.5, -0.5, -0.5, -1.0,  0.0,  0.0,  0.0,  1.0,
        -0.5, -0.5, -0.5, -1.0,  0.0,  0.0,  0.0,  1.0,
        -0.5, -0.5,  0.5, -1.0,  0.0,  0.0,  0.0,  0.0,
        -0.5,  0.5,  0.5, -1.0,  0.0,  0.0,  1.0,  0.0,

         0.5,  0.5,  0.5,  1.0,  0.0,  0.0,  1.0,  0.0,
         0.5,  0.5, -0.5,  1.0,  0.0,  0.0,  1.0,  1.0,
         0.5, -0.5, -0.5,  1.0,  0.0,  0.0,  0.0,  1.0,
         0.5, -0.5, -0.5,  1.0,  0.0,  0.0,  0.0,  1.0,
         0.5, -0.5,  0.5,  1.0,  0.0,  0.0,  0.0,  0.0,
         0.5,  0.5,  0.5,  1.0,  0.0,  0.0,  1.0,  0.0,

        -0.5, -0.5, -0.5,  0.0, -1.0,  0.0,  0.0,  1.0,
         0.5, -0.5, -0.5,  0.0, -1.0,  0.0,  1.0,  1.0,
         0.5, -0.5,  0.5,  0.0, -1.0,  0.0,  1.0,  0.0,
         0.5, -0.5,  0.5,  0.0, -1.0,  0.0,  1.0,  0.0,
        -0.5, -0.5,  0.5,  0.0, -1.0,  0.0,  0.0,  0.0,
        -0.5, -0.5, -0.5,  0.0, -1.0,  0.0,  0.0,  1.0,

        -0.5,  0.5, -0.5,  0.0,  1.0,  0.0,  0.0,  1.0,
         0.5,  0.5, -0.5,  0.0,  1.0,  0.0,  1.0,  1.0,
         0.5,  0.5,  0.5,  0.0,  1.0,  0.0,  1.0,  0.0,
         0.5,  0.5,  0.5,  0.0,  1.0,  0.0,  1.0,  0.0,
        -0.5,  0.5,  0.5,  0.0,  1.0,  0.0,  0.0,  0.0,
        -0.5,  0.5, -0.5,  0.0,  1.0,  0.0,  0.0,  1.0,
    ];

    // positions all containers
    let cube_positions: [glm::Vec3; 10] = [
        glm::vec3( 0.0,  0.0,  0.0),
        glm::vec3( 2.0,  5.0, -15.0),
        glm::vec3(-1.5, -2.2, -2.5),
        glm::vec3(-3.8, -2.0, -12.3),
        glm::vec3( 2.4, -0.4, -3.5),
        glm::vec3(-1.7,  3.0, -7.5),
        glm::vec3( 1.3, -2.0, -2.5),
        glm::vec3( 1.5,  2.0, -2.5),
        glm::vec3( 1.5,  0.2, -1.5),
        glm::vec3(-1.3,  1.0, -1.5)
    ];

    // positions of the point lights
    let point_light_positions: [glm::Vec3; 4] = [
        glm::vec3( 0.7,  0.2,  2.0),
        glm::vec3( 2.3, -3.3, -4.0),
        glm::vec3(-4.0,  2.0, -12.0),
        glm::vec3( 0.0,  0.0, -3.0)
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
        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, (8 * mem::size_of::<GLfloat>()) as GLsizei, std::ptr::null::<GLvoid>());
        gl::EnableVertexAttribArray(0);
        gl::VertexAttribPointer(1, 3, gl::FLOAT, gl::FALSE, (8 * mem::size_of::<GLfloat>()) as GLsizei, (3 * mem::size_of::<GLfloat>()) as *const GLvoid);
        gl::EnableVertexAttribArray(1);
        gl::VertexAttribPointer(2, 2, gl::FLOAT, gl::FALSE, (8 * mem::size_of::<GLfloat>()) as GLsizei, (6 * mem::size_of::<GLfloat>()) as *const GLvoid);
        gl::EnableVertexAttribArray(2);

        // second, configure the light's VAO (VBO stays the same; the vertices are the same for the light object which is also a 3D cube)
        gl::GenVertexArrays(1, &mut light_cube_vao);
        gl::BindVertexArray(light_cube_vao);

        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        // note that we update the lamp's position attribute's stride to reflect the updated buffer data
        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, (8 * mem::size_of::<GLfloat>()) as GLsizei, std::ptr::null::<GLvoid>());
        gl::EnableVertexAttribArray(0);
    }

    // load textures (we now use a utility function to keep the code more organized)
    // -----------------------------------------------------------------------------
    let diffuse_map = load_texture("resources/textures/container2.png");
    let specular_map = load_texture("resources/textures/container2_specular.png");

    // shader configuration
    // --------------------
    lighting_shader.use_program();
    lighting_shader.set_int("material.diffuse", 0);
    lighting_shader.set_int("material.specular", 1);

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
            lighting_shader.use_program();
            lighting_shader.set_vec3v("viewPos", &camera.position);
            lighting_shader.set_float("material.shininess", 32.0);

            /*
            Here we set all the uniforms for the 5/6 types of lights we have. We have to set them manually and index
            the proper PointLight struct in the array to set each uniform variable. This can be done more code-friendly
            by defining light types as classes and set their values in there, or by using a more efficient uniform approach
            by using 'Uniform buffer objects', but that is something we'll discuss in the 'Advanced GLSL' tutorial.
            */
            // directional light
            lighting_shader.set_vec3("dirLight.direction", -0.2, -1.0, -0.3);
            lighting_shader.set_vec3("dirLight.ambient", 0.05, 0.05, 0.05);
            lighting_shader.set_vec3("dirLight.diffuse", 0.4, 0.4, 0.4);
            lighting_shader.set_vec3("dirLight.specular", 0.5, 0.5, 0.5);
            // point light 1
            lighting_shader.set_vec3v("pointLights[0].position", &point_light_positions[0]);
            lighting_shader.set_vec3("pointLights[0].ambient", 0.05, 0.05, 0.05);
            lighting_shader.set_vec3("pointLights[0].diffuse", 0.8, 0.8, 0.8);
            lighting_shader.set_vec3("pointLights[0].specular", 1.0, 1.0, 1.0);
            lighting_shader.set_float("pointLights[0].constant", 1.0);
            lighting_shader.set_float("pointLights[0].linear", 0.09);
            lighting_shader.set_float("pointLights[0].quadratic", 0.032);
            // point light 2
            lighting_shader.set_vec3v("pointLights[1].position", &point_light_positions[1]);
            lighting_shader.set_vec3("pointLights[1].ambient", 0.05, 0.05, 0.05);
            lighting_shader.set_vec3("pointLights[1].diffuse", 0.8, 0.8, 0.8);
            lighting_shader.set_vec3("pointLights[1].specular", 1.0, 1.0, 1.0);
            lighting_shader.set_float("pointLights[1].constant", 1.0);
            lighting_shader.set_float("pointLights[1].linear", 0.09);
            lighting_shader.set_float("pointLights[1].quadratic", 0.032);
            // point light 3
            lighting_shader.set_vec3v("pointLights[2].position", &point_light_positions[2]);
            lighting_shader.set_vec3("pointLights[2].ambient", 0.05, 0.05, 0.05);
            lighting_shader.set_vec3("pointLights[2].diffuse", 0.8, 0.8, 0.8);
            lighting_shader.set_vec3("pointLights[2].specular", 1.0, 1.0, 1.0);
            lighting_shader.set_float("pointLights[2].constant", 1.0);
            lighting_shader.set_float("pointLights[2].linear", 0.09);
            lighting_shader.set_float("pointLights[2].quadratic", 0.032);
            // point light 4
            lighting_shader.set_vec3v("pointLights[3].position", &point_light_positions[3]);
            lighting_shader.set_vec3("pointLights[3].ambient", 0.05, 0.05, 0.05);
            lighting_shader.set_vec3("pointLights[3].diffuse", 0.8, 0.8, 0.8);
            lighting_shader.set_vec3("pointLights[3].specular", 1.0, 1.0, 1.0);
            lighting_shader.set_float("pointLights[3].constant", 1.0);
            lighting_shader.set_float("pointLights[3].linear", 0.09);
            lighting_shader.set_float("pointLights[3].quadratic", 0.032);
            // spotLight
            lighting_shader.set_vec3v("spotLight.position", &camera.position);
            lighting_shader.set_vec3v("spotLight.direction", &camera.front);
            lighting_shader.set_vec3("spotLight.ambient", 0.0, 0.0, 0.0);
            lighting_shader.set_vec3("spotLight.diffuse", 1.0, 1.0, 1.0);
            lighting_shader.set_vec3("spotLight.specular", 1.0, 1.0, 1.0);
            lighting_shader.set_float("spotLight.constant", 1.0);
            lighting_shader.set_float("spotLight.linear", 0.09);
            lighting_shader.set_float("spotLight.quadratic", 0.032);
            lighting_shader.set_float("spotLight.cutOff", 12.5_f32.to_radians().cos());
            lighting_shader.set_float("spotLight.outerCutOff",15.0_f32.to_radians().cos());

            // view/projection transformations
            let projection = glm::perspective(SCR_WIDTH as f32 / SCR_HEIGHT as f32, camera.zoom.to_radians(), 0.1, 100.0);
            let view = camera.get_view_matrix();
            lighting_shader.set_mat4("projection", &projection);
            lighting_shader.set_mat4("view", &view);

            // world transformation
            let mut model: glm::Mat4 = glm::identity();
            lighting_shader.set_mat4("model", &model);

            // bind diffuse map
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, diffuse_map);
            // bind specular map
            gl::ActiveTexture(gl::TEXTURE1);
            gl::BindTexture(gl::TEXTURE_2D, specular_map);

            // render containers
            gl::BindVertexArray(cube_vao);
            for (i, cube_position) in cube_positions.iter().enumerate() {
                // calculate the model matrix for each object and pass it to shader before drawing
                let mut model = glm::translate(&glm::identity(), cube_position);
                let angle = 20.0 * i as f32;
                model = glm::rotate(&model, angle.to_radians(), &glm::vec3(1.0, 0.3, 0.5));
                lighting_shader.set_mat4("model", &model);

                gl::DrawArrays(gl::TRIANGLES, 0, 36);
            }

            // also draw the lamp object(s)
            light_cube_shader.use_program();
            light_cube_shader.set_mat4("projection", &projection);
            light_cube_shader.set_mat4("view", &view);

            // we now draw as many light bulbs as we have point lights.
            gl::BindVertexArray(light_cube_vao);
            for point_light_position in point_light_positions.iter() {
                model = glm::translate(&glm::identity(), point_light_position);
                model = glm::scale(&model, &glm::vec3(0.2, 0.2, 0.2)); // Make it a smaller cube
                light_cube_shader.set_mat4("model", &model);
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

// utility function for loading a 2D texture from file
// ---------------------------------------------------
pub fn load_texture(path: &str) -> GLuint {
    let mut texture_id: GLuint = 0;

    unsafe { gl::GenTextures(1, &mut texture_id) };

    let image = image::open(path).expect("Texture failed to load at path: {path}");

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
