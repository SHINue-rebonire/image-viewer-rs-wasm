use image::{DynamicImage, GenericImageView, GrayImage, Luma, Pixel};

/// GrayScaleに変換する関数
fn to_grayscale_rgba(img: &DynamicImage) -> GrayImage {
    let (width, height) = img.dimensions();
    let mut gray_img = GrayImage::new(width, height);

    for y in 0..height {
        for x in 0..width {
            let pixel = img.get_pixel(x, y).to_luma();
            gray_img.put_pixel(x, y, pixel);
        }
    }

    gray_img
}

/// LBP特徴量を計算する関数
pub fn local_binary_pattern(img: &DynamicImage) -> GrayImage {
    let (width, height) = img.dimensions();
    let mut lbp_img = GrayImage::new(width, height);

    // GrayScale変換
    let gray_image = to_grayscale_rgba(img);

    for y in 1..height - 1 {
        for x in 1..width - 1 {
            let center_pixel = gray_image.get_pixel(x, y).0[0];
            // 左上から時計回りで中央画素値との比較を行う
            let directions = [
                (-1, -1),
                (0, -1),
                (1, -1),
                (1, 0),
                (1, 1),
                (0, 1),
                (-1, 1),
                (-1, 0),
            ];
            let mut pattern: u8 = 0;

            // 2進数として解釈した後に、注目画素値を10進数(0~255)の値に置き換える
            for (idx, &(dx, dy)) in directions.iter().enumerate() {
                let nx = (x as i32 + dx) as u32;
                let ny = (y as i32 + dy) as u32;
                let neighbor_pixel = gray_image.get_pixel(nx, ny).0[0];
                if neighbor_pixel >= center_pixel {
                    pattern |= 1 << (7 - idx);
                }
            }
            lbp_img.put_pixel(x, y, Luma([pattern]));
        }
    }

    lbp_img
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::{ImageBuffer, Rgba};

    #[test]
    fn test_local_binary_pattern() {
        // 3x3の画像を生成
        let img = ImageBuffer::from_fn(3, 3, |x, y| {
            let pixel_values = [48, 26, 105, 82, 56, 12, 64, 7, 32];
            let index = (y * 3 + x) as usize;
            Rgba([
                pixel_values[index],
                pixel_values[index],
                pixel_values[index],
                255,
            ])
        });
        let dyn_img = DynamicImage::ImageRgba8(img);
        let lbp_img = local_binary_pattern(&dyn_img);
        println!("{:?}", lbp_img);

        // 中央のピクセルのLBP値をチェックします
        let expected_value = 0b00100011; // 期待されるLBP値
        let computed_value = lbp_img.get_pixel(1, 1).0[0];
        assert_eq!(
            computed_value, expected_value,
            "LBP value did not match expected."
        );
    }
}
