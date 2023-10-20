use std::fs::File;
use std::io::Write;

use crate::vec3::Vec3;

pub struct PPMWriter {
    writer: std::io::BufWriter<File>
}


impl PPMWriter {
    pub fn new(width: u32, height: u32, filename: &str) -> Result<Self, std::io::Error> {
        let file = File::create(filename)?;
        let mut writer = std::io::BufWriter::new(file);
        writer.write_all(format!("P3\n{} {}\n255\n", width, height).as_bytes())?;
        Ok(PPMWriter { writer })
    }

    pub fn write_pixel(&mut self, pixel: &Vec3) -> Result<(), std::io::Error> {
        self.writer.write_all(pixel.write_ppm().as_bytes())?;
        Ok(())
    }
}