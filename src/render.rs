use image::{ImageBuffer, Rgb};
use imageproc::drawing::draw_text;
use rusttype::{Font, Scale};

pub fn render(count: u64) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let out = ImageBuffer::new(140, 30);

    let font_data: &[u8] = include_bytes!("../ComicMono-Bold.ttf");
    let font: Font<'static> = Font::try_from_bytes(font_data).unwrap();

    // draw_text(image, color, x, y, scale, font, text)
    let out = draw_text(
        &out,
        Rgb([255, 255, 255]),
        3,
        3,
        Scale { x: 30., y: 30. },
        &font,
        &format!("{count:09}"),
    );

    out
}
