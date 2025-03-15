#[macro_use]
mod macros;
mod traits;
pub use traits::SerdeType;

new_type! {
    struct Position {
        x: f32,
        y: f32,
        z: u32,
    }
}
