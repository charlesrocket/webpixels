use js_sys::{Array, Uint8Array};
use seed::{attrs, button, div, img, input, log, prelude::*, style, wasm_bindgen_futures};
use wasm_bindgen_futures::JsFuture;
use web_sys::{self, Blob, BlobPropertyBag, File};

use webpixels::{pixelmosh, Options};

fn init(_: Url, _: &mut impl Orders<Msg>) -> Model {
    Model {
        controls: false,
        image_view: "".to_string(),
        options: Options::default(),
        storage: vec![0, 0],
        storage_active: false,
    }
}

struct Model {
    controls: bool,
    image_view: String,
    options: Options,
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
            let blob = Blob::new_with_u8_array_sequence_and_options(
                &image,
                BlobPropertyBag::new().type_("image/png"),
            )
            .unwrap();

            let url = web_sys::Url::create_object_url_with_blob(&blob).unwrap();
            model.image_view = url;
        }
        Msg::PixelMosh => {
            log!(model.options.seed());
            match pixelmosh(&model.storage, &model.options) {
                Ok(moshed) => orders.send_msg(Msg::Convert(moshed)),
                Err(_) => orders.send_msg(Msg::Reload),
            };

            log!["PIXELMOSH: DONE"];
            model.options.set_seed(Options::default().seed());
        }
        Msg::Reload => {
            log!["ERROR! RESTARTING..."];
            Url::reload();
        }
        Msg::DecMinRate => {
            let value = model.options.min_rate() - 1;
            model.options.set_min_rate(value.clamp(1, 100));
            log!(model.options.min_rate());
        }
        Msg::IncMinRate => {
            let value = model.options.min_rate() + 1;
            model.options.set_min_rate(value.clamp(1, 100));
            log!(model.options.min_rate());
        }
        Msg::DecMaxRate => {
            let value = model.options.max_rate() - 1;
            model.options.set_max_rate(value.clamp(1, 100));
            log!(model.options.max_rate());
        }
        Msg::IncMaxRate => {
            let value = model.options.max_rate() + 1;
            model.options.set_max_rate(value.clamp(1, 100));
            log!(model.options.max_rate());
        }
        Msg::DecPixelation => {
            let value = model.options.pixelation() - 1;
            model.options.set_pixelation(value.clamp(1, 255));
            log!(model.options.pixelation());
        }
        Msg::IncPixelation => {
            let value = model.options.pixelation() + 1;
            model.options.set_pixelation(value.clamp(1, 255));
            log!(model.options.pixelation());
        }
        Msg::DecLineShift => {
            let value = model.options.line_shift() - 0.1;
            model.options.set_line_shift(value.clamp(0.0, 1.0));
            log!(model.options.line_shift());
        }
        Msg::IncLineShift => {
            let value = model.options.line_shift() + 0.1;
            model.options.set_line_shift(value.clamp(0.0, 1.0));
            log!(model.options.line_shift());
        }
        Msg::DecReverse => {
            let value = model.options.reverse() - 0.1;
            model.options.set_reverse(value.clamp(0.0, 1.0));
            log!(model.options.reverse());
        }
        Msg::IncReverse => {
            let value = model.options.reverse() + 0.1;
            model.options.set_reverse(value.clamp(0.0, 1.0));
            log!(model.options.reverse());
        }
        Msg::DecFlip => {
            let value = model.options.flip() - 0.1;
            model.options.set_flip(value.clamp(0.0, 1.0));
            log!(model.options.flip());
        }
        Msg::IncFlip => {
            let value = model.options.flip() + 0.1;
            model.options.set_flip(value.clamp(0.0, 1.0));
            log!(model.options.flip());
        }
        Msg::DecChannelSwap => {
            let value = model.options.channel_swap() - 0.1;
            model.options.set_channel_swap(value.clamp(0.0, 1.0));
            log!(model.options.channel_swap());
        }
        Msg::IncChannelSwap => {
            let value = model.options.channel_swap() + 0.1;
            model.options.set_channel_swap(value.clamp(0.0, 1.0));
            log!(model.options.channel_swap());
        }
        Msg::DecChannelShift => {
            let value = model.options.channel_shift() - 0.1;
            model.options.set_channel_shift(value.clamp(0.0, 1.0));
            log!(model.options.channel_shift());
        }
        Msg::IncChannelShift => {
            let value = model.options.channel_shift() + 0.1;
            model.options.set_channel_shift(value.clamp(0.0, 1.0));
            log!(model.options.channel_shift());
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
                            St::Border => [&px(3), "dashed", "black"].join(" "),
                        ],
                    ],
                    if model.controls {
                        div![
                            div![
                                div!["Min rate: ", model.options.min_rate().to_string()],
                                button![ev(Ev::Click, |_| Msg::DecMinRate), "-"],
                                button![ev(Ev::Click, |_| Msg::IncMinRate), "+"],
                                div!["Max rate: ", model.options.max_rate().to_string()],
                                button![ev(Ev::Click, |_| Msg::DecMaxRate), "-"],
                                button![ev(Ev::Click, |_| Msg::IncMaxRate), "+"],
                                style![
                                    St::Padding => "4px",
                                ],
                            ],
                            div![
                                div!["Pixelation: ", model.options.pixelation().to_string()],
                                button![ev(Ev::Click, |_| Msg::DecPixelation), "-"],
                                button![ev(Ev::Click, |_| Msg::IncPixelation), "+"],
                                div![
                                    "Line shift: ",
                                    float_to_str_trim(model.options.line_shift())
                                ],
                                button![ev(Ev::Click, |_| Msg::DecLineShift), "-"],
                                button![ev(Ev::Click, |_| Msg::IncLineShift), "+"],
                                style![
                                    St::Padding => "4px",
                                ],
                            ],
                            div![
                                div!["Reverse: ", float_to_str_trim(model.options.reverse())],
                                button![ev(Ev::Click, |_| Msg::DecReverse), "-"],
                                button![ev(Ev::Click, |_| Msg::IncReverse), "+"],
                                div!["Flip: ", float_to_str_trim(model.options.flip())],
                                button![ev(Ev::Click, |_| Msg::DecFlip), "-"],
                                button![ev(Ev::Click, |_| Msg::IncFlip), "+"],
                                style![
                                    St::Padding => "4px",
                                ],
                            ],
                            div![
                                div![
                                    "Channel Swap: ",
                                    float_to_str_trim(model.options.channel_swap())
                                ],
                                button![ev(Ev::Click, |_| Msg::DecChannelSwap), "-"],
                                button![ev(Ev::Click, |_| Msg::IncChannelSwap), "+"],
                                div![
                                    "Channel Shift: ",
                                    float_to_str_trim(model.options.channel_shift())
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
                                St::Padding => "5px",
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
