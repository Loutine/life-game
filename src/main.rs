mod moc;
use grid::Grid;
use moc::MapOfCell;
use piston_window::*;
use rectangle::square;
use std::time::{SystemTime, UNIX_EPOCH};
const MILLIS_PER_FRAME: u128 = 80;
const SQUARE_SIZE: i32 = 15;
fn main() -> Result<(), String> {
    let (x, y): (i32, i32) = (70, 50);
    let mut life: MapOfCell = MapOfCell::new(x, y);
    let square_of = Rectangle::new(color::BLACK);
    life.random(1000);
    let map = Grid {
        cols: x as u32,
        rows: y as u32,
        units: SQUARE_SIZE as f64,
    };
    let line = Line {
        color: [0.2; 4],
        radius: 1.0,
        shape: line::Shape::Bevel,
    };

    let mut window: PistonWindow = WindowSettings::new(
        "the game of life",
        [(x * SQUARE_SIZE) as u32, (y * SQUARE_SIZE) as u32],
    )
    .exit_on_esc(true)
    .build()
    .unwrap();
    window.set_lazy(false);
    let mut previous_update = UNIX_EPOCH;
    while let Some(e) = window.next() {
        if previous_update
            .elapsed()
            .map(|d| d.as_millis())
            .unwrap_or(0)
            > MILLIS_PER_FRAME
        {
            // NOTE: Uncomment for timing info
            // let step_start = SystemTime::now();
            life.next();
            // println!("Step took: {}ms", step_start.elapsed().map(|d| d.as_micros()).unwrap_or(0) as f32 / 1000.0);
            previous_update = SystemTime::now();
        }

        window.draw_2d(&e, |c, g, _| {
            clear([1.0; 4], g);
            map.draw(&line, &draw_state::DrawState::new_alpha(), c.transform, g);
            let mut sx = 0;
            for row in life.cell_info.iter() {
                let mut sy = 0;
                for col in row.iter() {
                    if *col {
                        println!("{:?}", (sx, sy));
                        let cood = map.cell_position((sx, sy));
                        let dims = square(cood[0], cood[1], SQUARE_SIZE as f64);
                        square_of.draw(dims, &draw_state::DrawState::default(), c.transform, g);
                    }
                    sy += 1;
                }
                sx += 1;
            }
        });
    }
    Ok(())
}
