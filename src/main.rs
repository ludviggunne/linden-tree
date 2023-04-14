
mod linden_tree;
use linden_tree as lt;
use image;

const IMG_WIDTH:  u32 = 800;
const IMG_HEIGHT: u32 = 800;

fn main() {

    let buf = run_system(15);
    let mut buf_bounds = get_bounds(&buf);  
    buf_bounds.add_margins(10.0);

    let mut img = image::RgbImage::new(IMG_WIDTH, IMG_HEIGHT);
    let img_bounds = Bounds {
        xmin: 0.0,
        ymin: 0.0,
        xmax: IMG_WIDTH as f32,
        ymax: IMG_HEIGHT as f32,
    };
    for (_, _, pixel) in img.enumerate_pixels_mut() {
        *pixel = image::Rgb([255, 255, 255]);
    }

    for i0 in (0..buf.len()).filter(|i| i % 2 == 0) {
        
        let i1 = i0 + 1;

        let p0 = buf_bounds.map(&img_bounds, &buf[i0]); 
        let p1 = buf_bounds.map(&img_bounds, &buf[i1]); 

        draw_line(&mut img, p0, p1);
    }

    img.save_with_format("output.png", image::ImageFormat::Png).unwrap();
}

fn run_system(steps: usize) -> Vec<lt::Point> {

    let mut system = lt::System::new(|symb, output| {

        use lt::Symbol::*;

        match symb {

            Push => output.push(Push),
            Pop => output.push(Pop),
            Turn(a) => output.push(Turn(*a)),
            Translate(x) => output.push(Translate(*x * 1.55)),
            Generic(0) => {

                output.push(Push);
                output.push(Turn(-std::f64::consts::PI / 6.0));
                output.push(Translate(10.0));
                output.push(Generic(1));
                output.push(Pop);

                output.push(Push);
                output.push(Turn(std::f64::consts::PI / 4.0));
                output.push(Translate(15.0));
                output.push(Generic(0));
                output.push(Pop);
            },
            Generic(1) => {

                output.push(Push);
                output.push(Turn(-std::f64::consts::PI / 12.0));
                output.push(Translate(12.0));
                output.push(Generic(1));
                output.push(Pop);

                output.push(Push);
                output.push(Turn(std::f64::consts::PI / 8.0));
                output.push(Translate(45.0));
                output.push(Generic(0));
                output.push(Pop);
            }
            Generic(_) => panic!("Unknown Generic Symbol"),
        }
        true
    });

    system.init(vec![
        lt::Symbol::Generic(0)
    ]);

    for _ in 0..steps {
        system.step();
    }

    system.gen_vbuf()
}

pub fn draw_line(img: &mut image::RgbImage, p1: lt::Point, p2: lt::Point) {
 
    let dx = p2.x - p1.x;
    let dy = p2.y - p1.y;
    let length = dx.abs().max(dy.abs());

    let x_step = dx / length;
    let y_step = dy / length;

    let step_count = length as u32;

    for i in 0..step_count {

        let x = p1.x + x_step * (i as f32);
        let y = p1.y + y_step * (i as f32);
    
        img.put_pixel(
            x as u32,
            y as u32,
            image::Rgb([0, 0, 0]));
    }
}

struct Bounds {
    xmin: f32,
    ymin: f32,
    xmax: f32,
    ymax: f32,
}

impl Bounds {

    pub fn add_margins(&mut self, size: f32) {

        self.xmin -= size;
        self.ymin -= size;
        self.xmax += size;
        self.ymax += size;
    }

    pub fn map(&self, target: &Bounds, point: &lt::Point) -> lt::Point {

        let x = (target.xmax - target.xmin) * (point.x - self.xmin) / (self.xmax - self.xmin) + target.xmin;
        let y = (target.ymax - target.ymin) * (point.y - self.ymin) / (self.ymax - self.ymin) + target.ymin;

        lt::Point { x, y }
    }
}

fn get_bounds(points: &Vec<lt::Point>) -> Bounds {

    let mut bounds = Bounds {

        xmin: 0.0,
        ymin: 0.0,
        xmax: 0.0,
        ymax: 0.0,
    };

    for point in points {

        let x = point.x;
        let y = point.y;

        if x < bounds.xmin { 
            bounds.xmin = x;
        }
        if y < bounds.ymin { 
            bounds.ymin = y;
        }
        if x > bounds.xmax { 
            bounds.xmax = x;
        }
        if y > bounds.ymax { 
            bounds.ymax = y;
        }
    }

    bounds
}