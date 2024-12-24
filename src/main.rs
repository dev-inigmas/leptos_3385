#[cfg(feature = "ssr")]
use {
  axum::Router,
  leptos::config::get_configuration,
  leptos_axum::{generate_route_list, LeptosRoutes},
  leptos_3385::shell,
};

use leptos_3385::App;

#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
  let conf = get_configuration(Some("Cargo.toml")).unwrap();
  let leptos_options = conf.leptos_options;
  let addr = leptos_options.site_addr;
  let routes = generate_route_list(App);

  let app = Router::new()
    .leptos_routes(&leptos_options, routes, {
      let leptos_options = leptos_options.clone();
      move || shell(leptos_options.clone())
    })
    .fallback(leptos_axum::file_and_error_handler(shell))
    .with_state(leptos_options);

  println!("listening on {}", addr);
  let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
  axum::serve(listener, app.into_make_service())
    .await.unwrap();
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
  _ = console_log::init_with_level(log::Level::Debug);
  console_error_panic_hook::set_once();
  mount_to_body(App);
}
