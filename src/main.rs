extern crate image;
extern crate imageproc;
use core::f32;
use imageproc::{
    definitions::Image,
    edges::canny,
    filter::{self, gaussian_blur_f32, median_filter, separable_filter},
};

use image::{
    imageops::blur, DynamicImage, GenericImage, GenericImageView, ImageBuffer, Luma, LumaA, Pixel,
    Rgb, Rgba, RgbaImage, SubImage,
};
use imageproc::hough;
use std::{collections::HashMap, path::Path};

struct DominoImageSection {
    top: u32,
    bottom: u32,
    left: u32,
    right: u32,
    middle: u32,
}

struct ColorRange {
    r: std::ops::Range<u32>,
    g: std::ops::Range<u32>,
    b: std::ops::Range<u32>,
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

struct DominoRange {
    ratio: std::ops::Range<f32>,
    leeway: u32,
    color_range: ColorRange,
    value: u32,
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

enum DominoPiece {
    ZERO(DominoRange),
    ONE(DominoRange),
    TWO(DominoRange),
    THREE(DominoRange),
    FOUR(DominoRange),
    FIVE(DominoRange),
    SIX(DominoRange),
    SEVEN(DominoRange),
    EIGHT(DominoRange),
    NINE(DominoRange),
    TEN(DominoRange),
    ELEVEN(DominoRange),
    TWELVE(DominoRange),
}

fn construct_dominoes() -> Vec<DominoPiece> {
    let mut dominoes = vec![];

    // 1.0..=1.1 => 9,
    // 1.55..=1.65 => 10,
    // 1.65..=1.70 => 7,
    // 1.70..=1.85 => 11,
    // 2.0..=2.12 => 10,
    // 2.12..=2.2 => 5, // also 12
    // 2.7..=2.85 => 3,
    // 3.0..=3.15 => 2,

    dominoes.push(DominoPiece::ZERO(DominoRange {
        ratio: (1.0..99.0),
        color_range: ColorRange {
            r: (240..240),
            g: (228..228),
            b: (200..200),
        },
        value: 0,
        ..Default::default()
    }));
    dominoes.push(DominoPiece::ONE(DominoRange {
        ratio: (6.91..6.91),
        color_range: ColorRange {
            r: (50..75),
            g: (100..130),
            b: (100..140),
        },
        value: 1,
        ..Default::default()
    }));
    dominoes.push(DominoPiece::TWO(DominoRange {
        ratio: (6.91..6.91),
        color_range: ColorRange {
            r: (50..75),
            g: (100..130),
            b: (100..140),
        },
        value: 2,
        ..Default::default()
    }));
    dominoes.push(DominoPiece::THREE(DominoRange {
        ratio: (6.91..6.91),
        color_range: ColorRange {
            r: (190..255),
            g: (20..45),
            b: (30..40),
        },
        value: 3,
        ..Default::default()
    }));
    dominoes.push(DominoPiece::FOUR(DominoRange {
        ratio: (6.91..6.91),
        color_range: ColorRange {
            r: (135..150),
            g: (45..65),
            b: (20..40),
        },
        value: 4,
        ..Default::default()
    }));
    dominoes.push(DominoPiece::FIVE(DominoRange {
        ratio: (6.91..6.91),
        color_range: ColorRange {
            r: (25..35),
            g: (60..80),
            b: (110..130),
        },
        value: 5,
        ..Default::default()
    }));
    dominoes.push(DominoPiece::SIX(DominoRange {
        ratio: (6.91..6.91),
        color_range: ColorRange {
            r: (200..255),
            g: (90..125),
            b: (0..40),
        },
        value: 6,
        ..Default::default()
    }));
    dominoes.push(DominoPiece::SEVEN(DominoRange {
        ratio: (6.91..6.91),
        color_range: ColorRange {
            r: (160..180),
            g: (35..50),
            b: (60..75),
        },
        value: 7,
        ..Default::default()
    }));
    dominoes.push(DominoPiece::EIGHT(DominoRange {
        ratio: (6.91..6.91),
        color_range: ColorRange {
            r: (50..75),
            g: (100..130),
            b: (100..140),
        },
        value: 1,
        ..Default::default()
    }));
    dominoes.push(DominoPiece::NINE(DominoRange {
        ratio: (6.91..6.91),
        color_range: ColorRange {
            r: (50..75),
            g: (100..130),
            b: (100..140),
        },
        value: 1,
        ..Default::default()
    }));
    dominoes.push(DominoPiece::TEN(DominoRange {
        ratio: (1.375..1.375),
        color_range: ColorRange {
            r: (200..255),
            g: (60..140),
            b: (0..50),
        },
        value: 1,
        ..Default::default()
    }));
    dominoes.push(DominoPiece::ELEVEN(DominoRange {
        ratio: (6.91..6.91),
        color_range: ColorRange {
            r: (50..75),
            g: (100..130),
            b: (100..140),
        },
        value: 1,
        ..Default::default()
    }));
    dominoes.push(DominoPiece::TWELVE(DominoRange {
        ratio: (6.91..6.91),
        color_range: ColorRange {
            r: (50..75),
            g: (100..130),
            b: (100..140),
        },
        value: 1,
        ..Default::default()
    }));

    return dominoes;
}

// fn construct_domino_ranges()

fn main() {
    // let doms = construct_dominoes();

    // manipulate_image()
    // count_circles();
    let domino_pic_path = "dominoes/eval/1-10.jpg";
    println!("{}", domino_pic_path); //"dominoes/Screenshot_20210309-204319_Photos~4.jpg"
    find_domino(domino_pic_path);

    /*
    overall goals:
    1. find all dominoes in the picture
        - find edges (whiteish color, canny)
    2. separate dominoes into top/bottom
        - find separator black line
        - number of pixels in a row?
    3. determine which halves are what domino numbers
        - ratio of white to non-white pixels
        - pixels colors
        - send all pixels of each half to ML algorithm
        - disregard the white color? - probably ML problem
    */
}

fn detect_domino_edges_eval_data(image: DynamicImage) -> DominoImageSection {
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
    draw_domino_lines(&mut img_clone, &result);

    return result;
}

fn detect_domino_edges(image: &mut DynamicImage) -> DominoImageSection {
    let height = image.height();
    let width = image.width();
    let mut topedge = 0;
    let mut bottomedge = 0;

    let mut gray_img = image.grayscale().as_mut_luma8().unwrap().clone();
    let edges: ImageBuffer<Luma<u8>, Vec<u8>> = canny(&gray_img, 70.0, 100.0);
    edges.save("tests/canny_edges.png").unwrap();

    // finding the top of the dominoe
    for y in (0..height).rev() {
        let pixel = edges.get_pixel(width / 2, y).to_rgb();
        if pixel[1] == 255 {
            println!("Found bottom edge at {}", y);
            bottomedge = y;
            break;
        }
    }

    for y in 0..height {
        let pixel = edges.get_pixel(width / 2, y).to_rgb();
        if pixel[1] == 255 {
            println!("Found top edge at {}", y);
            topedge = y;
            break;
        }
    }

    // instead of this, we can find X consecutive pixels that are "black" (less than 40-50) (40,40,40)
    let middle: f32 = ((bottomedge - topedge) / 2 + topedge) as f32;

    let mut left = 0;
    let mut right = 0;

    for x in 0..width {
        let pixel = edges.get_pixel(x, middle as u32).to_rgb();
        if pixel[1] == 255 {
            println!("Found left edge at {}", x);
            left = x;
            break;
        }
    }

    for x in (0..width).rev() {
        let pixel = edges.get_pixel(x, middle as u32).to_rgb();
        if pixel[1] == 255 {
            println!("Found right edge at {}", x);
            right = x;
            break;
        }
    }

    // lines_image.save("tests/found_squares.png").unwrap();

    let mut result = DominoImageSection {
        top: topedge,
        bottom: bottomedge,
        left: left,
        right: right,
        middle: middle as u32,
    };

    draw_domino_lines(image, &result);

    return result;
}

fn draw_domino_lines(image: &mut DynamicImage, dom_section: &DominoImageSection) {
    let line_colour = Rgba([122, 255, 0, 1]);

    // top line
    imageproc::drawing::draw_line_segment_mut(
        image,
        (0.0, 0.0),
        (dom_section.right as f32, 0 as f32),
        line_colour,
    );

    // bottom line
    imageproc::drawing::draw_line_segment_mut(
        image,
        (0.0 as f32, dom_section.bottom as f32 - 1.0 as f32),
        (
            dom_section.right as f32,
            dom_section.bottom as f32 - 1.0 as f32,
        ),
        line_colour,
    );

    // left line
    imageproc::drawing::draw_line_segment_mut(
        image,
        (0.0, 0.0),
        (0.0, dom_section.bottom as f32),
        line_colour,
    );

    // right line
    imageproc::drawing::draw_line_segment_mut(
        image,
        (dom_section.right as f32 - 1.0 as f32, 0.0),
        (
            dom_section.right as f32 - 1.0 as f32,
            dom_section.bottom as f32,
        ),
        line_colour,
    );

    // middle line
    let middle_point = (dom_section.bottom / 2) as f32;
    imageproc::drawing::draw_line_segment_mut(
        image,
        (0.0 as f32, middle_point),
        (dom_section.right as f32, middle_point),
        line_colour,
    );

    image.save("tests/found_squares.png").unwrap();
}

fn find_domino(image_path: &str) {
    // let domino_pic_path = "dominoes/eval/2-3.jpg"; //"dominoes/Screenshot_20210309-204319_Photos~4.jpg"
    let mut img = image::open(image_path).unwrap();

    // println!("{:#?}", &histo);
    // something
    // let domino = detect_domino_edges(&img);
    let domino = detect_domino_edges_eval_data(img.clone());

    let mut img_clone = img.clone();
    let domino_piece = img_clone.sub_image(
        domino.left,
        domino.top,
        domino.right - domino.left,
        domino.middle - domino.top,
    );

    // let top_piece = img.clone();
    let top_piece = img.crop(domino.left, domino.top, domino.right, domino.middle);
    let bottom_piece = img.crop(domino.left, domino.middle, domino.right, domino.bottom);

    println!("Top:");
    count_most_common_pixels(&top_piece);
    count_ratio(&domino_piece, 180);

    let domino_piece = img_clone.sub_image(
        domino.left as u32,
        domino.middle as u32,
        domino.right - domino.left,
        domino.bottom - domino.middle,
    );

    println!("Bottom:");
    count_most_common_pixels(&bottom_piece);
    count_ratio(&domino_piece, 180);
}

fn count_most_common_pixels(img: &DynamicImage) {
    /*
    Instead of doing the histogram, maybe doing a <(r,g,b), u32>
    map would be better than per channel histogram.
    Would give better exact pixels, which can be used to determine other colors other than white.
     */

    let gaussian_blur = 5.0;
    // let testblur = imageproc::filter::gaussian_blur_f32(&img.to_bgra8(), gaussian_blur);
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
    println!();
    // for channel in histo.
}

fn is_white_pixel(pixel: Rgba<u8>) -> bool {
    let white_pixel_threshold = 180;

    return pixel[0] > white_pixel_threshold
        && pixel[1] > white_pixel_threshold
        && pixel[2] > white_pixel_threshold;
}

fn count_ratio(top_domino: &SubImage<&mut DynamicImage>, white_pixel_threshold: u8) {
    let mut white_pixels = 0;
    let mut non_white = 0;

    for (_x, _y, rgb) in top_domino.pixels().into_iter() {
        if is_white_pixel(rgb) {
            white_pixels += 1;
        } else {
            non_white += 1;
        }
    }
    let ratio = (white_pixels as f32 / non_white as f32);
    println!(
        "White: {}, Non-white: {}, Ratio: {}",
        white_pixels, non_white, &ratio
    );

    let number: u32 = match ratio {
        1.0..=1.1 => 9,
        1.55..=1.65 => 10,
        1.65..=1.70 => 7,
        1.70..=1.85 => 11,
        2.0..=2.12 => 10,
        2.12..=2.2 => 5, // also 12
        2.7..=2.85 => 3,
        3.0..=3.15 => 2,
        _ => {
            println!("Could not find, default to 0");
            0
        }
    };
    println!("Found number: {}", number);
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
    let green = Rgb::<u8>([0, 255, 0]);
    let black = Rgb::<u8>([0, 0, 0]);

    let filter_vals = vec![50, 80, 100, 110, 120, 150, 170, 190, 210];

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
        if p[0] > filter_high_threshold
            && p[1] > filter_high_threshold
            && p[2] > filter_high_threshold
        {
            white
        } else if p[0] < filter_low_threshold
            && p[1] < filter_low_threshold
            && p[2] < filter_low_threshold
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
