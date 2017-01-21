extern crate image;
extern crate rand;

use std::f32;
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
    pub x0: f32, // related to extent?  maybe corner in mercator
    pub y1: f32,  // see above
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

    fn max_in_raster(&self,arr: &[f32; 66_049]) -> f32 {
        arr.iter().fold(f32::MIN, |acc, &pix_height| {
            if pix_height > acc {
                pix_height
            } else {
                acc
            }
        })
    }

    fn min_in_raster(&self,arr: &[f32; 66_049]) -> f32 {
        arr.iter().fold(f32::MAX, |acc, &pix_height| {
            if pix_height < acc {
                pix_height
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



    pub fn new(source_raster: [Option<f32>; 66_049], width: u32, x0: f32, y1: f32) -> Raster{
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
    pub fn get_dist(&self, p1: &Point::Point, p2: &Point::Point) -> f32 {
            (((p2.x-p1.x).pow(2) + (p2.y-p1.y).pow(2)) as f32).sqrt()
    }

    // slope formula
    pub fn get_slope(&self, p1: &Point::Point, p2: &Point::Point) -> Option<f32> {

        let h1 = match self.value_at(p1) {
            Some(h) => Some(h),
            None    => self.get_height_recur(p1)
        };

        let h2 = match self.value_at(p2) {
            Some(h) => Some(h),
            None    => self.get_height_recur(p2)
        };

        if h1.is_some() && h2.is_some() {
            Some(h2.unwrap()-h1.unwrap() as f32/self.get_dist(p1,p2))
        } else {
            None
        }
    }

    fn get_slope_from_idx(&self, idx: usize, target_idx: usize) -> Option<f32> {
        let h1 = match self.pixels[idx] {
            Some(h) => Some(h),
            None    => None
        };

        let h2 = match self.pixels[target_idx] {
            Some(h) => Some(h),
            None    => None
        };

        if h1.is_some() && h2.is_some() {
            Some(h2.unwrap()-h1.unwrap() as f32)
        } else {
            None
        }
    }

    fn get_pix_dist(&self, idx_1: usize, idx_2: usize) -> f32 {
        (idx_1 as isize % self.width as isize - idx_2 as isize % self.width as isize).abs() as f32
    }

    // check 8 pixels around for height
    pub fn get_height_recur(&self,p: &Point::Point) -> Option<f32> {
        let mut heights: [Option<f32>;8] = [None;8];

        heights[0] = self.value_at(&Point::Point{x:p.x+1,y:p.y});
        heights[1] = self.value_at(&Point::Point{x:p.x-1,y:p.y});
        heights[2] = self.value_at(&Point::Point{x:p.x,y:p.y+1});
        heights[3] = self.value_at(&Point::Point{x:p.x,y:p.y-1});
        heights[4] = self.value_at(&Point::Point{x:p.x+1,y:p.y+1});
        heights[5] = self.value_at(&Point::Point{x:p.x-1,y:p.y-1});
        heights[6] = self.value_at(&Point::Point{x:p.x-1,y:p.y+1});
        heights[7] = self.value_at(&Point::Point{x:p.x+1,y:p.y-1});

        let mut num_heights = 0;

        let heights_sum: f32 = heights.iter().fold(0.0, |acc, &pix_height| {
            match pix_height {
                Some(h) => { 
                            num_heights += 1;
                            acc + h 
                        },
                None    => acc
            }
        });

        if heights_sum != 0.0 {
            Some(heights_sum/num_heights as f32)
        } else {
            None
        }
    }

    // return pixel value @ x,y
    pub fn value_at(&self, point: &Point::Point) -> Option<f32> {
        let idx: i32 = (self.width as i32 * point.y as i32) + point.x as i32;
        if idx >= 0 && idx < 66_049 {
            self.pixels[idx as usize]          
        } else {
            None
        }

    }

    // generate a test raster
    pub fn rand_raster() -> Raster{
        Raster::new(Raster::rand_raster_source(), 256 as u32, rand::random::<f32>(), rand::random::<f32>())
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

    pub fn draw_circle(mid_point: &Point::Point, radius: u32) -> Circle::Circle{

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
            center: *mid_point,
            radius: radius
        }
    }

    // perform viewshed
    pub fn do_viewshed(&self, origin: &Point::Point, radius: u32) -> ResultRaster::ResultRaster {
        let circle = Raster::draw_circle(origin, radius);
        self.check_raster(circle, origin)
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
        let mut highest_slope: f32 = f32::NEG_INFINITY;
        let mut last_was_true: bool = true;

        // take line of points, map to line of slopes
        line.iter()
            .map(|p: &Point::Point| {
                if p == origin {
                    Some(f32::NEG_INFINITY)
                } else {
                    self.get_slope(origin, p)
                }
            })
            .collect::<Vec<Option<f32>>>()
            // take line of slopes, map to line of bools
            .iter()
            .map(|curr_slope: &Option<f32>|{
                match *curr_slope {
                    Some(x) => {
                        if x == f32::NEG_INFINITY {
                            true
                        } else {
                            if x >= highest_slope {
                                highest_slope = x;
                                last_was_true = true;
                                true
                            } else {
                                last_was_true = false;
                                false
                            }
                        }
                    },
                    None    => last_was_true
                }
            })
            .collect::<Vec<bool>>()
    }

    // pub fn wmercator_to_raster(&self,wmercator_point: Point) -> Point {

    // }

    // pub fn raster_to_wmercator(&self, raster_point: &Point) -> Point {

    // }

    // get the slope for a pixel: calculate slope between pixel and 8 neighbors, return greatest.
    fn get_max_slope(&self, p: Point::Point) -> Option<f32> {
        let mut slopes_arr: [Option<f32>; 8] = [None; 8];
        
        slopes_arr[0] = self.get_slope(&p,&Point::Point{x: p.x+1, y: p.y});
        slopes_arr[1] = self.get_slope(&p,&Point::Point{x: p.x-1, y: p.y});
        slopes_arr[2] = self.get_slope(&p,&Point::Point{x: p.x, y: p.y+1});
        slopes_arr[3] = self.get_slope(&p,&Point::Point{x: p.x, y: p.y-1});
        slopes_arr[4] = self.get_slope(&p,&Point::Point{x: p.x+1, y: p.y+1});
        slopes_arr[5] = self.get_slope(&p,&Point::Point{x: p.x-1, y: p.y-1});
        slopes_arr[6] = self.get_slope(&p,&Point::Point{x: p.x-1, y: p.y+1});
        slopes_arr[7] = self.get_slope(&p,&Point::Point{x: p.x+1, y: p.y-1});

        slopes_arr.iter().fold(Some(f32::MIN), |max_slope, &slope| {
            match slope {
                Some(x) => { if x > max_slope.unwrap() { Some(x) } else { max_slope } },
                None    => max_slope
            }
        })

    }

    fn get_max_slope_idx(&self, idx: usize) -> Option<f32> {
        let mut slopes_arr: [Option<f32>; 8] = [None; 8];
        
        if idx % self.width as usize != 0 {
            slopes_arr[1] = self.get_slope_from_idx(idx, idx - 1);
            if idx < 66_049 - self.width as usize {
                slopes_arr[5] = self.get_slope_from_idx(idx, idx + self.width as usize - 1);
            }
            if idx > self.width as usize {
                slopes_arr[7] = self.get_slope_from_idx(idx, idx - self.width as usize - 1);
            }
        }
        if (idx + 1) % self.width as usize != 0 {
            slopes_arr[0] = self.get_slope_from_idx(idx, idx + 1);
            if idx < 66_049 - self.width as usize {
                slopes_arr[4] = self.get_slope_from_idx(idx, idx + self.width as usize + 1);
            }
            if idx > self.width as usize {
                slopes_arr[6] = self.get_slope_from_idx(idx, idx + 1 - self.width as usize);
            }
        }
        if idx > self.width as usize {
            slopes_arr[3] = self.get_slope_from_idx(idx, idx - self.width as usize);
        }
        if idx < 66_049 - self.width as usize {
            slopes_arr[2] = self.get_slope_from_idx(idx, idx + self.width as usize);
        }

        slopes_arr.iter().fold(Some(f32::MIN), |max_slope, &slope| {
            match slope {
                Some(x) => { if x > max_slope.unwrap() { Some(x) } else { max_slope } },
                None    => max_slope
            }
        })
    }

    //return an array that has slope value for each pixel 
    pub fn to_slope_raster(&self) -> [f32; 66_049] {
        let mut ret_array: [f32; 66_049] = [0.0; 66_049];
        for idx in 0..66_048 {
            ret_array[idx] = match self.get_max_slope_idx(idx) {
                Some(slope) => slope,
                None        => 0.0
            }
        }
        ret_array
    }

    pub fn print_slope_png(&self, file_name: &str) {
        let slope_raster = self.to_slope_raster();
        let max_slope = self.max_in_raster(&slope_raster);
        let min_slope = self.min_in_raster(&slope_raster);
        let mut buf: [u8; 66_049] = [0; 66_049];
        for (idx, slope) in slope_raster.iter().enumerate() {
             buf[idx] = ((((slope - min_slope) * (255.0 as f32 - 0.0 as f32)) / (max_slope - min_slope)) + 0.0 as f32) as u8;
        } 
        image::save_buffer(&Path::new(file_name), &buf, self.width, self.pixels.len() as u32/self.width, image::Gray(8)).unwrap();

    }
}