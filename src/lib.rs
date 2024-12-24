//! # `lib.rs`
//!
//! Application Root

use leptos::prelude::*;
use leptos_meta::{provide_meta_context, Link, Meta, MetaTags, Stylesheet};
use leptos_router::{
  components::{Route, Router, Routes},
  path,
};

use layout::TopNav;

mod layout;
mod page;

pub fn shell(options: LeptosOptions) -> impl IntoView {
  view! {
    <!DOCTYPE html>
    <html lang="en">
      <head>
        <meta charset="utf-8" />
        <meta name="viewport" content="width=device-width, initial-scale=1.0" />
        <AutoReload options=options.clone() />
        <HydrationScripts options />
        <MetaTags />
      </head>
      <body>
        <App />
      </body>
    </html>
  }
}

#[component]
pub fn App() -> impl IntoView {
  provide_meta_context();

  view! {
    <Stylesheet id="leptos" href="/pkg/leptos_3385.css" />
    <Link rel="shortcut icon" type_="image/ico" href="/favicon.ico" />
    <Meta name="description" content="leptos bug reproduction" />
    <Router>
      <TopNav />
      <main>
        <Routes fallback=|| "Not Found">
          <Route path=path!("/") view=page::Home />
          <Route path=path!("/a") view=page::PageA />
        </Routes>
      </main>
    </Router>
  }
}

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
  console_error_panic_hook::set_once();
  hydrate_body(App);
}
