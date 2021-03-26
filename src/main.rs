extern crate image;
extern crate imageproc;
use core::f32;
use imageproc::edges::canny;

use image::{DynamicImage, GenericImageView, ImageBuffer, Luma, Pixel, Rgb, Rgba};

use std::{collections::HashMap, ops::Range, path::Path, usize};

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

// #[derive(Debug)]
// enum DominoPiece {
//     ZERO(DominoRange),
//     ONE(DominoRange),
//     TWO(DominoRange),
//     THREE(DominoRange),
//     FOUR(DominoRange),
//     FIVE(DominoRange),
//     SIX(DominoRange),
//     SEVEN(DominoRange),
//     EIGHT(DominoRange),
//     NINE(DominoRange),
//     TEN(DominoRange),
//     ELEVEN(DominoRange),
//     TWELVE(DominoRange),
// }

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
                r: (60..80),
                g: (100..130),
                b: (110..150),
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
                r: (190..220),
                g: (35..55),
                b: (30..50),
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
                r: (160..190),
                g: (35..70),
                b: (75..100),
            },
            value: 7,
            ..Default::default()
        },
        DominoRange {
            ratio: (6.91..6.91),
            color_range: ColorRange {
                r: (30..60),
                g: (110..150),
                b: (90..120),
            },
            value: 8,
            ..Default::default()
        },
        DominoRange {
            ratio: (6.91..6.91),
            color_range: ColorRange {
                r: (75..95),
                g: (25..55),
                b: (55..100),
            },
            value: 9,
            ..Default::default()
        },
        DominoRange {
            ratio: (1.375..1.375),
            color_range: ColorRange {
                r: (200..255),
                g: (80..120),
                b: (50..70),
            },
            value: 10,
            ..Default::default()
        },
        DominoRange {
            ratio: (6.91..6.91),
            color_range: ColorRange {
                r: (110..130),
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

// fn construct_domino_ranges()

fn main() {
    /*
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

    // let domino_filepath = "dominoes/eval/1-10.jpg";
    let domino_filepath = "dominoes/IMG-20210311-WA0002.jpg"; /* 148 */
    // let domino_filepath = "dominoes/IMG-20210306-WA0000.jpg"; // double domino line
    // let domino_filepath = "dominoes/IMG-20210324-WA0000.jpg"; /* 73 */
    // let domino_filepath = "dominoes/IMG-20210324-WA0002_landscape.jpg"; /* 54 */
    println!("{}", domino_filepath); //"dominoes/Screenshot_20210309-204319_Photos~4.jpg"
    find_dominos(domino_filepath);

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

    /*
    overall goals:
    1. find all dominoes in the picture
        - find edges (whiteish color, canny)
    2. separate dominoes into top/bottom
        - find separator black line
        - number of pixels in a row?
    3. determine which halves are what domino numbers
        - Domain expert
            - ratio of white to non-white pixels
            - pixels colors
            - bucket pixels using dominoes defined above
                - search for most likely domino based on values found in pixel buckets?
        - send all pixels of each half to ML algorithm
        - disregard the white color? - probably ML problem
    */
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
) -> Vec<u32> {
    let mut result = vec![];

    // dominos are likely about 1:2 (width:height), so
    // we should be able to find appropriately
    let dom_height = dom_section.bottom - dom_section.top;
    let dom_width = dom_section.right - dom_section.left;

    let num_dominos = ((dom_width as f32 / dom_height as f32) * 2.0).round() as u8;
    println!("Number of dominoes: {}", num_dominos);

    // inner edges of dominoes
    let dom_width = dom_section.right - dom_section.left;
    // let dom_count = 8;

    result = (0..=num_dominos)
        .into_iter()
        .map(|x| (dom_width / num_dominos as u32) * x as u32 + dom_section.left)
        .collect();

    println!("result: {:?}", result);
    result
}

fn detect_outer_domino_edges(image: &mut DynamicImage) -> DominoImageSection {
    // let pixels: Vec<i32> = vec![-10, -5, 0, 5, 10];
    let height = image.height();
    let width = image.width();
    let mut topedge = 0;
    let mut bottomedge = 0;

    let gray_img = image.grayscale().as_mut_luma8().unwrap().clone();
    let edges: ImageBuffer<Luma<u8>, Vec<u8>> = canny(&gray_img, 70.0, 100.0);
    edges.save("tests/canny_edges.png").unwrap();

    // finding the bottom of the domino
    bottomedge = find_edge(
        (0..height as u32).into_iter().rev(),
        // height..0,
        &edges,
        Direction::Vertical,
    );

    // find top
    topedge = find_edge(
        (0..height as u32).into_iter(),
        // height..0,
        &edges,
        Direction::Vertical,
    );

    // instead of this, we can find X consecutive pixels that are "black" (less than 40-50) (40,40,40)
    let middle: f32 = ((bottomedge - topedge) / 2 + topedge) as f32;

    let mut left = 0;
    let mut right = 0;

    left = find_edge(
        (0..width as u32).into_iter(),
        // height..0,
        &edges,
        Direction::Horizontal,
    );
    right = find_edge(
        (0..width as u32).into_iter().rev(),
        // height..0,
        &edges,
        Direction::Horizontal,
    );

    // lines_image.save("tests/found_squares.png").unwrap();

    let result = DominoImageSection {
        top: topedge,
        bottom: bottomedge,
        left,
        right,
        middle: middle as u32,
    };

    println!("Results of domino finding: {:?}", result);

    result
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

    // let mut iterator = range;
    // if reverse == true {
    //     iterator = range.rev().into_iter();
    // }
    // println!("start: {}, end: {}", start, end);
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
            println!("Found bottom edge at {}", pixel_location);
            // found_edge = Some(pixel_location as u32);
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

    // // inner edges of dominoes
    // let dom_width = dom_section.right - dom_section.left;
    // let dom_count = 8;

    for inner_edge in dom_inner_edges {
        // let line_loc_x = (dom_width / 8) * dom_num + dom_section.left;
        imageproc::drawing::draw_line_segment_mut(
            image,
            (*inner_edge as f32, dom_section.top as f32),
            (*inner_edge as f32, dom_section.bottom as f32),
            line_colour,
        );
    }

    image.save("tests/found_squares.png").unwrap();
}

fn find_dominos(image_path: &str) {
    let mut dominos_found: Vec<(u8, u8)> = vec![];
    let mut total_value: u32 = 0;

    println!("Trying to open: {}", image_path);
    let mut img = image::open(image_path).unwrap();

    // Always landscape
    if img.height() > img.width() {
        img = img.rotate270();
    }
    let domino = detect_outer_domino_edges(&mut img);

    let domino_inner_edges = detect_inner_domino_edges(&mut img, &domino);
    // let domino = detect_domino_edges_eval_data(&mut img);

    // draw all of the domino edges
    draw_domino_lines(&mut img, &domino, &domino_inner_edges);

    // for each domino found, do the following
    for dom_num in 1..domino_inner_edges.len() {
        let left = domino_inner_edges[dom_num - 1];
        let right = domino_inner_edges[dom_num];

        let _img_clone = img.clone();
        // let domino_piece =
        // img_clone.sub_image(left, domino.top, right - left, domino.middle - domino.top);

        // let top_piece = img.clone();
        let top_piece = img.crop(left, domino.top, right - left, domino.middle - domino.top);
        let bottom_piece = img.crop(
            left,
            domino.middle,
            right - left,
            domino.bottom - domino.middle,
        );
        // top_piece.save("tests/top.png").unwrap();
        // bottom_piece.save("tests/bottom.png").unwrap();

        println!("Top:");
        // count_most_common_pixels(&top_piece);
        let top_domino_buckets = count_pixel_ranges(&top_piece);
        let top_ratio = count_ratio(&top_piece, 180);

        println!("Bottom:");
        // count_most_common_pixels(&bottom_piece);
        let bottom_domino_buckets = count_pixel_ranges(&bottom_piece);
        let bottom_ratio = count_ratio(&bottom_piece, 180);

        let top_domino: u8 = guess_domino(&top_domino_buckets, &top_ratio);
        let bottom_domino: u8 = guess_domino(&bottom_domino_buckets, &bottom_ratio);

        if top_domino == 0 && bottom_domino == 0 {
            total_value += 50;
        } else {
            total_value += (top_domino + bottom_domino) as u32;
        }

        dominos_found.push((top_domino, bottom_domino));

        println!("Done analyzing.");
    }

    for (dominos_top, dominos_bottom) in dominos_found.iter() {
        println!("Domino: [{}/{}]", dominos_top, dominos_bottom);
    }

    println!("Finished counting! Results: {}", total_value);

    // let mut img_clone = img.clone();
    // let domino_piece = img_clone.sub_image(
    //     domino.left,
    //     domino.top,
    //     domino.right - domino.left,
    //     domino.middle - domino.top,
    // );

    // // let top_piece = img.clone();
    // let top_piece = img.crop(domino.left, domino.top, domino.right, domino.middle);
    // let bottom_piece = img.crop(domino.left, domino.middle, domino.right, domino.bottom);

    // println!("Top:");
    // // count_most_common_pixels(&top_piece);
    // count_pixel_ranges(&top_piece);
    // count_ratio(&domino_piece, 180);

    // let domino_piece = img_clone.sub_image(
    //     domino.left as u32,
    //     domino.middle as u32,
    //     domino.right - domino.left,
    //     domino.bottom - domino.middle,
    // );

    // println!("Bottom:");
    // // count_most_common_pixels(&bottom_piece);
    // count_pixel_ranges(&bottom_piece);
    // count_ratio(&domino_piece, 180);
}

fn guess_domino(buckets: &[(u8, u32)], ratio: &f32) -> u8 {
    let count_threshold = 200;

    let guessed_value = match buckets.first() {
        Some((value, count)) => {
            if count > &count_threshold {
                *value
            } else {
                // let number: u8 = match ratio {
                //     1.0..=1.1 => 9,
                //     1.35..=1.65 => 10,
                //     1.65..=1.70 => 7,
                //     1.70..=1.85 => 11,
                //     1.85..=1.99 => 6,
                //     2.0..=2.12 => 10,
                //     2.12..=2.2 => 5, // also 12
                //     2.7..=2.99 => 3,
                //     3.15..=3.7 => 4,
                //     4.0..=4.99 => 2,
                //     5.0..=6.0 => 1,
                //     8.0..=99.99 => 0,
                //     _ => {
                //         println!("Could not find, default to 0");
                //         0
                //     }
                // };
                0
            }
        }
        None => 0,
    };

    guessed_value
}

fn get_range_bounds(range: &Range<u8>, leeway: u8) -> Range<u8> {
    // println!("range: {:?}", range);
    // println!("leeway: {:?}", leeway);

    // println!("range start: {}, range end: {}", range.start, range.end);
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

fn count_pixel_ranges(img: &DynamicImage) -> Vec<(u8, u32)> {
    let doms = construct_dominoes();

    let mut buckets: HashMap<u8, u32> = HashMap::new();

    let median_radius = 10;
    let testblur = imageproc::filter::median_filter(&img.to_bgra8(), median_radius, median_radius);
    testblur.save(format!("tests/median_filter.jpg")).unwrap();

    // let mut pixel_histo: HashMap<(u8, u8, u8), u32> = HashMap::new();
    for pixel in testblur.pixels().into_iter() {
        let (r, g, b) = (pixel[2], pixel[1], pixel[0]);

        if is_black_pixel((r, g, b), None) || is_white_pixel((r, g, b), None) {
        } else {
            for dom_range in doms.iter() {
                // println!("{:?}", dom_range);
                // let new_range = ((dom_range.color_range.r.start - dom_range.leeway)..(dom_range.color_range.r.end + dom_range.leeway))

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

    println!("Domino buckets:\n{:?}", top_n);
    top_n
}

fn count_most_common_pixels(img: &DynamicImage) {
    /*
    Instead of doing the histogram, maybe doing a <(r,g,b), u32>
    map would be better than per channel histogram.
    Would give better exact pixels, which can be used to determine other colors other than white.
     */
    let bucket_mod = 5;
    let median_radius = 10;
    let testblur = imageproc::filter::median_filter(&img.to_bgra8(), median_radius, median_radius);
    testblur.save(format!("tests/median_filter.jpg")).unwrap();

    let mut pixel_histo: HashMap<(u8, u8, u8), u32> = HashMap::new();
    for pixel in testblur.pixels().into_iter() {
        let (r, g, b) = (pixel[2], pixel[1], pixel[0]);

        // pixel_histo
        //     .get_mut(&(pixel[0], pixel[1], pixel[2]))
        //     .unwrap() += 1;
        // println!("{:?}", (r, g, b));
        if !is_black_pixel((r, g, b), None) && !is_white_pixel((r, g, b), None) {
            // println!("Adding above");
            // println!("{:?}", (pixel[0] as u32, pixel[1] as u32, pixel[2] as u32));
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
    println!("{:?}", top_n);

    println!();
    // for channel in histo.
}

fn histogram(img: &DynamicImage) {
    let gaussian_blur = 5.0;

    let testblur = imageproc::filter::median_filter(&img.to_bgra8(), 5, 5);
    testblur
        .save(format!("tests/blur_{}.jpg", gaussian_blur))
        .unwrap();

    let histo = imageproc::stats::histogram(&testblur);

    let mut max_values: Vec<(usize, u32)>;
    for (i, channel) in histo.channels.into_iter().enumerate() {
        max_values = Vec::new();

        for (rgb_key, &value) in channel.iter().enumerate() {
            max_values.push((rgb_key, value));
            // println!("{:?}", max_values);
        }
        max_values.sort_by(|a, b| a.1.cmp(&b.1));

        // general white pixel filter
        max_values = max_values
            .iter()
            .cloned()
            .filter(|x| x.0 < 200)
            .collect::<Vec<(usize, u32)>>();

        match i {
            0 => {
                println!("R");

                max_values = max_values
                    .iter()
                    .cloned()
                    .filter(|x| x.0 < 240)
                    .collect::<Vec<(usize, u32)>>();
            }
            1 => {
                println!("G");
                max_values = max_values
                    .iter()
                    .cloned()
                    .filter(|x| x.0 < 220)
                    .collect::<Vec<(usize, u32)>>();
            }
            2 => {
                println!("B");
                max_values = max_values
                    .iter()
                    .cloned()
                    .filter(|x| x.0 < 200)
                    .collect::<Vec<(usize, u32)>>();
            }
            _ => (),
        }
        let top_n: Vec<(usize, u32)> = max_values.iter().cloned().rev().take(10).collect();

        println!("{:?}", top_n);
    }
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

fn manipulate_image() {
    // Use the open function to load an image from a Path.
    // `open` returns a `DynamicImage` on success.
    let img = image::open("dominoes/IMG-20210306-WA0000.jpg").unwrap();

    let output_dir = Path::new("tests");

    // println!("Grey");
    // let mut gray_img = img.grayscale().as_mut_luma8().unwrap().clone();
    // gray_img.save(&output_dir.join("grey.png")).unwrap();

    // println!("canny");
    // // Detect edges using Canny algorithm
    // let edges = canny(&gray_img, 70.0, 100.0);
    // let canny_path = output_dir.join("canny.png");
    // edges.save(&canny_path).unwrap();
    let white = Rgb::<u8>([255, 255, 255]);
    let _green = Rgb::<u8>([0, 255, 0]);
    let black = Rgb::<u8>([0, 0, 0]);

    let _filter_vals = vec![50, 80, 100, 110, 120, 150, 170, 190, 210];

    // for filter_val in filter_vals {
    //     let testmap = imageproc::map::map_colors(&img, |p| {
    //         if p[0] > filter_val && p[1] > filter_val && p[2] > filter_val {
    //             white
    //         } else {
    //             black
    //         }
    //     });
    //     testmap
    //         .save(&output_dir.join(format!("testmap_{}.png", filter_val)))
    //         .unwrap();
    // }
    let filter_high_threshold = 170;
    let filter_low_threshold = 80;

    // let boxfiltered = imageproc::fil
    let bw_img = imageproc::map::map_colors(&img, |p| {
        if (p[0] > filter_high_threshold
            && p[1] > filter_high_threshold
            && p[2] > filter_high_threshold)
            || (p[0] < filter_low_threshold
                && p[1] < filter_low_threshold
                && p[2] < filter_low_threshold)
        {
            white
        } else {
            black
        }
    });

    let bw_proper = DynamicImage::ImageRgb8(bw_img).into_luma8();
    println!("canny");
    // Detect edges using Canny algorithm
    let edges = canny(&bw_proper, 70.0, 100.0);
    let canny_path = output_dir.join("canny_v2.png");
    edges.save(&canny_path).unwrap();

    // let mut num_circles = OutputArray();
    // opencv::imgproc::hough_circles(
    //     &something,
    //     num_circles,
    //     HOUGH_GRADIENT,
    //     1.5,
    //     1.0,
    //     80.0,
    //     0.9,
    //     0,
    //     -1,
    // );

    bw_proper
        .save(&output_dir.join(format!("testmap_{}.png", filter_high_threshold)))
        .unwrap();
}

#[cfg(test)]
mod tests {
    use crate::{is_black_pixel, is_white_pixel};

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
}
