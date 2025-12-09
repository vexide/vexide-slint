//! Implementation of Slint's TargetPixel for the Color (BGR0) type used in
//! vexide.

use slint::platform::software_renderer::TargetPixel;
use vexide::color::Color;

#[repr(transparent)]
#[derive(Default, Debug, Clone, Copy, Eq, PartialEq, Hash, bytemuck::Zeroable, bytemuck::Pod)]
pub(crate) struct ColorPixel(pub Color);

impl TargetPixel for ColorPixel {
    fn blend(&mut self, color: slint::platform::software_renderer::PremultipliedRgbaColor) {
        let a = (u8::MAX - color.alpha) as u16;
        self.0.r = (self.0.r as u16 * a / 255) as u8 + color.red;
        self.0.g = (self.0.g as u16 * a / 255) as u8 + color.green;
        self.0.b = (self.0.b as u16 * a / 255) as u8 + color.blue;
    }

    fn from_rgb(red: u8, green: u8, blue: u8) -> Self {
        Self(Color::new(red, green, blue))
    }
}
