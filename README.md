# 3D Software Renderer in Rust
![alt showcase](https://s11.gifyu.com/images/ezgif.com-gif-makerf7d4858ecfe92130.gif)
Simple software renderer implemented in rust from ([pretty much](#Dependencies)) scratch without the use of the graphics API, with the goal to understand how does the GPU work.
# Features
- [x] Depth buffering
- [x] Face culling
- [x] Near and Viewport triangle clipping
- [x] Smooth shading
- [x] Camera
- [x] Flat triangle filling
- [x] Interpolated triangle filling
- [x] Affine texture mapping

### Dependencies:
* **glam** (`glam`  is a simple and fast linear algebra library for games and graphics).
* **SDL2** (used only to open up a window and present the pixel data).
