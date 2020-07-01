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

const NUMBER_OF_VERTICES: i32 = 36;

const RIGHT_EXTENT: f32 = 0.8;
const LEFT_EXTENT: f32 = -1.0 * RIGHT_EXTENT;
const TOP_EXTENT: f32 = 0.20f32;
const MIDDLE_EXTENT: f32 = 0.0f32;
const BOTTOM_EXTENT: f32 = -1.0 * TOP_EXTENT;
const FRONT_EXTENT: f32 = -1.25f32;
const REAR_EXTENT: f32 = -1.75f32;

const GREEN_COLOR: (f32, f32, f32, f32) = (0.75, 0.75, 1.0, 1.0);
const BLUE_COLOR: (f32, f32, f32, f32) = (0.0, 0.5, 0.0, 1.0);
const RED_COLOR: (f32, f32, f32, f32) = (0.0, 0.0, 0.0, 1.0);
const GREY_COLOR: (f32, f32, f32, f32) = (0.8, 0.8, 0.8, 1.0);
const BROWN_COLOR: (f32, f32, f32, f32) = (0.5, 0.5, 0.0, 1.0);

// set up vertex data (and buffer(s)) and configure vertex attributes
// ------------------------------------------------------------------
// HINT: type annotation is crucial since default for float literals is f64
#[rustfmt::skip]
const vertices: [f32; 252] = [
  //Object 1 positions
	LEFT_EXTENT,	TOP_EXTENT,		REAR_EXTENT,
	LEFT_EXTENT,	MIDDLE_EXTENT,	FRONT_EXTENT,
	RIGHT_EXTENT,	MIDDLE_EXTENT,	FRONT_EXTENT,
	RIGHT_EXTENT,	TOP_EXTENT,		REAR_EXTENT,

	LEFT_EXTENT,	BOTTOM_EXTENT,	REAR_EXTENT,
	LEFT_EXTENT,	MIDDLE_EXTENT,	FRONT_EXTENT,
	RIGHT_EXTENT,	MIDDLE_EXTENT,	FRONT_EXTENT,
	RIGHT_EXTENT,	BOTTOM_EXTENT,	REAR_EXTENT,

	LEFT_EXTENT,	TOP_EXTENT,		REAR_EXTENT,
	LEFT_EXTENT,	MIDDLE_EXTENT,	FRONT_EXTENT,
	LEFT_EXTENT,	BOTTOM_EXTENT,	REAR_EXTENT,

	RIGHT_EXTENT,	TOP_EXTENT,		REAR_EXTENT,
	RIGHT_EXTENT,	MIDDLE_EXTENT,	FRONT_EXTENT,
	RIGHT_EXTENT,	BOTTOM_EXTENT,	REAR_EXTENT,

	LEFT_EXTENT,	BOTTOM_EXTENT,	REAR_EXTENT,
	LEFT_EXTENT,	TOP_EXTENT,		REAR_EXTENT,
	RIGHT_EXTENT,	TOP_EXTENT,		REAR_EXTENT,
	RIGHT_EXTENT,	BOTTOM_EXTENT,	REAR_EXTENT,

	//Object 2 positions
	TOP_EXTENT,		RIGHT_EXTENT,	REAR_EXTENT,
	MIDDLE_EXTENT,	RIGHT_EXTENT,	FRONT_EXTENT,
	MIDDLE_EXTENT,	LEFT_EXTENT,	FRONT_EXTENT,
	TOP_EXTENT,		LEFT_EXTENT,	REAR_EXTENT,

	BOTTOM_EXTENT,	RIGHT_EXTENT,	REAR_EXTENT,
	MIDDLE_EXTENT,	RIGHT_EXTENT,	FRONT_EXTENT,
	MIDDLE_EXTENT,	LEFT_EXTENT,	FRONT_EXTENT,
	BOTTOM_EXTENT,	LEFT_EXTENT,	REAR_EXTENT,

	TOP_EXTENT,		RIGHT_EXTENT,	REAR_EXTENT,
	MIDDLE_EXTENT,	RIGHT_EXTENT,	FRONT_EXTENT,
	BOTTOM_EXTENT,	RIGHT_EXTENT,	REAR_EXTENT,

	TOP_EXTENT,		LEFT_EXTENT,	REAR_EXTENT,
	MIDDLE_EXTENT,	LEFT_EXTENT,	FRONT_EXTENT,
	BOTTOM_EXTENT,	LEFT_EXTENT,	REAR_EXTENT,

	BOTTOM_EXTENT,	RIGHT_EXTENT,	REAR_EXTENT,
	TOP_EXTENT,		RIGHT_EXTENT,	REAR_EXTENT,
	TOP_EXTENT,		LEFT_EXTENT,	REAR_EXTENT,
	BOTTOM_EXTENT,	LEFT_EXTENT,	REAR_EXTENT,

	//Object 1 colors
	GREEN_COLOR.0, GREEN_COLOR.1, GREEN_COLOR.2, GREEN_COLOR.3,
	GREEN_COLOR.0, GREEN_COLOR.1, GREEN_COLOR.2, GREEN_COLOR.3,
	GREEN_COLOR.0, GREEN_COLOR.1, GREEN_COLOR.2, GREEN_COLOR.3,
	GREEN_COLOR.0, GREEN_COLOR.1, GREEN_COLOR.2, GREEN_COLOR.3,

	BLUE_COLOR.0, BLUE_COLOR.1, BLUE_COLOR.2, BLUE_COLOR.3,
	BLUE_COLOR.0, BLUE_COLOR.1, BLUE_COLOR.2, BLUE_COLOR.3,
	BLUE_COLOR.0, BLUE_COLOR.1, BLUE_COLOR.2, BLUE_COLOR.3,
	BLUE_COLOR.0, BLUE_COLOR.1, BLUE_COLOR.2, BLUE_COLOR.3,

	RED_COLOR.0, RED_COLOR.1, RED_COLOR.2, RED_COLOR.3,
	RED_COLOR.0, RED_COLOR.1, RED_COLOR.2, RED_COLOR.3,
	RED_COLOR.0, RED_COLOR.1, RED_COLOR.2, RED_COLOR.3,

	GREY_COLOR.0, GREY_COLOR.1, GREY_COLOR.2, GREY_COLOR.3,
	GREY_COLOR.0, GREY_COLOR.1, GREY_COLOR.2, GREY_COLOR.3,
	GREY_COLOR.0, GREY_COLOR.1, GREY_COLOR.2, GREY_COLOR.3,

	BROWN_COLOR.0, BROWN_COLOR.1, BROWN_COLOR.2, BROWN_COLOR.3,
	BROWN_COLOR.0, BROWN_COLOR.1, BROWN_COLOR.2, BROWN_COLOR.3,
	BROWN_COLOR.0, BROWN_COLOR.1, BROWN_COLOR.2, BROWN_COLOR.3,
	BROWN_COLOR.0, BROWN_COLOR.1, BROWN_COLOR.2, BROWN_COLOR.3,

	//Object 2 colors
	RED_COLOR.0, RED_COLOR.1, RED_COLOR.2, RED_COLOR.3,
	RED_COLOR.0, RED_COLOR.1, RED_COLOR.2, RED_COLOR.3,
	RED_COLOR.0, RED_COLOR.1, RED_COLOR.2, RED_COLOR.3,
	RED_COLOR.0, RED_COLOR.1, RED_COLOR.2, RED_COLOR.3,

	BROWN_COLOR.0, BROWN_COLOR.1, BROWN_COLOR.2, BROWN_COLOR.3,
	BROWN_COLOR.0, BROWN_COLOR.1, BROWN_COLOR.2, BROWN_COLOR.3,
	BROWN_COLOR.0, BROWN_COLOR.1, BROWN_COLOR.2, BROWN_COLOR.3,
	BROWN_COLOR.0, BROWN_COLOR.1, BROWN_COLOR.2, BROWN_COLOR.3,

	BLUE_COLOR.0, BLUE_COLOR.1, BLUE_COLOR.2, BLUE_COLOR.3,
	BLUE_COLOR.0, BLUE_COLOR.1, BLUE_COLOR.2, BLUE_COLOR.3,
	BLUE_COLOR.0, BLUE_COLOR.1, BLUE_COLOR.2, BLUE_COLOR.3,

	GREEN_COLOR.0, GREEN_COLOR.1, GREEN_COLOR.2, GREEN_COLOR.3,
	GREEN_COLOR.0, GREEN_COLOR.1, GREEN_COLOR.2, GREEN_COLOR.3,
	GREEN_COLOR.0, GREEN_COLOR.1, GREEN_COLOR.2, GREEN_COLOR.3,

	GREY_COLOR.0, GREY_COLOR.1, GREY_COLOR.2, GREY_COLOR.3,
	GREY_COLOR.0, GREY_COLOR.1, GREY_COLOR.2, GREY_COLOR.3,
	GREY_COLOR.0, GREY_COLOR.1, GREY_COLOR.2, GREY_COLOR.3,
	GREY_COLOR.0, GREY_COLOR.1, GREY_COLOR.2, GREY_COLOR.3,
];

