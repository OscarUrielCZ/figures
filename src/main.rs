use std::io::{self, BufRead};

/*struct Rectangle {
	top_left: Point,
	bottom_right: Point
}

#[derive(Clone)]
struct Point {
	x: f32,
	y: f32
}

fn rect_area(r: Rectangle) -> f32 {
	let Rectangle {top_left: tl, bottom_right: br} = r;
	let w: f32 = (tl.y-br.y).abs();
	let l: f32 = (tl.x-br.x).abs();
	let area = w*l;

	return area;
}

fn square(p: Point, l: f32) -> Rectangle {
	let x = p.x+l;
	let y = p.y+l;

	return Rectangle {
		top_left: p,
		bottom_right: Point {x, y}
	};
}*/

fn main() {
	let stdin = io::stdin();
	let mut iterator = stdin.lock().lines();
	let line1: str = iterator.next().unwrap().unwrap() ;
	
	println!("The line1 read: {}", line1);
	// let x = line1+1;
	// println!("X {}", x);
	/*let point: Point = Point {x: 10.2f32, y: 0.2};
	let data = (point.clone(), Point {x: 12.3, y: -1.2});
	let rectangle = Rectangle {
		top_left: data.0,
		bottom_right: data.1
	};

	let area = rect_area(rectangle);
	let r: Rectangle = square(point, 10f32);

	println!("The rectangle area is {}", area);
	println!("Bottom right point ({}, {})", r.bottom_right.x, r.bottom_right.y);*/
}
