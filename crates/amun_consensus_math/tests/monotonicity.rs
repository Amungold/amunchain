use amun_consensus_math::*;

fn fixed_from_f64(v: f64) -> Fixed {
    Fixed::from_raw((v * SCALE as f64) as i64)
}

#[test]
fn test_sqrt_monotonicity() {
    let values = [0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9];
    
    for i in 1..values.len() {
        let a = fixed_from_f64(values[i-1]);
        let b = fixed_from_f64(values[i]);
        
        let sqrt_a = f_sqrt(a);
        let sqrt_b = f_sqrt(b);
        
        assert!(sqrt_a < sqrt_b, "sqrt({}) < sqrt({}) failed", values[i-1], values[i]);
    }
}

#[test]
fn test_sqrt_identity() {
    for i in 1..10 {
        let a = Fixed::from_int(i);
        let aa = a * a;
        let sqrt_aa = f_sqrt(aa);
        let diff = (sqrt_aa - a).abs();
        assert!(diff.raw() <= 2, "sqrt({}²) = {} vs {}", i, sqrt_aa.raw(), a.raw());
    }
}
