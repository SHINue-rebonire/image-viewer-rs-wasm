use core_lib::local_binary_pattern;
use image::{DynamicImage, ImageBuffer};
use wasm_bindgen::prelude::*;

mod image_processing_generated;
use flatbuffers::FlatBufferBuilder;
use image_processing_generated::image_processing::{ImageProcessing, ImageProcessingArgs};

#[wasm_bindgen]
pub fn process_image(data: &[u8]) -> Vec<u8> {
    let img_proc = flatbuffers::root::<ImageProcessing>(data).expect("Failed");

    // img_proc.buf() からバイナリデータを取得
    let pixel_data = img_proc.buf().bytes().to_vec();
    let width = img_proc.width() as u32;
    let height = img_proc.height() as u32;

    let buffer = ImageBuffer::from_raw(width, height, pixel_data).expect("Failed");
    let dyn_image = DynamicImage::ImageRgba8(buffer);

    let lbp_image = local_binary_pattern(&dyn_image);

    // FlatBuffersのバイナリデータを作成
    let mut builder = FlatBufferBuilder::new();
    let lbp_data = lbp_image.as_raw();
    let lbp_vector = builder.create_vector(lbp_data);

    let image_data = ImageProcessing::create(
        &mut builder,
        &ImageProcessingArgs {
            width: width,
            height: height,
            buf: Some(lbp_vector),
        },
    );

    builder.finish(image_data, None);
    builder.finished_data().to_vec()
}
