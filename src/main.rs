extern crate image;
extern crate imageproc;
use core::f32;
use imageproc::edges::canny;

use image::{
    DynamicImage, GenericImage, GenericImageView, ImageBuffer, Luma, Pixel, Rgb, Rgba, SubImage,
};
use imageproc::hough;
use std::path::Path;

struct DominoImageSection {
    top: u64,
    bottom: u64,
    left: u64,
    right: u64,
    middle: u64,
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
    ratio: std::ops::Range<u32>,
    leeway: u32,
    color_range: ColorRange,
    value: u32,
}

impl Default for DominoRange {
    fn default() -> Self {
        DominoRange {
            ratio: (0..0),
            leeway: 10,
            color_range: Default::default(),
            value: 0,
        }
    }
}

enum DominoPiece {
    ZERO(DominoRange),
    ONE(DominoRange),
    TWO(u32),
    THREE(u32),
    FOUR(u32),
    FIVE(u32),
    SIX(u32),
    SEVEN(u32),
    EIGHT(u32),
    NINE(u32),
    TEN(u32),
    ELEVEN(u32),
    TWELVE(u32),
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
        ratio: (1..99),
        color_range: ColorRange {
            r: (240..240),
            g: (228..228),
            b: (200..200),
        },
        value: 0,
        ..Default::default()
    }));
    dominoes.push(DominoPiece::ONE(DominoRange {
        ratio: (0..0),
        color_range: ColorRange {
            r: (1..1),
            g: (1..1),
            b: (1..1),
        },
        value: 1,
        ..Default::default()
    }));
    dominoes.push(DominoPiece::TWO(2));
    dominoes.push(DominoPiece::THREE(3));
    dominoes.push(DominoPiece::FOUR(4));
    dominoes.push(DominoPiece::FIVE(5));
    dominoes.push(DominoPiece::SIX(6));
    dominoes.push(DominoPiece::SEVEN(7));
    dominoes.push(DominoPiece::EIGHT(8));
    dominoes.push(DominoPiece::NINE(9));
    dominoes.push(DominoPiece::TEN(10));
    dominoes.push(DominoPiece::ELEVEN(11));
    dominoes.push(DominoPiece::TWELVE(12));

    return dominoes;
}

// fn construct_domino_ranges()

fn main() {
    // let doms = construct_dominoes();

    // let range: ColorRange = ColorRange {
    //     r: (0..255),
    //     g: (0..122),
    //     b: (0..111),
    // };
    // manipulate_image()
    // count_circles();
    find_domino();
}

fn detect_domino_edges(image: DynamicImage) {}

fn find_domino() {
    let domino_pic_path = "dominoes/eval/1-10.jpg"; //"dominoes/Screenshot_20210309-204319_Photos~4.jpg"
    let img = image::open(domino_pic_path).unwrap();

    let line_colour = Rgba([255, 0, 0, 1]);

    let mut gray_img = img.grayscale().as_mut_luma8().unwrap().clone();
    let edges: ImageBuffer<Luma<u8>, Vec<u8>> = canny(&gray_img, 70.0, 100.0);
    edges.save("tests/canny_edges.png");

    let top_line = 0;
    let height = img.height();
    let width = img.width();
    let mut topedge = 0;
    let mut bottomedge = 0;
    let mut leftedge = 0;
    let mut rightedge = 0;

    let mut lines_image = img.clone().into_rgba8();

    // finding the top of the dominoe
    for y in (0..height).rev() {
        let pixel = edges.get_pixel(width / 2, y).to_rgb();
        if pixel[1] == 255 {
            println!("Found bottom edge at {}", y);
            bottomedge = y;
            imageproc::drawing::draw_line_segment_mut(
                &mut lines_image,
                (0.0, y as f32),
                (width as f32, y as f32),
                line_colour,
            );
            break;
        }
    }

    for y in 0..height {
        let pixel = edges.get_pixel(width / 2, y).to_rgb();
        if pixel[1] == 255 {
            println!("Found top edge at {}", y);
            topedge = y;
            imageproc::drawing::draw_line_segment_mut(
                &mut lines_image,
                (0.0, y as f32),
                (width as f32, y as f32),
                line_colour,
            );
            break;
        }
    }

    // instead of this, we can find X consecutive pixels that are "black" (less than 40-50) (40,40,40)
    let middle: f32 = ((bottomedge - topedge) / 2 + topedge) as f32;
    imageproc::drawing::draw_line_segment_mut(
        &mut lines_image,
        (0.0, middle as f32),
        (width as f32, middle as f32),
        line_colour,
    );

    let mut left = 0;
    let mut right = 0;

    for x in 0..width {
        let pixel = edges.get_pixel(x, middle as u32).to_rgb();
        if pixel[1] == 255 {
            println!("Found left edge at {}", x);
            left = x;
            imageproc::drawing::draw_line_segment_mut(
                &mut lines_image,
                (x as f32, 0.0),
                (x as f32, height as f32),
                line_colour,
            );
            break;
        }
    }

    for x in (0..width).rev() {
        let pixel = edges.get_pixel(x, middle as u32).to_rgb();
        if pixel[1] == 255 {
            println!("Found right edge at {}", x);
            right = x;
            imageproc::drawing::draw_line_segment_mut(
                &mut lines_image,
                (x as f32, 0.0),
                (x as f32, height as f32),
                line_colour,
            );
            break;
        }
    }

    let mut img_clone = img.clone();
    let domino_piece =
        img_clone.sub_image(left, topedge, right - left, middle as u32 - topedge as u32);
    count_ratio(&domino_piece, 180);

    let domino_piece = img_clone.sub_image(
        left,
        middle as u32,
        right - left,
        bottomedge as u32 - middle as u32,
    );
    count_ratio(&domino_piece, 180);

    lines_image.save("tests/found_squares.png").unwrap();
}

fn find_domino_edges() {}

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
