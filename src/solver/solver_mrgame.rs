use ndarray::Array1;

#[derive(Copy, Clone)]
pub struct MRGameConstraints {}

#[derive(Copy, Clone)]
pub struct MRGameLambdas {}

impl MRGameConstraints {
    fn array_len(&self) -> usize {
        0
    }

    fn append_lambdas(&self, array: &mut Array1<f64>, index: usize, lambdas: MRGameLambdas) {}
}
