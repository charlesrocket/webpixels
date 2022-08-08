use sycamore::prelude::*;

use webpixels::{pixelmosh, Options};

fn main() {
    webpixels::utils::set_panic_hook();
    sycamore::render(|cx| {
        view! { cx,
            p { "Hello, you!" }
        }
    });
}
