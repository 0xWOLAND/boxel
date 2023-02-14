use rust_ca::run;

pub fn main() {
    pollster::block_on(run());
}
