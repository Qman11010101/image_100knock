use image::{DynamicImage, GenericImage, ImageBuffer, Rgba};

const COLOR_STEPS: usize = 256;

// ヒストグラム表示(使えそうなので取っておく)
// let div_value = histogram.iter().max().unwrap() / 120;

// for i in 0..histogram.len() {
//     for _ in 0..histogram[i] / div_value {
//         print!("■")
//     }
//     println!("")
// }

// 問題ベース
// #[allow(dead_code)]
// pub fn q0(img: ImageBuffer<Rgba<u8>, Vec<u8>>) {
//     let mut out_img = DynamicImage::new_rgba8(img.width(), img.height());

//     for y in 0..img.height() {
//         for x in 0..img.width() {
//             let p = img.get_pixel(x, y).clone();

//             let p_conv = Rgba([r, g, b, 255]);
//             out_img.put_pixel(x, y, p_conv);
//         }
//     }

//     out_img.save("q0.png").unwrap();
// }

// Q0. 画像テスト
#[allow(dead_code)]
pub fn q0(img: ImageBuffer<Rgba<u8>, Vec<u8>>) {
    let mut out_img = DynamicImage::new_rgba8(img.width(), img.height());

    for y in 0..img.height() {
        for x in 0..img.width() {
            let p = img.get_pixel(x, y);
            out_img.put_pixel(x, y, p.clone());
        }
    }

    out_img.save("q0_output.png").unwrap();
}

// Q1. チャネル入れかえ(RGB→BGR)
#[allow(dead_code)]
pub fn q1(img: ImageBuffer<Rgba<u8>, Vec<u8>>) {
    let mut out_img = DynamicImage::new_rgba8(img.width(), img.height());

    for y in 0..img.height() {
        for x in 0..img.width() {
            let p = img.get_pixel(x, y).clone();
            let p_conv = Rgba([p[2], p[1], p[0], 255]);
            out_img.put_pixel(x, y, p_conv);
        }
    }

    out_img.save("q1_rgb2bgr.png").unwrap();
}

// Q2. グレースケール
#[allow(dead_code)]
pub fn q2(img: ImageBuffer<Rgba<u8>, Vec<u8>>) {
    let mut out_img = DynamicImage::new_rgba8(img.width(), img.height());

    for y in 0..img.height() {
        for x in 0..img.width() {
            let p = img.get_pixel(x, y).clone();
            let px_val = p[0] as f32 * 0.2126 + p[1] as f32 * 0.7152 + p[2] as f32 * 0.0722;
            // let px_val = (p[0] as f32 + p[1] as f32 + p[2] as f32) / 3.0; // RGB平均値
            let p_conv = Rgba([
                px_val.floor() as u8,
                px_val.floor() as u8,
                px_val.floor() as u8,
                255,
            ]);
            out_img.put_pixel(x, y, p_conv);
        }
    }

    out_img.save("q2_grayscale_a.png").unwrap();
}

// Q3. 二値化
#[allow(dead_code)]
pub fn q3(img: ImageBuffer<Rgba<u8>, Vec<u8>>) {
    let mut out_img = DynamicImage::new_rgba8(img.width(), img.height());

    for y in 0..img.height() {
        for x in 0..img.width() {
            let p = img.get_pixel(x, y).clone();
            let grayscale_val =
                (p[0] as f32 * 0.2126 + p[1] as f32 * 0.7152 + p[2] as f32 * 0.0722) as u8;
            let threshold: u8 = 128;
            let px_val = if grayscale_val > threshold { 255 } else { 0 };
            let p_conv = Rgba([px_val, px_val, px_val, 255]);
            out_img.put_pixel(x, y, p_conv);
        }
    }

    out_img.save("q3_binarize.png").unwrap();
}

