use crate::new::State;

pub mod old;
pub mod new;

pub const WIDTH: usize = 10;
pub const HEIGHT: usize = 10;

pub fn print_state(state: State) {
    for row in 0..HEIGHT {
        for col in 0..WIDTH {
            if state[row][col] {
                print!("*");
            } else {
                print!(".");
            }
        }
        println!();
    }
}