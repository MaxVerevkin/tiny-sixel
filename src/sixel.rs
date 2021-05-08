use std::io::{self, Write};

pub const ESC: u8 = 0x1b;

pub struct Sixel {
    width: usize,
    height: usize,
    buf: Vec<u16>,
}

impl Sixel {
    pub fn new(width: usize, height: usize) -> Option<Self> {
        if height % 6 == 0 {
            Some(Self {
                width,
                height,
                buf: vec![0; width * height],
            })
        } else {
            None
        }
    }

    pub fn init(output: &mut impl Write) -> io::Result<()> {
        output.write_all(&[ESC, b'P', b'q'])
    }

    pub fn deinit(output: &mut impl Write) -> io::Result<()> {
        output.write_all(&[ESC, b'\\'])
    }

    pub fn hls(output: &mut impl Write) -> io::Result<()> {
        for i in 0..360 {
            //let col = i;
            let col = (i as f64 / 360. * 100.) as i32;
            output.write_all(format!("#{};1;{};50;100", i, col).as_bytes())?;
        }
        Ok(())
    }

    pub fn set(&mut self, x: usize, y: usize, color: u16) {
        self.buf[x + y * self.width] = color;
    }

    pub fn get(&self, x: usize, y: usize) -> u16 {
        self.buf[x + y * self.width]
    }

    fn row_colors(&self, row: usize) -> Vec<u16> {
        let mut colors = Vec::with_capacity(360);
        let base_y = row * 6;
        for y in base_y..(base_y + 6) {
            for x in 0..(self.width) {
                let color = self.get(x, y);
                if !colors.iter().any(|x| *x == color) {
                    colors.push(color);
                }
            }
        }
        colors
    }

    pub fn print(&self, output: &mut impl Write) -> io::Result<()> {
        // Row's buffer
        let mut buf = Vec::with_capacity(self.width);
        for row in 0..(self.height / 6) {
            // Get list of colors on given row
            let colors = self.row_colors(row);
            // Print all colors
            for color in colors {
                buf.clear();
                write!(buf, "#{}", color)?;
                let mut last_char = 0;
                let mut last_count = 0;
                for x in 0..(self.width) {
                    let mut ch = 63;
                    for i in 0..6 {
                        ch += ((color == self.get(x, row * 6 + i)) as u8) << i;
                    }
                    if ch == last_char {
                        last_count += 1;
                    } else {
                        if last_count > 2 {
                            write!(buf, "!{}", last_count)?;
                            buf.push(last_char);
                        } else if last_count == 2 {
                            buf.push(last_char);
                            buf.push(last_char);
                        } else if last_count == 1 {
                            buf.push(last_char);
                        }
                        last_char = ch;
                        last_count = 1;
                    }
                }
                if last_count > 2 {
                    write!(buf, "!{}", last_count)?;
                    buf.push(last_char);
                } else if last_count == 2 {
                    buf.push(last_char);
                    buf.push(last_char);
                } else if last_count == 1 {
                    buf.push(last_char);
                }
                buf.push(b'$');
                output.write_all(&buf)?;
            }
            output.write_all(&[b'-'])?;
        }
        Ok(())
    }
}
