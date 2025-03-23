use gloo_console::log;
use js_sys::{Array, Uint8Array};
use seed::{attrs, button, div, img, input, prelude::*, style, wasm_bindgen_futures};
use wasm_bindgen_futures::JsFuture;
use web_sys::{self, Blob, BlobPropertyBag, File};

use webpixels::Core;

fn init(_: Url, _: &mut impl Orders<Msg>) -> Model {
    Model {
        controls: false,
        image_view: "".to_string(),
        core: Core::default(),
        storage: vec![0, 0],
        storage_active: false,
    }
}

struct Model {
    controls: bool,
    image_view: String,
    core: Core,
    storage: Vec<u8>,
    storage_active: bool,
}

enum Msg {
    ControlsRequested,
    Convert(Vec<u8>),
    Download,
    FileChanged(Option<File>),
    FileStore(JsValue),
    FileView(Uint8Array),
    PixelMosh,
    Reload,
    // Options
    Ansi,
    DecMinRate,
    IncMinRate,
    DecMaxRate,
    IncMaxRate,
    DecPixelation,
    IncPixelation,
    DecLineShift,
    IncLineShift,
    DecReverse,
    IncReverse,
    DecFlip,
    IncFlip,
    DecChannelSwap,
    IncChannelSwap,
    DecChannelShift,
    IncChannelShift,
}

fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::ControlsRequested => model.controls = true,
        Msg::Convert(input) => {
            let array = Uint8Array::new(&unsafe { Uint8Array::view(&input) }.into());
            orders.send_msg(Msg::FileView(array));
        }
        Msg::Download => {
            let window = web_sys::window().unwrap();
            window.open_with_url(&model.image_view).unwrap();
        }
        Msg::FileChanged(file) => {
            model.image_view.clear();
            model.storage.clear();

            orders.perform_cmd(async move {
                let image = JsFuture::from(file.unwrap().array_buffer())
                    .await
                    .expect("Can not read file");

                Msg::FileStore(image)
            });
        }
        Msg::FileStore(file) => {
            let array = Uint8Array::new(&file);
            let bytes: Vec<u8> = array.to_vec();

            model.storage = bytes;
            model.storage_active = true;

            log!["FILE LOADED"];
            orders.send_msg(Msg::PixelMosh);
        }
        Msg::FileView(file) => {
            let array = Array::new();
            array.push(&file.buffer());

            let image = JsValue::from(array);
            let blob_prop = BlobPropertyBag::new();
            blob_prop.set_type("image/png");

            let blob = Blob::new_with_u8_array_sequence_and_options(&image, &blob_prop).unwrap();

            let url = web_sys::Url::create_object_url_with_blob(&blob).unwrap();
            model.image_view = url;
        }
        Msg::PixelMosh => {
            log!(model.core.seed());
            match model.core.pixelmosh(&model.storage) {
                Ok(moshed) => orders.send_msg(Msg::Convert(moshed)),
                Err(_) => orders.send_msg(Msg::Reload),
            };

            log!["PIXELMOSH: DONE"];
            model.core.new_seed();
        }
        Msg::Reload => {
            log!["ERROR! RESTARTING..."];
            Url::reload();
        }
        Msg::Ansi => {
            let value = !model.core.ansi();
            model.core.set_ansi(value);
            log!(model.core.ansi());
        }
        Msg::DecMinRate => {
            let value = model.core.min_rate() - 1;
            model.core.set_min_rate(value.clamp(1, 100));
            log!(model.core.min_rate());
        }
        Msg::IncMinRate => {
            let value = model.core.min_rate() + 1;
            model.core.set_min_rate(value.clamp(1, 100));
            log!(model.core.min_rate());
        }
        Msg::DecMaxRate => {
            let value = model.core.max_rate() - 1;
            model.core.set_max_rate(value.clamp(1, 100));
            log!(model.core.max_rate());
        }
        Msg::IncMaxRate => {
            let value = model.core.max_rate() + 1;
            model.core.set_max_rate(value.clamp(1, 100));
            log!(model.core.max_rate());
        }
        Msg::DecPixelation => {
            let value = model.core.pixelation() - 1;
            model.core.set_pixelation(value.clamp(1, 255));
            log!(model.core.pixelation());
        }
        Msg::IncPixelation => {
            let value = model.core.pixelation() + 1;
            model.core.set_pixelation(value.clamp(1, 255));
            log!(model.core.pixelation());
        }
        Msg::DecLineShift => {
            let value = model.core.line_shift() - 0.1;
            model.core.set_line_shift(value.clamp(0.0, 1.0));
            log!(model.core.line_shift());
        }
        Msg::IncLineShift => {
            let value = model.core.line_shift() + 0.1;
            model.core.set_line_shift(value.clamp(0.0, 1.0));
            log!(model.core.line_shift());
        }
        Msg::DecReverse => {
            let value = model.core.reverse() - 0.1;
            model.core.set_reverse(value.clamp(0.0, 1.0));
            log!(model.core.reverse());
        }
        Msg::IncReverse => {
            let value = model.core.reverse() + 0.1;
            model.core.set_reverse(value.clamp(0.0, 1.0));
            log!(model.core.reverse());
        }
        Msg::DecFlip => {
            let value = model.core.flip() - 0.1;
            model.core.set_flip(value.clamp(0.0, 1.0));
            log!(model.core.flip());
        }
        Msg::IncFlip => {
            let value = model.core.flip() + 0.1;
            model.core.set_flip(value.clamp(0.0, 1.0));
            log!(model.core.flip());
        }
        Msg::DecChannelSwap => {
            let value = model.core.channel_swap() - 0.1;
            model.core.set_channel_swap(value.clamp(0.0, 1.0));
            log!(model.core.channel_swap());
        }
        Msg::IncChannelSwap => {
            let value = model.core.channel_swap() + 0.1;
            model.core.set_channel_swap(value.clamp(0.0, 1.0));
            log!(model.core.channel_swap());
        }
        Msg::DecChannelShift => {
            let value = model.core.channel_shift() - 0.1;
            model.core.set_channel_shift(value.clamp(0.0, 1.0));
            log!(model.core.channel_shift());
        }
        Msg::IncChannelShift => {
            let value = model.core.channel_shift() + 0.1;
            model.core.set_channel_shift(value.clamp(0.0, 1.0));
            log!(model.core.channel_shift());
        }
    }
}

