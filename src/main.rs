mod sixel;

use std::io;

use crate::sixel::Sixel;

fn main() {
    // Example drawing
    let w = 600;
    let h = 600;
    let r = f64::hypot(w as f64, h as f64) / 2.;
    let ox = (w / 2) as f64;
    let oy = (h / 2) as f64;
    let mut sixel = Sixel::new(w, h).unwrap();
    for x in 0..w {
        for y in 0..h {
            let dx = ox - (x as f64);
            let dy = oy - (y as f64);
            let val = f64::hypot(dx, dy) / r * 360.;
            sixel.set(x, y, val as u16);
        }
    }

    let stdout = io::stdout();
    let mut stdout = stdout.lock();

    Sixel::init(&mut stdout).unwrap();
    Sixel::hls(&mut stdout).unwrap();
    sixel.print(&mut stdout).unwrap();
    Sixel::deinit(&mut stdout).unwrap();
}
