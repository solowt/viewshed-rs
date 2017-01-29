use Point;

#[derive(Debug)]
pub struct Path {
	pub area: i32,
	pub points: Vec<Point::Point>,
	pub min_x: i32,
	pub min_y: i32,
	pub max_x: i32,
	pub max_y: i32
}
