use crate::{Pt3, VertexId, id_map::IdType};
use hilbert::{point_list, Point, fast_hilbert};
use float_ord::FloatOrd;

/// Sorts some points along the Hilbert curve.
/// This makes adjacent points in the list be close together in 3D.
pub(crate) fn hilbert_sort<V: Clone, I: IntoIterator<Item = (Pt3, V)>>(points: I) -> Vec<(VertexId, Pt3, V)> {
    const PRECISION: usize = 32;

    let mut points = points.into_iter().enumerate()
        .map(|(i, (p, v))| (VertexId(i as IdType), p, v)).collect::<Vec<_>>();

    // Range
    let min_x = points.iter().map(|p| p.1.x).min_by_key(|x| FloatOrd(*x)).unwrap();
    let min_y = points.iter().map(|p| p.1.y).min_by_key(|x| FloatOrd(*x)).unwrap();
    let min_z = points.iter().map(|p| p.1.z).min_by_key(|x| FloatOrd(*x)).unwrap();

    let max_x = points.iter().map(|p| p.1.x).max_by_key(|x| FloatOrd(*x)).unwrap();
    let max_y = points.iter().map(|p| p.1.y).max_by_key(|x| FloatOrd(*x)).unwrap();
    let max_z = points.iter().map(|p| p.1.z).max_by_key(|x| FloatOrd(*x)).unwrap();

    // Calculate scale to match Hilbert curve precision
    let scale_x = (2.0f64.powi(PRECISION as i32) - 1.0) / (max_x - min_x);
    let scale_y = (2.0f64.powi(PRECISION as i32) - 1.0) / (max_y - min_y);
    let scale_z = (2.0f64.powi(PRECISION as i32) - 1.0) / (max_z - min_z);

    // The good stuff: sorting.
    // Can't use hilbert::Point because it calculates squared_magnitude, which can overflow
    // and isn't going to be used anyway.
    points.sort_by_cached_key(|(_, point, _)| {
        let normalized = [
            ((point.x - min_x) * scale_x) as u32,
            ((point.y - min_y) * scale_y) as u32,
            ((point.z - min_z) * scale_z) as u32,
        ];
        fast_hilbert::hilbert_index(&normalized, PRECISION, None)
    });

    points
}