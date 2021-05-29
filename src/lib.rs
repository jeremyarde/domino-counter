// mod ml::log;
mod log;
mod ml;
// mod log::{log};
// mod log::Platform;

extern crate image;
extern crate imageproc;

use image::{
    math, DynamicImage, GenericImage, GenericImageView, ImageBuffer, Luma, Pixel, Rgb, RgbImage,
    Rgba,
};
use imageproc::{contours::find_contours, edges::canny};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, convert::TryInto, io::Cursor, ops::Range, path::Path, usize};
// use utils::set_panic_hook;
use wasm_bindgen::prelude::*;
use web_sys::console;

#[derive(Debug)]
struct DominoImageSection {
    top: u32,
    bottom: u32,
    left: u32,
    right: u32,
    middle: u32,
}

#[derive(Debug)]
struct ColorRange {
    r: std::ops::Range<u8>,
    g: std::ops::Range<u8>,
    b: std::ops::Range<u8>,
}

impl Default for ColorRange {
    fn default() -> Self {
        ColorRange {
            r: (0..255),
            g: (0..255),
            b: (0..255),
        }
    }
}

#[derive(Debug)]
struct DominoRange {
    ratio: std::ops::Range<f32>,
    leeway: u8,
    color_range: ColorRange,
    value: u8,
}

impl Default for DominoRange {
    fn default() -> Self {
        DominoRange {
            ratio: (0.0..0.0),
            leeway: 10,
            color_range: Default::default(),
            value: 0,
        }
    }
}

fn construct_dominoes() -> Vec<DominoRange> {
    let dominoes = vec![
        DominoRange {
            ratio: (1.0..99.0),
            color_range: ColorRange {
                r: (240..240),
                g: (228..228),
                b: (200..200),
            },
            value: 0,
            ..Default::default()
        },
        DominoRange {
            ratio: (6.91..6.91),
            color_range: ColorRange {
                r: (40..100),
                g: (100..150),
                b: (100..190),
            },
            value: 1,
            ..Default::default()
        },
        DominoRange {
            ratio: (6.91..6.91),
            color_range: ColorRange {
                r: (65..85),
                g: (130..150),
                b: (25..45),
            },
            value: 2,
            ..Default::default()
        },
        DominoRange {
            ratio: (6.91..6.91),
            color_range: ColorRange {
                r: (190..255),
                g: (30..80),
                b: (30..70),
            },
            value: 3,
            ..Default::default()
        },
        DominoRange {
            ratio: (6.91..6.91),
            color_range: ColorRange {
                r: (135..155),
                g: (65..85),
                b: (30..50),
            },
            value: 4,
            ..Default::default()
        },
        DominoRange {
            ratio: (6.91..6.91),
            color_range: ColorRange {
                r: (15..35),
                g: (65..85),
                b: (120..160),
            },
            value: 5,
            ..Default::default()
        },
        DominoRange {
            ratio: (6.91..6.91),
            color_range: ColorRange {
                r: (200..255),
                g: (110..160),
                b: (0..50),
            },
            value: 6,
            ..Default::default()
        },
        DominoRange {
            ratio: (6.91..6.91),
            color_range: ColorRange {
                r: (180..210),
                g: (35..70),
                b: (80..130),
            },
            value: 7,
            ..Default::default()
        },
        DominoRange {
            ratio: (6.91..6.91),
            color_range: ColorRange {
                r: (15..60),
                g: (110..190),
                b: (110..160),
            },
            value: 8,
            ..Default::default()
        },
        DominoRange {
            ratio: (6.91..6.91),
            color_range: ColorRange {
                r: (55..100),
                g: (20..70),
                b: (40..110),
            },
            value: 9,
            ..Default::default()
        },
        DominoRange {
            ratio: (1.375..1.375),
            color_range: ColorRange {
                r: (215..255),
                g: (60..100),
                b: (0..40),
            },
            value: 10,
            ..Default::default()
        },
        DominoRange {
            ratio: (6.91..6.91),
            color_range: ColorRange {
                r: (120..150),
                g: (35..65),
                b: (50..85),
            },
            value: 11,
            ..Default::default()
        },
        DominoRange {
            ratio: (6.91..6.91),
            color_range: ColorRange {
                r: (130..170),
                g: (130..170),
                b: (120..140),
            },
            value: 12,
            ..Default::default()
        },
    ];

    dominoes
}

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

