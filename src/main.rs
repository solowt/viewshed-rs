#![allow(dead_code)]

extern crate rand;
mod Raster;
mod ResultRaster;
mod Point;
mod Circle;
mod RasterUtils;
mod Path;

static NO_VALUE: f32 = std::f32::MAX;
static LEN: usize = 66_049;

fn read_vec_from_file(filename: &str) -> Vec<Option<f32>> {
    use std::io::prelude::*;
    use std::io::BufReader;
    use std::fs::File;


    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);

    let mut ret_vec: Vec<Option<f32>> = Vec::with_capacity(LEN);

    for val in buf.split(b',') {
        let byte_vec = &val.unwrap();

        let height = std::str::from_utf8(byte_vec).unwrap().parse::<f32>().unwrap();
        if height == NO_VALUE {
            ret_vec.push(None)
        } else {
           ret_vec.push(Some(height));    
        }
    }

    ret_vec
}

fn get_task(raster: &Raster::Raster){
    println!("1: Print raster as PNG.\n2: Print no zones as PNG.\n3: Perform viewshed.\n4: Print slope raster as PNG.\n5: Borders Result Raster.\n6: Read more data.\n");
    
    let input = get_input();
    let trimmed = input.trim();

    if trimmed == "1" {

        println!("Enter Filename.\n");
        let input = get_input();

        raster.save_png(input.trim());
        println!("PNG saved as {}",input);

        get_task(raster);

    } else if trimmed == "2" {

        println!("Enter Filename.\n");
        let input = get_input();

        raster.save_png_no_data(input.trim());
        println!("PNG saved as {}",input);

        get_task(raster);

    } else if trimmed == "3" {

        println!("Enter X and Y coordinates - X,Y.\n");
        let input = get_input();
        let coords_vec: Vec<&str> = input.trim().split(',').collect();
        let raw_x = coords_vec[0].parse::<i32>();
        let raw_y = coords_vec[1].parse::<i32>();
        let x = match raw_x {
            Ok(x) => x,
            Err(e) => panic!(e)
        };
        let y = match raw_y {
            Ok(y) => y,
            Err(e) => panic!(e)
        };

        println!("Enter distance.\n");
        let raw_dist = get_input();
        // let raw_dist = raw_dist.trim().parse::<u32>();
        let dist = match raw_dist.trim().parse::<u32>() {
            Ok(d)   => d,
            Err(e)  => panic!(e) 
        };

        println!("Enter filename to save result.\n");
        let input2 = get_input();
        let f_name = input2.trim();
        let result = raster.do_viewshed(&Point::Point{x: x, y: y}, dist);
        result.check_result();
        result.save_png(f_name);

        get_task(raster);

    } else if trimmed == "4"{

        println!("Enter Filename.\n");
        let input = get_input();

        raster.print_slope_png(input.trim());
        println!("PNG saved as {}",input);

        get_task(raster);
    
    } else if trimmed == "5" {

        println!("Enter X and Y coordinates - X,Y.\n");
        let input = get_input();
        let coords_vec: Vec<&str> = input.trim().split(',').collect();
        let raw_x = coords_vec[0].parse::<i32>();
        let raw_y = coords_vec[1].parse::<i32>();
        let x = match raw_x {
            Ok(x) => x,
            Err(e) => panic!(e)
        };
        let y = match raw_y {
            Ok(y) => y,
            Err(e) => panic!(e)
        };

        println!("Enter distance.\n");
        let raw_dist = get_input();
        // let raw_dist = raw_dist.trim().parse::<u32>();
        let dist = match raw_dist.trim().parse::<u32>() {
            Ok(d)   => d,
            Err(e)  => panic!(e) 
        };

        println!("Enter filename to save result.\n");
        let input2 = get_input();
        let f_name = input2.trim();
        let result = raster.do_viewshed(&Point::Point{x: x, y: y}, dist);
        result.check_result();
        let borders = result.smart_borders();
        borders.check_result();
        borders.save_png(f_name);

        get_task(raster);

    } else if trimmed == "6" {
        return;
    } else {
        println!("\nInvalid Input.\n");
        get_task(raster);
    }
}

fn get_input() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)
                    .ok()
                    .expect("Couldn't read line");
    input
}

fn get_filename() -> Raster::Raster {
    println!("Enter Filename.\n");

    let input = get_input();
    let raster = Raster::Raster::new(read_vec_from_file(input.trim()), 257 as u32, rand::random::<f32>(), rand::random::<f32>());
    
    raster
}


fn main() {
    loop {
        get_task(&get_filename());
    }
}