//Stars  Program of dynamic art like a screen saver. Colors emerge from center of screen and do random walk
/*MIT License
Copyright (c) 2023 Darwin Geiselbrecht
Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:
The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.
THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
*/ 

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::{Rect};
use sdl2::video::Window;
use sdl2::render::Canvas;

use rand::Rng;
use std::time::Duration;

const X_MAX: u32= 2200;             // size of window in x direction
const Y_MAX: u32 = 1300;            // size of window in y direction
const SIZE: u32 = 5;                // size of each critter
const NUM_CRITTERS:usize = 200; 

#[derive(Clone,Copy)]
struct Critter {
    size: u32,
    x: f32,
    y: f32,
    color: Color,
}

// The critter that emerges from the center of the screen and does a random walk.
impl Critter {

    fn move_critter(&mut self, walls: &mut Walls) {

        // walk in one of 4 directions randomly    
        let mut rng = rand::thread_rng();
        let guess = rng.gen_range(0,4);
        match guess {
            0 => {self.x -= self.size as f32},
            1 => {self.x += self.size as f32},
            2 => {self.y -= self.size as f32},
            3 => {self.y += self.size as f32},
            _ => {}
        }

        // ensure we don't walk off screen
        if self.x >= (X_MAX - self.size) as f32 {                   // check right wsll
             self.x = (X_MAX - self.size) as f32 ;
             walls.right = true;
        }    

        if self.x <= 0.0 {                                          // check left wall
            self.x =  0.;
            walls.left = true;
        }

        if self.y >= (Y_MAX - self.size) as f32 {                   // check bottom wall
            self.y = (Y_MAX - self.size) as f32;
            walls.bottom = true;
        }
        if self.y <= 0.0 {                                          // check top wall
            self.y = 0.0;
            walls.top = true;
        }
    }
    // draws the critter as a filled rectangle
    fn draw(&mut self,canvas:&mut Canvas<Window>){     
        let x_pos:i32 = self.x as i32;
        let y_pos:i32 = self.y as i32;
        
        canvas.set_draw_color(self.color);               // redraw in updated location
        canvas.fill_rect(Rect::new(x_pos,y_pos,self.size as u32,self.size as u32)).unwrap(); 
    }
    
    
    // set the critter's color to a random hue
    fn random_color(&mut self) {
        let mut rng = rand::thread_rng();
        let red: u8 =rng.gen_range(0,255);
        let green: u8 =rng.gen_range(0,255);
        let blue: u8 =rng.gen_range(0,255);
        self.color = Color::RGB(red,green,blue);     
    }
    // reset the critter to the center of the screen and random color
    fn reset(&mut self) {
        
        self.x = X_MAX as f32/2.;
        self.y = Y_MAX as f32/2.;
        self.random_color();                       
    }
}

#[derive(Clone,Copy)]

// a structure to keep up with whhether or not a wall has been reached
struct Walls{
    top: bool,
    bottom:bool,
    left: bool,
    right:bool
}
impl Walls{
 
 // reset the wall flags to all false
    fn reset (&mut self){                                   
         self.top = false;
        self.bottom = false;
        self.left = false;
        self.right = false;
    }

// return a true if all walls have been touched
    fn check_all(&mut self) -> bool{

        let mut all_set: bool = false;
        
        if self.top && self.bottom && self.left && self.right{
            all_set = true;
        }
        all_set     
    }
}    

fn main() -> Result<(), String> {
    


    let sdl_context = sdl2::init()?;
        let video_subsystem = sdl_context.video()?;

    let window = video_subsystem.window("Emerge", X_MAX, Y_MAX)
        .position_centered()
        .build()
        .expect("could not initialize video subsystem");

    let mut canvas = window.into_canvas().present_vsync().build()
        .expect("could not make a canvas");

    let mut event_pump = sdl_context.event_pump()?;

    canvas.set_draw_color(Color::BLACK);  
    canvas.clear();

    let mut critters = Vec::with_capacity(NUM_CRITTERS);        // populate the critters vector
    for _i in 0 ..NUM_CRITTERS {                        
       critters.push(Critter {size:SIZE,x:X_MAX as f32/2.,y:Y_MAX as f32/2.,color:Color::BLACK});
     }

    for  _i in 0 .. critters.len(){                             // Randomize color
        critters[_i].random_color();                           
    }

    let mut walls = Walls{top:false,bottom:false,left:false,right:false};       // set up the walls flags

    let mut rng = rand::thread_rng();

    'running: loop {
        // Handle events
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running;
                },
                Event::KeyDown { keycode: Some(Keycode::Space), ..}  => {   // re-start the world on space

                    for _i in 0 .. NUM_CRITTERS{
                        critters[_i].reset();
                    }
                    walls.reset();
                    canvas.set_draw_color(Color::BLACK);  
                    canvas.clear();
                },
                _ => {}
            }
        }

        

  
        for  _i in 0 .. critters.len(){                                 // Give all critters a random chance to move

            let k =rng.gen_range(0,critters.len());                     // throw in entropy to equalize priority     
            critters[k].move_critter(&mut walls);
            critters[k].draw(&mut canvas);

            if walls.check_all() {                                        // if we have all the walls, restart
                ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 * 3));  // pause a bit before restarting
                for _i in 0 .. NUM_CRITTERS{
                    critters[_i].reset();
                }
                walls.reset();
                canvas.set_draw_color(Color::BLACK);  
                canvas.clear();
            }
         }

        canvas.present();

        // Time management! not needed, synced to vsync
        //::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    } //running loop

    Ok(())
}
