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
use std::{borrow::Borrow, collections::HashMap, ops::Range, path::Path};

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
    let mut dominoes = vec![];

    dominoes.push(DominoRange {
        ratio: (1.0..99.0),
        color_range: ColorRange {
            r: (240..240),
            g: (228..228),
            b: (200..200),
        },
        value: 0,
        ..Default::default()
    });
    dominoes.push(DominoRange {
        ratio: (6.91..6.91),
        color_range: ColorRange {
            r: (60..80),
            g: (100..130),
            b: (110..150),
        },
        value: 1,
        ..Default::default()
    });
    dominoes.push(DominoRange {
        ratio: (6.91..6.91),
        color_range: ColorRange {
            r: (65..85),
            g: (130..150),
            b: (25..45),
        },
        value: 2,
        ..Default::default()
    });
    dominoes.push(DominoRange {
        ratio: (6.91..6.91),
        color_range: ColorRange {
            r: (190..220),
            g: (35..55),
            b: (30..50),
        },
        value: 3,
        ..Default::default()
    });
    dominoes.push(DominoRange {
        ratio: (6.91..6.91),
        color_range: ColorRange {
            r: (135..155),
            g: (65..85),
            b: (30..50),
        },
        value: 4,
        ..Default::default()
    });
    dominoes.push(DominoRange {
        ratio: (6.91..6.91),
        color_range: ColorRange {
            r: (15..35),
            g: (65..85),
            b: (120..160),
        },
        value: 5,
        ..Default::default()
    });
    dominoes.push(DominoRange {
        ratio: (6.91..6.91),
        color_range: ColorRange {
            r: (200..255),
            g: (110..160),
            b: (0..50),
        },
        value: 6,
        ..Default::default()
    });
    dominoes.push(DominoRange {
        ratio: (6.91..6.91),
        color_range: ColorRange {
            r: (160..190),
            g: (35..70),
            b: (75..100),
        },
        value: 7,
        ..Default::default()
    });
    dominoes.push(DominoRange {
        ratio: (6.91..6.91),
        color_range: ColorRange {
            r: (30..60),
            g: (110..150),
            b: (90..120),
        },
        value: 8,
        ..Default::default()
    });
    dominoes.push(DominoRange {
        ratio: (6.91..6.91),
        color_range: ColorRange {
            r: (75..95),
            g: (25..55),
            b: (55..100),
        },
        value: 9,
        ..Default::default()
    });
    dominoes.push(DominoRange {
        ratio: (1.375..1.375),
        color_range: ColorRange {
            r: (200..255),
            g: (80..120),
            b: (50..70),
        },
        value: 10,
        ..Default::default()
    });
    dominoes.push(DominoRange {
        ratio: (6.91..6.91),
        color_range: ColorRange {
            r: (110..130),
            g: (35..65),
            b: (50..85),
        },
        value: 11,
        ..Default::default()
    });
    dominoes.push(DominoRange {
        ratio: (6.91..6.91),
        color_range: ColorRange {
            r: (130..170),
            g: (130..170),
            b: (120..140),
        },
        value: 12,
        ..Default::default()
    });

    // dominoes.push(DominoPiece::ZERO(DominoRange {
    //     ratio: (1.0..99.0),
    //     color_range: ColorRange {
    //         r: (240..240),
    //         g: (228..228),
    //         b: (200..200),
    //     },
    //     value: 0,
    //     ..Default::default()
    // }));
    // dominoes.push(DominoPiece::ONE(DominoRange {
    //     ratio: (6.91..6.91),
    //     color_range: ColorRange {
    //         r: (50..75),
    //         g: (100..130),
    //         b: (100..140),
    //     },
    //     value: 1,
    //     ..Default::default()
    // }));
    // dominoes.push(DominoPiece::TWO(DominoRange {
    //     ratio: (6.91..6.91),
    //     color_range: ColorRange {
    //         r: (50..75),
    //         g: (100..130),
    //         b: (20..70),
    //     },
    //     value: 2,
    //     ..Default::default()
    // }));
    // dominoes.push(DominoPiece::THREE(DominoRange {
    //     ratio: (6.91..6.91),
    //     color_range: ColorRange {
    //         r: (190..255),
    //         g: (20..45),
    //         b: (30..40),
    //     },
    //     value: 3,
    //     ..Default::default()
    // }));
    // dominoes.push(DominoPiece::FOUR(DominoRange {
    //     ratio: (6.91..6.91),
    //     color_range: ColorRange {
    //         r: (135..150),
    //         g: (45..65),
    //         b: (20..40),
    //     },
    //     value: 4,
    //     ..Default::default()
    // }));
    // dominoes.push(DominoPiece::FIVE(DominoRange {
    //     ratio: (6.91..6.91),
    //     color_range: ColorRange {
    //         r: (25..35),
    //         g: (60..80),
    //         b: (110..130),
    //     },
    //     value: 5,
    //     ..Default::default()
    // }));
    // dominoes.push(DominoPiece::SIX(DominoRange {
    //     ratio: (6.91..6.91),
    //     color_range: ColorRange {
    //         r: (200..255),
    //         g: (90..125),
    //         b: (0..40),
    //     },
    //     value: 6,
    //     ..Default::default()
    // }));
    // dominoes.push(DominoPiece::SEVEN(DominoRange {
    //     ratio: (6.91..6.91),
    //     color_range: ColorRange {
    //         r: (160..180),
    //         g: (35..50),
    //         b: (60..85),
    //     },
    //     value: 7,
    //     ..Default::default()
    // }));
    // dominoes.push(DominoPiece::EIGHT(DominoRange {
    //     ratio: (6.91..6.91),
    //     color_range: ColorRange {
    //         r: (20..50),
    //         g: (100..150),
    //         b: (80..100),
    //     },
    //     value: 8,
    //     ..Default::default()
    // }));
    // dominoes.push(DominoPiece::NINE(DominoRange {
    //     ratio: (6.91..6.91),
    //     color_range: ColorRange {
    //         r: (60..80),
    //         g: (25..45),
    //         b: (70..90),
    //     },
    //     value: 9,
    //     ..Default::default()
    // }));
    // dominoes.push(DominoPiece::TEN(DominoRange {
    //     ratio: (1.375..1.375),
    //     color_range: ColorRange {
    //         r: (200..255),
    //         g: (80..100),
    //         b: (15..35),
    //     },
    //     value: 10,
    //     ..Default::default()
    // }));
    // dominoes.push(DominoPiece::ELEVEN(DominoRange {
    //     ratio: (6.91..6.91),
    //     color_range: ColorRange {
    //         r: (100..130),
    //         g: (35..55),
    //         b: (25..45),
    //     },
    //     value: 11,
    //     ..Default::default()
    // }));
    // dominoes.push(DominoPiece::TWELVE(DominoRange {
    //     ratio: (6.91..6.91),
    //     color_range: ColorRange {
    //         r: (130..150),
    //         g: (115..135),
    //         b: (80..100),
    //     },
    //     value: 12,
    //     ..Default::default()
    // }));

    return dominoes;
}

