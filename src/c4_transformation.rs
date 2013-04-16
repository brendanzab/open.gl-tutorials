extern mod glfw;
extern mod glcore;
extern mod lmath;
extern mod numeric;
extern mod stb_image;

use glcore::*;
use lmath::vec3::*;
use lmath::mat::*;
use lmath::quat::*;
use numeric::radians;
use stb_image::image::*;

// Vertex data
static vertices: [GLfloat, ..28] = [
//   Position     Color            Texcoords
    -0.5,  0.5,   1.0, 0.0, 0.0,   0.0, 0.0, // Top-left
     0.5,  0.5,   0.0, 1.0, 0.0,   1.0, 0.0, // Top-right
     0.5, -0.5,   0.0, 0.0, 1.0,   1.0, 1.0, // Bottom-right
    -0.5, -0.5,   1.0, 1.0, 1.0,   0.0, 1.0  // Bottom-left
];

static elements: [GLuint, ..6] = [
    0, 1, 2,
    2, 3, 0
];
        
// Shader sources
static vertex_src: &'static str =
   "#version 150\n\
    in vec2 position;\n\
    in vec3 color;\n\
    in vec2 texcoord;\n\
    out vec3 Color;\n\
    out vec2 Texcoord;\n\
    uniform mat4 trans;\n\
    void main() {\n\
        Color = color;\n\
        Texcoord = texcoord;\n\
        gl_Position = trans * vec4(position, 0.0, 1.0);\n\
    }";

static fragment_src: &'static str =
   "#version 150\n\
    in vec3 Color;\n\
    in vec2 Texcoord;\n\
    out vec4 outColor;\n\
    uniform sampler2D texKitten;\n\
    uniform sampler2D texPuppy;\n\
    void main() {\n\
        outColor = mix(texture(texKitten, Texcoord), texture(texPuppy, Texcoord), 0.5);\n\
    }";

