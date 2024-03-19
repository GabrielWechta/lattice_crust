pub fn scal_mul_vec(scalar: i32, vec: &Vec<f64>) -> Vec<f64> {
    vec.iter().map(|x| x * scalar as f64).collect()
}

pub fn add_vecs(vec1: &Vec<f64>, vec2: &Vec<f64>) -> Vec<f64> {
    let mut return_vec = vec![0.0; vec1.len()];
    for i in 0..vec1.len() {
        return_vec[i] = vec1[i] + vec2[i];
    }
    return_vec
}
