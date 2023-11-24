use std::string::String;

use image::io::Reader as ImageReader;
use nsfw::{create_model, examine};
use nsfw::model::Classification;
use nsfw::model::Metric::{Hentai, Porn, Sexy};
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "assets/"]
#[include = "model.onnx"]
struct Asset;

#[napi(constructor)]
pub struct Nsfw {
    pub sensitive: bool,
    pub porn: bool,
}

#[napi]
pub async fn detect_sensitivity(path: String, mime: String, sensitive_threshold: f64, sensitive_threshold_for_porn: f64) -> napi::Result<Option<Nsfw>> {
    let mut sensitive = false;
    let mut porn = false;
    fn judge_prediction(result: &[Classification], sensitive_threshold: f64, sensitive_threshold_for_porn: f64) -> (bool, bool) {
        let mut sensitive = false;
        let mut porn = false;

        if result.iter().any(|x| x.metric == Sexy && x.score > sensitive_threshold as f32) {
            sensitive = true;
        }

        if result.iter().any(|x| x.metric == Hentai && x.score > sensitive_threshold as f32) {
            sensitive = true;
        }

        if result.iter().any(|x| x.metric == Porn && x.score > sensitive_threshold as f32) {
            sensitive = true;
        }

        if result.iter().any(|x| x.metric == Porn && x.score > sensitive_threshold_for_porn as f32) {
            porn = true;
        }

        (sensitive, porn)
    }
    if match mime.as_str() {
        "image/jpeg" => true,
        "image/png" => true,
        "image/webp" => true,
        _ => false
    } {
        match detect_image(path).await {
            Some(result) => {
                let judge = judge_prediction(&result, sensitive_threshold, sensitive_threshold_for_porn);
                sensitive = judge.0;
                porn = judge.1;
            }
            _ => {
                return Ok(None);
            }
        };
    }
    Ok(
        Some(
            Nsfw {
                sensitive,
                porn,
            }
        )
    )
}

async fn get_model() -> nsfw::Model {
    let onnx = Asset::get("model.onnx").unwrap();
    let model = create_model(&mut onnx.data.as_ref()).unwrap();
    model
}

async fn detect_image(path: String) -> Option<Vec<Classification>> {
    let model = get_model().await;

    let image = match ImageReader::open(&path)
        .map_err(|err| {
            println!("Failed to open image: {:?}", err);
        })
        .and_then(|image| image.with_guessed_format().map_err(|err| {
            println!("Failed to guess image format: {:?}", err);
        }))
        .and_then(|image| image.decode().map_err(|err| {
            println!("Failed to decode image: {:?}", err);
        }))
        .map(|image| image.to_rgba8()) {
        Ok(result) => Some(result),
        Err(_) => return None,
    };
    match examine(&model, &image.unwrap()) {
        Ok(result) => Some(result),
        Err(err) => {
            println!("Failed to examine image: {:?}", err);
            None
        }
    }
}