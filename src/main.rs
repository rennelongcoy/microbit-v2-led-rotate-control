#![no_main]
#![no_std]

use panic_halt as _;

use cortex_m_rt::entry;
use embedded_hal::digital::InputPin;
use microbit::{
    board::Board,
    display::blocking::Display,
    hal::Timer,
};

const PIXELS: [(usize, usize); 16] = [
    (0,0), (0,1), (0,2), (0,3), (0,4), (1,4), (2,4), (3,4), (4,4),
    (4,3), (4,2), (4,1), (4,0), (3,0), (2,0), (1,0)
];

enum Direction {
    CLOCKWISE,
    COUNTERCLOCKWISE,
}

#[entry]
fn main() -> ! {
    let mut board: Board = Board::take().unwrap();
    let mut timer: Timer<nrf52833_pac::TIMER0> = Timer::new(board.TIMER0);
    let mut display: Display = Display::new(board.display_pins);
    let mut leds: [[u8; 5]; 5] = [
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
    ];

    let mut current_index: usize = 0;
    let mut last_index: usize = 0;
    let mut direction: Direction = Direction::CLOCKWISE;

    loop {
        leds[PIXELS[last_index].0][PIXELS[last_index].1] = 0;
        leds[PIXELS[current_index].0][PIXELS[current_index].1] = 1;
        display.show(&mut timer, leds, 30);
        last_index = current_index;

        if let Ok(true) = board.buttons.button_b.is_low() { // is_low == grounded == pressed
            direction = Direction::CLOCKWISE;
        } else if let Ok(true) = board.buttons.button_a.is_low() {
            direction = Direction::COUNTERCLOCKWISE;
        }

        match direction {
            Direction::CLOCKWISE => {
                current_index += 1;
                if current_index >= PIXELS.len() {
                    current_index = 0;
                }
            },
            Direction::COUNTERCLOCKWISE => {
                if current_index == 0 {
                    current_index = PIXELS.len() - 1;
                }
                else {
                    current_index -= 1;
                }
            }
        }
    }
}