fn main(domino_filepath: &str) {
    log::logger(
        String::from("Greetings from main!"),
        &log::Platform::windows,
    );
    /*
    todo:
    - wasm compatible
        - client side
        - sending photos via wasm to rust:
        <https://www.reddit.com/r/rust/comments/czn7qm/sending_an_image_from_js_to_rust_wasm/>
    cli todos:
    - image (path)
    - domino coords
    - domino orientation

    misc todos:
    - loading domino colors from config
    - setting/update domino colors
    - better background detection
    - better domino detection - ML?
        - position
        - color

     */

    // manipulate_image()
    // count_circles();

    // let domino_filepath = "dominoes/IMG-20210324-WA0002_landscape.jpg"; /* 54 */
    println!("{}", domino_filepath); //"dominoes/Screenshot_20210309-204319_Photos~4.jpg"

    let image = image::open(domino_filepath).unwrap();
    let (result, result_string) = find_dominos(image, log::Platform::windows, false);

    println!("Result: {}", result);

    // Reading all files in folder
    // let folder_path = Path::new("dominoes/eval/");
    // for file in std::fs::read_dir(folder_path).unwrap() {
    //     println!("{:?}", file);
    //     match file {
    //         Ok(x) => {
    //             println!("{:?}", x.file_name());
    //             // let mut path = Path::new("./dominoes/eval/");
    //             let filepath = &folder_path.join(x.file_name().to_str().unwrap());

    //             // find all domino coordinates, send to find_domino?

    //             find_domino(filepath.to_str().unwrap());
    //         }
    //         Err(_) => {
    //             println!("No file available");
    //         }
    //     };
    // }
}

#[derive(Serialize, Deserialize)]
struct DominoResult {
    value: u32,
    string_rep: String,
}

#[wasm_bindgen]
pub fn count_dominoes_from_base64(
    buffer: &[u8],
    width: u32,
    height: u32,
    cropped: bool,
) -> JsValue {
    use web_sys::console;

    // utils::set_panic_hook();

    // let actual_filepath = filepath.as_string().unwrap().as_str();

    // let filepath = "dominoes/IMG-20210324-WA0000.jpg";

    // let result = find_dominos("string");
    // let result = 0;
    log::logger(String::from("Starting to process"), &log::Platform::wasm);

    // let constructed_image: ImageBuffer<u8> = ImageBuffer::from_raw(width, height, buffer);
    let mut offset_multiple = 0;
    let reconstructed_image = ImageBuffer::from_fn(width, height, |x, y| {
        let slice = &buffer[offset_multiple * 4..offset_multiple * 4 + 4];
        offset_multiple += 1;
        image::Rgb([slice[0], slice[1], slice[2]])
    });

    log::logger(
        format!(
            "Reconstructed image H: {}, W: {}.",
            reconstructed_image.width(),
            reconstructed_image.height()
        ),
        &log::Platform::wasm,
    );

    let t = DynamicImage::ImageRgb8(reconstructed_image);

    log::logger(
        format!("test image H: {}, W: {}.", t.width(), t.height()),
        &log::Platform::wasm,
    );

    let (result, result_string) = find_dominos(t, log::Platform::wasm, cropped);

    let domino_result = DominoResult {
        value: result,
        string_rep: result_string,
    };

    return JsValue::from_serde(&domino_result).unwrap();
}

fn detect_domino_edges_eval_data(image: &DynamicImage) -> DominoImageSection {
    let height = image.height();
    let width = image.width();

    let result = DominoImageSection {
        top: 0,
        bottom: height,
        left: 0,
        right: width,
        middle: height / 2,
    };

    let mut img_clone = image.clone();
    draw_domino_lines(&mut img_clone, &result, &[]);

    result
}

