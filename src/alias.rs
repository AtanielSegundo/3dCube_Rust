use sdl2::VideoSubsystem;
use sdl2::pixels::Color;
use sdl2::event::{Event, EventPollIterator};
use sdl2::keyboard::Keycode;
use sdl2::rect::Point;
use sdl2::render::Canvas;
use std::f32::consts::PI;
use std::time::Duration;

#[derive(Default)]
pub struct offset_maker{
    pub width: i32,
    pub height: i32
}
impl offset_maker{
    pub fn point_maker(&self,cords:(i32,i32)) -> Point{
       let trans = Point::new((cords.0+self.width)/2,(cords.1+self.height)/2);
       return  trans;
    }
    pub fn draw_axis(&self,canvas:&mut Canvas<sdl2::video::Window>){
        canvas.set_draw_color(Color::WHITE);
        canvas.draw_line(self.point_maker((-1*self.width,0)),self.point_maker((self.width,0)));
        canvas.draw_line(self.point_maker((0,-1*self.height )),self.point_maker((0,self.height)));
    }
}

pub fn quit_check(pool_iter:EventPollIterator) -> bool{
for event in pool_iter {
    match event {
        Event::Quit {..} |
        Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
            return true     
        },
        _ => {}
    }
}
return false
}

pub fn rect_make(center:(i32,i32,i32),side_len:i32) -> [(i32,i32,i32);8]{
    todo!()
}

pub fn rotation_2d(x:i32,y:i32,angle:f64) -> (i32,i32){
    let x_dot = (x as f64 * angle.cos()) as i32 - (y as f64 *angle.sin()) as i32;
    let y_dot =  (x as f64 * angle.sin()) as i32 + (y as f64*angle.cos()) as i32;

    (x_dot,y_dot)
}

pub fn ortho_norm(cords_3d:(i32,i32,i32),scale:f64) -> (i32,i32){
    ((cords_3d.0 as f64/scale) as i32 ,(cords_3d.1 as f64/scale) as i32)
}

pub fn rotation_3d_z(cords_3d:(i32,i32,i32),angle:f64) -> (i32,i32,i32){
    let x_dot = ((cords_3d.0 as f64 * angle.cos()) - (cords_3d.1 as f64 * angle.sin())) as i32;
    let y_dot = ((cords_3d.0 as f64 * angle.sin()) + (cords_3d.1 as f64 * angle.cos())) as i32;
    let z_dot = cords_3d.2;
    
    (x_dot,y_dot,z_dot)
}

pub fn rotation_3d_y(cords_3d:(i32,i32,i32),angle:f64) -> (i32,i32,i32){
    let x_dot = ((cords_3d.0 as f64 * angle.cos()) + (cords_3d.2 as f64 * angle.sin())) as i32;
    let y_dot = cords_3d.1;
    let z_dot = ((cords_3d.2 as f64 * angle.cos()) - (cords_3d.0 as f64 * angle.sin())) as i32;
    
    (x_dot,y_dot,z_dot)
}

pub fn rotation_3d_x(cords_3d:(i32,i32,i32),angle:f64) -> (i32,i32,i32){
    let x_dot = cords_3d.0;
    let y_dot = ((cords_3d.1 as f64 * angle.cos()) - (cords_3d.2 as f64 * angle.sin())) as i32;
    let z_dot = ((cords_3d.1 as f64 * angle.sin()) + (cords_3d.2 as f64 * angle.cos())) as i32;
    
    (x_dot,y_dot,z_dot)
}

pub fn rotation_3d(cords_3d:(i32,i32,i32),rot_x:f64,rot_y:f64,rot_z:f64) -> (i32,i32,i32){
    return rotation_3d_z(rotation_3d_y(rotation_3d_x(cords_3d,rot_x),rot_y),rot_z);
}

pub fn clear_canvas(canvas:&mut Canvas<sdl2::video::Window>) -> (){
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
}

pub fn draw_circle(canvas:&mut Canvas<sdl2::video::Window>,centroid:Point,ray:f64) -> (){
    let mut theta = 0.0 as f64;
    while theta < (2.0*PI) as f64{
        canvas.draw_line(centroid, 
                    Point::new(centroid.x() + (ray*theta.cos()) as i32
                         ,centroid.y() + (ray*theta.sin()) as i32));
        theta = theta + (PI/360.0) as f64;
    }
    return ();
    
}

