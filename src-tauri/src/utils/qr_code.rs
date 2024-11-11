#[allow(dead_code, unused_variables)]

/// 获取二维码
/// text: 二维码内容信息；
/// size: 二维码大小；
/// error_correction_level：生成二维码的容错率;
/// image_format: 二维码图片格式。
#[tauri::command]
pub fn get_qr_code(
    text: &str,
    size: Option<i16>,
    error_correction_level: u8,
    image_format: &str,
) -> String {
    use base64::{engine::general_purpose, Engine as _};
    use image::Luma;
    use qrcode::QrCode;
    use std::io::Cursor;

    let format = match image_format {
        "png" => image::ImageFormat::Png,
        "jpeg" => image::ImageFormat::Jpeg,
        _ => image::ImageFormat::Png,
    };
    let ec_level: qrcode::EcLevel = match error_correction_level {
        0 => qrcode::EcLevel::L,
        1 => qrcode::EcLevel::M,
        2 => qrcode::EcLevel::Q,
        3 => qrcode::EcLevel::H,
        _ => qrcode::EcLevel::M,
    };

    let code = match size {
        Some(size_value) => {
            if size_value > 40 || size_value < 0 {
                panic!("size的取值范围为0～40");
            }
            QrCode::with_version(text, qrcode::Version::Normal(size_value), ec_level).unwrap()
        }
        None => QrCode::with_error_correction_level(text, ec_level).unwrap(),
    };
    let img = code.render::<Luma<u8>>().build();
    let img = image::DynamicImage::ImageLuma8(img);
    let mut buf = vec![];
    img.write_to(&mut Cursor::new(&mut buf), format).unwrap();
    let image_base64 = general_purpose::STANDARD.encode(buf);
    return image_base64;
}
