use core_lib::local_binary_pattern;
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

    // Local Binary Pattern特徴量計算
    let lbp_data = local_binary_pattern(pixel_data, width, height);

    // FlatBuffersのバイナリデータを作成
    let mut builder = FlatBufferBuilder::new();
    let lbp_vector = builder.create_vector(&lbp_data);
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
