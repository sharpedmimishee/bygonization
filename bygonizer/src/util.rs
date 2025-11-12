#[derive(Clone, Copy, Debug)]
pub struct Point2D {
    x: f64,
    y: f64,
}

impl Point2D {
    pub fn new(x: f64, y: f64) -> Self {
        return Self { x, y };
    }
}


fn bezier(points: &[Point2D], t: f64) -> Point2D {
    let n = points.len() - 1;
    let mut result = Point2D { x: 0.0, y: 0.0 };

    for (i, p) in points.iter().enumerate() {
        let binomial = binomial_coefficient(n, i) as f64;
        let coeff = binomial * (1.0 - t).powi((n - i) as i32) * t.powi(i as i32);
        result.x += coeff * p.x;
        result.y += coeff * p.y;
    }

    result
}

fn binomial_coefficient(n: usize, k: usize) -> usize {
    (1..=n).product::<usize>() / ((1..=k).product::<usize>() * (1..=(n - k)).product::<usize>())
}

pub fn bezier_y_from_x(points: &[Point2D], frame: f64) -> f64 {
    let mut t_low = 0.0;
    let mut t_high = 1.0;

    for _ in 0..50 {
        let t_mid = (t_low + t_high) / 2.0;
        let point = bezier(points, t_mid);

        if point.x < frame {
            t_low = t_mid;
        } else {
            t_high = t_mid;
        }
    }

    let point = bezier(points, (t_low + t_high) / 2.0);
    point.y
}


