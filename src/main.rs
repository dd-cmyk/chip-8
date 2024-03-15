// Button import
use crate::piston::ButtonEvent;
// Imports for reading files
use std::fs::File;
use std::io;
use std::io::prelude::*;
// Import so I can read the timestamp of the window
use crate::piston::GenericEvent;
// Imports for adding the window
extern crate piston; // What does this do?
use piston::{RenderEvent, WindowSettings};
extern crate glutin_window;
use glutin_window::GlutinWindow;
use piston::event_loop::{EventLoop, EventSettings, Events};
// Imports for adding the graphics
extern crate graphics;
extern crate opengl_graphics;
//Import for update handling
use crate::piston::UpdateEvent;

use array2d::{Array2D, Error};
use opengl_graphics::{GlGraphics, OpenGL};
mod cpu;
use cpu::Cpu;
mod fonts;
use cpu::{CHIP_8_HEIGHT, CHIP_8_WIDTH};
type Colour = [f32; 4];
const PIXEL_SIZE: f64 = 7.0;
const WHITE: Colour = [1.0; 4];
const BLACK: Colour = [0.0, 0.0, 0.0, 1.0];
const UPDATE_RATE: u64 = 10; // I need 500hz for the CPU
const FPS: u64 = 60; // How often I will refresh the display
fn read_rom(path: &str) -> Vec<u8> {
    let mut f = File::open(path).expect("Failed to open rom!");
    let mut buffer: Vec<u8> = vec![];
    f.read_to_end(&mut buffer).expect("Failed to read rom!");
    buffer
}

fn main() {
    let curr_rom: usize = 1;
    // Read the rom
    let rom_list: Vec<&str> = vec!["1-chip8-logo.ch8", "2-ibm-logo.ch8"];
    let cycles: Vec<u8> = vec![39, 20];
    let rom: Vec<u8> = read_rom(format!("roms/{}", rom_list[curr_rom]).as_str());
    let mut cpu = cpu::Cpu::new();
    cpu.load_rom(rom);
    start_game(cpu);
}
fn start_game(mut cpu: cpu::Cpu) {
    // Initialize settings
    let w_width: f64 = CHIP_8_WIDTH as f64 * PIXEL_SIZE;
    let w_height: f64 = CHIP_8_HEIGHT as f64 * PIXEL_SIZE;
    let settings = WindowSettings::new("Chip 8 Emulator", (w_width, w_height)).exit_on_esc(true);
    // Create window
    let mut window: GlutinWindow = settings.build().expect("Could not create window");
    // Create the event
    let mut event_settings = EventSettings::new();
    event_settings.ups = UPDATE_RATE; // 500
    event_settings.max_fps = FPS; // 60
    let mut events = Events::new(event_settings);
    // Initialize OpenGL
    let opengl = OpenGL::V3_2;
    let mut gl = GlGraphics::new(opengl);
    // Probe the system for events
    let mut update: u64 = 0;
    while let Some(e) = events.next(&mut window) {
        if let Some(x) = e.update_args() { // Every update equals one cpu cycle
            cpu.emulate_cycle();
        }

        if let Some(r) = e.render_args() {
            gl.draw(r.viewport(), |c, g| {
                graphics::clear(BLACK, g);
                // Draw all the pixels
                for x in 0..CHIP_8_WIDTH {
                    for y in 0..CHIP_8_HEIGHT {
                        let pos: [f64; 4] = [
                            PIXEL_SIZE * x as f64,
                            PIXEL_SIZE * y as f64,
                            PIXEL_SIZE * (x + 1) as f64,
                            PIXEL_SIZE * (y + 1) as f64,
                        ];
                        let colour;
                        if cpu.display[(x, y)] {
                            colour = BLACK;
                        } else {
                            colour = WHITE;
                        }
                        graphics::Rectangle::new(colour).draw(pos, &c.draw_state, c.transform, g);
                    }
                }
            });
        }
    }
}
