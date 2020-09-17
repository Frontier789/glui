# glui
An OpenGL game engine built around an Entity Component System concept. Behaviour is implemented in systems, while data is stored in components exclusively.
The library also contains utility classes for OpenGL objects, such as FBOs, shaders and textures and a modular, declaratve GUI system.

I have created glui to be a framework for my OpenGL demos/assignments.

For a GUI example, see `src\bin.rs`

## Related
[Skyrace](https://github.com/frontier789/skyrace), my computer graphics assignment for University of Trento.

## Dependencies
glutin for windowing
image for image loading
rusttype for font loading and caching
