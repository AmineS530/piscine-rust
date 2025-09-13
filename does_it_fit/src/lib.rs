pub mod areas_volumes;

pub use areas_volumes::*;

pub fn area_fit(
    (x, y): (usize, usize),
    kind: GeometricalShapes,
    times: usize,
    (a, b): (usize, usize),
) -> bool {
    let total_area = x * y;

    let required_area = match kind {
        GeometricalShapes::Square => square_area(a),
        GeometricalShapes::Circle => circle_area(a).ceil() as usize,
        GeometricalShapes::Rectangle => rectangle_area(a, b),
        GeometricalShapes::Triangle => triangle_area(a, b).ceil() as usize,
    };

    total_area >= required_area * times
}

pub fn volume_fit(
    (x, y, z): (usize, usize, usize),
    kind: GeometricalVolumes,
    times: usize,
    (a, b, c): (usize, usize, usize),
) -> bool {
    let total_volume = x * y * z;

    let required_volume = match kind {
        GeometricalVolumes::Cube => cube_volume(a),
        GeometricalVolumes::Sphere => sphere_volume(a).ceil() as usize,
        GeometricalVolumes::Cone => cone_volume(a, b).ceil() as usize,
        GeometricalVolumes::TriangularPyramid => {
            triangular_pyramid_volume(a as f64, b).ceil() as usize
        }
        GeometricalVolumes::Parallelepiped => parallelepiped_volume(a, b, c),
    };

    total_volume >= required_volume * times
}
