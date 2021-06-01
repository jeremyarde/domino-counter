// example: https://github.com/bminixhofer/tractjs/blob/main/src/lib.rs
// other example: https://github.com/JosephCatrambone/pixelbox/blob/4eec34673d1eeead98b62040c39b1ca18b631dd9/src/image_hashes/convnet.rs

use std::hash::Hash;
use std::io::Cursor;
use std::sync::{Arc, Mutex, Once};

use image::{DynamicImage, GenericImageView};
// use tract_nnef::prelude::*;
use serde_json::{Result, Value};
// use tract_flavour::tract_hir;
// use tract_flavour::tract_hir::infer::InferenceModelPatch;
use tract_flavour::{
    prelude::*,
    tract_hir::{internal::ToDim, tract_ndarray::Array},
};

// use wasm_bindgen::JsValue;
// #[no_mangle]
// pub fn load_model() -> TractResult<()> {
//     let batch_size = 1;
//     let mut model_file = include_bytes!("..\\model_building\\dominoes-torch_2021-05-29.onnx");

//     let loaded_model = tract_flavour::onnx()
//         .model_for_read(&mut Cursor::new(model))?
//         .with_input_fact(
//             0,
//             InferenceFact::dt_shape(f32::datum_type(), tvec!(batch_size, 3, 224, 224)),
//         )?
//         // .with_output_fact(0, InferenceFact::default())?
//         .into_optimized()?
//         .into_runnable()?;

//     unsafe { MODEL = loaded_model };
//     Ok(())
// }

// #[wasm_bindgen]
pub struct Model {
    // model: tract_hir::infer::InferenceSimplePlan<InferenceModel>,
    model: SimplePlan<
        TypedFact,
        Box<dyn TypedOp>,
        tract_flavour::prelude::Graph<TypedFact, Box<dyn TypedOp>>,
    >,
    classes: Value,
}

// #[wasm_bindgen]
impl Model {
    pub fn new() -> Model {
        // let model: tract_hir::infer::InferenceSimplePlan<InferenceModel>;
        let batch_size = 1;
        // let model_flavour = "dominoes-torch_2021-05-29-opt";
        let model_file = include_bytes!("..\\model_building\\simplified.quant.onnx");
        let class_file = include_bytes!("..\\model_building\\labels_dict.json");

        let loaded_model = tract_flavour::onnx()
            .model_for_read(&mut Cursor::new(model_file))
            .expect("Model file not working properly.")
            .with_input_fact(
                0,
                InferenceFact::dt_shape(f32::datum_type(), tvec!(batch_size, 3, 224, 224)),
            )
            .expect("Failed to specify input fact.")
            // .with_output_fact(0, InferenceFact::default())?
            .into_optimized()
            .expect("Failed to optimize model.")
            .into_runnable()
            .expect("Failed to make model runnable.");

        Model {
            model: loaded_model,
            classes: serde_json::from_slice(class_file).expect("Classes json file is not loading."),
        }
    }

    pub fn predict(&mut self, image: &DynamicImage) -> Result<(f32, i32)> {
        let mut images = vec![];
        let resized_img_one = image
            .resize_exact(224, 224, image::imageops::FilterType::Triangle)
            .to_rgb8();
        images.push(resized_img_one);

        let mean = Array::from_shape_vec((1, 3, 1, 1), vec![0.485, 0.456, 0.406])
            .expect("Mean matrix not created.");
        let std = Array::from_shape_vec((1, 3, 1, 1), vec![0.229, 0.224, 0.225])
            .expect("Std matrix not created.");

        let batch_size = images.len();
        let input: Tensor =
            ((tract_ndarray::Array4::from_shape_fn((batch_size, 3, 224, 224), |(n, c, y, x)| {
                images[n][(x as _, y as _)][c] as f32 / 255.0
            }) - mean)
                / std)
                .into();
        println!("{:?}", input.datum_type());

        // Run the model on the input.
        let result = self
            .model
            .run(tvec!(input))
            .expect("Model could not process inputs.");
        let mut predicted_class_real = 0;
        let mut predicted_prob = 0.0;
        // Find the max value with its index.
        let best: (f32, i32) = match result[0]
            .to_array_view::<f32>()
            .expect("Could not create array view from results.")
            .iter()
            .cloned()
            .zip(0..)
            .max_by(|a, b| a.0.partial_cmp(&b.0).unwrap())
        {
            Some((prob, index)) => {
                // let mut predicted_class = &classes[index.to_string()];
                let mut predicted_class = self.classes.get(index.to_string()).unwrap();
                predicted_prob = prob;

                println!("Best prediction: {:?}", predicted_class);
                println!("classes: {:?}", self.classes);
                println!("index: {:?}", index);
                println!("prob: {:?}", prob);

                predicted_class_real = predicted_class.as_str().unwrap().parse().unwrap();

                println!("predicted_class_real: {:?}", predicted_class_real);

                (prob, index as i32)
            }
            None => (0.0, 0),
        };

        Ok((predicted_prob, predicted_class_real as i32))
    }
    // model: tract_hir::infer::InferenceSimplePlan<InferenceModel>,
}

#[cfg(test)]
mod tests {
    use crate::ml;

    #[test]
    fn testing_model() {
        let domino_filepath = "model_building\\data\\train\\5\\5-2.jpg";

        // let img = image::load_from_memory(image_bytes)?.to_rgb8();
        // let resized = image::imageops::resize(&img, 224, 224, image::imageops::FilterType::Nearest);

        let img = image::open(domino_filepath).unwrap();

        let mut loaded_model = ml::Model::new();
        let result = loaded_model.predict(&img);
        // find_dominos

        println!("{:?}", result);
    }
}