#[rustfmt::skip]
const index_data: [GLushort; 24] = [
  0, 2, 1,
	3, 2, 0,

	4, 5, 6,
	6, 7, 4,

	8, 9, 10,
	11, 13, 12,

	14, 16, 15,
	17, 16, 14,
];

#[allow(non_snake_case)]
unsafe fn initialize_vertex_buffer() -> (u32, u32) {
    let mut VBO = 0;
    let mut index_buffer_object = 0;

    // Initialize Buffer Object
    gl::GenBuffers(1, &mut VBO);
    gl::BindBuffer(gl::ARRAY_BUFFER, VBO);
    gl::BufferData(
        gl::ARRAY_BUFFER,
        (vertices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
        &vertices[0] as *const f32 as *const c_void,
        gl::STATIC_DRAW,
    );
    gl::BindBuffer(gl::ARRAY_BUFFER, 0);

    // Initialize the index buffer
    gl::GenBuffers(1, &mut index_buffer_object);
    gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, index_buffer_object);
    // Copy the index data over
    gl::BufferData(
        gl::ELEMENT_ARRAY_BUFFER,
        (index_data.len() * mem::size_of::<GLushort>()) as GLsizeiptr,
        &index_data[0] as *const GLushort as *const c_void,
        gl::STATIC_DRAW,
    );
    gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);

    (VBO, index_buffer_object)
}

