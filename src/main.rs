extern crate rand;
static NO_VALUE: f32 = std::f32::MAX;
use std::cmp::*;

// x, y point
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
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

impl ResultRaster {
    fn new(result_vec: Vec<bool>, width:u32, x0: f64, y1: f64) -> ResultRaster {
        ResultRaster{
            pixels: result_vec,
            width: width,
            x0: x0,
            y1: y1
        }
    }
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

    fn get_slope(&self, p1: &Point, p2: &Point) -> f64{
        let h1 = self.value_at(p1);
        let h2 = self.value_at(p2);
        // println!("p1 height: {}, p2 height: {}", h1, h2);
        (h2-h1) as f64/self.get_dist(p1,p2)
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

    fn set_result(raster: &mut Vec<bool>, width: u32, point: &Point, value: bool){
        let idx: u32 = (width * point.y.abs() as u32) + point.x.abs() as u32;
        raster[idx as usize] = value;
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
        let mut last_height: f32 = 0.0;
        let mut arr: [f32; 65_000] = [0.0; 65_000];
        for i in 0..arr.len() {
            let pos_or_neg: f32 = if rand::random::<f32>() > 0.5 { 1.0 } else { -1.0 };
            let curr_height = last_height + rand::random::<f32>() * pos_or_neg;
            arr[i] = curr_height;
            last_height = curr_height;
        }
        arr
    }

    // bresenham's line algorithm http://tech-algorithm.com/articles/drawing-line-using-bresenham-algorithm/
    // give two points draw a line between them.  return vector of points as the line
    fn draw_line(p1: &Point, p2: &Point) -> Vec<Point>{

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

    fn draw_circle(mid_point: &Point, radius: u32) -> Vec<Point>{

        let mut ret_vec = Vec::new();

        let mut x: i32 = radius as i32;
        let mut y: i32 = 0;
        let mut err = 0;

        while x >= y {

            ret_vec.push(Point{x: mid_point.x + x, y: mid_point.y + y});
            ret_vec.push(Point{x: mid_point.x + x, y: mid_point.y - y});

            ret_vec.push(Point{x: mid_point.x + y, y: mid_point.y + x});
            ret_vec.push(Point{x: mid_point.x - y, y: mid_point.y + x});

            ret_vec.push(Point{x: mid_point.x - x, y: mid_point.y + y});
            ret_vec.push(Point{x: mid_point.x - x, y: mid_point.y - y});

            ret_vec.push(Point{x: mid_point.x - y, y: mid_point.y - x});
            ret_vec.push(Point{x: mid_point.x + y, y: mid_point.y - x});

            if err <= 0 {
                y += 1;
                err += 2*y + 1;
            }
            if err > 0 {
                x -= 1;
                err -= 2*x + 1;
            }
        }

        ret_vec.sort_by(|a,b|{
            if a.x == b.x && a.y == b.y {
                Ordering::Equal
            } else if a.y >= 0 {
                if b.y < 0 {
                    Ordering::Less
                } else {
                    if a.x != b.x {
                        b.x.cmp(&a.x)
                    } else {
                        if a.x > 0 {
                            a.y.cmp(&b.y)
                        } else {
                            b.y.cmp(&a.y)
                        }
                    }
                }
            } else {
                if b.y > 0 {
                    Ordering::Greater
                } else {
                    if a.x != b.x {
                        a.x.cmp(&b.x)
                    } else {
                        if a.x < 0 {
                            b.y.cmp(&a.y)
                        } else {
                            a.y.cmp(&b.y)
                        }
                    }
                }
            }
        });

        ret_vec.dedup();
        ret_vec
    }

    fn do_viewshed(&self, origin: Point, radius: u32) -> ResultRaster {
        let circle = Raster::draw_circle(&origin, radius);
        self.check_raster(&circle, &origin)
    }

    //
    fn check_raster(&self, circle: &Vec<Point>, origin: &Point) -> ResultRaster {
        let mut result_vec = vec![false; 65_000];
        for idx in 0..circle.len() {
            let line = Raster::draw_line(origin,&circle[idx]);
            let line_result = self.check_line(&line);
            // println!("{:?}",line.iter().map(|el: &Point|{self.value_at(el) as f64}).collect::<Vec<f64>>());
            // println!("{:?}",line_result);
            let iter = line.iter().zip(line_result.iter());
            for (point, result) in iter {
                Raster::set_result(&mut result_vec, self.width, point, *result);
            }
        }
        ResultRaster::new(result_vec, self.width, self.x0, self.y1)
    }

    // check a line of points for visibility from the first point
    fn check_line(&self, line: &Vec<Point>) -> Vec<bool> {
        let origin = &line[0];

        let mut highest_slope: f64 = std::f64::NEG_INFINITY;

        // take line of points, map to line of slopes
        line.iter()
            .map(|p: &Point| {
                if p.x == origin.x && p.y == origin.y {
                    std::f64::NEG_INFINITY
                } else {
                    self.get_slope(origin, p)

                }
            })
            .collect::<Vec<f64>>()
            // take line of slopes, map to line of bools
            .iter()
            .map(|curr_slope: &f64|{
                if *curr_slope == std::f64::NEG_INFINITY {
                    true
                } else {
                    if *curr_slope > highest_slope {
                        highest_slope = *curr_slope;
                        true
                    } else {
                        false
                    }
                }
            })
            .collect::<Vec<bool>>()
    }
}

fn main() {
    let a: Raster = Raster::rand_raster();
    let result = a.do_viewshed(Point{x:50, y:50}, 50);
    // println!("{:?}",result.pixels);
    // let height = a.value_at(Point{x:5,y:5});
    // println!("{:?}", a.check_line(&Raster::draw_line(Point{x:0,y:0}, Point{x:0,y:10})));
    // println!("{:?}",line);
}
