#![allow(dead_code)]

extern crate rand;
extern crate image;

use std::cmp::*;
use std::fs::File;
use std::path::Path;

static NO_VALUE: f32 = std::f32::MAX;
static LEN: usize = 66_049;

// x, y point
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
struct Point{
    x: i32,
    y: i32
}


// result of viewshed - array of bools
#[allow(dead_code)]
struct ResultRaster{
    pixels: [bool; 66_049],
    width: u32,
    x0: f64,
    y1: f64
}

// methods for result raster
impl ResultRaster {
    
    // create new result raster
    fn new(result_array: [bool; 66_049], width:u32, x0: f64, y1: f64) -> ResultRaster {
        ResultRaster{
            pixels: result_array,
            width: width,
            x0: x0,
            y1: y1
        }
    }

    // save the result raster an image: png.  uses to_img
    fn save_png(&self, file_path: &str){
        let ref mut fout = File::create(&Path::new(file_path)).unwrap();
        // We must indicate the image’s color type and what format to save as
        let _ = image::ImageLuma8(self.to_img()).save(fout, image::PNG);
    }

    // convert result raster's pixels to an image buffer, will be black and white
    fn to_img(&self) -> image::ImageBuffer<image::Luma<u8>, Vec<u8>> {
        
        // image buffer, size matches result's pixels
        let mut imgbuf = image::ImageBuffer::new(self.width, self.pixels.len() as u32/self.width);
        
        // iterate over image buffer
        for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
            // convert bool to black or white: u8
            let grey_scale_val = if self.value_at(&Point{x:x as i32,y:y as i32}) == true { 255 } else { 0 };
            *pixel = image::Luma([grey_scale_val]);
        }
        imgbuf
    }

    // check the result's pixels.  count how many trues and falses and print
    fn check_result(&self) {
        let mut num_false = 0;
        let mut num_true = 0;

        for pixel in self.pixels.iter() {
            if *pixel == true { num_true+=1; } else { num_false+=1; }
        }

        println!("True: {}; False: {}",num_true, num_false);
    }

    // return pixel value @ x,y
    fn value_at(&self, point: &Point) -> bool {
        let idx: u32 = (self.width * point.y.abs() as u32) + point.x.abs() as u32;
        
        self.pixels[idx as usize]

    }

}

// struct for a raster: this raster's pixels contain elevation data
struct Raster{
    pixels: [f32; 66_049], // height array
    width: u32, // width of raster
    x0: f64, // related to extent?  maybe corner in mercator
    y1: f64,  // see above
    max_height: Option<f32>,
    min_height: Option<f32>
}

// add methods to raster
impl Raster {

