use std::io::Write;
use ray_trace::vec3::Vec3;

fn main() {
    let output_file = std::fs::File::create("output.ppm").unwrap();
    let mut writer = std::io::BufWriter::new(output_file);

    // output the PPM header
    writer.write_all(b"P3\n256 256\n255\n").unwrap();
    for col in 0..=255 {
        for row in 0..=255 {
            let pixel = Vec3::new(
                col as f64 / 255.0,
                row as f64 / 255.0,
                0.25
            );
            writer.write_all(pixel.write_ppm().as_bytes()).unwrap();
        }
    }
}