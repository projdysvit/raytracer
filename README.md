# Capstone project: Raytracer

## Overview
The Raytracer project is a GPU-accelerated ray tracing engine implemented in Rust using the wgpu graphics API.

## Preview
![image](https://i.imgur.com/vniH2aj.png)

## Key Features
- Real-time ray tracing
- GPU acceleration via wgpu
- Multiple sphere rendering
- Reflection and shading effects

## Technical Stack
- Language: Rust
- Graphics API: wgpu
- Shader Language: WGSL (WebGPU Shading Language)
- Window Management: winit

## Technologies Used
1. **Rust**: The primary programming language used for the project.
2. **wgpu**: A cross-platform, safe, pure-rust graphics API.
3. **winit**: A window creation and management library for Rust.
4. **WGSL** (WebGPU Shading Language): Used for writing the raytracing shader.
5. **anyhow**: For flexible error handling in Rust.
6. **pollster**: An async runtime for blocking on futures.
7. **bytemuck**: Used for casting between byte arrays and other types.
