extern crate gl;

mod shaders;
mod drawer;
mod color;
mod types;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
