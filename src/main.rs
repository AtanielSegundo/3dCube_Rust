#[allow(dead_code)]
extern crate sdl2;
#[path = "./alias.rs"] mod alias;
use alias::*;
use sdl2::VideoSubsystem;
use sdl2::pixels::Color;
use sdl2::event::{Event, EventPollIterator};
use sdl2::keyboard::Keycode;
use sdl2::rect::{Point, Rect};
use sdl2::render::Canvas;
use std::f32::consts::PI;
use std::time::Duration;


pub fn main() {
    let WIDTH =  1280 as u16;
    let HEIGHT =  720 as u16;
    let pmaker = offset_maker{width:(WIDTH as i32),height:(HEIGHT as i32) };
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem:VideoSubsystem = sdl_context.video().unwrap();

    let window: sdl2::video::Window = video_subsystem.window(
        "3d Cube from scratch", WIDTH.into(), HEIGHT.into())
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut angle = 0.0 as f64;
    let mut x_rot = 0.0 as f64;
    let mut y_rot = 0.0 as f64;
    let mut z_rot = 0.0 as f64;
    let mut x_inc = 0.0 as f64;
    let mut y_inc = 0.0 as f64;
    let mut z_inc = 0.0 as f64;
    let inc = (PI as f64/180.0);
    //Main loop
    'running: loop {
        clear_canvas(&mut canvas);

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running;     
                }
                Event::KeyDown {keycode:Some(Keycode::W),..} => {
                    x_inc = x_inc + inc;
                }
                Event::KeyDown {keycode:Some(Keycode::S),..} => {
                    x_inc = x_inc - inc;
                }
                Event::KeyDown {keycode:Some(Keycode::Q),..} => {
                    z_inc = z_inc + inc;
                }
                Event::KeyDown {keycode:Some(Keycode::E),..} => {
                    z_inc = z_inc - inc;
                }
                Event::KeyDown {keycode:Some(Keycode::A),..} => {
                    y_inc = y_inc + inc;
                }
                Event::KeyDown {keycode:Some(Keycode::D),..} => {
                    y_inc = y_inc - inc;
                }
                Event::KeyDown {keycode:Some(Keycode::R),..} => {
                    x_rot = 0.0;
                    y_rot = 0.0;
                    z_rot = 0.0;
                    x_inc = 0.0;
                    y_inc = 0.0;
                    z_inc = 0.0;
                }
                Event::KeyDown {keycode:Some(Keycode::P),..} => {
                    x_inc = 0.0;
                    y_inc = 0.0;
                    z_inc = 0.0;
                }
                _ => {}
            }
        }
        x_rot = x_rot + x_inc;
        y_rot = y_rot + y_inc;
        z_rot = z_rot + z_inc;
        //pmaker.draw_axis(&mut canvas);
        
        let points_buffer = [
            (50,0,0),
            (-50,0,0),
            (50,100,0),
            (-50,100,0),
            (50,0,100),
            (-50,0,100),
            (50,100,100),
            (-50,100,100)
        ];
        
        let mut point_vec = Vec::new();
        
        for cord in points_buffer{
            point_vec.push(
                pmaker.point_maker(
                    ortho_norm(
                        rotation_3d(cord,x_rot,y_rot,z_rot),0.2)));
        }
        
        let r = 4.0;
        canvas.set_draw_color(Color::RED);
        
        for point in &point_vec{
            draw_circle(&mut canvas, point.clone(), r);
        }
        
        canvas.set_draw_color(Color::WHITE);
        canvas.draw_line(point_vec[0], point_vec[2]);
        canvas.draw_line(point_vec[1], point_vec[3]);
        canvas.draw_line(point_vec[0], point_vec[1]);
        canvas.draw_line(point_vec[2], point_vec[3]);
        canvas.draw_line(point_vec[0], point_vec[4]);
        canvas.draw_line(point_vec[1], point_vec[5]);
        canvas.draw_line(point_vec[2], point_vec[6]);
        canvas.draw_line(point_vec[3], point_vec[7]);
        canvas.draw_line(point_vec[4], point_vec[6]);
        canvas.draw_line(point_vec[5], point_vec[7]);
        canvas.draw_line(point_vec[4], point_vec[5]);
        canvas.draw_line(point_vec[6], point_vec[7]);
        
        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        angle = angle + (PI/180.0) as f64;
    }
    
}