extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, Texture, TextureCreator};
use sdl2::video::{Window, WindowContext};
use std::time::{Duration, SystemTime};
use std::thread::sleep;

//To make things easier to read, well create a constant which will be the testure's size
const TEXTURE_SIZE: u32 = 32;

#[derive(Clone, Copy)]
enum TextureColor {
    Green, 
    Blue
}

fn create_texture_rect<'a>(
    canvas: &mut Canvas<Window>,
    texture_creator: &'a TextureCreator<WindowContext>,
    color: TextureColor,
    size: u32
) -> Option<Texture<'a>> {
    //well want to handle failures outside of this function
    if let Ok(mut square_texture) = texture_creator.create_texture_target(None, size, size) {
        canvas.with_texture_canvas(&mut square_texture, |texture| {
            match color {
                // for now, TextureColor only handles two colors
                TextureColor::Green => texture.set_draw_color(Color::RGB(0, 255, 0)),
                TextureColor::Blue => texture.set_draw_color(Color::RGB(0, 0, 255)),
            }
            texture.clear();
        }).expect("Failed to color a texture");
        Some(square_texture)
    } else {
        // An error ocurred so we return nothing and let the function caller hanlde the error
        None
    }
}

fn main() {
    //Initialize an SDL context and get video subsystem
    let sdl_context = sdl2::init().expect("SDL initialization failed");
    let video_subsystem = sdl_context.video().expect("Couldnt get SDL Video subsytem");

    //create window
    let window = video_subsystem.window("Tetris", 800, 600)
        .position_centered()
        .build()
        .expect("Failed to create window");

    let mut canvas = window.into_canvas()
        .target_texture()
        .present_vsync()
        .build()
        .expect("Couldnt get windows canvas");

    let texture_creator: TextureCreator<_> = canvas.texture_creator();
    

    //we create a texture with a 32x32 size
    let mut green_square= create_texture_rect(
        &mut canvas, 
        &texture_creator, 
        TextureColor::Green,    
        TEXTURE_SIZE
    ).expect("Failed to create texture");

    let mut bluesquare= create_texture_rect(
        &mut canvas, 
        &texture_creator, 
        TextureColor::Blue,    
        TEXTURE_SIZE
    ).expect("Failed to create texture");

    let timer = SystemTime::now();

    // First we get the event handler
    let mut event_pump = sdl_context.event_pump().expect("Failed to cget SDL event pump");

    //If we try to run this without the loop the window will open and exit very quick

    //create an infinte loop to loop over events
    'running: loop {
        for event in event_pump.poll_iter() {
            match  event {
                //if we receive a 'quit' event or if the user presses 
                //the ESC key, we quit
                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), ..} => {
                    break 'running
                },
                _ => {}
            }
        }
        // we fill our window with red
        canvas.set_draw_color(Color::RGB(255, 0, 0));
        // we draw it
        canvas.clear();

        //the rectangle switch happens here:
        let display_green = match timer.elapsed() {
            Ok(elapsed) => elapsed.as_secs() % 2 == 0,
            Err(_) => true //In case of error we do nothing
        };

        let square_texture = if display_green {
            &green_square
        } else {
            &bluesquare
        };

        // copy our texture into the window
        canvas.copy(
            &square_texture, 
            None, 
            //we copy it at the top-left of the window with a 32x32 size
            Rect::new(0, 0, TEXTURE_SIZE, TEXTURE_SIZE)
        ).expect("Couldnt copy texture into window");
        //we update windows display
        canvas.present();


        //we sleep enough to get ~60 fps. If we dont call this the program
        // will take 100% of a CPU time
        sleep(Duration::new(0, 1_000_000_000u32) / 60);
    }
}


//Page 66, Playing with images