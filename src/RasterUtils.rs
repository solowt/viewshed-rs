use Point;
use Circle;
use std::f32;
use std::cmp::*;

static LEN: usize = 66_049;

// pub fn bilinear_interp(raster: [Option<f32>; 66_049], idx: usize, width: u32) -> f32 {
// 	bilinear_interp_point(raster, idx_to_point(width,idx), width)
// }

// pub fn bilinear_interp_point(raster: [Option<f32>; 66_049], point: Point::Point, width: u32) -> f32 {
// 	3.3
// } 

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
        .filter(|value_opt| value_opt.is_some())
        .any(|value_valid| {
            value_valid.unwrap() == search_value
        })
}

pub fn aggregate_valid_pix<T: PartialEq + Copy>(raster: &[Option<T>], width: u32, search_value: T) -> Vec<usize> {
    raster.iter()
          .enumerate()
          .map(|idx_tuple: (usize, &Option<T>)| {
                match *idx_tuple.1 {
                    Some(_) => match bordering_on(raster,idx_tuple.0,width,search_value) {
                        true    => Some(idx_tuple.0),
                        false   => None
                    },
                    None    => None
                }
          })
          .filter(|idx_opt| idx_opt.is_some())
          .map(|valid_idx| valid_idx.unwrap())
          .collect::<Vec<usize>>()
}

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
