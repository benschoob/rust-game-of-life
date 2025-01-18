mod life;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;
use life::Life;
use piston::{Button, MouseButton, MouseCursorEvent, PressEvent};
use piston::Key;

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    game: Life,     // Game of life representation
    sim: bool,      // Toggles the simulation on and off
    cell_size: f64, // Size of cells when rendered
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
        const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
        const GRAY:  [f32; 4] = [0.1, 0.1, 0.1, 1.0];

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(BLACK, gl);

            let transform = c.transform.trans(0.0, 0.0);

            for y in 0..self.game.width {
                for x in 0..self.game.height {
                    // Draw vertical gridlines
                    line(GRAY, 1.0, [x as f64 * self.cell_size, 0.0, x as f64 * self.cell_size, 1000.0], transform, gl);

                    // Draw the cell if it is true
                    if self.game.cell_at(x, y) {
                        let r = rectangle::rectangle_by_corners(
                            (x as f64) * self.cell_size, 
                            (y as f64) * self.cell_size,
                            (x as f64) * self.cell_size + self.cell_size,
                            (y as f64) * self.cell_size + self.cell_size
                        );
                        rectangle(WHITE, r, transform, gl);
                    }
                }
                // Draw horizontal gridlines
                line(GRAY, 1.0, [0.0, y as f64 * self.cell_size, 1000.0, y as f64 * self.cell_size], transform, gl);
            }
        });
    }

    fn update(&mut self, _args: &UpdateArgs) {
        if self.sim {
            self.game.update();
        }
    }
}

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create a Glutin window.
    let mut window: Window = WindowSettings::new("Game of Life", [1000, 1000])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
        game: Life::new(100, 100),
        sim: false,
        cell_size: 10.0
    };

    let mut mouse_x: f64 = 0.0;
    let mut mouse_y: f64 = 0.0;

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        // Update mouse position
        if let Some(mousepos_args) = e.mouse_cursor_args() {
            mouse_x = mousepos_args[0];
            mouse_y = mousepos_args[1];
        }
        // Get press events
        if let Some(press_args) = e.press_args() {
            match press_args {
                Button::Keyboard(Key::Space) => {
                    app.sim = !app.sim;
                }
                Button::Keyboard(Key::Backspace) => {
                    app.game.clear();
                }
                Button::Mouse(MouseButton::Left) => {
                    let x: usize = (mouse_x / app.cell_size) as usize;
                    let y: usize = (mouse_y / app.cell_size) as usize;
                    app.game.set_cell(x, y, !app.game.cell_at(x, y))
                }
                _ => {}
            }
        }

        if let Some(args) = e.render_args() {
            app.render(&args);
        }

        if let Some(args) = e.update_args() {
            app.update(&args);
        }
    }
}
