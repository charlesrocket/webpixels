use js_sys::{Array, Uint8Array};
use seed::{prelude::*, *};
use wasm_bindgen_futures::JsFuture;
use web_sys::{self, Blob, BlobPropertyBag, DragEvent, Event, FileList};

use webpixels::{pixelmosh, Options};

fn init(_: Url, _: &mut impl Orders<Msg>) -> Model {
    Model {
        drop_zone_active: false,
        drop_zone_content: vec![div!["PNG DROP ZONE"]],
        image_view: "".to_string(),
        options: Options::default(),
        pixelmosh_active: false,
        storage: vec![0, 0],
        storage_active: false,
    }
}

struct Model {
    drop_zone_active: bool,
    drop_zone_content: Vec<Node<Msg>>,
    image_view: String,
    options: Options,
    pixelmosh_active: bool,
    storage: Vec<u8>,
    storage_active: bool,
}

enum Msg {
    Download,
    DragEnter,
    DragOver,
    DragLeave,
    Drop(FileList),
    FileStore(JsValue),
    PixelMosh,
}

fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::Download => {
            let window = web_sys::window().unwrap();
            window.open_with_url(&model.image_view).unwrap();
        }
        Msg::DragEnter => model.drop_zone_active = true,
        Msg::DragOver => (),
        Msg::DragLeave => model.drop_zone_active = false,
        Msg::Drop(file_list) => {
            model.drop_zone_active = false;
            model.storage.clear();
            model.image_view.clear();

            let files = (0..file_list.length())
                .map(|index| file_list.get(index).unwrap())
                .collect::<Vec<_>>();

            model.drop_zone_content = files.iter().map(|file| div![file.name()]).collect();

            orders.perform_cmd(async move {
                let image = JsFuture::from(files.last().unwrap().array_buffer())
                    .await
                    .expect("read file");

                Msg::FileStore(image)
            });
        }
        Msg::FileStore(image) => {
            let array = Uint8Array::new(&image);
            let bytes: Vec<u8> = array.to_vec();
            model.storage = bytes;
            model.storage_active = true;
            log!("IMAGE LOADED");
        }
        Msg::PixelMosh => {
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
            log!("PIXELMOSH: DONE");
            log!(model.options.seed());
            model.options.set_seed(Options::default().seed());
            model.pixelmosh_active = true;
        }
    }
}

trait IntoDragEvent {
    fn into_drag_event(self) -> DragEvent;
}

impl IntoDragEvent for Event {
    fn into_drag_event(self) -> DragEvent {
        self.dyn_into::<web_sys::DragEvent>()
            .expect("cannot cast given event into DragEvent")
    }
}

macro_rules! stop_and_prevent {
    { $event:expr } => {
        {
            $event.stop_propagation();
            $event.prevent_default();
        }
     };
}

fn view(model: &Model) -> Node<Msg> {
    div![
        div![
            style![
                St::Height => "auto",
                St::Width => "auto",
                St::Margin => "auto",
                St::Background => if model.drop_zone_active { "lightgreen" } else { "orange" },
                St::FontFamily => "monospace",
                St::Color => "black",
                St::FontSize => "42px",
                St::Padding => "13px",
                St::FontWeight => "bold",
                St::Display => "flex",
                St::FlexDirection => "column",
                St::JustifyContent => "center",
                St::AlignItems => "center",
                St::Border => [&px(7), "dashed", "black"].join(" ");
            ],
            ev(Ev::DragEnter, |event| {
                stop_and_prevent!(event);
                Msg::DragEnter
            }),
            ev(Ev::DragOver, |event| {
                let drag_event = event.into_drag_event();
                stop_and_prevent!(drag_event);
                drag_event.data_transfer().unwrap().set_drop_effect("copy");
                Msg::DragOver
            }),
            ev(Ev::DragLeave, |event| {
                stop_and_prevent!(event);
                Msg::DragLeave
            }),
            ev(Ev::Drop, |event| {
                let drag_event = event.into_drag_event();
                stop_and_prevent!(drag_event);
                let file_list = drag_event.data_transfer().unwrap().files().unwrap();
                Msg::Drop(file_list)
            }),
            div![
                style! {
                    St::PointerEvents => "none",
                },
                model.drop_zone_content.clone(),
            ],
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
                    button![
                        "PROCESS",
                        ev(Ev::Click, |_| Msg::PixelMosh),
                        style![
                            St::Padding => "4px",

                        ],
                    ],
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
