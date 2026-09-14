use tmig::boundary::spatial::{classify, SpatialBound};

#[test]
fn inside_outside_boundary() {
    assert_eq!(classify(0.5, 0.0, 1.0), SpatialBound::Inside);
    assert_eq!(classify(-0.1, 0.0, 1.0), SpatialBound::Outside);
    assert_eq!(classify(0.0, 0.0, 1.0), SpatialBound::OnBoundary);
    assert_eq!(classify(1.0, 0.0, 1.0), SpatialBound::OnBoundary);
}
