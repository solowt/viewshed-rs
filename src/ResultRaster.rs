extern crate image;
use Circle;
use Point;
use std::fs::File;
use std::path::Path;

// result of viewshed - array of bools
#[allow(dead_code)]
pub struct ResultRaster{
    pub pixels: [bool; 66_049],
    pub width: u32,
    pub x0: f64,
    pub y1: f64,
    pub circle: Circle::Circle
}

// methods for result raster
impl ResultRaster {
    
    // create new result raster
    pub fn new(result_array: [bool; 66_049], width:u32, x0: f64, y1: f64, circle: Circle::Circle) -> ResultRaster {
        ResultRaster{
            pixels: result_array,
            width: width,
            x0: x0,
            y1: y1,
            circle: circle
        }
    }

    // save the result raster an image: png.  uses to_img
    pub fn save_png(&self, file_path: &str){
        let ref mut fout = File::create(&Path::new(file_path)).unwrap();
        // We must indicate the image’s color type and what format to save as
        let _ = image::ImageLuma8(self.to_img()).save(fout, image::PNG);
    }

    // convert result raster's pixels to an image buffer, will be black and white
    pub fn to_img(&self) -> image::ImageBuffer<image::Luma<u8>, Vec<u8>> {
        
        // image buffer, size matches result's pixels
        let mut imgbuf = image::ImageBuffer::new(self.width, self.pixels.len() as u32/self.width);
        
        // iterate over image buffer
        for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
            // convert bool to black or white: u8
            let grey_scale_val = if self.value_at(&Point::Point{x:x as i32,y:y as i32}) == true { 255 } else { 0 };
            *pixel = image::Luma([grey_scale_val]);
        }
        imgbuf
    }

    // check the result's pixels.  count how many trues and falses and print
    pub fn check_result(&self) {
        let mut num_false = 0;
        let mut num_true = 0;

        for pixel in self.pixels.iter() {
            if *pixel == true { num_true+=1; } else { num_false+=1; }
        }

        println!("True: {}; False: {}",num_true, num_false);
    }

    // return pixel value @ x,y
    pub fn value_at(&self, point: &Point::Point) -> bool {
        let idx: u32 = (self.width * point.y.abs() as u32) + point.x.abs() as u32;
        
        self.pixels[idx as usize]

    }

    /*
    fn get_polygons(&self) -> Vec<Vec<Point>>{
        find all true pixels inside circle including edge of circle push to v1
        init v2, empty vec of vecs of pixels
        for px in v1
            check every px in every vec in v2
            if px borders on px in one vec in v2, push to that vec
            if more than one, unify all of those vecs into one vec and push px
            if no relationship found, push px to new vec on v2
        result is v2 which is a vec<vec<px>>
        
    }

    fn wmercator_to_raster(&self,wmercator_point: Point) -> Point {

    }

    fn raster_to_wmercator(&self, raster_point: &Point) -> Point {

    }
    */

}