// fn construct_domino_ranges()

fn main() {
    // manipulate_image()
    // count_circles();

    // let domino_filepath = "dominoes/eval/1-10.jpg";
    let domino_filepath = "dominoes/IMG-20210311-WA0002.jpg";
    println!("{}", domino_filepath); //"dominoes/Screenshot_20210309-204319_Photos~4.jpg"
    find_domino(domino_filepath);

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

fn detect_inner_domino_edges(
    image: &mut DynamicImage,
    dom_section: DominoImageSection,
) -> Vec<u16> {
    let result = vec![];

    return result;
}

fn detect_outer_domino_edges(image: &mut DynamicImage) -> DominoImageSection {
    let height = image.height();
    let width = image.width();
    let mut topedge = 0;
    let mut bottomedge = 0;

    let mut gray_img = image.grayscale().as_mut_luma8().unwrap().clone();
    let edges: ImageBuffer<Luma<u8>, Vec<u8>> = canny(&gray_img, 70.0, 100.0);
    edges.save("tests/canny_edges.png").unwrap();

    // finding the bottom of the domino
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

    println!("Results of domino finding: {:?}", result);
    draw_domino_lines(image, &result);

    return result;
}

fn draw_domino_lines(image: &mut DynamicImage, dom_section: &DominoImageSection) {
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

    // inner edges of dominoes
    let dom_width = dom_section.right - dom_section.left;
    let dom_count = 8;

    for dom_num in 0..dom_count {
        let line_loc_x = (dom_width / 8) * dom_num + dom_section.left;
        imageproc::drawing::draw_line_segment_mut(
            image,
            (line_loc_x as f32, dom_section.top as f32),
            (line_loc_x as f32, dom_section.bottom as f32),
            line_colour,
        );
    }

    image.save("tests/found_squares.png").unwrap();
}

fn find_domino(image_path: &str) {
    // let domino_pic_path = "dominoes/eval/2-3.jpg"; //"dominoes/Screenshot_20210309-204319_Photos~4.jpg"
    println!("Trying to open: {}", image_path);
    let mut img = image::open(image_path).unwrap();

    if img.height() > img.width() {
        img = img.rotate270();
    }

    let domino = detect_outer_domino_edges(&mut img);

    let domino_inner_edges = detect_inner_domino_edges(&mut img, domino);
    // let domino = detect_domino_edges_eval_data(&mut img);

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
    // count_most_common_pixels(&top_piece);
    count_pixel_ranges(&top_piece);
    count_ratio(&domino_piece, 180);

    let domino_piece = img_clone.sub_image(
        domino.left as u32,
        domino.middle as u32,
        domino.right - domino.left,
        domino.bottom - domino.middle,
    );

    println!("Bottom:");
    // count_most_common_pixels(&bottom_piece);
    count_pixel_ranges(&bottom_piece);
    count_ratio(&domino_piece, 180);
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

    return Range {
        start: range_start,
        end: range_end,
    };
}

fn count_pixel_ranges(img: &DynamicImage) {
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
        if is_black_pixel((r, g, b), None) == false && is_white_pixel((r, g, b), None) == false {
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
        Some(x) => threshold.unwrap(),
        None => 180,
    };

    let result =
        r > white_pixel_threshold && g > white_pixel_threshold && b > white_pixel_threshold;
    // println!("is white?: {}", result);
    return result;
}

fn is_black_pixel((r, g, b): (u8, u8, u8), threshold: Option<u8>) -> bool {
    let black_pixel_threshold = match threshold {
        Some(_) => threshold.unwrap(),
        None => 60,
    };

    return r < black_pixel_threshold && g < black_pixel_threshold && b < black_pixel_threshold;
}

fn count_ratio(top_domino: &SubImage<&mut DynamicImage>, white_pixel_threshold: u8) {
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
}
