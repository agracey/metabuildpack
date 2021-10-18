use crate::buildspec::Buildspec;

pub fn build(spec: Buildspec) {
    println!("Building {}", spec.name);
}
