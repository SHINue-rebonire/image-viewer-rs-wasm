use image::{DynamicImage, GenericImageView, GrayImage, ImageBuffer, Luma, Pixel};

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

fn transform_lbp_image(
    gray_image: &ImageBuffer<Luma<u8>, Vec<u8>>,
    width: u32,
    height: u32,
) -> ImageBuffer<Luma<u8>, Vec<u8>> {
    let mut lbp_img = ImageBuffer::new(width, height);
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

    for y in 1..height - 1 {
        for x in 1..width - 1 {
            let center_pixel = gray_image.get_pixel(x, y).0[0];
            // 左上から時計回りで注目画素(中央)の値との比較を行う
            let pattern = directions
                .iter()
                .enumerate()
                .fold(0u8, |acc, (idx, &(dx, dy))| {
                    let nx = (x as i32 + dx) as u32;
                    let ny = (y as i32 + dy) as u32;
                    let neighbor_pixel = gray_image.get_pixel(nx, ny).0[0];
                    acc | ((neighbor_pixel >= center_pixel) as u8) << (7 - idx)
                });
            lbp_img.put_pixel(x, y, Luma([pattern]));
        }
    }

    lbp_img
}

/// 画像からLocal Binary Pattern特徴量を計算し、輝度値を変換する関数
pub fn local_binary_pattern(pixel_data: Vec<u8>, width: u32, height: u32) -> Vec<u8> {
    // DynamicImageに変換
    let buffer = ImageBuffer::from_raw(width, height, pixel_data).expect("Failed");
    let img = DynamicImage::ImageRgba8(buffer);

    // GrayScaleに変換
    let gray_image = to_grayscale_rgba(&img);

    // LBP画像に変換
    let lbp_img = transform_lbp_image(&gray_image, width, height);

    lbp_img.to_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_local_binary_pattern() {
        // 3x3の画像を生成
        // let pixel_data = vec![48, 26, 105, 82, 56, 12, 64, 7, 32];
        let pixel_data = vec![
            255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 0, 0,
            0, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
        ];
        let lbp_result = local_binary_pattern(pixel_data, 3, 3);

        // 中央のピクセルのLBP値をチェックします
        let expected_value = 0b11111111; // 期待されるLBP値
        let computed_value = lbp_result[4];
        assert_eq!(
            computed_value, expected_value,
            "LBP value did not match expected."
        );
    }
}