fn detect_inner_domino_edges(
    _image: &mut DynamicImage,
    dom_section: &DominoImageSection,
    platform: &log::Platform,
) -> Vec<u32> {
    // let mut result = vec![];

    // dominos are likely about 1:2 (width:height), so
    // we should be able to find appropriately
    let dom_height = dom_section.bottom - dom_section.top;
    let dom_width = dom_section.right - dom_section.left;

    log::logger(
        format!("Domino H: {}, W: {}.", dom_height, dom_width),
        platform,
    );

    let num_dominos = ((dom_width as f32 / dom_height as f32) * 2.0).round() as u8;

    log::logger(format!("Number of dominoes: {}", num_dominos), platform);

    // inner edges of dominoes
    let dom_width = dom_section.right - dom_section.left;
    // let dom_count = 8;

    let result = (0..=num_dominos)
        .into_iter()
        .map(|x| (dom_width / num_dominos as u32) * x as u32 + dom_section.left)
        .collect();

    println!("result: {:?}", result);
    result
}

fn detect_outer_domino_edges(
    image: &mut DynamicImage,
    platform: &log::Platform,
) -> DominoImageSection {
    // let pixels: Vec<i32> = vec![-10, -5, 0, 5, 10];
    let height = image.height();
    let width = image.width();
    // let mut topedge = 0;
    // let mut bottomedge = 0;

    let mut domino_image_section = DominoImageSection {
        top: 0,
        bottom: 0,
        left: 0,
        right: 0,
        middle: 0,
    };

    let gray_img = image.grayscale().as_mut_luma8().unwrap().clone();
    let edges: ImageBuffer<Luma<u8>, Vec<u8>> = canny(&gray_img, 70.0, 100.0);
    // edges.save("tests/canny_edges.png").unwrap();

    // finding the bottom of the domino
    domino_image_section.bottom = find_edge(
        (0..height as u32).into_iter().rev(),
        // height..0,
        &edges,
        Direction::Vertical,
    );

    // find top
    domino_image_section.top = find_edge(
        (0..height as u32).into_iter(),
        // height..0,
        &edges,
        Direction::Vertical,
    );

    domino_image_section.left = find_edge(
        (0..width as u32).into_iter(),
        // height..0,
        &edges,
        Direction::Horizontal,
    );
    domino_image_section.right = find_edge(
        (0..width as u32).into_iter().rev(),
        // height..0,
        &edges,
        Direction::Horizontal,
    );

    domino_image_section.middle = ((domino_image_section.bottom - domino_image_section.top) / 2
        + domino_image_section.top) as u32;

    // lines_image.save("tests/found_squares.png").unwrap();

    log::logger(
        format!("Domino edges: {:#?}.", domino_image_section),
        platform,
    );

    domino_image_section
}

enum Direction {
    Horizontal,
    Vertical,
}

fn find_edge(
    // (start, end): (u32, u32),
    iterator: impl Iterator<Item = u32>,
    edge_image: &ImageBuffer<Luma<u8>, Vec<u8>>,
    direction: Direction,
) -> u32 {
    let mut found_edge: u32 = 0;
    let pixels: Vec<i32> = vec![-10, -5, 0, 5, 10];

    for pixel_location in iterator {
        // for pixel_location in range {
        let pixel_sample: Vec<Rgb<u8>> = match direction {
            Direction::Vertical => {
                let middle = edge_image.width() / 2;
                pixels
                    .iter()
                    .map(|_x| edge_image.get_pixel(middle, pixel_location as u32).to_rgb())
                    .collect()
            }
            Direction::Horizontal => {
                let middle = edge_image.height() / 2;
                pixels
                    .iter()
                    .map(|_x| {
                        edge_image
                            .get_pixel(pixel_location as u32, middle as u32)
                            .to_rgb()
                    })
                    .collect()
            }
        };

        if pixel_sample.iter().any(|x| x[0] == 255) {
            found_edge = pixel_location as u32;
            break;
        }
    }
    found_edge
}

