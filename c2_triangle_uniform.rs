extern mod glfw3;
extern mod glcore;

use cast::transmute;
use float::sin;
use ptr::{is_null, null, to_unsafe_ptr};
use str::as_c_str;
use sys::size_of;
use vec::raw::to_ptr;

use glcore::*;

fn macros() { include!("macros.rs"); }

fn main() {
    do task::task().sched_mode(task::PlatformThread).spawn {
        if (glfw3::init() == 0) {
            glfw3::terminate();
            fail(~"glfwInit() failed\n");
        }
        
        // Choose a GL profile that is compatible with OS X 10.7+
        glfw3::window_hint(glfw3::OPENGL_VERSION_MAJOR, 3);
        glfw3::window_hint(glfw3::OPENGL_VERSION_MINOR, 2);
        glfw3::window_hint(glfw3::OPENGL_PROFILE, glfw3::OPENGL_CORE_PROFILE);
        glfw3::window_hint(glfw3::OPENGL_FORWARD_COMPAT, 1);
        
        let mut window = glfw3::create_window(800, 600, glfw3::WINDOWED, ~"OpenGL");
        
        if (is_null(window.ptr)) {
            glfw3::terminate();
            io::println(~"Error: " + glfw3::error_string(glfw3::get_error()));
            fail(~"glfwOpenWindow() failed\n");
        }
        
        window.make_context_current();
        
        // Create Vertex Array Object
        let vao: GLuint = 0;
        glGenVertexArrays(1, to_unsafe_ptr(&vao));
        glBindVertexArray(vao);
        
        // Create a Vertex Buffer Object and copy the vertex data to it
        let vbo: GLuint = 0;
        glGenBuffers(1, to_unsafe_ptr(&vbo));
        
        let vertices = map_cast!(~[
             0.0,  0.5,
             0.5, -0.5,
            -0.5, -0.5
        ]: GLfloat);
        
        glBindBuffer(GL_ARRAY_BUFFER, vbo);
        
        unsafe {
            glBufferData(GL_ARRAY_BUFFER,
                         (vertices.len() * size_of::<GLfloat>()) as GLsizeiptr,
                         transmute(to_ptr(vertices)),
                         GL_STATIC_DRAW);
        }
        
        // Shader sources
        let vertexSource =
          ~"#version 150\n\
            in vec2 position;\n\
            void main() {\n\
               gl_Position = vec4(position, 0.0, 1.0);\n\
            }";

        let fragmentSource =
          ~"#version 150\n\
            out vec4 outColor;\n\
            uniform vec3 triangleColor;\n
            void main() {\n\
               outColor = vec4(triangleColor, 1.0);\n\
            }";

        // Create and compile the vertex shader
        let vertexShader = glCreateShader(GL_VERTEX_SHADER);
        do as_c_str(vertexSource) |data| {
            glShaderSource(vertexShader, 1, to_unsafe_ptr(&data), null());
            glCompileShader(vertexShader);
        }
        
        // Create and compile the fragment shader
        let fragmentShader = glCreateShader(GL_FRAGMENT_SHADER);
        do as_c_str(fragmentSource) |data| {
            glShaderSource(fragmentShader, 1, to_unsafe_ptr(&data), null());
            glCompileShader(fragmentShader);
        }
        
        // Link the vertex and fragment shader into a shader program
        let shaderProgram = glCreateProgram();
        glAttachShader(shaderProgram, vertexShader);
        glAttachShader(shaderProgram, fragmentShader);
        glBindFragDataLocation(shaderProgram, 0, as_c_str("outColor", |s| s));
        glLinkProgram(shaderProgram);
        glUseProgram(shaderProgram);
        
        // Specify the layout of the vertex data
        let posAttrib = glGetAttribLocation(shaderProgram, as_c_str("position", |s| s)) as GLuint;
        glEnableVertexAttribArray(posAttrib);
        glVertexAttribPointer(posAttrib, 2, GL_FLOAT, GL_FALSE, 0, null());

        // Get the location of the color uniform
        let uniColor = glGetUniformLocation(shaderProgram, as_c_str("triangleColor", |s| s));
        
        while window.get_param(glfw3::CLOSE_REQUESTED) == 0 {
            // Poll events
            glfw3::poll_events();
            
            // Set the color of the triangle
            let time = glfw3::get_time() as float;
            glUniform3f(uniColor,
                        ((sin(time * 4f) + 1f) / 2f) as GLfloat,
                        0.0 as GLfloat,
                        0.0 as GLfloat);
            
            // Clear the screen to black
            glClearColor(0.1 as GLfloat,
                         0.1 as GLfloat,
                         0.1 as GLfloat,
                         1.0 as GLfloat);
            glClear(GL_COLOR_BUFFER_BIT);
        
            // Draw a triangle from the 3 vertices
            glDrawArrays(GL_TRIANGLES, 0, 3);
            
            // Swap buffers
            window.swap_buffers();
        }
        
        glDeleteProgram(shaderProgram);
        glDeleteShader(fragmentShader);
        glDeleteShader(vertexShader);
        
        glDeleteBuffers(1, to_unsafe_ptr(&vbo));
        
        glDeleteVertexArrays(1, to_unsafe_ptr(&vao));
        
        glfw3::terminate();
    }
}