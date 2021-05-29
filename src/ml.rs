use std::io::Cursor;

use tract_nnef::prelude::*;
use tract_onnx::prelude::*;

pub fn model_loading(image_bytes: &[u8]) -> TractResult<(f32, i32)> {
    let mut model = include_bytes!("..\\model_building\\domino_model.onnx");
    // let loaded_model = tract_onnx::onnx()
    let nnef = tract_nnef::nnef();
    let loaded_model = nnef
        .model_for_read(&mut Cursor::new(model))?
        // .with_input_fact(
        //     0,
        //     InferenceFact::dt_shape(f32::datum_type(), tvec!(1, 224, 224, 3)),
        // )?
        .into_optimized()?
        .into_runnable()?;

    let img = image::load_from_memory(image_bytes)?.to_rgb8();
    let resized = image::imageops::resize(&img, 224, 224, image::imageops::FilterType::Nearest);
    // Make a Tensor out of it.
    let img: Tensor = tract_ndarray::Array4::from_shape_fn((1, 224, 224, 3), |(_, y, x, c)| {
        resized[(x as _, y as _)][c] as f32 / 255.0
    })
    .into();

    // Run the model on the input.
    let result = loaded_model.run(tvec!(img))?;
    // emit_log(
    //     context,
    //     session,
    //     &format!("Inference complete. Traversing results graph to find a best-confidence fit..."),
    // );

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