// Q4. 大津の二値化
#[allow(dead_code)]
pub fn q4(img: ImageBuffer<Rgba<u8>, Vec<u8>>) {
    let mut out_img = DynamicImage::new_rgba8(img.width(), img.height());
    let mut histogram: [u32; COLOR_STEPS] = [0; COLOR_STEPS];

    // ヒストグラム生成
    for y in 0..img.height() {
        for x in 0..img.width() {
            let p = img.get_pixel(x, y).clone();
            let grayscale_val =
                (p[0] as f32 * 0.2126 + p[1] as f32 * 0.7152 + p[2] as f32 * 0.0722) as u8;
            histogram[grayscale_val as usize] += 1;
        }
    }

    // 閾値計算
    let mut threshold: u8 = 0;
    let mut separation_max: f32 = 0.0;
    for t in 0..histogram.len() {
        let former = &histogram[..t];
        let latter = &histogram[t..];

        let former_px = former.iter().sum::<u32>() as f32;
        let latter_px = latter.iter().sum::<u32>() as f32;

        // 画素値の平均計算
        let mut former_px_avg: f32 = 0.0;
        let mut latter_px_avg: f32 = 0.0;

        for i in 0..former.len() {
            former_px_avg += i as f32 * former[i] as f32;
        }
        former_px_avg /= former_px;
        for i in 0..latter.len() {
            latter_px_avg += (former.len() as f32 + i as f32) * latter[i] as f32;
        }
        latter_px_avg /= latter_px;

        let separation = former_px * latter_px * (former_px_avg - latter_px_avg).powf(2.0);

        if separation > separation_max {
            threshold = t as u8;
            separation_max = separation;
        }
    }

    println!("Threshold is {}", threshold);

    for y in 0..img.height() {
        for x in 0..img.width() {
            let p = img.get_pixel(x, y).clone();
            let grayscale_val =
                (p[0] as f32 * 0.2126 + p[1] as f32 * 0.7152 + p[2] as f32 * 0.0722) as u8;
            let px_val = if grayscale_val > threshold { 255 } else { 0 };
            let p_conv = Rgba([px_val, px_val, px_val, 255]);
            out_img.put_pixel(x, y, p_conv);
        }
    }

    out_img.save("q4_otsu_binarize.png").unwrap();
}

// Q5. HSV<->RGB変換
#[allow(dead_code)]
pub fn q5(img: ImageBuffer<Rgba<u8>, Vec<u8>>) {
    let mut out_img = DynamicImage::new_rgba8(img.width(), img.height());

    for y in 0..img.height() {
        for x in 0..img.width() {
            let p = img.get_pixel(x, y).clone();

            let mut r = p[0] as f32;
            let mut g = p[1] as f32;
            let mut b = p[2] as f32;

            // RGB -> HSV変換
            let i_max = [p[0], p[1], p[2]].iter().max().unwrap().clone() as f32;
            let i_min = [p[0], p[1], p[2]].iter().min().unwrap().clone() as f32;

            let mut hue = if i_max == g {
                (b - r) / (i_max - i_min) * 60.0 + 120.0
            } else if i_max == b {
                (r - g) / (i_max - i_min) * 60.0 + 240.0
            } else if g < b {
                (g - b) / (i_max - i_min) * 60.0 + 360.0
            } else {
                (g - b) / (i_max - i_min) * 60.0
            };

            let saturation = i_max - i_min;
            let value = i_max;

            // Hue反転
            hue = (hue + 180.0) % 360.0;

            // HSV -> RGB変換
            // Ref: https://tomari.org/main/java/color/ccal.html
            let hsv_max = value;
            let hsv_min = hsv_max - ((saturation / 255.0) * hsv_max);

            if hue >= 0.0 && hue < 60.0 {
                r = hsv_max;
                g = (hue / 60.0) * (hsv_max - hsv_min) + hsv_min;
                b = hsv_min;
            } else if hue >= 60.0 && hue < 120.0 {
                r = ((120.0 - hue) / 60.0) * (hsv_max - hsv_min) + hsv_min;
                g = hsv_max;
                b = hsv_min;
            } else if hue >= 120.0 && hue < 180.0 {
                r = hsv_min;
                g = hsv_max;
                b = ((hue - 120.0) / 60.0) * (hsv_max - hsv_min) + hsv_min;
            } else if hue >= 180.0 && hue < 240.0 {
                r = hsv_min;
                g = ((240.0 - hue) / 60.0) * (hsv_max - hsv_min) + hsv_min;
                b = hsv_max;
            } else if hue >= 240.0 && hue < 300.0 {
                r = ((hue - 240.0) / 60.0) * (hsv_max - hsv_min) + hsv_min;
                g = hsv_min;
                b = hsv_max;
            } else {
                r = hsv_max;
                g = hsv_min;
                b = ((360.0 - hue) / 60.0) * (hsv_max - hsv_min) + hsv_min;
            }

            let p_conv = Rgba([r as u8, g as u8, b as u8, 255]);
            out_img.put_pixel(x, y, p_conv);
        }
    }

    out_img.save("q5_hsv_reverse_h.png").unwrap();
}

