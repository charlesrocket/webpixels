use sycamore::prelude::*;

use webpixels::{pixelmosh, Options};

fn main() {
    sycamore::render(|cx| view! { cx,
        p { "Hello, you!" }
    });
}