unsafe fn initialize_vertex_array_objects(VBO: u32, index_buffer_object: u32) -> (u32, u32) {
    let mut VAO_object_1 = 0;
    let mut VAO_object_2 = 0;

    gl::GenVertexArrays(1, &mut VAO_object_1);
    gl::BindVertexArray(VAO_object_1);

    let mut color_data_offset = mem::size_of::<f32>() * 3 * NUMBER_OF_VERTICES as usize;

    gl::BindBuffer(gl::ARRAY_BUFFER, VBO);
    gl::EnableVertexAttribArray(0);
    gl::EnableVertexAttribArray(1);
    gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 0, 0 as *const c_void);
    gl::VertexAttribPointer(
        1,
        4,
        gl::FLOAT,
        gl::FALSE,
        0,
        color_data_offset as *const c_void,
    );
    gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, index_buffer_object);

    gl::BindVertexArray(0);

    gl::GenVertexArrays(1, &mut VAO_object_2);
    gl::BindVertexArray(VAO_object_2);

    let pos_data_offset = mem::size_of::<f32>() * 3 * (NUMBER_OF_VERTICES >> 1) as usize;
    color_data_offset += mem::size_of::<f32>() * 4 * (NUMBER_OF_VERTICES >> 1) as usize;

    gl::EnableVertexAttribArray(0);
    gl::EnableVertexAttribArray(1);

    gl::VertexAttribPointer(
        0,
        3,
        gl::FLOAT,
        gl::FALSE,
        0,
        pos_data_offset as *const c_void,
    );
    gl::VertexAttribPointer(
        1,
        4,
        gl::FLOAT,
        gl::FALSE,
        0,
        color_data_offset as *const c_void,
    );
    gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, index_buffer_object);

    gl::BindVertexArray(0);

    (VAO_object_1, VAO_object_2)
}

