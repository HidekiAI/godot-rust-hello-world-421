# godot-rust-hello-world-421
hello world (combination of rust-godot rust-book and godot tutorial)

Key interest:
* How critical it was/is to expose extension entry-point - without out it,
you'd get "GDExtension entry point 'gdext_rust_init' not found in library ..."
error (thanks to couple folks on Discord for point this out)
* It's for 4.2.1, and tested ONLY on Windows (Debian is currently not booting,
so had to test only on Windows)

