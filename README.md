# Slint WGPU Shader Demo

## **Project Structure**

```
slint-shader/
├── Cargo.toml                   # WGPU 26.0 + Slint + bytemuck dependencies
├── build.rs                     # Slint UI compilation
├── src/
│   ├── lib.rs                   # Module organization and exports
│   ├── main.rs                  # Application entry point and animation loop
│   ├── shaders.rs               # WGSL shader source code with time uniforms
│   └── renderer.rs              # WGPU rendering and animation management
├── ui/
│   └── wgpu_shader_app.slint    # Large viewer UI with selector buttons
└── README.md                    # This documentation
```


## **How to Use**

1. **Prerequisites**: Rust and Cargo installed
2. **Clone/Setup**: Navigate to project directory
3. **Run**: 
   ```bash
   cargo run
   ```