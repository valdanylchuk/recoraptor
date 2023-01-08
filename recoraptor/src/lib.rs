#[allow(dead_code)]

pub struct VectorOp;

impl VectorOp {
    pub fn dot(v: &[f32], that: &[f32]) -> f32 {
        assert_eq!(v.len(), that.len(), "vector size mismatch");

        let n = v.len();
        let mut sum = 0.0;
        for i in 0..n {
            sum = sum + v[i] * that[i];
        }
        sum
    }

    pub fn sub(v: &[f32], that: &[f32]) -> Vec<f32> {
        assert_eq!(v.len(), that.len(), "vector size mismatch");

        v.iter().zip(that.iter()).map(|(a, b)| a - b).collect()
    }

    pub fn add(v: &[f32], that: &[f32]) -> Vec<f32> {
        assert_eq!(v.len(), that.len(), "vector size mismatch");

        v.iter().zip(that.iter()).map(|(a, b)| a + b).collect()
    }

    pub fn squared_distance(x: &[f32], y: &[f32]) -> f32 {
        x.iter().zip(y.iter()).map(|(xi, yi)| { let d = xi - yi; d * d }).sum()
    }

    pub fn distance(x: &[f32], y: &[f32]) -> f32 {
        Self::squared_distance(x, y).sqrt()
    }
}

// // generated example
// pub fn add(left: usize, right: usize) -> usize {
//     left + right
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     // generated example
//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dot_product() {
        assert_eq!(VectorOp::dot(&[2.0, 3.0], &[4.0, 5.0]), 2.0 * 4.0 + 3.0 * 5.0);
        let result = std::panic::catch_unwind(|| VectorOp::dot(&[2.0, 3.0], &[4.0]));
        assert!(result.is_err());
    }

    #[test]
    fn test_distance() {
        assert_eq!(VectorOp::distance(&[0.0, 0.0], &[10.0, 0.0]), 10.0);
        assert_eq!(VectorOp::distance(&[0.0, 0.0], &[0.0, 10.0]), 10.0);
        assert_eq!(VectorOp::distance(&[0.0, 0.0], &[3.0, 4.0]), 5.0);
        assert_eq!(VectorOp::distance(&[0.0, 0.0], &[4.0, 3.0]), 5.0);
        assert_eq!(VectorOp::distance(&[3.0, 0.0], &[0.0, 4.0]), 5.0);
    }
}