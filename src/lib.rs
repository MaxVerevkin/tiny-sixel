use std::io;
use std::io::Write;

/// ASCII Escape character
pub const ESC: u8 = 0x1b;

/// Sixel image
pub struct Sixel {
    width: usize,
    height: usize,
    buf: Vec<u16>,
}

/// Sixel's color varinats
#[derive(Debug, Clone, Copy)]
pub enum SixelColor {
    /// Sixel uses plain RGB where all components are given in percents, from `0` to `100`, which gives
    /// us about one million colors (101^3)
    Rgb(u8, u8, u8),
    /// In HLS lightness and saturation are given in percents (from `0` to `100`) whereas hue is given
    /// in degrees (from `0` to `360`). For details refer to the [SIXEL GRAPHICS EXTENSION chapter in a DEC manual](https://archive.org/details/bitsavers_decstandar0VideoSystemsReferenceManualDec91_74264381/page/n927/mode/2up).
    Hls(u16, u8, u8),
}

impl Sixel {
    /// Enter sixel mode
    pub fn init(output: &mut impl Write) -> io::Result<()> {
        // Write `^[Pq`
        output.write_all(&[ESC, b'P', b'q'])
    }

    /// Quit sixel mode
    pub fn deinit(output: &mut impl Write) -> io::Result<()> {
        // Write `^[\`
        output.write_all(&[ESC, b'\\'])
    }

    /// Set sixel's color register `code` to `color`
    ///
    /// None that your termainal must alreay be in sixel mode.
    pub fn init_color(output: &mut impl Write, code: u16, color: SixelColor) -> io::Result<()> {
        match color {
            SixelColor::Hls(h, l, s) => write!(output, "#{};{};{};{};{}", code, 1, h, l, s),
            SixelColor::Rgb(r, g, b) => write!(output, "#{};{};{};{};{}", code, 2, r, g, b),
        }
    }

    /// Create new sixel image
    ///
    /// Note that height must be divisible by six.
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

    /// Set pixel `(x, y)` to a color with code `color_code`
    ///
    /// Note that you have to init the color using `init_color` function.
    pub fn set(&mut self, x: usize, y: usize, color_code: u16) {
        self.buf[x + y * self.width] = color_code;
    }

    /// Get the color code of a pixel `(x, y)`
    pub fn get(&self, x: usize, y: usize) -> u16 {
        self.buf[x + y * self.width]
    }

    /// Print the image
    ///
    /// None that your termainal must alreay be in sixel mode.
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

    /// Return an array of colors' codes that apper on a given row
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
}
