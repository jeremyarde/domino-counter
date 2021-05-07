// fn test_ml() {
//     let (train, valid) = linfa_datasets::winequality()
//         .map_targets(|x| *x > 6)
//         .split_with_ratio(0.9);

//     // train SVM with nu=0.01 and RBF with eps=80.0
//     let model = Svm::params()
//         .nu_weight(0.01)
//         .gaussian_kernel(80.0)
//         .fit(&train)?;

//     // print model performance and number of SVs
//     println!("{}", model);
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         assert_eq!(2 + 2, 4);
//     }
// }
