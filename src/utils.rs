pub fn scal_mul_vec(scalar: i32, vec: &Vec<f64>) -> Vec<f64> {
    vec.iter().map(|x| x * scalar as f64).collect()
}
pub fn dot_prod_vecs(vec1: &Vec<f64>, vec2: &Vec<f64>) -> f64 {
    let dot_product: f64 = vec1.iter().zip(vec2.iter()).map(|(a, b)| a * b).sum();
    dot_product
}

pub fn add_vecs(vec1: &Vec<f64>, vec2: &Vec<f64>) -> Vec<f64> {
    let mut return_vec = vec![0.0; vec1.len()];
    for i in 0..vec1.len() {
        return_vec[i] = vec1[i] + vec2[i];
    }
    return_vec
}

pub fn normalize_vec(vec: &mut Vec<f64>) {
    let norm = vec.iter().map(|&x| x * x).sum::<f64>().sqrt();
    for elem in vec.iter_mut() {
        *elem /= norm;
    }
}