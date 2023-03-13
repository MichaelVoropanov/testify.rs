
#[derive(Copy, Clone)]
struct Coord(i32, i32);
struct Line(Coord,Coord);

impl Line {
    fn get_angle(center:Coord, point:Coord) -> f32 {
        #[allow(unused_variables)]
        #[allow(unused_assignments)]
        let mut angle:f32 = 0.0;
        let x = point.0 - center.0;
        let y = point.1 - center.1;
        if x == 0 {
            if y > 0 {
                angle = 90.0;
            } else {
                angle = 270.0;
            }
        } else if y == 0 {
            if x > 0 {
                angle = 0.0;
            } else {
                angle = 180.0;
            }
        } else {
            angle = (y as f32 / x as f32).atan() * 180.0 / std::f32::consts::PI;
            if x < 0 {
                angle += 180.0;
            } else if y < 0 {
                angle += 360.0;
            }
        }
        angle
    }
}

fn main() -> ! {
   let center:Coord = Coord(0,0);
   let mut point:Coord = Coord(1,10);
   loop{
       let angle1:f32 = Line::get_angle(center, point);
       println!("Angle: {}", angle1);
       let rng = rand::thread_rng();
       point.1 += rng.gen_range(-10..10);
       point.0 += rng.gen_range(-10..10);
}
}

