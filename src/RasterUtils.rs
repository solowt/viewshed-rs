use Point;
use Circle;
use Path;
use std::f32;
use std::i32;
use std::cmp::*;
// use stf::fmt::*;

static LEN: usize = 66_049;

pub fn point_to_idx(point: &Point::Point, size: usize, width: u32) -> Option<usize> {
    if point.x < 0 || point.y < 0 || point.x >= width as i32 || point.y > size as i32 / width as i32 {
        None
    } else {
        let idx: u32 = (width * point.y as u32) + point.x as u32;
        Some(idx as usize)
    }
}   

pub fn bilinear_interp_mid(raster: &[Option<f32>], idx: usize, width: u32) -> Option<f32> {
    let mut val_arr: [Option<f32>;4] = [None;4];
    let size = raster.len();
    let mut right_edge: bool = true;
    let mut top_row: bool = true;

    val_arr[0] = raster[idx];
    if idx < size - width as usize {
        val_arr[1] = raster[idx + width as usize];
        top_row = false;
    }
    if (idx + 1) % width as usize != 0 {
        val_arr[2] = raster[idx + 1];
        right_edge = false;
    }
    if !top_row && !right_edge{
        val_arr[3] = raster[idx + width as usize +1];
    }
    
    let mut i = 0;
    let sum = val_arr.iter()
                     .filter(|&x| x.is_some())
                     .fold(0.0 as f32,|acc,&x| {
                        i += 1;
                        acc + x.unwrap()
                     });

    match i {
        0   => None,
        _   => Some(sum / i as f32)
    }
}

pub fn bilinear_interp_mid_point(raster: &[Option<f32>], point: &Point::Point, width: u32) -> Option<f32> {
    let idx_opt = point_to_idx(point, raster.len(), width);
    match idx_opt {
        Some(idx)   => bilinear_interp_mid(raster, idx, width),
        None        => None	
    }
} 

pub fn idx_to_point(width: u32, idx: usize) -> Point::Point {
	let x = idx % width as usize;
	let y = (idx - x) / width as usize; 
	Point::Point{x: x as i32, y: y as i32}
}

pub fn min_in_raster(raster: &[f32]) -> f32 {
	raster.iter().fold(f32::MAX, |acc, &pix_height| {
            if pix_height < acc {
                pix_height
            } else {
                acc
            }
        })
}

pub fn max_in_raster(raster: &[f32]) -> f32 {
	raster.iter().fold(f32::MIN, |acc, &pix_height| {
            if pix_height > acc {
                pix_height
            } else {
                acc
            }
        })
}

pub fn min_in_raster_opt(raster: &[Option<f32>]) -> f32 {
	raster.iter().fold(f32::MAX, |acc, &pix_height| {
		match pix_height {
			Some(x)	=> if x < acc { x } else { acc },
			None	=> acc
		}
    })
}

