use ggez::mint::Point2;

pub fn add_point2f(p1: Point2<f32>, p2: Point2<f32>) -> Point2::<f32> {
	Point2::<f32> {
		x: p1.x + p2.x,
		y: p1.y + p2.y,
	}
}

