use seed::{prelude::*, *};
use wasm_bindgen_futures::JsFuture;
use web_sys::{self, DragEvent, Event, FileList};

use webpixels::{pixelmosh, Options};

fn init(_: Url, _: &mut impl Orders<Msg>) -> Model {
    Model {
        drop_zone_active: false,
        drop_zone_content: vec![div!["Drop images here"]],
        file_texts: Vec::new(),
    }
}

struct Model {
    drop_zone_active: bool,
    drop_zone_content: Vec<Node<Msg>>,
    file_texts: Vec<String>,
}

enum Msg {
    DragEnter,
    DragOver,
    DragLeave,
    Drop(FileList),
}

fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::DragEnter => model.drop_zone_active = true,
        Msg::DragOver => (),
        Msg::DragLeave => model.drop_zone_active = false,
        Msg::Drop(file_list) => {
            model.drop_zone_active = false;
            model.file_texts.clear();

            let files = (0..file_list.length())
                .map(|index| file_list.get(index).expect("get file with given index"))
                .collect::<Vec<_>>();

            model.drop_zone_content = files.iter().map(|file| div![file.name()]).collect();

            for file in files {
                orders.perform_cmd(async move {
                    let image = JsFuture::from(file.array_buffer())
                        .await
                        .expect("read file");

                    let options = Options::default();
                    let array = js_sys::Uint8Array::new(&image);
                    let bytes: Vec<u8> = array.to_vec();
                    let new_array = js_sys::Uint8Array::new(
                        &unsafe {
                            js_sys::Uint8Array::view(
                                &pixelmosh(&bytes, &options).expect("PIXELMOSH failed"),
                            )
                        }
                        .into(),
                    );

                    log!("PIXELMOSH: DONE", file.name());

                    let array = js_sys::Array::new();
                    array.push(&new_array.buffer());

                    let new_image = JsValue::from(array);
                    let blob = web_sys::Blob::new_with_u8_array_sequence_and_options(
                        &new_image,
                        web_sys::BlobPropertyBag::new().type_("image/png"),
                    )
                    .unwrap();
                    let url = web_sys::Url::create_object_url_with_blob(&blob).unwrap();
                    let window = web_sys::window().unwrap();
                    window.open_with_url(&url).unwrap();
                });
            }
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
                St::Height => px(200),
                St::Width => px(200),
                St::Margin => "auto",
                St::Background => if model.drop_zone_active { "lightgreen" } else { "lightgray" },
                St::FontFamily => "sans-serif",
                St::Display => "flex",
                St::FlexDirection => "column",
                St::JustifyContent => "center",
                St::AlignItems => "center",
                St::Border => [&px(2), "dashed", "black"].join(" ");
                St::BorderRadius => px(20),
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
        if model.file_texts.is_empty() {
            div!["TODO"]
        } else {
            pre![&model.file_texts]
        }
    ]
}

pub fn main() {
    App::start("Pixelmosh", init, update, view);
}