unsafe fn reshape(
    width: i32,
    height: i32,
    frustum_scale: f32,
    perspective_matrix: &mut cgmath::Matrix4<f32>,
    program: &Shader,
) {
    perspective_matrix.x[0] = frustum_scale / (width as f32 / height as f32);
    perspective_matrix.y[1] = frustum_scale;
    gl::UseProgram(program.ID);
    program.setMat4(
        &CString::new("perspectiveMatrix").unwrap(),
        &perspective_matrix,
    );
    gl::UseProgram(0);
    gl::Viewport(0, 0, width, height);
}

unsafe fn display(shader: &Shader, VAO_1: u32, VAO_2: u32) {
    gl::ClearColor(0.2, 0.3, 0.3, 1.0);
    gl::Clear(gl::COLOR_BUFFER_BIT);

    gl::UseProgram(shader.ID);

    gl::BindVertexArray(VAO_1);
    shader.setVec3(&CString::new("offset").unwrap(), 0.0, 0.0, 0.0);
    gl::DrawElements(
        gl::TRIANGLES,
        index_data.len() as i32,
        gl::UNSIGNED_SHORT,
        0 as *const c_void,
    );

    gl::BindVertexArray(VAO_2);
    shader.setVec3(&CString::new("offset").unwrap(), 0.0, 0.0, -1.0);
    gl::DrawElements(
        gl::TRIANGLES,
        index_data.len() as i32,
        gl::UNSIGNED_SHORT,
        0 as *const c_void,
    );

    gl::BindVertexArray(0);
    gl::UseProgram(0);
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
        "./src/section_ii/shaders/standard.vs",
        "./src/section_ii/shaders/standard.fs",
    );

    let frustum_scale = 1.0f32;
    let z_near = 0.5f32;
    let z_far = 3.0f32;

    // Setup Perspective Matrix
    #[rustfmt::skip]
    let mut matrix = cgmath::Matrix4::new(
        frustum_scale, 0.0,           0.0,                               0.0,
        0.0,           frustum_scale, 0.0,                               0.0,
        0.0,           0.0,           (z_far + z_near)/(z_near - z_far), 2.0 * z_far * z_near / (z_near - z_far),
        0.0,           0.0,                                                    -1.0, 0.0
    ).transpose(); // Transpose because I wrote it in row major order, but it should be column major order

    let (shaderProgram, VAO_1, VAO_2, VBO) = unsafe {
        // link shaders
        let shaderProgram = shader.ID;

        let (VBO, index_buffer_object) = initialize_vertex_buffer();
        let (VAO_1, VAO_2) = initialize_vertex_array_objects(VBO, index_buffer_object);

        gl::Enable(gl::CULL_FACE);
        gl::CullFace(gl::BACK);
        gl::FrontFace(gl::CW);

        (shaderProgram, VAO_1, VAO_2, VBO)
    };

    unsafe {
        gl::UseProgram(shaderProgram);
        // shader.setFloat(&CString::new("frustumScale").unwrap(), 1.0);
        // shader.setFloat(&CString::new("zNear").unwrap(), 1.0);
        // shader.setFloat(&CString::new("zFar").unwrap(), 3.0);

        shader.setMat4(&CString::new("perspectiveMatrix").unwrap(), &matrix);
        gl::UseProgram(0);
    }

    // render loop
    // -----------
    while !window.should_close() {
        // events
        // -----
        process_events(&mut window, &events, &shader, frustum_scale, &mut matrix);

        // render
        // ------
        unsafe {
            display(&shader, VAO_1, VAO_2);
        }

        // glfw: swap buffers and poll IO events (keys pressed/released, mouse moved etc.)
        // -------------------------------------------------------------------------------
        window.swap_buffers();
        glfw.poll_events();
    }
}

// NOTE: not the same version as in common.rs!
fn process_events(
    window: &mut glfw::Window,
    events: &Receiver<(f64, glfw::WindowEvent)>,
    program: &Shader,
    frustum_scale: f32,
    perspective_matrix: &mut cgmath::Matrix4<f32>,
) {
    for (_, event) in glfw::flush_messages(events) {
        match event {
            glfw::WindowEvent::FramebufferSize(width, height) => {
                // make sure the viewport matches the new window dimensions; note that width and
                // height will be significantly larger than specified on retina displays.
                unsafe { reshape(width, height, frustum_scale, perspective_matrix, program) }
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
