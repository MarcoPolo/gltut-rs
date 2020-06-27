#![allow(non_upper_case_globals)]
extern crate glfw;
use self::glfw::{Action, Context, Key};

use crate::shader::Shader;

extern crate gl;
use self::gl::types::*;

use cgmath::Matrix;
use std::ffi::CString;
use std::mem;
use std::os::raw::c_void;
use std::ptr;
use std::str;
use std::sync::mpsc::Receiver;

// settings
const SCR_WIDTH: u32 = 800;
const SCR_HEIGHT: u32 = 600;

// set up vertex data (and buffer(s)) and configure vertex attributes
// ------------------------------------------------------------------
// HINT: type annotation is crucial since default for float literals is f64
#[rustfmt::skip]
const vertices: [f32; 288] = [
    0.25f32,  0.25f32, -1.25f32, 1.0f32,
    0.25f32, -0.25f32, -1.25f32, 1.0f32,
   -0.25f32,  0.25f32, -1.25f32, 1.0f32,

    0.25f32, -0.25f32, -1.25f32, 1.0f32,
   -0.25f32, -0.25f32, -1.25f32, 1.0f32,
   -0.25f32,  0.25f32, -1.25f32, 1.0f32,

    0.25f32,  0.25f32, -2.75f32, 1.0f32,
   -0.25f32,  0.25f32, -2.75f32, 1.0f32,
    0.25f32, -0.25f32, -2.75f32, 1.0f32,

    0.25f32, -0.25f32, -2.75f32, 1.0f32,
   -0.25f32,  0.25f32, -2.75f32, 1.0f32,
   -0.25f32, -0.25f32, -2.75f32, 1.0f32,

   -0.25f32,  0.25f32, -1.25f32, 1.0f32,
   -0.25f32, -0.25f32, -1.25f32, 1.0f32,
   -0.25f32, -0.25f32, -2.75f32, 1.0f32,

   -0.25f32,  0.25f32, -1.25f32, 1.0f32,
   -0.25f32, -0.25f32, -2.75f32, 1.0f32,
   -0.25f32,  0.25f32, -2.75f32, 1.0f32,

    0.25f32,  0.25f32, -1.25f32, 1.0f32,
    0.25f32, -0.25f32, -2.75f32, 1.0f32,
    0.25f32, -0.25f32, -1.25f32, 1.0f32,

    0.25f32,  0.25f32, -1.25f32, 1.0f32,
    0.25f32,  0.25f32, -2.75f32, 1.0f32,
    0.25f32, -0.25f32, -2.75f32, 1.0f32,

    0.25f32,  0.25f32, -2.75f32, 1.0f32,
    0.25f32,  0.25f32, -1.25f32, 1.0f32,
   -0.25f32,  0.25f32, -1.25f32, 1.0f32,

    0.25f32,  0.25f32, -2.75f32, 1.0f32,
   -0.25f32,  0.25f32, -1.25f32, 1.0f32,
   -0.25f32,  0.25f32, -2.75f32, 1.0f32,

    0.25f32, -0.25f32, -2.75f32, 1.0f32,
   -0.25f32, -0.25f32, -1.25f32, 1.0f32,
    0.25f32, -0.25f32, -1.25f32, 1.0f32,

    0.25f32, -0.25f32, -2.75f32, 1.0f32,
   -0.25f32, -0.25f32, -2.75f32, 1.0f32,
   -0.25f32, -0.25f32, -1.25f32, 1.0f32,




   0.0f32, 0.0f32, 1.0f32, 1.0f32,
   0.0f32, 0.0f32, 1.0f32, 1.0f32,
   0.0f32, 0.0f32, 1.0f32, 1.0f32,

   0.0f32, 0.0f32, 1.0f32, 1.0f32,
   0.0f32, 0.0f32, 1.0f32, 1.0f32,
   0.0f32, 0.0f32, 1.0f32, 1.0f32,

   0.8f32, 0.8f32, 0.8f32, 1.0f32,
   0.8f32, 0.8f32, 0.8f32, 1.0f32,
   0.8f32, 0.8f32, 0.8f32, 1.0f32,

   0.8f32, 0.8f32, 0.8f32, 1.0f32,
   0.8f32, 0.8f32, 0.8f32, 1.0f32,
   0.8f32, 0.8f32, 0.8f32, 1.0f32,

   0.0f32, 1.0f32, 0.0f32, 1.0f32,
   0.0f32, 1.0f32, 0.0f32, 1.0f32,
   0.0f32, 1.0f32, 0.0f32, 1.0f32,

   0.0f32, 1.0f32, 0.0f32, 1.0f32,
   0.0f32, 1.0f32, 0.0f32, 1.0f32,
   0.0f32, 1.0f32, 0.0f32, 1.0f32,

   0.5f32, 0.5f32, 0.0f32, 1.0f32,
   0.5f32, 0.5f32, 0.0f32, 1.0f32,
   0.5f32, 0.5f32, 0.0f32, 1.0f32,

   0.5f32, 0.5f32, 0.0f32, 1.0f32,
   0.5f32, 0.5f32, 0.0f32, 1.0f32,
   0.5f32, 0.5f32, 0.0f32, 1.0f32,

   1.0f32, 0.0f32, 0.0f32, 1.0f32,
   1.0f32, 0.0f32, 0.0f32, 1.0f32,
   1.0f32, 0.0f32, 0.0f32, 1.0f32,

   1.0f32, 0.0f32, 0.0f32, 1.0f32,
   1.0f32, 0.0f32, 0.0f32, 1.0f32,
   1.0f32, 0.0f32, 0.0f32, 1.0f32,

   0.0f32, 1.0f32, 1.0f32, 1.0f32,
   0.0f32, 1.0f32, 1.0f32, 1.0f32,
   0.0f32, 1.0f32, 1.0f32, 1.0f32,

   0.0f32, 1.0f32, 1.0f32, 1.0f32,
   0.0f32, 1.0f32, 1.0f32, 1.0f32,
   0.0f32, 1.0f32, 1.0f32, 1.0f32,
];

