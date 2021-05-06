// fn bin_pixels(img: &DynamicImage, platform: &Platform) -> Vec<(u8, u32)> {
//     let doms = construct_dominoes();

//     // let mut buckets: HashMap<u8, u32> = HashMap::new();
//     let num_buckets = 25;
//     let denominator = 255 / num_buckets;

//     let r_bucket: [u32; num_buckets] = [0; num_buckets];
//     let g_bucket: [u32; num_buckets] = [0; num_buckets];
//     let b_bucket: [u32; num_buckets] = [0; num_buckets];

//     // let median_radius = 10;
//     // let testblur = imageproc::filter::median_filter(&img.to_rgb(), median_radius, median_radius);
//     // testblur.save(format!("tests/median_filter.jpg")).unwrap();

//     // let mut pixel_histo: HashMap<(u8, u8, u8), u32> = HashMap::new();
//     for pixel in img.pixels().into_iter() {
//         let (r, g, b) = (pixel[2], pixel[1], pixel[0]);
//         r_bucket[r / denominator] += 1;
//         g_bucket[g / denominator] += 1;
//         b_bucket[b / denominator] += 1;

//         // *buckets.entry(dom_range.value).or_insert(0) += 1;
//     }
// }