// Q6. 減色
#[allow(dead_code)]
pub fn q6(img: ImageBuffer<Rgba<u8>, Vec<u8>>) {
    let mut out_img = DynamicImage::new_rgba8(img.width(), img.height());

    for y in 0..img.height() {
        for x in 0..img.width() {
            let px = img.get_pixel(x, y).clone();
            let p = [px[0], px[1], px[2]];
            let mut rgb = [0, 0, 0];

            for i in 0..3 {
                rgb[i] = match p[i] {
                    n if n < 64 => 32,
                    n if 64 <= n && n < 128 => 96,
                    n if 128 <= n && n < 192 => 160,
                    n if 192 <= n => 224,
                    _ => {
                        panic!("Err!");
                    }
                }
            }

            let p_conv = Rgba([rgb[0], rgb[1], rgb[2], 255]);
            out_img.put_pixel(x, y, p_conv);
        }
    }

    out_img.save("q6_quantize.png").unwrap();
}

// Q7. プーリング
// 画像サイズは縦横が8で割り切れるものとする
#[allow(dead_code)]
pub fn q7(img: ImageBuffer<Rgba<u8>, Vec<u8>>) {
    let all_grids: u32 = 8;
    let mut out_img = DynamicImage::new_rgba8(img.width(), img.height());

    let grid_size_h = img.height() / all_grids;
    let grid_size_w = img.width() / all_grids;
    for y in 0..(img.height() / grid_size_h) {
        for x in 0..(img.width() / grid_size_w) {
            let mut vec_pxs: Vec<Rgba<u8>> = Vec::new();
            let offset_h = grid_size_h * y;
            let offset_w = grid_size_w * x;
            for yg in offset_h..grid_size_h + offset_h {
                for xg in offset_w..grid_size_w + offset_w {
                    vec_pxs.push(img.get_pixel(xg, yg).clone());
                }
            }
            // 平均値
            let mut vec_px_color: Vec<u8> = vec![0, 0, 0];
            for c in 0..3 {
                let mut color_value_sum: u32 = 0;
                for px in 0..vec_pxs.len() {
                    color_value_sum += vec_pxs[px][c] as u32;
                }
                vec_px_color[c] = (color_value_sum / (grid_size_h * grid_size_w) as u32) as u8;
            }
            for yg in offset_h..grid_size_h + offset_h {
                for xg in offset_w..grid_size_w + offset_w {
                    out_img.put_pixel(
                        xg,
                        yg,
                        Rgba([vec_px_color[0], vec_px_color[1], vec_px_color[2], 255]),
                    );
                }
            }
        }
    }

    out_img.save("q7_pooling_avg.png").unwrap();
}

// Q8. Maxプーリング
#[allow(dead_code)]
pub fn q8(img: ImageBuffer<Rgba<u8>, Vec<u8>>) {
    let all_grids: u32 = 8;
    let mut out_img = DynamicImage::new_rgba8(img.width(), img.height());

    let grid_size_h = img.height() / all_grids;
    let grid_size_w = img.width() / all_grids;
    for y in 0..(img.height() / grid_size_h) {
        for x in 0..(img.width() / grid_size_w) {
            let mut vec_pxs: Vec<Rgba<u8>> = Vec::new();
            let offset_h = grid_size_h * y;
            let offset_w = grid_size_w * x;
            for yg in offset_h..grid_size_h + offset_h {
                for xg in offset_w..grid_size_w + offset_w {
                    vec_pxs.push(img.get_pixel(xg, yg).clone());
                }
            }
            // 平均値
            let mut vec_px_color: Vec<u8> = vec![0, 0, 0];
            for c in 0..3 {
                let mut color_value_max: u32 = 0;
                for px in 0..vec_pxs.len() {
                    // color_value_sum += vec_pxs[px][c] as u32;
                    if vec_pxs[px][c] as u32 > color_value_max {
                        color_value_max = vec_pxs[px][c] as u32;
                    }
                }
                vec_px_color[c] = color_value_max as u8;
            }
            for yg in offset_h..grid_size_h + offset_h {
                for xg in offset_w..grid_size_w + offset_w {
                    out_img.put_pixel(
                        xg,
                        yg,
                        Rgba([vec_px_color[0], vec_px_color[1], vec_px_color[2], 255]),
                    );
                }
            }
        }
    }

    out_img.save("q8_pooling_max.png").unwrap();
}

