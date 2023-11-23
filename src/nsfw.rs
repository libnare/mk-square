use std::fs::File;
use std::io::BufReader;
use image::codecs::gif::GifDecoder;
use std::string::String;
use nsfw::{create_model, examine};
use nsfw::gif::GifParser;
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
pub async fn detect_sensitivity(path: String, mime: String, sensitive_threshold: f64, sensitive_threshold_for_porn: f64, _analyze_video: bool) -> napi::Result<Option<Nsfw>> {
    let mut sensitive = false;
    let mut porn = false;
    fn judge_prediction(result: &[Classification], sensitive_threshold: f32, sensitive_threshold_for_porn: f32) -> (bool, bool) {
        let mut sensitive = false;
        let mut porn = false;

        if result.iter().any(|x| x.metric == Sexy && x.score > sensitive_threshold) {
            sensitive = true;
        }

        if result.iter().any(|x| x.metric == Hentai && x.score > sensitive_threshold) {
            sensitive = true;
        }

        if result.iter().any(|x| x.metric == Porn && x.score > sensitive_threshold) {
            sensitive = true;
        }

        if result.iter().any(|x| x.metric == Porn && x.score > sensitive_threshold_for_porn) {
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
                let judge = judge_prediction(&result, sensitive_threshold as f32, sensitive_threshold_for_porn as f32);
                sensitive = judge.0;
                porn = judge.1;
            },
            _ => {
                return Ok(None);
            }
        };
    } else if mime == "image/gif" {
        if let Some(result) = detect_gif(path).await {
            for frame in result {
                if let Ok(frame) = frame {
                    let (is_sensitive, is_porn) = judge_prediction(
                        &frame,
                        sensitive_threshold as f32,
                        sensitive_threshold_for_porn as f32,
                    );

                    sensitive |= is_sensitive;
                    porn |= is_porn;
                }
            }
        } else {
            return Ok(None);
        }
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
    let image = match image::open(&path) {
        Ok(image) => image.to_rgba8(),
        Err(err) => {
            println!("Failed to open image: {:?}", err);
            return None;
        }
    };
    match examine(&model, &image) {
        Ok(result) => Some(result),
        Err(err) => {
            println!("Failed to examine image: {:?}", err);
            None
        }
    }
}

async fn detect_gif(path: String) -> Option<GifParser<'static>> {
    let model = get_model().await;
    let file = BufReader::new(match File::open(path) {
        Ok(file) => file,
        Err(err) => {
            println!("Failed to open file: {:?}", err);
            return None;
        }
    });
    let frames = GifParser::new(GifDecoder::new(file).unwrap(), Box::leak(Box::new(model)));

    Some(frames)
}