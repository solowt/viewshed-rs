extern crate rand;
static NO_VALUE: f32 = std::f32::MAX;

// x, y point
#[derive(Debug)]
struct Point{
    x: i32,
    y: i32
}

// result of viewshed
#[allow(dead_code)]
struct ResultRaster{
    pixels: Vec<bool>,
    width: u32,
    x0: f64,
    y1: f64
}

// struct for a raster
#[allow(dead_code)]
struct Raster{
    pixels: Vec<f32>, // num pixels
    width: u32,
    x0: f64, // related to extent?  maybe corner in mercator
    y1: f64  // see above
}

// add methods to raster
#[allow(dead_code)]
impl Raster {

    // take an array[f32] and convert it to a vec[f32]
    fn array_to_vec(arr: &[f32]) -> Vec<f32> {
        arr.iter().cloned().collect()
    }

    fn new(source_raster: &[f32], width: u32, x0: f64, y1: f64) -> Raster{
        Raster{
            pixels: Raster::array_to_vec(source_raster),
            width: width,
            x0: x0,
            y1: y1
        }
    }

    #[allow(dead_code)]
    fn get_dist(&self, p1: &Point, p2: &Point) -> f64{
            (((p2.x-p1.x).pow(2) + (p2.y-p1.y).pow(2)) as f64).sqrt()
    }

    fn get_slope(&self, p1: Point, p2: Point) -> f64{
        let h1 = self.value_at(&p1);
        let h2 = self.value_at(&p2);
        println!("p1 height: {}, p2 height: {}", h1, h2);
        (h2-h1) as f64/self.get_dist(&p1,&p2)
    }

    #[allow(dead_code)]
    // return pixel value @ x,y
    fn value_at(&self, point: &Point) -> f32 {
        if self.pixels.len() > 0 {
            let idx: u32 = (self.width * point.y.abs() as u32) + point.x.abs() as u32;
            self.pixels[idx as usize]
        } else {
            // else return max val for f32
            NO_VALUE
        }
    }

    #[allow(dead_code)]
    // set raster source
    fn set_raster(&mut self, raster: &[f32]){
        self.pixels = Raster::array_to_vec(raster);
    }

    fn rand_raster() -> Raster{
        Raster::new(&Raster::rand_raster_source(), 255 as u32, rand::random::<f64>(), rand::random::<f64>())
    }

    fn rand_raster_source() -> [f32;65_000]{
        let mut arr: [f32;65_000] = [0.0;65_000];
        for i in 0..arr.len() {
            arr[i] = rand::random::<f32>() * 1_000.0;
        }
        arr
    }

    // bresenham's line algorithm http://tech-algorithm.com/articles/drawing-line-using-bresenham-algorithm/
    // give two points draw a line between them.  return vector of points as the line
    fn draw_line(p1: Point, p2: Point) -> Vec<Point>{
        let mut ret_vec = Vec::new();
        let delta_x: i32 = p2.x - p1.x;
        let delta_y: i32 = p2.y - p1.y;

        let dx1: i32 = if delta_x < 0 { -1 } else { 1 };
        let dy1: i32 = if delta_y < 0 { -1 } else { 1 };
        let mut dx2: i32 = if delta_x < 0 { -1 } else { 1 };
        let mut dy2: i32 = 0;

        let mut longest = delta_x.abs();
        let mut shortest = delta_y.abs();

        if !(longest>shortest){
            longest = delta_y.abs();
            shortest = delta_x.abs();
            if delta_y < 0 { dy2 = -1 } else { dy2 = 1 }
            dx2 = 0;
        }

        let mut numerator = longest >> 1;

        let mut curr_x = p1.x;
        let mut curr_y = p1.y;

        for iter in 0..longest+1 {
            ret_vec.push(Point{ x:curr_x, y: curr_y });
            numerator += shortest;
            if !(numerator < longest){
                numerator -= longest;
                curr_x += dx1;
                curr_y += dy1;
            } else {
                curr_x += dx2;
                curr_y += dy2;
            }
        }
        ret_vec
    }

    fn draw_circle(mid_point: Point, radius: u32) -> Vec<Point>{
        let mut ret_vec = Vec::new();

        let mut x: i32 = radius as i32;
        let mut y: i32 = 0;
        let mut err = 0;

        while x >= y {

            ret_vec.push(Point{x: mid_point.x + x, y: mid_point.y + y});
            if y != 0 { ret_vec.push(Point{x: mid_point.x + x, y: mid_point.y - y}); }

            ret_vec.push(Point{x: mid_point.x + y, y: mid_point.y + x});
            if y != 0 { ret_vec.push(Point{x: mid_point.x - y, y: mid_point.y + x}); }

            ret_vec.push(Point{x: mid_point.x - x, y: mid_point.y + y});
            if y != 0 { ret_vec.push(Point{x: mid_point.x - x, y: mid_point.y - y}); }

            ret_vec.push(Point{x: mid_point.x - y, y: mid_point.y - x});
            if y != 0 { ret_vec.push(Point{x: mid_point.x + y, y: mid_point.y - x}); }

            if err <= 0 {
                y += 1;
                err += 2*y + 1;
            }
            if err > 0 {
                x -= 1;
                err -= 2*x + 1;
            }
        }
        ret_vec
    }
}

fn main() {
    let a: Raster = Raster::rand_raster();
    // let height = a.value_at(Point{x:5,y:5});
    let line = Raster::draw_circle(Point{x:50,y:50}, 50);
    println!("{:?}",line);
}