// Q9. ガウシアンフィルタ
#[allow(dead_code)]
pub fn q9(img: ImageBuffer<Rgba<u8>, Vec<u8>>) {
    let mut out_img = DynamicImage::new_rgba8(img.width(), img.height());

    for y in 0..img.height() {
        for x in 0..img.width() {
            let tx = x as i32;
            let ty = y as i32;
            let around_px = vec![
                (tx - 1, ty - 1),
                (tx - 1, ty),
                (tx - 1, ty + 1),
                (tx, ty - 1),
                (tx, ty),
                (tx, ty + 1),
                (tx + 1, ty - 1),
                (tx + 1, ty),
                (tx + 1, ty + 1),
            ];
            let kernel = vec![1, 2, 1, 2, 4, 2, 1, 2, 1];
            let mut conv_px_r = 0;
            let mut conv_px_g = 0;
            let mut conv_px_b = 0;

            for i in 0..around_px.len() {
                if (around_px[i].0 < 0 || around_px[i].1 < 0)
                    || (around_px[i].0 >= img.height() as i32
                        || around_px[i].1 >= img.width() as i32)
                {
                    conv_px_r += 0;
                    conv_px_g += 0;
                    conv_px_b += 0;
                } else {
                    let tpx = img
                        .get_pixel(around_px[i].0 as u32, around_px[i].1 as u32)
                        .clone();
                    conv_px_r += tpx[0] as i32 * kernel[i];
                    conv_px_g += tpx[1] as i32 * kernel[i];
                    conv_px_b += tpx[2] as i32 * kernel[i];
                }
            }

            let r = (conv_px_r / 16) as u8;
            let g = (conv_px_g / 16) as u8;
            let b = (conv_px_b / 16) as u8;

            let p_conv = Rgba([r, g, b, 255]);
            out_img.put_pixel(x, y, p_conv);
        }
    }

    out_img.save("q9_gaussian_filter.png").unwrap();
}

// Q10. メディアンフィルタ
#[allow(dead_code)]
pub fn q10(img: ImageBuffer<Rgba<u8>, Vec<u8>>) {
    let mut out_img = DynamicImage::new_rgba8(img.width(), img.height());

    for y in 0..img.height() {
        for x in 0..img.width() {
            let tx = x as i32;
            let ty = y as i32;
            let around_px = vec![
                (tx - 1, ty - 1),
                (tx - 1, ty),
                (tx - 1, ty + 1),
                (tx, ty - 1),
                (tx, ty),
                (tx, ty + 1),
                (tx + 1, ty - 1),
                (tx + 1, ty),
                (tx + 1, ty + 1),
            ];
            let mut conv_px_r = vec![0; 9];
            let mut conv_px_g = vec![0; 9];
            let mut conv_px_b = vec![0; 9];

            for i in 0..around_px.len() {
                if !((around_px[i].0 < 0 || around_px[i].1 < 0)
                    || (around_px[i].0 >= img.height() as i32
                        || around_px[i].1 >= img.width() as i32))
                {
                    let tpx = img
                        .get_pixel(around_px[i].0 as u32, around_px[i].1 as u32)
                        .clone();
                    conv_px_r[i] = tpx[0];
                    conv_px_g[i] = tpx[1];
                    conv_px_b[i] = tpx[2];
                }
            }

            conv_px_r.sort();
            conv_px_g.sort();
            conv_px_b.sort();

            let r = conv_px_r[4];
            let g = conv_px_g[4];
            let b = conv_px_b[4];

            let p_conv = Rgba([r, g, b, 255]);
            out_img.put_pixel(x, y, p_conv);
        }
    }

    out_img.save("q10_median_filter.png").unwrap();
}

// Q11. 平滑化フィルタ
#[allow(dead_code)]
pub fn q11(img: ImageBuffer<Rgba<u8>, Vec<u8>>) {
    let mut out_img = DynamicImage::new_rgba8(img.width(), img.height());

    for y in 0..img.height() {
        for x in 0..img.width() {
            let tx = x as i32;
            let ty = y as i32;
            let around_px = vec![
                (tx - 1, ty - 1),
                (tx - 1, ty),
                (tx - 1, ty + 1),
                (tx, ty - 1),
                (tx, ty),
                (tx, ty + 1),
                (tx + 1, ty - 1),
                (tx + 1, ty),
                (tx + 1, ty + 1),
            ];
            let mut conv_px_r = 0;
            let mut conv_px_g = 0;
            let mut conv_px_b = 0;

            for i in 0..around_px.len() {
                if !((around_px[i].0 < 0 || around_px[i].1 < 0)
                    || (around_px[i].0 >= img.height() as i32
                        || around_px[i].1 >= img.width() as i32))
                {
                    let tpx = img
                        .get_pixel(around_px[i].0 as u32, around_px[i].1 as u32)
                        .clone();
                    conv_px_r += tpx[0] as i32;
                    conv_px_g += tpx[1] as i32;
                    conv_px_b += tpx[2] as i32;
                }
            }

            let r = (conv_px_r / 9) as u8;
            let g = (conv_px_g / 9) as u8;
            let b = (conv_px_b / 9) as u8;

            let p_conv = Rgba([r, g, b, 255]);
            out_img.put_pixel(x, y, p_conv);
        }
    }

    out_img.save("q11_average_filter.png").unwrap();
}