#[allow(non_snake_case)]
unsafe fn initialize_vertex_buffer() -> u32 {
    let mut VBO = 0;
    // Initialize Buffer Object
    gl::GenBuffers(1, &mut VBO);
    gl::BindBuffer(gl::ARRAY_BUFFER, VBO);
    gl::BufferData(
        gl::ARRAY_BUFFER,
        (vertices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
        &vertices[0] as *const f32 as *const c_void,
        gl::STREAM_DRAW,
    );
    gl::BindBuffer(gl::ARRAY_BUFFER, 0);
    // We've filled the GPU memory buffer with our vertex data, but opengl
    // doesn't know that's what is inside. Next step is to tell it.
    VBO
}

fn compute_position_offsets(glfw: &glfw::Glfw) -> (f32, f32) {
    let loop_duration = 5.0;
    let scale = std::f64::consts::PI / loop_duration;

    let elapsed_time = glfw.get_time();

    let curr_time_through_loop: f64 = (elapsed_time % (2f64 * loop_duration)) - loop_duration;
    // cgmath::BaseFloat::From(1.0f64);

    // (xOffset, yOffset)
    (
        (curr_time_through_loop * scale).cos() as f32,
        (curr_time_through_loop * scale).sin() as f32,
    )
}

#[allow(non_snake_case)]
pub fn main() {
    // glfw: initialize and configure
    // ------------------------------
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(
        glfw::OpenGlProfileHint::Core,
    ));
    #[cfg(target_os = "macos")]
    glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));

    // glfw window creation
    // --------------------
    let (mut window, events) = glfw
        .create_window(
            SCR_WIDTH,
            SCR_HEIGHT,
            "LearnOpenGL",
            glfw::WindowMode::Windowed,
        )
        .expect("Failed to create GLFW window");

    window.make_current();
    window.set_key_polling(true);
    window.set_framebuffer_size_polling(true);

    // gl: load all OpenGL function pointers
    // ---------------------------------------
    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

    let shader = Shader::new(
        "./src/ch_4/shaders/matrix_perspective.vs",
        "./src/ch_4/shaders/standard_color.fs",
    );

    let (shaderProgram, VAO, VBO) = unsafe {
        // link shaders
        let shaderProgram = shader.ID;

        let mut VAO = 0;
        gl::GenVertexArrays(1, &mut VAO);
        // bind the Vertex Array Object first, then bind and set vertex buffer(s), and then configure vertex attributes(s).
        gl::BindVertexArray(VAO);
        gl::Enable(gl::CULL_FACE);
        gl::CullFace(gl::BACK);
        gl::FrontFace(gl::CW);

        let VBO = initialize_vertex_buffer();

        // You can unbind the VAO afterwards so other VAO calls won't accidentally modify this VAO, but this rarely happens. Modifying other
        // VAOs requires a call to glBindVertexArray anyways so we generally don't unbind VAOs (nor VBOs) when it's not directly necessary.
        gl::BindVertexArray(0);

        // uncomment this call to draw in wireframe polygons.
        // gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);

        (shaderProgram, VAO, VBO)
    };

    unsafe {
        gl::UseProgram(shaderProgram);
        shader.setFloat(&CString::new("frustumScale").unwrap(), 1.0);
        shader.setFloat(&CString::new("zNear").unwrap(), 1.0);
        shader.setFloat(&CString::new("zFar").unwrap(), 3.0);

        let frustum_scale = 1.0f32;
        let z_near = 0.5f32;
        let z_far = 3.0f32;

        // Setup Perspective Matrix
        #[rustfmt::skip]
        let matrix = cgmath::Matrix4::new(
            frustum_scale, 0.0,           0.0,                               0.0,
            0.0,           frustum_scale, 0.0,                               0.0,
            0.0,           0.0,           (z_far + z_near)/(z_near - z_far), 2.0 * z_far * z_near / (z_near - z_far),
            0.0,           0.0,                                                    -1.0, 0.0
        ).transpose(); // Transpose because I wrote it in row major order, but it should be column major order
        shader.setMat4(&CString::new("perspectiveMatrix").unwrap(), &matrix);
        gl::UseProgram(0);
    }

    // render loop
    // -----------
    while !window.should_close() {
        // events
        // -----
        process_events(&mut window, &events);

        // render
        // ------
        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            // // draw our first triangle
            gl::UseProgram(shaderProgram);

            shader.set2F(&CString::new("offset").unwrap(), (0.5, 0.5));

            gl::BindVertexArray(VAO); // seeing as we only have a single VAO there's no need to bind it every time, but we'll do so to keep things a bit more organized

            let color_data = (vertices.len() * mem::size_of::<GLfloat>()) / 2;

            // Now we tell oepnGL what the format of the data is.
            gl::BindBuffer(gl::ARRAY_BUFFER, VBO);
            gl::EnableVertexAttribArray(0);
            gl::EnableVertexAttribArray(1);
            //                      vertex attribute index,  How many of these values represent a single
            //                      piece of data, What the data type is, ?, spacing
            //                      between data, the byte offset from the start
            gl::VertexAttribPointer(0, 4, gl::FLOAT, gl::FALSE, 0, ptr::null());
            gl::VertexAttribPointer(1, 4, gl::FLOAT, gl::FALSE, 0, color_data as *const c_void);

            gl::DrawArrays(gl::TRIANGLES, 0, 36);

            gl::DisableVertexAttribArray(0);
            gl::DisableVertexAttribArray(1);
            gl::UseProgram(0);

            // note that this is allowed, the call to gl::VertexAttribPointer registered VBO as the vertex attribute's bound vertex buffer object so afterwards we can safely unbind
            // gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            // gl::BindVertexArray(0);
        }

        // glfw: swap buffers and poll IO events (keys pressed/released, mouse moved etc.)
        // -------------------------------------------------------------------------------
        window.swap_buffers();
        glfw.poll_events();
    }
}

// NOTE: not the same version as in common.rs!
fn process_events(window: &mut glfw::Window, events: &Receiver<(f64, glfw::WindowEvent)>) {
    for (_, event) in glfw::flush_messages(events) {
        match event {
            glfw::WindowEvent::FramebufferSize(width, height) => {
                // make sure the viewport matches the new window dimensions; note that width and
                // height will be significantly larger than specified on retina displays.
                unsafe { gl::Viewport(0, 0, width, height) }
                // unsafe { gl::Viewport(width / 2, height / 2, width, height) }
                // unsafe { gl::Viewport(0, 0, width, height / 2) }
            }
            glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                window.set_should_close(true)
            }
            _ => {}
        }
    }
}
