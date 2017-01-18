extern crate image;
extern crate rand;

use std::f32;
use std::f64;
use Circle;
use Point;
use ResultRaster;
use std::fs::File;
use std::path::Path;
use std::cmp::*;


// struct for a raster: this raster's pixels contain elevation data
pub struct Raster{
    pub pixels: [Option<f32>; 66_049], // height array
    pub width: u32, // width of raster
    pub x0: f64, // related to extent?  maybe corner in mercator
    pub y1: f64,  // see above
    pub max_height: Option<f32>,
    pub min_height: Option<f32>
}

// add methods to raster
impl Raster {

    // convert raster's pixel to image buffer
    pub fn to_img(&self) -> image::ImageBuffer<image::Luma<u8>, Vec<u8>> {
        let mut imgbuf = image::ImageBuffer::new(self.width, self.pixels.len() as u32/self.width);

        for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
            let height = self.value_at(&Point::Point{x:x as i32,y:y as i32});

            *pixel = match height {
                Some(h) => image::Luma([self.f32_to_u8(h)]),
                None    => image::Luma([0])
            };
        }

        imgbuf
    }

    pub fn to_no_data(&self) -> image::ImageBuffer<image::Luma<u8>, Vec<u8>> {
        let mut imgbuf = image::ImageBuffer::new(self.width, self.pixels.len() as u32/self.width);
        let mut no_data_num = 0;

        for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
            
            *pixel = match self.value_at(&Point::Point{x:x as i32,y:y as i32}) {
                Some(h) => image::Luma([255]),
                None    => { no_data_num+=1; image::Luma([0]) }
            };
        }

        println!("{} pixels without data.",no_data_num);
        imgbuf
    }

    // set the min and max vals on a raster
    pub fn set_min_max(& mut self){
        self.max_height = Some(self.max());
        self.min_height = Some(self.min());
    }

    // find max pixel in raster, returns value
    pub fn max(&self) -> f32 {
        // self.pixels.max()
        self.pixels.iter().fold(f32::MIN, |acc, &pix_height| {
            if pix_height.is_some() && pix_height.unwrap() > acc {
                pix_height.unwrap()
            } else {
                acc
            }
        })
    }

    // find min pixel in raster, returns value
    pub fn min(&self) -> f32 {
        self.pixels.iter().fold(f32::MAX, |acc, &pix_height| {
            if pix_height.is_some() && pix_height.unwrap() < acc {
                pix_height.unwrap()
            } else {
                acc
            }
        })
    }

    // fit to 0-255 - go from f32 height to u8 for greyscale image
    pub fn f32_to_u8(&self, height: f32) -> u8 {

        let max = self.max_height.unwrap();
        let min = self.min_height.unwrap();

        let grey_scale_val = (((height - min) * (255.0 as f32 - 0.0 as f32)) / (max - min)) + 0.0 as f32;
        grey_scale_val as u8
        
    }

    // save raster's pixels as png.  this uses to_img to first get an image buf
    pub fn save_png(&self, file_path: &str){
        let ref mut fout = File::create(&Path::new(file_path)).unwrap();
        let _ = image::ImageLuma8(self.to_img()).save(fout, image::PNG);
    }

    // save raster's pixels as png.  this uses to_img to first get an image buf
    pub fn save_png_no_data(&self, file_path: &str){
        let ref mut fout = File::create(&Path::new(file_path)).unwrap();

        let _ = image::ImageLuma8(self.to_no_data()).save(fout, image::PNG);
    }



    pub fn new(source_raster: [Option<f32>; 66_049], width: u32, x0: f64, y1: f64) -> Raster{
        let mut r = Raster{
            pixels: source_raster,
            width: width,
            x0: x0,
            y1: y1,
            max_height: None,
            min_height: None
        };

        r.set_min_max();
        r
    }

    // distance formula
    pub fn get_dist(&self, p1: &Point::Point, p2: &Point::Point) -> f64{
            (((p2.x-p1.x).pow(2) + (p2.y-p1.y).pow(2)) as f64).sqrt()
    }

    // slope formula
    pub fn get_slope(&self, p1: &Point::Point, p2: &Point::Point) -> f64{

        let h1 = match self.value_at(p1) {
            Some(h) => h,
            None    => self.get_height_recur(p1)
        };

        let h2 = match self.value_at(p2) {
            Some(h) => h,
            None    => self.get_height_recur(p2)
        };

        (h2-h1) as f64/self.get_dist(p1,p2)
    }

    pub fn get_height_recur(&self,p: &Point::Point) -> f32 {
        let mut heights = Vec::new();

        if let Some(h) = self.value_at(&Point::Point{x:p.x+1,y:p.y}) {
            heights.push(h);
        }

        if let Some(h) = self.value_at(&Point::Point{x:p.x-1,y:p.y}) {
            heights.push(h);
        }

        if let Some(h) = self.value_at(&Point::Point{x:p.x,y:p.y+1}) {
            heights.push(h);
        }

        if let Some(h) = self.value_at(&Point::Point{x:p.x,y:p.y-1}) {
            heights.push(h);
        }

        if let Some(h) = self.value_at(&Point::Point{x:p.x+1,y:p.y+1}) {
            heights.push(h);
        }

        if let Some(h) = self.value_at(&Point::Point{x:p.x-1,y:p.y-1}) {
            heights.push(h);
        }

        if let Some(h) = self.value_at(&Point::Point{x:p.x-1,y:p.y+1}) {
            heights.push(h);
        }

        if let Some(h) = self.value_at(&Point::Point{x:p.x+1,y:p.y-1}) {
            heights.push(h);
        }


        let sum = heights.iter().fold(0.0 as f32,|acc, &pix_height|{
            acc + pix_height
        });

        sum / heights.len() as f32
    }

    // return pixel value @ x,y
    pub fn value_at(&self, point: &Point::Point) -> Option<f32> {
        let idx: u32 = (self.width * point.y.abs() as u32) + point.x.abs() as u32;
        self.pixels[idx as usize]          
    }

    // generate a test raster
    pub fn rand_raster() -> Raster{
        Raster::new(Raster::rand_raster_source(), 256 as u32, rand::random::<f64>(), rand::random::<f64>())
    }

    // do the generation here: currently the raster is flat.
    pub fn rand_raster_source() -> [Option<f32>;66_049]{
        // let mut last_height: f32 = 0.0;
        // let mut arr: [Option<f32>; 66_049] = [Some(5.5); 66_049];
        // for i in 0..arr.len() {
            // let pos_or_neg: f32 = if rand::random::<f32>() > 0.5 { 1.0 } else { -1.0 };
            // let curr_height = last_height + rand::random::<f32>() * pos_or_neg;
            // arr[i] = curr_height;
            // last_height = curr_height;
        // }
        // arr
        [Some(5.5); 66_049]
    }

    // bresenham's line algorithm http://tech-algorithm.com/articles/drawing-line-using-bresenham-algorithm/
    // give two points draw a line between them.  return vector of points as the line
    pub fn draw_line(p1: &Point::Point, p2: &Point::Point) -> Vec<Point::Point>{

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
            ret_vec.push(Point::Point{ x:curr_x, y: curr_y });
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

    pub fn draw_circle(mid_point: Point::Point, radius: u32) -> Circle::Circle{

        let mut ret_vec = Vec::new();

        let mut x: i32 = radius as i32;
        let mut y: i32 = 0;
        let mut err = 0;

        while x >= y {

            ret_vec.push(Point::Point{x: mid_point.x + x, y: mid_point.y + y});
            ret_vec.push(Point::Point{x: mid_point.x + x, y: mid_point.y - y});

            ret_vec.push(Point::Point{x: mid_point.x + y, y: mid_point.y + x});
            ret_vec.push(Point::Point{x: mid_point.x - y, y: mid_point.y + x});

            ret_vec.push(Point::Point{x: mid_point.x - x, y: mid_point.y + y});
            ret_vec.push(Point::Point{x: mid_point.x - x, y: mid_point.y - y});

            ret_vec.push(Point::Point{x: mid_point.x - y, y: mid_point.y - x});
            ret_vec.push(Point::Point{x: mid_point.x + y, y: mid_point.y - x});

            if err <= 0 {
                y += 1;
                err += 2*y + 1;
            } else if err > 0 { // else if makes this a "thick" circle.  no diagnal connections
                x -= 1;
                err -= 2*x + 1;
            }
        }

        ret_vec.sort_by(|a,b|{
            // if a.x == b.x && a.y == b.y {
            //     Ordering::Equal
            // } else if a.x >= b.x {
            //     Ordering::Greater
            // } else {
            //     Ordering::Less
            // }
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
        // println!("{:?}, len: {}",ret_vec, ret_vec.len());
        // println!("{:?}",ret_vec.last().unwrap());
        Circle::Circle{
            edge: ret_vec,
            center: mid_point,
            radius: radius
        }
    }

    // perform viewshed
    pub fn do_viewshed(&self, origin: Point::Point, radius: u32) -> ResultRaster::ResultRaster {
        let circle = Raster::draw_circle(origin, radius);
        self.check_raster(circle, &origin)
    }

    // check a raster
    pub fn check_raster(&self, circle: Circle::Circle, origin: &Point::Point) -> ResultRaster::ResultRaster {
        let mut result_array = [false; 66_049];
        for point in &circle.edge {

            let line = Raster::draw_line(origin,&point);
            let line_result = self.check_line(&line);
            let iter = line.iter().zip(line_result.iter());

            for (point, result) in iter {
                result_array[self.point_to_idx(point)] = *result;
                // Raster::set_result(&mut result_vec, self.width, point, *result);
            }
        }
        ResultRaster::ResultRaster::new(result_array, self.width, self.x0, self.y1, circle)
    }

    pub fn point_to_idx(&self, point: &Point::Point) -> usize {
        let idx: u32 = (self.width * point.y.abs() as u32) + point.x.abs() as u32;
        idx as usize
    }

    // check a line of points for visibility from the first point
    pub fn check_line(&self, line: &Vec<Point::Point>) -> Vec<bool> {
        let origin = &line[0];
        let mut highest_slope: f64 = f64::NEG_INFINITY;

        // take line of points, map to line of slopes
        line.iter()
            .map(|p: &Point::Point| {
                if p == origin {
                    f64::NEG_INFINITY
                } else {
                    self.get_slope(origin, p)
                }
            })
            .collect::<Vec<f64>>()
            // take line of slopes, map to line of bools
            .iter()
            .map(|curr_slope: &f64|{
                if *curr_slope == f64::NEG_INFINITY {
                    true
                } else {
                    if *curr_slope >= highest_slope {
                        highest_slope = *curr_slope;
                        true
                    } else {
                        false
                    }
                }
            })
            .collect::<Vec<bool>>()
    }

    // pub fn wmercator_to_raster(&self,wmercator_point: Point) -> Point {

    // }

    // pub fn raster_to_wmercator(&self, raster_point: &Point) -> Point {

    // }
}