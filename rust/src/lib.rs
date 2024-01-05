use godot::prelude::*;
pub mod editor;
pub mod map;
struct Rust;

#[gdextension]
unsafe impl ExtensionLibrary for Rust {}