// Q12. モーションフィルタ
#[allow(dead_code)]
pub fn q12(img: ImageBuffer<Rgba<u8>, Vec<u8>>) {
    let mut out_img = DynamicImage::new_rgba8(img.width(), img.height());

    for y in 0..img.height() {
        for x in 0..img.width() {
            let tx = x as i32;
            let ty = y as i32;
            let around_px = vec![
                (tx - 1, ty - 1),
                (tx - 1, ty),
                (tx - 1, ty + 1),
                (tx, ty - 1),
                (tx, ty),
                (tx, ty + 1),
                (tx + 1, ty - 1),
                (tx + 1, ty),
                (tx + 1, ty + 1),
            ];
            let kernel = vec![1, 0, 0, 0, 1, 0, 0, 0, 1];
            let mut conv_px_r = 0;
            let mut conv_px_g = 0;
            let mut conv_px_b = 0;

            for i in 0..around_px.len() {
                if (around_px[i].0 < 0 || around_px[i].1 < 0)
                    || (around_px[i].0 >= img.height() as i32
                        || around_px[i].1 >= img.width() as i32)
                {
                    conv_px_r += 0;
                    conv_px_g += 0;
                    conv_px_b += 0;
                } else {
                    let tpx = img
                        .get_pixel(around_px[i].0 as u32, around_px[i].1 as u32)
                        .clone();
                    conv_px_r += tpx[0] as i32 * kernel[i];
                    conv_px_g += tpx[1] as i32 * kernel[i];
                    conv_px_b += tpx[2] as i32 * kernel[i];
                }
            }

            let r = (conv_px_r / 3) as u8;
            let g = (conv_px_g / 3) as u8;
            let b = (conv_px_b / 3) as u8;

            let p_conv = Rgba([r, g, b, 255]);
            out_img.put_pixel(x, y, p_conv);
        }
    }

    out_img.save("q12_motion_filter.png").unwrap();
}

// Q13. MAX-MINフィルタ
#[allow(dead_code)]
pub fn q13(img: ImageBuffer<Rgba<u8>, Vec<u8>>) {
    let mut out_img = DynamicImage::new_rgba8(img.width(), img.height());
    let mut gray_img = DynamicImage::new_rgba8(img.width(), img.height());

    for y in 0..img.height() {
        for x in 0..img.width() {
            let p = img.get_pixel(x, y).clone();
            let px_val = p[0] as f32 * 0.2126 + p[1] as f32 * 0.7152 + p[2] as f32 * 0.0722;
            let p_conv = Rgba([
                px_val.floor() as u8,
                px_val.floor() as u8,
                px_val.floor() as u8,
                255,
            ]);
            gray_img.put_pixel(x, y, p_conv);
        }
    }

    let gray_img_conv = gray_img.to_rgba8();

    for y in 0..gray_img.height() {
        for x in 0..gray_img.width() {
            let tx = x as i32;
            let ty = y as i32;
            let around_px = vec![
                (tx - 1, ty - 1),
                (tx - 1, ty),
                (tx - 1, ty + 1),
                (tx, ty - 1),
                (tx, ty),
                (tx, ty + 1),
                (tx + 1, ty - 1),
                (tx + 1, ty),
                (tx + 1, ty + 1),
            ];
            let mut conv_px_r = vec![0; 9];
            let mut conv_px_g = vec![0; 9];
            let mut conv_px_b = vec![0; 9];

            for i in 0..around_px.len() {
                if !((around_px[i].0 < 0 || around_px[i].1 < 0)
                    || (around_px[i].0 >= gray_img_conv.height() as i32
                        || around_px[i].1 >= gray_img_conv.width() as i32))
                {
                    let tpx = gray_img_conv
                        .get_pixel(around_px[i].0 as u32, around_px[i].1 as u32)
                        .clone();
                    conv_px_r[i] = tpx[0] as i32;
                    conv_px_g[i] = tpx[1] as i32;
                    conv_px_b[i] = tpx[2] as i32;
                }
            }

            let r = (conv_px_r.iter().max().unwrap() - conv_px_r.iter().min().unwrap()) as u8;
            let g = (conv_px_g.iter().max().unwrap() - conv_px_g.iter().min().unwrap()) as u8;
            let b = (conv_px_b.iter().max().unwrap() - conv_px_b.iter().min().unwrap()) as u8;

            let p_conv = Rgba([r, g, b, 255]);
            out_img.put_pixel(x, y, p_conv);
        }
    }

    out_img.save("q13_max-min_filter.png").unwrap();
}