pub fn max_in_raster_opt(raster: &[Option<f32>]) -> f32 {
	raster.iter().fold(f32::MIN, |acc, &pix_height| {
		match pix_height {
			Some(x)	=> if x > acc { x } else { acc },
			None	=> acc
		}
    })
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

    for _ in 0..longest+1 {
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
    Circle::Circle{
        edge: ret_vec,
        center: *mid_point,
        radius: radius
    }
}

fn bordering_on<T: PartialEq + Copy>(raster: &[Option<T>], idx: usize, width: u32, search_value: T) -> bool {
    get_neighbors_less(idx, width, raster.len())
        .iter()
        .filter(|idx_opt| idx_opt.is_some())
        .map(|idx_valid| raster[idx_valid.unwrap()])
        .any(|value_opt| {
            match value_opt {
                Some(b) => b == search_value,
                None    => true
            }
        })
}

// 'dumb' brute force method
pub fn aggregate_valid_pix(raster: &[Option<bool>], width: u32, search_value: bool) -> Vec<usize> {
    raster.iter()
          .enumerate()
          .map(|idx_tuple: (usize, &Option<bool>)| {
                match *idx_tuple.1 {
                    Some(t) =>   {
                        if t == true {
                            match bordering_on(raster,idx_tuple.0,width,search_value) {
                                true    =>  Some(idx_tuple.0),
                                false   =>  None
                            }
                        } else {
                            None
                        }
                    },
                    None    => None
                }
          })
          .filter(|idx_opt| idx_opt.is_some())
          .map(|valid_idx| valid_idx.unwrap())
          .collect::<Vec<usize>>()
}

// pub fn aggregate_valid_pix<T: PartialEq + Copy>(raster: &[Option<T>], width: u32, search_value: T) -> Vec<usize> {
//     raster.iter()
//           .enumerate()
//           .map(|idx_tuple: (usize, &Option<T>)| {
//                 match *idx_tuple.1 {
//                     Some(t) =>   {
//                         if t: bool == true {
//                             match bordering_on(raster,idx_tuple.0,width,search_value) {
//                                 true    =>  Some(idx_tuple.0),
//                                 false   =>  None
//                             }
//                         } else {
//                             None
//                         }
//                     },
//                     None       => None
//                 }
//           })
//           .filter(|idx_opt| idx_opt.is_some())
//           .map(|valid_idx| valid_idx.unwrap())
//           .collect::<Vec<usize>>()
// }

fn get_slope_from_idx(pixels: &[Option<f32>], idx: usize, target_idx: usize) -> Option<f32> {
    
    let h1 = pixels[idx];
    let h2 = pixels[target_idx];

    if h1.is_some() && h2.is_some() {
        Some((h2.unwrap()-h1.unwrap()).abs())
    } else {
        None
    }
}

fn get_neighbors_less(idx: usize, width: u32, size: usize) -> [Option<usize>; 4] {
    let mut ret_arr: [Option<usize>; 4] = [None; 4];

    if idx % width as usize != 0 {
        ret_arr[0] = Some(idx - 1);
    }
    if (idx + 1) % width as usize != 0 {
        ret_arr[1] = Some(idx + 1);
    }
    if idx >= width as usize {
        ret_arr[2] = Some(idx - width as usize);
    }
    if idx < size - width as usize {
        ret_arr[3] = Some(idx + width as usize);
    }

    ret_arr
}

fn get_neighbors(idx: usize, width: u32, size: usize) -> [Option<usize>; 8] {
	let mut ret_arr: [Option<usize>;8] = [None;8];

	if idx % width as usize != 0 {
        ret_arr[0] = Some(idx - 1);
        if idx < size - width as usize {
            ret_arr[1] = Some(idx + width as usize - 1);
        }
        if idx > width as usize {
            ret_arr[2] = Some(idx - width as usize - 1);
        }
    }
    if (idx + 1) % width as usize != 0 {
        ret_arr[3] = Some(idx + 1);
        if idx < size - width as usize {
            ret_arr[4] = Some(idx + width as usize + 1);
        }
        if idx > width as usize {
            ret_arr[5] = Some(idx - width as usize + 1);
        }
    }
    if idx >= width as usize {
        ret_arr[6] = Some(idx - width as usize);
    }
    if idx < size - width as usize {
        ret_arr[7] = Some(idx + width as usize);
    }

    ret_arr
}

pub fn get_max_slope_idx(pixels: &[Option<f32>], width: u32, idx: usize) -> Option<f32> {
	get_neighbors(idx, width, pixels.len())
		.iter()
	 	.filter(|idx_opt| idx_opt.is_some())
		.map(|valid_idx| get_slope_from_idx(pixels, idx, valid_idx.unwrap()))
		.filter(|slope_opt| slope_opt.is_some())
		.fold(None, |acc, valid_slope| {
			if acc.is_none() { valid_slope }
			else if valid_slope.unwrap() > acc.unwrap() { valid_slope }
			else { acc }
		})
}

pub fn f32_to_u8(min: f32, max: f32, val: f32) -> u8 {
	let grey_scale_val = (((val - min) * (255.0 as f32 - 0.0 as f32)) / (max - min)) + 0.0 as f32;
    grey_scale_val as u8
}

/***/
// potrace // 
/****/

pub fn point_to_bool(point: &Point::Point, raster: &[Option<bool>], width: u32) -> bool {
    if point.x < 0 || point.y < 0 || point.x >= width as i32 || point.y >= width as i32 {
        false
    } else {
        let idx: u32 = (width * point.y as u32) + point.x as u32;
        match raster[idx as usize] {
            Some(true)  => true,
            Some(false) => false,
            None        => false
        }
    }
} 

pub fn find_path(raster: &[Option<bool>], source_width: u32, origin: Point::Point) -> Path::Path {
    let mut ret_vec: Vec<Point::Point> = Vec::new();
    let mut x: i32 = origin.x;
    let mut y: i32 = origin.y;
    let mut dir_x: i32 = 0;
    let mut dir_y: i32 = 1;
    let mut max_x: i32 = i32::MIN;
    let mut max_y: i32 = i32::MIN;
    let mut min_x: i32 = i32::MAX;
    let mut min_y: i32 = i32::MAX;
    let mut area: i32 = 0;
    let mut temp: i32;

    loop {
        ret_vec.push(Point::Point{ x: x, y: y });

        if x > max_x {
            max_x = x;
        }
        if x < min_x {
            min_x = x;
        }
        if y > max_y {
            max_y = y;
        }
        if y < min_y {
            min_y = y;
        }

        x += dir_x;
        y += dir_y;
        area -= x * dir_y;

        if x == origin.x && y == origin.y {
            break;
        }

        let l: bool = point_to_bool(&Point::Point{x: x + (dir_x + dir_y -1) / 2, y: y + (dir_y - dir_x -1) / 2}, &raster, source_width);
        let r: bool = point_to_bool(&Point::Point{x: x + (dir_x - dir_y -1) / 2, y: y + (dir_y + dir_x -1) / 2}, &raster, source_width);
        // println!("{},{}",l,r);
        if r == true && l == false {
            temp = dir_x;
            dir_x = dir_y;
            dir_y = -temp;
        } else if r == true {
            temp = dir_x;
            dir_x = -dir_y;
            dir_y = temp;
        } else if l == false {
            temp = dir_x;
            dir_x = dir_y;
            dir_y = -temp;
        }
    }
    Path::Path {
        area: area,
        points: ret_vec,
        min_x: min_x,
        min_y: min_y,
        max_x: max_x,
        max_y: max_y
    }
}

fn flip_point(raster: &mut Vec<Option<bool>>, width: u32, point: Point::Point){
    if let Some(idx) = point_to_idx(&point, raster.len(), width) {
        match raster[idx] {
            Some(true)  => raster[idx] = None,
            Some(false) => raster[idx] = Some(true),
            None        => raster[idx] = Some(true)
        }
    }
}

fn flip_path(mut raster: Vec<Option<bool>>, width: u32, path: &Path::Path) -> Vec<Option<bool>> {
    let mut y1 = path.points[0].y;
    let mut x;
    let mut y;
    let mut x_max;
    let mut y_min;

    for idx in 1..path.points.len() {
        x = path.points[idx].x;
        y = path.points[idx].y;
        if y != y1 {
            y_min = if y1 < y { y1 } else { y };
            x_max = path.max_x;
            for j in x..x_max {
                // if let Some(idx) = point_to_idx(&Point::Point{ x: j, y: y_min }, raster.len(), width) {
                //     raster[idx] = None;
                // }
                flip_point(&mut raster,width,   Point::Point{ x: j, y: y_min });
            }
            y1 = y;
        }
    }
    raster
}

fn find_next(point: &Point::Point, width: u32, raster: &[Option<bool>]) -> Option<Point::Point> {
    let mut i: usize = width as usize * point.y as usize + point.x as usize;
    while i < raster.len() && is_none_or_false(raster, i) {
        i += 1;
    }

    if i >= raster.len() {
        None
    } else {
        Some(idx_to_point(width, i))
    }
}

fn is_none_or_false(raster: &[Option<bool>], idx: usize) -> bool {
    match raster[idx] {
        Some(true)  => false,
        Some(false) => true,
        None        => true
    }
}

pub fn get_paths(mut raster: Vec<Option<bool>>, width: u32) -> Vec<Path::Path> {
    let mut current_point: Point::Point = Point::Point{ x: 0, y: 0 };
    let mut ret_vec: Vec<Path::Path> = Vec::new();

    loop {
        match find_next(&current_point, width, &raster) {
            Some(p) => current_point = p,
            None    => break
        }

        let new_path = find_path(&raster, width, current_point);
        raster = flip_path(raster, width, &new_path);
        if new_path.area > 5{
            ret_vec.push(new_path)
        }
    }

    ret_vec
}