fn draw_domino_lines(
    image: &mut DynamicImage,
    dom_section: &DominoImageSection,
    dom_inner_edges: &[u32],
) {
    let line_colour = Rgba([122, 255, 0, 1]);

    // top line
    imageproc::drawing::draw_line_segment_mut(
        image,
        (dom_section.left as f32, dom_section.top as f32),
        (dom_section.right as f32, dom_section.top as f32),
        line_colour,
    );

    // bottom line
    imageproc::drawing::draw_line_segment_mut(
        image,
        (dom_section.left as f32, dom_section.bottom as f32 as f32),
        (dom_section.right as f32, dom_section.bottom as f32 as f32),
        line_colour,
    );

    // left line
    imageproc::drawing::draw_line_segment_mut(
        image,
        (dom_section.left as f32, dom_section.top as f32),
        (dom_section.left as f32, dom_section.bottom as f32),
        line_colour,
    );

    // right line
    imageproc::drawing::draw_line_segment_mut(
        image,
        (dom_section.right as f32 as f32, dom_section.top as f32),
        (dom_section.right as f32 as f32, dom_section.bottom as f32),
        line_colour,
    );

    // middle line
    let middle_point = ((dom_section.bottom - dom_section.top) / 2) as u32 + (dom_section.top);
    imageproc::drawing::draw_line_segment_mut(
        image,
        (dom_section.left as f32, middle_point as f32),
        (dom_section.right as f32, middle_point as f32),
        line_colour,
    );
    let middle_point = ((dom_section.bottom - dom_section.top) / 2) as u32 + (dom_section.top);
    imageproc::drawing::draw_line_segment_mut(
        image,
        (dom_section.left as f32, (middle_point + 5) as f32),
        (dom_section.right as f32, (middle_point + 5) as f32),
        line_colour,
    );
    let middle_point = ((dom_section.bottom - dom_section.top) / 2) as u32 + (dom_section.top);
    imageproc::drawing::draw_line_segment_mut(
        image,
        (dom_section.left as f32, (middle_point - 5) as f32),
        (dom_section.right as f32, (middle_point - 5) as f32),
        line_colour,
    );

    for inner_edge in dom_inner_edges {
        imageproc::drawing::draw_line_segment_mut(
            image,
            (*inner_edge as f32, dom_section.top as f32),
            (*inner_edge as f32, dom_section.bottom as f32),
            line_colour,
        );
    }

    // image.save("tests/found_squares.png").unwrap();
}

fn find_dominos(mut image: DynamicImage, platform: log::Platform, cropped: bool) -> (u32, String) {
    bin_pixels(&image, &platform);

    let mut dominos_found: Vec<(u8, u8)> = vec![];
    let mut total_value: u32 = 0;
    // let mut img = image;
    // println!("Trying to open: {}", image_path);
    // let mut img = image::open(image_path).unwrap();

    // Always landscape
    image = match cropped {
        true => {
            if image.height() / image.width() > 1 {
                log::logger(String::from("Rotating image (cropped)"), &platform);
                image = image.rotate270();
            }
            image
        }
        false => {
            if image.height() > image.width() {
                log::logger(String::from("Rotating image (non-cropped)"), &platform);
                image = image.rotate270();
            }
            image
        }
    };

    for x in 0..5 {
        // println!("pixel x ({}), 0: {:?}", x, image.get_pixel(x, 0));
        log::logger(
            format!("pixel x ({}), 0: {:?}", x, image.get_pixel(x, 0)),
            &platform,
        );
    }

    let domino = match cropped {
        true => DominoImageSection {
            top: 0,
            bottom: image.height(),
            left: 0,
            right: image.width(),
            middle: image.height() / 2,
        },
        false => detect_outer_domino_edges(&mut image, &platform),
    };

    let domino_inner_edges = detect_inner_domino_edges(&mut image, &domino, &platform);
    // let domino = detect_domino_edges_eval_data(&mut img);

    // draw all of the domino edges
    // draw_domino_lines(&mut img, &domino, &domino_inner_edges);

    // for each domino found, do the following
    for dom_num in 1..domino_inner_edges.len() {
        let left = domino_inner_edges[dom_num - 1];
        let right = domino_inner_edges[dom_num];

        let _img_clone = image.clone();

        let top_piece = image.crop(left, domino.top, right - left, domino.middle - domino.top);
        let bottom_piece = image.crop(
            left,
            domino.middle,
            right - left,
            domino.bottom - domino.middle,
        );
        // top_piece.save("tests/top.png").unwrap();
        // bottom_piece.save("tests/bottom.png").unwrap();

        log::logger(String::from("Top"), &platform);
        // count_most_common_pixels(&top_piece);
        let top_domino_buckets = count_pixel_ranges(&top_piece, &platform);
        let top_ratio = count_ratio(&top_piece, 180);

        log::logger(String::from("Bottom"), &platform);
        // count_most_common_pixels(&bottom_piece);
        let bottom_domino_buckets = count_pixel_ranges(&bottom_piece, &platform);
        let bottom_ratio = count_ratio(&bottom_piece, 180);

        // let top_domino: u8 = guess_domino(&top_domino_buckets, &top_ratio);
        let bottom_domino: u8 = guess_domino(&bottom_domino_buckets, &bottom_ratio);
        let top_domino: u8 = match ml::model_loading(top_piece.as_bytes()) {
            Ok((value, index)) => index as u8,
            Err(_) => 0,
        };

        if top_domino == 0 && bottom_domino == 0 {
            total_value += 50;
        } else {
            total_value += (top_domino + bottom_domino) as u32;
        }

        dominos_found.push((top_domino, bottom_domino));

        log::logger(String::from("Done analyzing!"), &platform);
    }

    let mut domino_string = String::new();
    for (dominos_top, dominos_bottom) in dominos_found.iter() {
        // println!("Domino: [{}/{}]", dominos_top, dominos_bottom);
        log::logger(
            format!("Domino: [{}/{}]", dominos_top, dominos_bottom),
            &platform,
        );
        domino_string.push_str(format!("[{}/{}]", dominos_top, dominos_bottom).as_str());
    }

    log::logger(
        format!("Finished counting! Results: {}", total_value),
        &platform,
    );

    (total_value, domino_string)
}

