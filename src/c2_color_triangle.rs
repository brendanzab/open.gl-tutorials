extern mod glfw;
extern mod glcore;

use glcore::*;

// Vertex data
static vertices: [GLfloat, ..15] = [
     0.0,  0.5,   1.0, 0.0, 0.0,
     0.5, -0.5,   0.0, 1.0, 0.0,
    -0.5, -0.5,   0.0, 0.0, 1.0
];

// Shader sources
static vertex_src: &'static str =
   "#version 150\n\
    in vec2 position;\n\
    in vec3 color;\n\
    out vec3 Color;\n\
    void main() {\n\
        Color = color;\n\
        gl_Position = vec4(position, 0.0, 1.0);\n\
    }";

static fragment_src: &'static str =
   "#version 150\n\
    in vec3 Color;\n\
    out vec4 outColor;\n\
    void main() {\n\
        outColor = vec4(Color, 1.0);\n\
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
                              5 * sys::size_of::<GLfloat>() as GLsizei,
                              ptr::null());
        
        let col_attrib = glGetAttribLocation(shader_program, str::as_c_str("color", |s|s)) as GLuint;
        glEnableVertexAttribArray(col_attrib);
        unsafe {
            glVertexAttribPointer(col_attrib, 3, GL_FLOAT, GL_FALSE,
                                  5 * sys::size_of::<GLfloat>() as GLsizei,
                                  cast::transmute(2 * sys::size_of::<GLfloat>() as uint));
        }
        
        while !window.should_close() {
            // Poll events
            glfw::poll_events();
            
            // Clear the screen to black
            glClearColor(0.1, 0.1, 0.1, 1.0);
            glClear(GL_COLOR_BUFFER_BIT);
        
            // Draw a triangle from the 3 vertices
            glDrawArrays(GL_TRIANGLES, 0, 3);
            
            // Swap buffers
            window.swap_buffers();
        }
        
        glDeleteProgram(shader_program);
        glDeleteShader(fragment_shader);
        glDeleteShader(vertex_shader);
        
        glDeleteBuffers(1, &vbo);
        
        glDeleteVertexArrays(1, &vao);
    }
}