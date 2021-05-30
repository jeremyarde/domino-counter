use std::io::Cursor;

use image::{DynamicImage, GenericImageView};
// use tract_nnef::prelude::*;
use tract_flavour::{
    prelude::*,
    tract_hir::{internal::ToDim, tract_ndarray::Array},
};
// use tract_onnx::prelude::*;

pub fn model_loading(
    image_one: &DynamicImage,
    image_two: &DynamicImage,
) -> TractResult<(f32, i32)> {
    // let loaded_model = tract_onnx::onnx()
    // let nnef = tract_nnef::nnef();
    // let loaded_model = nnef
    //     .model_for_read(&mut Cursor::new(model))?
    //     // .with_input_fact(
    //     //     0,
    //     //     InferenceFact::dt_shape(f32::datum_type(), tvec!(1, 224, 224, 3)),
    //     // )?
    //     .into_optimized()?
    //     .into_runnable()?;

    // let mut model = include_bytes!("..\\model_building\\domino-onnx_2021-05-29.quant.onnx");
    let mut model = include_bytes!("..\\model_building\\dominoes-torch_2021-05-29.onnx");

    // let loaded_model = tract_onnx::onnx()
    // let loaded_model = tract_flavour::onnx()
    //     .model_for_read(&mut Cursor::new(model))?
    //     // .with_input_fact(
    //     //     0,
    //     //     InferenceFact::dt_shape(f32::datum_type(), tvec!(1, 224, 224, 3)),
    //     // )?
    //     .into_optimized()?
    //     .into_runnable()?;
    let batch = Symbol::new('N');
    let loaded_model = tract_flavour::onnx()
        .model_for_read(&mut Cursor::new(model))?
        .with_input_fact(
            0,
            InferenceFact::dt_shape(f32::datum_type(), tvec!(2, 3, 224, 224)),
        )?
        // .with_output_fact(0, InferenceFact::default())?
        .into_optimized()?
        .into_runnable()?;

    let mut images = vec![];
    let resized_img_one = image_one
        .resize_exact(224, 224, image::imageops::FilterType::Triangle)
        .to_rgb8();
    let resized_img_two = image_two
        .resize_exact(224, 224, image::imageops::FilterType::Triangle)
        .to_rgb8();

    images.push(resized_img_one);
    images.push(resized_img_two);

    // println!("w: {}, h: {}", &resized_img.width(), &resized_img.height());

    // Make a Tensor out of it.
    let batch_size = images.len();
    // let input: Tensor =
    //     tract_ndarray::Array4::from_shape_fn((batch_size, 3, 224, 224), |(n, c, y, x)| {
    //         (images[n][(x as _, y as _)][c] as f32 / 255.0 - mean) / std
    //     })
    //     .into();
    let mean = Array::from_shape_vec((1, 3, 1, 1), vec![0.485, 0.456, 0.406])?;
    let std = Array::from_shape_vec((1, 3, 1, 1), vec![0.229, 0.224, 0.225])?;

    let input: Tensor =
        ((tract_ndarray::Array4::from_shape_fn((2, 3, 224, 224), |(n, c, y, x)| {
            images[n][(x as _, y as _)][c] as f32 / 255.0
        }) - mean)
            / std)
            .into();
    println!("{:?}", input.datum_type());

    // Run the model on the input.
    let result = loaded_model.run(tvec!(input))?;

    // Find the max value with its index.
    let best = result[0]
        .to_array_view::<f32>()?
        .iter()
        .cloned()
        .zip(1..)
        .max_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    println!("Best prediction: {:?}", best);

    Ok(best.unwrap())
}