fn view(model: &Model) -> Node<Msg> {
    div![
        style![
            St::Display => "flex",
            St::FlexDirection => "column",
            St::JustifyContent => "center",
            St::MinHeight => "95vh",
        ],
        div![
            input![
                ev(Ev::Change, |event| {
                    let file = event
                        .target()
                        .and_then(|target| target.dyn_into::<web_sys::HtmlInputElement>().ok())
                        .and_then(|file_input| file_input.files())
                        .and_then(|file_list| file_list.get(0));

                    Msg::FileChanged(file)
                }),
                style![
                    St::Border => [&px(5), "dashed", "black"].join(" "),
                    St::Padding => "9px",
                    St::FontFamily => "monospace",
                ],
                attrs! {
                    At::Type => "file",
                    At::Id => "form-file",
                    At::Accept => "image/png",
                }
            ],
            style![
                St::Display => "flex",
                St::FlexDirection => "column",
                St::AlignItems => "center",
            ],
            if model.storage_active {
                div![
                    div![
                        style![
                            St::Display => "flex",
                            St::FlexDirection => "column",
                            St::AlignItems => "center",
                            St::Padding => "12px",
                        ],
                        img![
                            attrs! {
                                At::Src => model.image_view
                                At::Width => "500px"
                            },
                            style![
                                St::Border => [&px(7), "solid", "black"].join(" "),
                            ],
                        ],
                    ],
                    style![
                        St::Display => "flex",
                        St::FlexDirection => "column",
                        St::AlignItems => "center",
                    ],
                    div![
                        button![
                            "MOSH",
                            ev(Ev::Click, |_| Msg::PixelMosh),
                            style![
                                St::Padding => "4px",
                            ],
                        ],
                        button![
                            "ANSI",
                            ev(Ev::Click, |_| Msg::Ansi),
                            style![
                                St::Padding => "4px",
                                St::BackgroundColor => if model.core.ansi() {"green"} else {"gray"},
                            ],
                        ],
                        button![
                            "DOWNLOAD",
                            ev(Ev::Click, |_| Msg::Download),
                            style![
                                St::Padding => "4px",
                            ],
                        ],
                        style![
                            St::Display => "flex",
                            St::FlexDirection => "row",
                            St::AlignItems => "center",
                            St::Padding => "5px",
                            St::Gap => "3px",
                            St::Border => [&px(3), "dashed", "black"].join(" "),
                        ],
                    ],
                    if model.controls {
                        div![
                            div![
                                div![
                                    "Min rate: ",
                                    model.core.min_rate().to_string(),
                                    style![St::MarginTop => "4px", St::MarginBottom => "4px"]
                                ],
                                button![ev(Ev::Click, |_| Msg::DecMinRate), "-"],
                                button![ev(Ev::Click, |_| Msg::IncMinRate), "+"],
                                div![
                                    "Max rate: ",
                                    model.core.max_rate().to_string(),
                                    style![St::MarginTop => "4px", St::MarginBottom => "4px"]
                                ],
                                button![ev(Ev::Click, |_| Msg::DecMaxRate), "-"],
                                button![ev(Ev::Click, |_| Msg::IncMaxRate), "+"],
                                style![
                                    St::Padding => "4px",
                                ],
                            ],
                            div![
                                div![
                                    "Pixelation: ",
                                    model.core.pixelation().to_string(),
                                    style![St::MarginTop => "4px", St::MarginBottom => "4px"]
                                ],
                                button![ev(Ev::Click, |_| Msg::DecPixelation), "-"],
                                button![ev(Ev::Click, |_| Msg::IncPixelation), "+"],
                                div![
                                    "Line shift: ",
                                    float_to_str_trim(model.core.line_shift()),
                                    style![St::MarginTop => "4px", St::MarginBottom => "4px"]
                                ],
                                button![ev(Ev::Click, |_| Msg::DecLineShift), "-"],
                                button![ev(Ev::Click, |_| Msg::IncLineShift), "+"],
                                style![
                                    St::Padding => "4px",
                                ],
                            ],
                            div![
                                div![
                                    "Reverse: ",
                                    float_to_str_trim(model.core.reverse()),
                                    style![St::MarginTop => "4px", St::MarginBottom => "4px"]
                                ],
                                button![ev(Ev::Click, |_| Msg::DecReverse), "-"],
                                button![ev(Ev::Click, |_| Msg::IncReverse), "+"],
                                div![
                                    "Flip: ",
                                    float_to_str_trim(model.core.flip()),
                                    style![St::MarginTop => "4px", St::MarginBottom => "4px"]
                                ],
                                button![ev(Ev::Click, |_| Msg::DecFlip), "-"],
                                button![ev(Ev::Click, |_| Msg::IncFlip), "+"],
                                style![
                                    St::Padding => "4px",
                                ],
                            ],
                            div![
                                div![
                                    "Channel Swap: ",
                                    float_to_str_trim(model.core.channel_swap()),
                                    style![St::MarginTop => "4px", St::MarginBottom => "4px"]
                                ],
                                button![ev(Ev::Click, |_| Msg::DecChannelSwap), "-"],
                                button![ev(Ev::Click, |_| Msg::IncChannelSwap), "+"],
                                div![
                                    "Channel Shift: ",
                                    float_to_str_trim(model.core.channel_shift()),
                                    style![St::MarginTop => "4px", St::MarginBottom => "4px"]
                                ],
                                button![ev(Ev::Click, |_| Msg::DecChannelShift), "-"],
                                button![ev(Ev::Click, |_| Msg::IncChannelShift), "+"],
                                style![
                                    St::Padding => "4px",
                                ],
                            ],
                            style![
                                St::Display => "flex",
                                St::FlexDirection => "row",
                                St::AlignItems => "center",
                                St::TextAlign => "center",
                                St::FontFamily => "monospace",
                                St::FontSize => "x-small",
                                St::Padding => "4px",
                                St::Margin => "10px",
                                St::Border => [&px(3), "dashed", "black"].join(" "),
                            ],
                        ]
                    } else {
                        div![
                            button![
                                "SETTINGS",
                                ev(Ev::Click, |_| Msg::ControlsRequested),
                                style![
                                    St::Padding => "4px",
                                    St::FontSize => "small",
                                ],
                            ],
                            style![
                                    St::Border => [&px(3), "dashed", "black"].join(" "),
                                    St::Margin => "10px",
                                    St::Padding => "4px",
                            ]
                        ]
                    }
                ]
            } else {
                div![
                    "INSERT PNG FILE",
                    style![
                        St::Display => "flex",
                        St::FlexDirection => "column",
                        St::AlignItems => "center",
                        St::Margin => "11px",
                        St::Padding => "4px",
                        St::FontFamily => "monospace",
                        St::FontSize => "medium",
                    ]
                ]
            }
        ]
    ]
}

pub fn main() {
    App::start("Pixelmosh", init, update, view);
}

fn float_to_str_trim(float: f64) -> String {
    if float != 1.0 && float != 0.0 {
        let str = float.to_string();
        str[..3].to_string()
    } else {
        float.to_string()
    }
}