    // convert raster's pixel to image buffer
    fn to_img(&self) -> image::ImageBuffer<image::Luma<u8>, Vec<u8>> {
        let mut imgbuf = image::ImageBuffer::new(self.width, self.pixels.len() as u32/self.width);

        for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
            let grey_scale_val = self.f32_to_u8(self.value_at(&Point{x:x as i32,y:y as i32}));
            *pixel = image::Luma([grey_scale_val]);
        }
        imgbuf
    }

    fn to_no_data(&self) -> image::ImageBuffer<image::Luma<u8>, Vec<u8>> {
        let mut imgbuf = image::ImageBuffer::new(self.width, self.pixels.len() as u32/self.width);
        let mut no_data_num = 0;

        for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
            let grey_scale_val = if self.value_at(&Point{x:x as i32,y:y as i32}) == NO_VALUE { no_data_num+=1; 0 } else { 255 };
            *pixel = image::Luma([grey_scale_val]);
        }
        println!("{}",no_data_num);
        imgbuf
    }

    // set the min and max vals on a raster
    fn set_min_max(& mut self){
        self.max_height = Some(self.max());
        self.min_height = Some(self.min());
    }

    // find max pixel in raster, returns value
    fn max(&self) -> f32 {
        // self.pixels.max()
        self.pixels.iter().fold(std::f32::MIN, |acc, &pix_height| {
            if pix_height > acc && pix_height != std::f32::MAX {
                pix_height
            } else {
                acc
            }
        })
    }

    // find min pixel in raster, returns value
    fn min(&self) -> f32 {
        self.pixels.iter().fold(std::f32::MAX, |acc, &pix_height| {
            if pix_height < acc {
                pix_height
            } else {
                acc
            }
        })
    }

    // fit to 0-255 - go from f32 height to u8 for greyscale image
    fn f32_to_u8(&self, height: f32) -> u8 {
        if self.max_height.is_some() && self.min_height.is_some() {

            let max = self.max_height.unwrap();
            let min = self.min_height.unwrap();

            let grey_scale_val = (((height - min) * (255.0 as f32 - 0.0 as f32)) / (max - min)) + 0.0 as f32;
            grey_scale_val as u8
        } else {
            panic!("Called f32_to_u8 without having a max and min!");
        }
    }

    // save raster's pixels as png.  this uses to_img to first get an image buf
    fn save_png(&self, file_path: &str){
        let ref mut fout = File::create(&Path::new(file_path)).unwrap();

        let _ = image::ImageLuma8(self.to_img()).save(fout, image::PNG);
    }

    // save raster's pixels as png.  this uses to_img to first get an image buf
    fn save_png_no_data(&self, file_path: &str){
        let ref mut fout = File::create(&Path::new(file_path)).unwrap();

        let _ = image::ImageLuma8(self.to_no_data()).save(fout, image::PNG);
    }



    fn new(source_raster: [f32; 66_049], width: u32, x0: f64, y1: f64) -> Raster{
        Raster{
            pixels: source_raster,
            width: width,
            x0: x0,
            y1: y1,
            max_height: None,
            min_height: None
        }
    }

    // distance formula
    fn get_dist(&self, p1: &Point, p2: &Point) -> f64{
            (((p2.x-p1.x).pow(2) + (p2.y-p1.y).pow(2)) as f64).sqrt()
    }

    // slope formula
    fn get_slope(&self, p1: &Point, p2: &Point) -> f64{
        let h1 = self.value_at(p1);
        let h2 = self.value_at(p2);

        (h2-h1) as f64/self.get_dist(p1,p2)
    }

    // return pixel value @ x,y
    fn value_at(&self, point: &Point) -> f32 {
        let idx: u32 = (self.width * point.y.abs() as u32) + point.x.abs() as u32;

        self.pixels[idx as usize]
    }

    // generate a test raster
    fn rand_raster() -> Raster{
        Raster::new(Raster::rand_raster_source(), 256 as u32, rand::random::<f64>(), rand::random::<f64>())
    }

    // do the generation here: currently the raster is flat.
    fn rand_raster_source() -> [f32;66_049]{
        let mut last_height: f32 = 0.0;
        let mut arr: [f32; 66_049] = [5.5; 66_049];
        // for i in 0..arr.len() {
            // let pos_or_neg: f32 = if rand::random::<f32>() > 0.5 { 1.0 } else { -1.0 };
            // let curr_height = last_height + rand::random::<f32>() * pos_or_neg;
            // arr[i] = curr_height;
            // last_height = curr_height;
        // }
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
            } else if err > 0 {
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

    // perform viewshed
    fn do_viewshed(&self, origin: Point, radius: u32) -> ResultRaster {
        let circle = Raster::draw_circle(&origin, radius);
        self.check_raster(&circle, &origin)
    }

    // check a raster
    fn check_raster(&self, circle: &Vec<Point>, origin: &Point) -> ResultRaster {
        let mut result_array = [false; 66_049];
        let mut i = 0;
        for point in circle {

            let line = Raster::draw_line(origin,point);
            let line_result = self.check_line(&line);
            let iter = line.iter().zip(line_result.iter());

            for (point, result) in iter {
                result_array[self.point_to_idx(point)] = *result;
                // Raster::set_result(&mut result_vec, self.width, point, *result);
            }
        }
        ResultRaster::new(result_array, self.width, self.x0, self.y1)
    }

    fn point_to_idx(&self, point: &Point) -> usize {
        let idx: u32 = (self.width * point.y.abs() as u32) + point.x.abs() as u32;
        idx as usize
    }

    // check a line of points for visibility from the first point
    fn check_line(&self, line: &Vec<Point>) -> Vec<bool> {
        let origin = &line[0];
        let mut highest_slope: f64 = std::f64::NEG_INFINITY;

        // take line of points, map to line of slopes
        line.iter()
            .map(|p: &Point| {
                if p == origin {
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
}

fn read_array_from_file(filename: &str) -> [f32; 66_049] {
    use std::io::prelude::*;
    use std::io::BufReader;
    use std::fs::File;
    use std::path::Path;

    let mut ret_array: [f32; 66_049] = [NO_VALUE; 66_049];

    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    for (idx, val) in buf.split(b',').enumerate() {
        let byte_vec = &val.unwrap();
        // let my_int: i32 = my_string.parse().unwrap();

        let height = std::str::from_utf8(byte_vec).unwrap().parse::<f32>().unwrap();
        ret_array[idx] = height;
    }

    ret_array
}


fn main() {

    // println!("{}",NO_VALUE);
    let mut raster = Raster::new(read_array_from_file("child2.txt"), 257 as u32, rand::random::<f64>(), rand::random::<f64>());
    // let mut raster: Raster = Raster::rand_raster();
    raster.set_min_max();
    // raster.save_png("sampleChild2.png");
    // raster.save_png_no_data("data-gaps.png");

    let result = raster.do_viewshed(Point{x:128, y:200}, 50);
    result.save_png("result.png");
    // result.check_result();

    // random_raster.save_png("sample.png");
    // let result = random_raster.do_viewshed(Point{x:128, y:128}, 100);
    // result.save_png("result.png");
    // result.check_result();
}
