use axum::{
    extract::Json,
    response::IntoResponse,
    routing::post,
    Router,
};
use serde::Deserialize;
use crate::detect;
use crate::image_proc;
use image;

use image::DynamicImage;
use std::io;
use std::io::Write;

#[derive(serde::Serialize)]
struct JsonResponse {
    cropped_image: String,
    error_code: u8,
}

#[derive(Debug)]
#[derive(Deserialize)]
struct ImageData {
    image: String, // Base64-encoded image data
}

pub fn create_api() -> Router {
    Router::new()
        .route("/upload", post(remove_background))
 
}

async fn remove_background(Json(image_data): Json<ImageData>) -> impl IntoResponse {
    print!("Removing background of image ");

    print!(" | Converting base64 to a usable image");
    io::stdout().flush().unwrap();
    let original_image = match image_proc::base64_to_image(image_data.image) {
        Ok(unpacked_image) => unpacked_image,
        Err(error) => {
            println!("Something went wrong in converting the image {:?}", error);
            return axum::Json(JsonResponse {
                cropped_image: "".to_string(),
                error_code: 1,
            })
        }
    };

    print!(" | Performing inference...");
    io::stdout().flush().unwrap();
    let detections = detect::detect_objects_on_image(&original_image);
    let main_detection = match detect::find_largest_object(&detections) {
        Some(detection) => detection,
        None => {
            if detections.len() == 0 {
                return axum::Json(JsonResponse {
                    cropped_image: "".to_string(),
                    error_code: 2,
                })
            }
            &detections[0]
        },
    };

    print!(" | Found: {}, now filtering out the image.", main_detection.class);
    io::stdout().flush().unwrap();
    let rgb = image_proc::create_filtered_image(
        DynamicImage::ImageRgba8(original_image.into()),
        &(main_detection.bbox.x1 , main_detection.bbox.y1),
        &main_detection.mask
    );

    print!(" | Translating the final image to base64");
    io::stdout().flush().unwrap();
    let new_base64_image = match image_proc::image_to_base64(rgb) {
        Ok(result) => result,
        Err(error) => {
            println!("Something went wrong with encoding the new image to base64: {:?}", error);
            return axum::Json(JsonResponse {
                cropped_image: "".to_string(),
                error_code: 3,
            }) 
        }
    };

    println!(" | Done!");
    let response_body = JsonResponse {
        cropped_image: new_base64_image,
        error_code: 0,
    };
    axum::Json(response_body)
}

