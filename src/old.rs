use crate::{HEIGHT, WIDTH};

pub fn next_step(game_array: [[bool; WIDTH]; HEIGHT]) -> [[bool; WIDTH]; HEIGHT] {
    let mut next_state = [[false; WIDTH]; HEIGHT];
    let mut neighbours: u8; // Will never be above 8
    const HEIGHT_I: isize = HEIGHT as isize;
    const WIDTH_I: isize = WIDTH as isize;
    const NEIGHBOUR_LIST: [[isize; 2]; 8] = [[-1, -1], [-1, 0], [-1, 1],
        [0, -1], [0, 1],
        [1, -1], [1, 0], [1, 1]];
    for rownum in 0..HEIGHT {
        for cellnum in 0..WIDTH {


            // FROM HERE
            neighbours = 0;
            for [j, k] in NEIGHBOUR_LIST {
                // This will break if width and height are set to larger than isize::MAX
                if game_array[(((rownum as isize + j % HEIGHT_I) + HEIGHT_I) % HEIGHT_I) as usize]
                    [(((cellnum as isize + k % WIDTH_I) + WIDTH_I) % WIDTH_I) as usize] {
                    neighbours += 1;
                }
            }
            // TO HERE


            // This is the cleanest way I could find to implement Life rules
            if neighbours == 3 || (game_array[rownum][cellnum] && neighbours == 2) {
                next_state[rownum][cellnum] = true;
            }
        }
    }
    next_state
}