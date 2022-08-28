use js_sys::{Array, Uint8Array};
use seed::{prelude::*, *};
use wasm_bindgen_futures::JsFuture;
use web_sys::{self, Blob, BlobPropertyBag, File};

use webpixels::{pixelmosh, Options};

fn init(_: Url, _: &mut impl Orders<Msg>) -> Model {
    Model {
        image_view: "".to_string(),
        options: Options::default(),
        pixelmosh_active: false,
        storage: vec![0, 0],
        storage_active: false,
    }
}

struct Model {
    image_view: String,
    options: Options,
    pixelmosh_active: bool,
    storage: Vec<u8>,
    storage_active: bool,
}

enum Msg {
    Download,
    FileChanged(Option<File>),
    PixelMosh(JsValue),
    ReMosh,
}

fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
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
                    .expect("read file");

                Msg::PixelMosh(image)
            });
        }
        Msg::PixelMosh(image) => {
            let array = Uint8Array::new(&image);
            let bytes: Vec<u8> = array.to_vec();
            model.storage = bytes;
            model.storage_active = true;

            let new_array = Uint8Array::new(
                &unsafe {
                    Uint8Array::view(
                        &pixelmosh(&model.storage, &model.options).expect("PIXELMOSH failed"),
                    )
                }
                .into(),
            );

            let array = Array::new();
            array.push(&new_array.buffer());

            let image = JsValue::from(array);
            let blob = Blob::new_with_u8_array_sequence_and_options(
                &image,
                BlobPropertyBag::new().type_("image/png"),
            )
            .unwrap();

            let url = web_sys::Url::create_object_url_with_blob(&blob).unwrap();
            model.image_view = url;
            log!(model.options.seed());
            model.options.set_seed(Options::default().seed());
            model.pixelmosh_active = true;
        }
        Msg::ReMosh => {
            let new_array = Uint8Array::new(
                &unsafe {
                    Uint8Array::view(
                        &pixelmosh(&model.storage, &model.options).expect("PIXELMOSH failed"),
                    )
                }
                .into(),
            );

            let array = Array::new();
            array.push(&new_array.buffer());

            let image = JsValue::from(array);
            let blob = Blob::new_with_u8_array_sequence_and_options(
                &image,
                BlobPropertyBag::new().type_("image/png"),
            )
            .unwrap();

            let url = web_sys::Url::create_object_url_with_blob(&blob).unwrap();
            model.image_view = url;
            log!(model.options.seed());
            model.options.set_seed(Options::default().seed());
        }
    }
}

fn view(model: &Model) -> Node<Msg> {
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
                    IF!(model.pixelmosh_active => img![
                        attrs! {
                            At::Src => model.image_view
                            At::Width => "650px"
                        },
                        style![
                            St::Border => [&px(7), "solid", "black"].join(" "),
                        ],
                    ])
                ],
                style![
                    St::Display => "flex",
                    St::FlexDirection => "column",
                    St::AlignItems => "center",
                ],
                div![
                    IF!(model.pixelmosh_active => button![
                        "MOSH",
                        ev(Ev::Click, |_| Msg::ReMosh),
                        style![
                            St::Padding => "4px",
                        ],
                    ]),
                    IF!(model.pixelmosh_active => button![
                        "DOWNLOAD",
                        ev(Ev::Click, |_| Msg::Download),
                        style![
                            St::Padding => "4px",
                        ],
                    ]),
                    style![
                        St::Display => "flex",
                        St::FlexDirection => "row",
                        St::AlignItems => "center",
                    ],
                ],
            ]
        } else {
            empty![]
        }
    ]
}

pub fn main() {
    App::start("Pixelmosh", init, update, view);
}
