use godot::prelude::*;

// CRITICAL: __MUST__ expose entry-point of GDExtension, so that we can avoid the error:
// >> "GDExtension entry point 'gdext_rust_init' not found in library ..."
// For more details, ee https://godot-rust.github.io/book/intro/hello-world.html?highlight=GDExtension#rust-entry-point
#[gdextension]
unsafe impl ExtensionLibrary for my_rust_lib_player::MyRustLibPlayer {}

pub mod my_rust_lib_player {
    use godot::engine::Sprite2D;
    use godot::prelude::*;

    #[derive(GodotClass)]
    #[class(base=Sprite2D)]
    pub struct MyRustLibPlayer {
        speed: f64,
        angular_speed: f64,

        base: Base<Sprite2D>,
    }
    use godot::engine::ISprite2D;

    #[godot_api]
    impl ISprite2D for MyRustLibPlayer {
        fn init(base: Base<Sprite2D>) -> Self {
            godot_print!("my_rust_lib_player::MyRustLibPlayer::init(): Hello, world!"); // Prints to the Godot console

            Self {
                speed: 400.0,
                angular_speed: std::f64::consts::PI,
                base,
            }
        }

        fn physics_process(&mut self, delta: f64) {
            // In GDScript, this would be:
            // rotation += angular_speed * delta

            let radians = (self.angular_speed * delta) as f32;
            self.base_mut().rotate(radians);
            // The 'rotate' method requires a f32,
            // therefore we convert 'self.angular_speed * delta' which is a f64 to a f32
        }
    }
}