fn guess_domino(buckets: &[(u8, u32)], ratio: &f32) -> u8 {
    let count_threshold = 40;

    let guessed_value = match buckets.first() {
        Some((value, count)) => {
            if count > &count_threshold {
                *value
            } else {
                0
            }
        }
        None => 0,
    };

    guessed_value
}

fn get_range_bounds(range: &Range<u8>, leeway: u8) -> Range<u8> {
    let range_start = if (range.start as i16 - leeway as i16) > u8::MIN as i16
        && (range.start as i16 - leeway as i16) < u8::MAX as i16
    {
        range.start - leeway
    } else {
        0
    };

    let range_end = if (range.end as i16 + leeway as i16) < u8::MAX as i16
        && (range.end as i16 + leeway as i16) > u8::MIN as i16
    {
        range.end + leeway
    } else {
        255
    };

    Range {
        start: range_start,
        end: range_end,
    }
}

fn count_pixel_ranges(img: &DynamicImage, platform: &log::Platform) -> Vec<(u8, u32)> {
    let doms = construct_dominoes();

    let mut buckets: HashMap<u8, u32> = HashMap::new();

    let median_radius = 10;
    let testblur = imageproc::filter::median_filter(&img.to_bgra8(), median_radius, median_radius);
    // testblur.save(format!("tests/median_filter.jpg")).unwrap();

    // let mut pixel_histo: HashMap<(u8, u8, u8), u32> = HashMap::new();
    for pixel in testblur.pixels().into_iter() {
        let (r, g, b) = (pixel[2], pixel[1], pixel[0]);

        if is_black_pixel((r, g, b), None) || is_white_pixel((r, g, b), None) {
        } else {
            for dom_range in doms.iter() {
                if (get_range_bounds(&dom_range.color_range.r, dom_range.leeway)).contains(&r)
                    && get_range_bounds(&dom_range.color_range.g, dom_range.leeway).contains(&g)
                    && get_range_bounds(&dom_range.color_range.b, dom_range.leeway).contains(&b)
                {
                    *buckets.entry(dom_range.value).or_insert(0) += 1;
                }
            }
        }
    }

    let mut top_n: Vec<(u8, u32)> = buckets
        .iter()
        .map(|(&domino_value, &count)| (domino_value, count))
        .collect();
    top_n.sort_by(|(_, a), (_, b)| b.cmp(&a));

    // println!("Domino buckets:\n{:?}", top_n);
    log::logger(format!("Domino buckets:\n{:?}", top_n), &platform);

    top_n
}

