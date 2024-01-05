use godot::prelude::*;
mod editor;
mod map;
struct Rust;

#[gdextension]
unsafe impl ExtensionLibrary for Rust {}
