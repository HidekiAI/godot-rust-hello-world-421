use godot::prelude::*;
mod my_rust_lib_player;

// CRITICAL: __MUST__ expose entry-point of GDExtension, so that we can avoid the error:
// >> "GDExtension entry point 'gdext_rust_init' not found in library ..."
// For more details, ee https://godot-rust.github.io/book/intro/hello-world.html?highlight=GDExtension#rust-entry-point
//#[gdextension]
//unsafe impl ExtensionLibrary for my_rust_lib_player::MyRustLibPlayer {}
