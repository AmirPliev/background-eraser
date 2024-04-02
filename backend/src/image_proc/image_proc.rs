use image;
use image::{GenericImage, DynamicImage, ImageError };
use image::GenericImageView;

use std::io::Cursor;
use base64::{Engine as _, DecodeError, engine::{general_purpose}};

#[derive(Debug)]
pub enum ImageDecodingError{
    Base43Error,
    ToImageError,
}

pub fn create_filtered_image(original_image: &DynamicImage, new_image: &mut DynamicImage, offset: &(u32, u32), mask: &Vec<Vec<u8>>)  {
    for (y_index, row) in mask.iter().enumerate() {
        for (x_index, pixel_mask_value) in row.iter().enumerate() {
            if y_index as u32 >= mask.len() as u32 - 4 || x_index as u32 == row.len() as u32 - 4 {
                continue;
            }
            if *pixel_mask_value != 0 {
                let actual_x_index = offset.0 + x_index as u32 ;
                let actual_y_index = offset.1 + y_index as u32 ;
                let pixel = original_image.get_pixel(actual_x_index, actual_y_index);
                new_image.put_pixel(actual_x_index, actual_y_index, pixel);
            }
        }
    }
}


pub fn base64_to_image(image_string: String)  ->  Result<DynamicImage, ImageDecodingError> {
    let image_string = image_string.split(",").collect::<Vec<&str>>()[1];

    let image_buffer_result: Result<Vec<u8>, DecodeError> = general_purpose::STANDARD.decode(&image_string);
    let image_buffer: Vec<u8>;
    match image_buffer_result {
        Ok(buffer) => {image_buffer = buffer},
        Err(_error) => {
            return Err(ImageDecodingError::Base43Error)
        },
            
    };

    let actual_image_result: Result<DynamicImage, ImageError> = image::load_from_memory(&image_buffer);
    let actual_image: DynamicImage;
    match actual_image_result {
        Ok(image) => {
            actual_image = image
        },
        Err(_error) => {
            return Err(ImageDecodingError::ToImageError);
        }
    };

    Ok(actual_image)
}


pub fn image_to_base64(image: DynamicImage) -> Result<String, u32> {
    let mut buffer: Vec<u8> = Vec::new();
    let mut cursor = Cursor::new(&mut buffer);
    match image.write_to(&mut cursor, image::ImageOutputFormat::Png) {
        Ok(_value) => {},
        Err(_error) => {
            return Err(0);
        }
    };

    let b64 = general_purpose::STANDARD.encode(&buffer);
    let mut final_result = String::new();
    final_result.push_str("data:image/png;base64");
    final_result.push_str(",");
    final_result.push_str(&b64);

    Ok(final_result)
}
