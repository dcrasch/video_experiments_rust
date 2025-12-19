use dioxus::prelude::*;
mod kaleidoscope;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const START_IMAGE: &[u8] = include_bytes!("../assets/start.png");
const ORIGINAL_IMAGE: Asset = asset!("/assets/start.png");

fn load_start_image() -> image::RgbImage {
    image::load_from_memory(START_IMAGE).unwrap().to_rgb8()
}

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        KaleidoscopeEditor {}
    }
}

#[component]
fn KaleidoscopeEditor() -> Element {
    let mut segments = use_signal(|| 12u32);
    let mut rotation = use_signal(|| 0_u32);
    let mut scale = use_signal(|| 1.0f64);
    let mut started = use_signal(|| false);
    let mut image_url = use_signal(|| None::<String>);
    let mut offset_x = use_signal(|| None::<u32>);
    let mut offset_y = use_signal(|| None::<u32>);

    // Run image processing when controls change
    use_effect(move || {
        if !*started.read() {
            return;
        }

        let src = load_start_image();

        let out = kaleidoscope::kaleidoscope(
            &src,
            segments(),
            None,
            rotation() as f64 * PI / 180.0_f64,
            0.0,
            match (offset_x(), offset_y()) {
                (Some(x), Some(y)) => Some((x, y)),
                _ => None,
            },
            None,
            scale(),
        );

        image_url.set(Some(rgb_to_data_url(&out)));
    });

    rsx! {
        if !*started.read() {
            div {
                button { onclick: move |_| started.set(true), "Start" }
            }
        } else {
            div {
                div {
                    label { "Segments:" }
                    input {
                        r#type: "range",
                        min: 3,
                        max: 36,
                        value: "{segments}",
                        oninput: move |e| segments.set(e.value().parse().unwrap()),
                    }
                    label { "Scale:" }
                    input {
                        r#type: "range",
                        min: 10,
                        max: 300,
                        value: "{(scale() * 100.0) as i32}",
                        oninput: move |e| { scale.set(e.value().parse::<f64>().unwrap() / 100.0) },
                    }
                    label { "Start Angle" }
                    input {
                        r#type: "range",
                        min: 0,
                        max: 360,
                        value: "{rotation}",
                        oninput: move |e| rotation.set(e.value().parse().unwrap()),
                    }
                    div { class: "original",
                        svg {
                            width: "200",
                            height: "200",
                            view_box: "0 0 200 200",

                            image {
                                class: "original",
                                href: "{ORIGINAL_IMAGE}",
                                x: "0",
                                y: "0",
                                width: "100",
                                height: "75",
                                preserve_aspect_ratio: "xMidYMid slice",
                                onclick: move |evt| {
                                    let click = evt.element_coordinates();
                                    let click_x = click.x;
                                    let click_y = click.y;

                                    let scale_x = 1450.0 as f64 / 100.0;
                                    let scale_y = 1082.0 as f64 / 75.0;

                                    let img_x = (click_x * scale_x).round() as u32;
                                    let img_y = (click_y * scale_y).round() as u32;

                                    offset_x.set(Some(img_x));
                                    offset_y.set(Some(img_y));
                                },
                            }
                            polyline {
                                points: {
                                    let scale_x = 1450.0 as f64 / 100.0;
                                    let scale_y = 1082.0 as f64 / 75.0;

                                    let x1 = offset_x().unwrap_or_default() as f64 / scale_x;
                                    let y1 = offset_y().unwrap_or_default() as f64 / scale_y;

                                    let theta2 = (rotation() as f64).to_radians();
                                    let x2 = (x1 + 100.0 * theta2.cos()).clamp(0.0, 100.0);
                                    let y2 = (y1 + 100.0 * theta2.sin()).clamp(0.0, 75.0);

                                    let theta3 = theta2 + (360.0 / (segments() as f64)).to_radians();
                                    let x3 = (x1 + 100.0 * theta3.cos()).clamp(0.0, 100.0);
                                    let y3 = (y1 + 100.0 * theta3.sin()).clamp(0.0, 75.0);

                                    format!("{x2},{y2} {x1},{y1} {x3},{y3}")
                                },
                                fill: "none",
                                stroke: "black",
                                stroke_width: "2",
                            }
                        }
                    }
                }
                div { class: "viewer",
                    if let Some(url) = image_url() {
                        img { src: "{url}" }
                    }
                }
            }
        }
    }
}

use base64::{engine::general_purpose, Engine};
use std::{f64::consts::PI, io::Cursor};

fn rgb_to_data_url(img: &image::RgbImage) -> String {
    let mut bytes: Vec<u8> = Vec::new();
    img.write_to(&mut Cursor::new(&mut bytes), image::ImageFormat::Png)
        .expect("Couldn't write image to bytes.");

    let b64 = general_purpose::STANDARD.encode(bytes);
    format!("data:image/png;base64,{}", b64)
}
