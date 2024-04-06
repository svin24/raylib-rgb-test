use raylib::{color::Color, prelude::*};

fn main() {
    let (mut rl, thread) = raylib::init().size(640, 480).title("rgbeeee").build();

    rl.set_target_fps(120);

    let img = Image::load_image("resources/bee.png").expect("could not load image");
    let _ = rl
        .load_texture(&thread, "resources/bee.png")
        .expect("could not load texture bee");
    let marisa = rl
        .load_texture_from_image(&thread, &img)
        .expect("could not load texture from image");

    // start with red
    let mut red = 255;
    let mut green = 0;
    let mut blue = 0;

    while !rl.window_should_close() {
        if green < 255 && blue == 0 {
            green = green + 1;
        }
        if red > 0 && green == 255 {
            red = red - 1;
        }
        if blue < 255 && red == 0 {
            blue = blue + 1;
        }
        if green > 0 && blue == 255 {
            green = green - 1;
        }
        if red < 255 && green == 0 {
            red = red + 1;
        }
        if blue > 0 && red == 255 {
            blue = blue - 1;
        }

        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::new(red, green, blue, 255));

        d.draw_texture(&marisa, 350, 200, Color::WHITE);
        d.draw_text("bee", 12, 12, 60, Color::BLACK);
    }
}
