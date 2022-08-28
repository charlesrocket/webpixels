use js_sys::{Array, Uint8Array};
use seed::{prelude::*, *};
use wasm_bindgen_futures::JsFuture;
use web_sys::{self, Blob, BlobPropertyBag, File};

use webpixels::{pixelmosh, Options};

fn init(_: Url, _: &mut impl Orders<Msg>) -> Model {
    Model {
        image_view: "".to_string(),
        options: Options::default(),
        storage: vec![0, 0],
        storage_active: false,
    }
}

struct Model {
    image_view: String,
    options: Options,
    storage: Vec<u8>,
    storage_active: bool,
}

enum Msg {
    Download,
    FileChanged(Option<File>),
    FileStore(JsValue),
    FileView(Uint8Array),
    PixelMosh,
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
                    .expect("Can not read file");

                Msg::FileStore(image)
            });
        }
        Msg::FileStore(file) => {
            let array = Uint8Array::new(&file);
            let bytes: Vec<u8> = array.to_vec();

            model.storage = bytes;
            model.storage_active = true;

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
            let mosh_array = Uint8Array::new(
                &unsafe {
                    Uint8Array::view(
                        &pixelmosh(&model.storage, &model.options).expect("PIXELMOSH failed"),
                    )
                }
                .into(),
            );

            orders.send_msg(Msg::FileView(mosh_array));
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
            ]
        } else {
            div![
                "WAITING FOR PNG FILE",
                style![
                    St::Display => "flex",
                    St::FlexDirection => "column",
                    St::AlignItems => "center",
                    St::Margin => "11px",
                    St::Padding => "3px",
                    St::FontFamily => "monospace",
                    St::FontSize => "medium",
                    St::Border => [&px(3), "dashed", "black"].join(" "),
                ]
            ]
        }
    ]
}

pub fn main() {
    App::start("Pixelmosh", init, update, view);
}