fn main() {
    do glfw::spawn {
        
        // Choose a GL profile that is compatible with OS X 10.7+
        glfw::window_hint::context_version_major(3);
        glfw::window_hint::context_version_minor(2);
        glfw::window_hint::opengl_profile(glfw::OPENGL_CORE_PROFILE);
        glfw::window_hint::opengl_forward_compat(true);
        
        let window = glfw::Window::create(800, 600, "OpenGL", glfw::Windowed).unwrap();
        
        window.make_context_current();
        
        // Create Vertex Array Object
        let mut vao: GLuint = 0;
        glGenVertexArrays(1, &vao);
        glBindVertexArray(vao);
        
        // Create a Vertex Buffer Object and copy the vertex data to it
        let mut vbo: GLuint = 0;
        glGenBuffers(1, &vbo);
        glBindBuffer(GL_ARRAY_BUFFER, vbo);
        unsafe {
            glBufferData(GL_ARRAY_BUFFER,
                         (vertices.len() * sys::size_of::<GLfloat>()) as GLsizeiptr,
                         cast::transmute(&vertices[0]),
                         GL_STATIC_DRAW);
        }
        
        // Create an element array
        let mut ebo: GLuint = 0;
        glGenBuffers(1, &ebo);
        glBindBuffer(GL_ELEMENT_ARRAY_BUFFER, ebo);
        unsafe {
            glBufferData(GL_ELEMENT_ARRAY_BUFFER,
                         (elements.len() * sys::size_of::<GLfloat>()) as GLsizeiptr,
                         cast::transmute(&elements),
                         GL_STATIC_DRAW);
        }
        
        // Create and compile the vertex shader
        let vertex_shader = glCreateShader(GL_VERTEX_SHADER);
        glShaderSource(vertex_shader, 1, &str::as_c_str(vertex_src, |s|s), ptr::null());
        glCompileShader(vertex_shader);
        
        // Create and compile the fragment shader
        let fragment_shader = glCreateShader(GL_FRAGMENT_SHADER);
        glShaderSource(fragment_shader, 1, &str::as_c_str(fragment_src, |s|s), ptr::null());
        glCompileShader(fragment_shader);
        
        // Link the vertex and fragment shader into a shader program
        let shader_program = glCreateProgram();
        glAttachShader(shader_program, vertex_shader);
        glAttachShader(shader_program, fragment_shader);
        glBindFragDataLocation(shader_program, 0, str::as_c_str("outColor", |s|s));
        glLinkProgram(shader_program);
        glUseProgram(shader_program);
        
        // Specify the layout of the vertex data
        let pos_attrib = glGetAttribLocation(shader_program, str::as_c_str("position", |s|s)) as GLuint;
        glEnableVertexAttribArray(pos_attrib);
        glVertexAttribPointer(pos_attrib, 2, GL_FLOAT, GL_FALSE,
                              7 * sys::size_of::<GLfloat>() as GLsizei,
                              ptr::null());
        
        let col_attrib = glGetAttribLocation(shader_program, str::as_c_str("color", |s|s)) as GLuint;
        glEnableVertexAttribArray(col_attrib);
        unsafe {
            glVertexAttribPointer(col_attrib, 3, GL_FLOAT, GL_FALSE,
                                  7 * sys::size_of::<GLfloat>() as GLsizei,
                                  cast::transmute(2 * sys::size_of::<GLfloat>()));
        }
        
        let tex_attrib = glGetAttribLocation(shader_program, str::as_c_str("texcoord", |s|s)) as GLuint;
        glEnableVertexAttribArray(tex_attrib);
        unsafe {
            glVertexAttribPointer(tex_attrib, 2, GL_FLOAT, GL_FALSE,
                                  7 * sys::size_of::<GLfloat>() as GLsizei,
                                  cast::transmute(5 * sys::size_of::<GLfloat>()));
        }
        
        // Load textures
        let textures: ~[GLuint] = ~[0, 0];
        glGenTextures(2, &textures[0]);
        
        let kitten_loaded: bool;
        match load_with_depth(~"resources/sample.png", 3, false) {
            ImageU8(image) => {
                glActiveTexture(GL_TEXTURE0);
                glBindTexture(GL_TEXTURE_2D, textures[0]);
                
                glUniform1i(glGetUniformLocation(shader_program, str::as_c_str("texKitten", |s|s)), 0);
                
                unsafe {
                    glTexImage2D(
                        GL_TEXTURE_2D, 0,
                        GL_RGB as GLint,
                        image.width as GLsizei,
                        image.height as GLsizei,
                        0, GL_RGB, GL_UNSIGNED_BYTE,
                        cast::transmute(&image.data[0])
                    );
                }
                
                glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_S, GL_CLAMP_TO_EDGE as GLint);
                glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_T, GL_CLAMP_TO_EDGE as GLint);
                glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MIN_FILTER, GL_LINEAR as GLint);
                glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MAG_FILTER, GL_LINEAR as GLint);
                
                kitten_loaded = true;
            }
            _ => {
                io::println(~"Failed to load kitten.");
                kitten_loaded = false;
            }
        }
        
        let puppy_loaded: bool;
        match load_with_depth(~"resources/sample2.png", 3, false) {
            ImageU8(image) => {
                glActiveTexture(GL_TEXTURE1);
                glBindTexture(GL_TEXTURE_2D, textures[1]);
                
                glUniform1i(glGetUniformLocation(shader_program, str::as_c_str("texPuppy", |s|s)), 1);
                
                unsafe {
                    glTexImage2D(
                        GL_TEXTURE_2D, 0,
                        GL_RGB as GLint,
                        image.width as GLsizei,
                        image.height as GLsizei,
                        0, GL_RGB, GL_UNSIGNED_BYTE,
                        cast::transmute(&image.data[0])
                    );
                }
                
                glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_S, GL_CLAMP_TO_EDGE as GLint);
                glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_T, GL_CLAMP_TO_EDGE as GLint);
                glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MIN_FILTER, GL_LINEAR as GLint);
                glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MAG_FILTER, GL_LINEAR as GLint);
                
                puppy_loaded = true;
            }
            _ => {
                io::println(~"Failed to load puppy.");
                puppy_loaded = false;
            }
        }
        
        let uni_trans = glGetUniformLocation(shader_program, str::as_c_str("trans", |s|s));
        
        if kitten_loaded && puppy_loaded {
            while !window.should_close() {
                // Poll events
                glfw::poll_events();
                
                // Clear the screen to black
                glClearColor(0.1, 0.1, 0.1, 1.0);
                glClear(GL_COLOR_BUFFER_BIT);
                
                // Calculate transformation
                let trans = quat::from_angle_axis(
                    radians(glfw::get_time() * 180.0) as GLfloat,
                    &vec3::unit_z()
                ).to_mat3().to_mat4();
                
                // Set uniform to transform
                glUniformMatrix4fv(uni_trans, 1, GL_FALSE, trans.to_ptr());
            
                // Draw a rectangle from the 2 triangles using 6 indices
                glDrawElements(GL_TRIANGLES, 6, GL_UNSIGNED_INT, ptr::null());
                
                // Swap buffers
                window.swap_buffers();
            }
        }
        
        glDeleteTextures(2, &textures[0]);
        
        glDeleteProgram(shader_program);
        glDeleteShader(fragment_shader);
        glDeleteShader(vertex_shader);
        
        glDeleteBuffers(1, &vbo);
        
        glDeleteVertexArrays(1, &vao);
    }
}