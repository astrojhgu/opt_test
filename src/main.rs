extern crate ndarray;
extern crate optimize;

use optimize::Minimizer;
use optimize::NelderMeadBuilder;
use ndarray::{Array1, ArrayView1};


fn main() {
    let minimizer = NelderMeadBuilder::default()
        .xtol(1e-9)
        .ftol(1e-9)
        .maxiter(500000)
        .build()
        .unwrap();

    let result = minimizer.minimize(
        move |x: ArrayView1<f64>| {
            x.iter().map(|x1|x1.powi(2)).fold(0.0, |a,b|{a+b})
        },
        Array1::<f64>::from_vec(vec![100.0,100.0]).view()
    );

    println!("{:?}", result)
}