fn count_most_common_pixels(img: &DynamicImage, platform: &log::Platform) {
    /*
    Instead of doing the histogram, maybe doing a <(r,g,b), u32>
    map would be better than per channel histogram.
    Would give better exact pixels, which can be used to determine other colors other than white.
     */
    let bucket_mod = 5;
    let median_radius = 10;
    let testblur = imageproc::filter::median_filter(&img.to_bgra8(), median_radius, median_radius);
    // testblur.save(format!("tests/median_filter.jpg")).unwrap();

    let mut pixel_histo: HashMap<(u8, u8, u8), u32> = HashMap::new();
    for pixel in testblur.pixels().into_iter() {
        let (r, g, b) = (pixel[2], pixel[1], pixel[0]);

        if !is_black_pixel((r, g, b), None) && !is_white_pixel((r, g, b), None) {
            *pixel_histo
                .entry((r % bucket_mod, g % bucket_mod, b % bucket_mod))
                .or_insert(0) += 1;
        }
    }

    let mut top_n: Vec<((u8, u8, u8), u32)> = pixel_histo
        .iter()
        .map(|(&rgb, &val)| {
            (
                (rgb.0 * bucket_mod, rgb.1 * bucket_mod, rgb.2 * bucket_mod),
                val,
            )
        })
        .collect();

    top_n.sort_by(|(_, a), (_, b)| a.cmp(&b));
    top_n = top_n.into_iter().take(10).collect();
    log::logger(format!("{:?}", top_n), &platform);
}

fn is_white_pixel((r, g, b): (u8, u8, u8), threshold: Option<u8>) -> bool {
    let white_pixel_threshold = match threshold {
        Some(_x) => threshold.unwrap(),
        None => 180,
    };

    r > white_pixel_threshold && g > white_pixel_threshold && b > white_pixel_threshold
}

fn is_black_pixel((r, g, b): (u8, u8, u8), threshold: Option<u8>) -> bool {
    let black_pixel_threshold = match threshold {
        Some(_) => threshold.unwrap(),
        None => 60,
    };

    r < black_pixel_threshold && g < black_pixel_threshold && b < black_pixel_threshold
}

fn count_ratio(top_domino: &DynamicImage, _white_pixel_threshold: u8) -> f32 {
    let mut white_pixels = 0;
    let mut non_white = 0;

    for (_x, _y, rgb) in top_domino.pixels().into_iter() {
        if is_white_pixel((rgb[0], rgb[1], rgb[2]), None) {
            white_pixels += 1;
        } else {
            non_white += 1;
        }
    }
    let ratio = white_pixels as f32 / non_white as f32;
    println!(
        "White: {}, Non-white: {}, Ratio: {}",
        white_pixels, non_white, &ratio
    );

    ratio
}

fn bin_pixels(img: &DynamicImage, platform: &log::Platform) {
    // let doms = construct_dominoes();

    // let mut buckets: HashMap<u8, u32> = HashMap::new();
    const NUM_BUCKETS: usize = 26;
    let denominator = (255 as f32 / NUM_BUCKETS as f32).ceil() as usize;

    let mut r_bucket: [u32; NUM_BUCKETS] = [0; NUM_BUCKETS];
    let mut g_bucket: [u32; NUM_BUCKETS] = [0; NUM_BUCKETS];
    let mut b_bucket: [u32; NUM_BUCKETS] = [0; NUM_BUCKETS];

    // let median_radius = 10;
    // let testblur = imageproc::filter::median_filter(&img.to_rgb(), median_radius, median_radius);
    // testblur.save(format!("tests/median_filter.jpg")).unwrap();

    // let mut pixel_histo: HashMap<(u8, u8, u8), u32> = HashMap::new();
    for (x, y, pixel) in img.pixels() {
        // let (r, g, b) = (pixel[2], pixel[1], pixel[0]);
        let (r, g, b) = (pixel[0], pixel[1], pixel[2]);
        // log::logger(format!("Pixels r/g/b: {}/{}/{}.", r, g, b), &log::Platform);
        // log::logger(
        //     format!("bins: \n{:?}\n{:?}\n{:?}", r_bucket, g_bucket, b_bucket),
        //     &log::Platform,
        // );
        // log::logger(format!("{}", r as usize % denominator), &platform);
        r_bucket[r as usize / denominator] += 1;
        g_bucket[g as usize / denominator] += 1;
        b_bucket[b as usize / denominator] += 1;
    }

    log::logger(
        format!("bins: \n{:?}\n{:?}\n{:?}", r_bucket, g_bucket, b_bucket),
        &platform,
    );
}

