use image::{RgbImage, Rgb};
use std::f64::consts::PI;

#[inline]
fn mirror(v: i32, max: i32) -> i32 {
    // Mirror into [0, max)
    let period = 2 * max;
    let mut v = v.rem_euclid(period);
    if v >= max {
        v = period - v - 1;
    }
    v
}

pub fn kaleidoscope(
    img: &RgbImage,
    n: u32,
    out_img: Option<RgbImage>,
    r_start: f64,
    r_out: f64,
    c_in: Option<(u32, u32)>,
    c_out: Option<(u32, u32)>,
    scale: f64,
) -> RgbImage {
    let (in_cols, in_rows) = (img.width() as i32, img.height() as i32);
    let (c_x, c_y) = c_in.unwrap_or((in_cols as u32 / 2, in_rows as u32 / 2));

    // Maximum usable source radius (circular cutoff)
    let max_radius = (in_cols.min(in_rows) as f64) * 0.5;

    let r_start = r_start.rem_euclid(2.0 * PI);
    let width = PI / n as f64;

    let mut output = out_img.unwrap_or_else(|| {
        RgbImage::from_pixel(img.width(), img.height(), Rgb([0, 0, 0]))
    });

    let (out_cols, out_rows) = (output.width() as i32, output.height() as i32);
    let (co_x, co_y) = c_out.unwrap_or((output.width() / 2, output.height() / 2));

    for y in 0..out_rows {
        for x in 0..out_cols {
            let dx = x - co_x as i32;
            let dy = y - co_y as i32;

            let mag_p = ((dx * dx + dy * dy) as f64).sqrt() / scale;

            // Radial cutoff in source space
            if mag_p > max_radius {
              //  continue;
            }

            let theta_p =
                (((dx as f64).atan2(dy as f64) - r_out)
                    .rem_euclid(2.0 * width)
                    - width)
                    .abs()
                    + r_start;

            let src_x = (mag_p * theta_p.cos() + c_x as f64).round() as i32;
            let src_y = (mag_p * theta_p.sin() + c_y as f64).round() as i32;

            let src_x = mirror(src_x, in_cols);
            let src_y = mirror(src_y, in_rows);

            output.put_pixel(
                x as u32,
                y as u32,
                *img.get_pixel(src_x as u32, src_y as u32),
            );
        }
    }

    output
}