fn testing_new_stuff(domino_filepath: &str) {
    let img = image::open(domino_filepath).unwrap();

    let median_radius = 5;
    let mut testblur =
        imageproc::filter::median_filter(&img.to_rgb8(), median_radius, median_radius);

    // let gray_img = img.grayscale().as_mut_luma8().unwrap().clone();
    // // let edges: ImageBuffer<Luma<u8>, Vec<u8>> = canny(&gray_img, 70.0, 100.0);
    // let testimg = find_contours::<u64>(&gray_img);
    // edges.save("tests/canny_edges.png").unwrap();

    // let mut new_image = testblur.clone();
    let mut image_ut = testblur;

    for rgba in image_ut.pixels_mut() {
        let brightness = rgba[0] as f32 * 0.299 + rgba[1] as f32 * 0.587 + rgba[2] as f32 * 0.114;
        // log::logger(format!("{}", brightness), &log::Platform::windows);
        match brightness {
            0.0..=80.0 => {
                rgba[0] = 0;
                rgba[1] = 0;
                rgba[2] = 0;
            }
            // 200.0..=255.0 => {
            //     rgba[0] = 0;
            //     rgba[1] = 0;
            //     rgba[2] = 0;
            // }
            _ => {}
        }
        // let brightness = rgba[0] as f32 * 0.21 + rgba[1] as f32 * 0.72 + rgba[2] as f32 * 0.07;
        // // log::logger(format!("{}", brightness), &log::Platform::windows);
        // match brightness {
        //     0.0..=80.0 => {
        //         rgba[0] = 0;
        //         rgba[1] = 0;
        //         rgba[2] = 0;
        //     }
        //     // 200.0..=255.0 => {
        //     //     rgba[0] = 0;
        //     //     rgba[1] = 0;
        //     //     rgba[2] = 0;
        //     // }
        //     _ => {}
        // }
        // log::logger("some message".to_string(), &log::Platform::windows);
    }
    image_ut
        .save("tests/testing_new_stuff-brightness.jpg")
        .unwrap();

    // gray_img.save("tests/testing_new_stuff-gray.jpg").unwrap();
    // testblur.save("tests/testing_new_stuff-blur.jpg").unwrap();
}

#[cfg(test)]
mod tests {
    use crate::{is_black_pixel, is_white_pixel, main, ml, testing_new_stuff};

    #[test]
    fn testing_model() {
        let domino_filepath = "model_building\\data\\train\\5\\5-2.jpg";

        let img = image::open(domino_filepath).unwrap();
        let result = ml::model_loading(&img.into_bytes());
        println!("{:?}", result);
    }

    #[test]
    fn test_new_things() {
        // let domino_filepath = "dominoes/halves/1_one/1-2.jpg";
        // let domino_filepath = "dominoes/IMG-20210324-WA0002_landscape.jpg";
        let domino_filepath = "dominoes/IMG-20210306-WA0001.jpg";

        testing_new_stuff(domino_filepath)
    }

    #[test]
    fn test_is_black_pixel() {
        assert_eq!(is_black_pixel((0, 0, 0), None), true);
        assert_eq!(is_black_pixel((40, 41, 39), Some(41)), false);
        assert_eq!(is_black_pixel((39, 20, 39), Some(40)), true);
    }

    #[test]
    fn test_is_white_pixel() {
        assert_eq!(is_white_pixel((180, 180, 180), Some(160)), true);
        assert_eq!(is_white_pixel((200, 0, 0), Some(199)), false);
        assert_eq!(is_white_pixel((100, 100, 100), Some(90)), true);
        assert_eq!(is_white_pixel((255, 255, 255), None), true);
    }

    #[test]
    fn test_rounding() {
        assert!((8.1 as f32).round() as i32 == 8);
        assert!((7.9 as f32).round() as i32 == 8);
    }

    #[test]
    fn test_main() {
        // let domino_filepath = "dominoes/5-9.jpg";
        // let domino_filepath = "dominoes/IMG-20210311-WA0002.jpg"; /* 148 */
        // let domino_filepath = "dominoes/IMG-20210306-WA0000.jpg"; // double domino line
        let domino_filepath = "dominoes/IMG-20210324-WA0000.jpg"; /* 73 */
        // let domino_filepath = "dominoes/5_test.jpg"; /* 73 */
        main(domino_filepath)
    }
}
