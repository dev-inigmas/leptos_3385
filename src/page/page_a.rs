//! # `leptos_3385::page::page_a`
//!
//! Page A

use leptos::prelude::*;

#[server]
async fn fetch_list() -> Result<Vec<(String, String)>, ServerFnError> {
  Ok(vec![
    ("1".to_string(), "qwe".to_string()),
    ("2".to_string(), "asd".to_string()),
    ("3".to_string(), "zxc".to_string()),
  ])
}

#[component]
pub fn PageA() -> impl IntoView {
  let str_list_rsrc = Resource::new(|| (), move |_| fetch_list());

  view! {
    <h1>"Page A"</h1>

    <ul>
      <Transition
        fallback=|| view! { <li>"Loading items..."</li> }
      >
        <For
          each=move || {
            str_list_rsrc
              .get()
              .and_then(Result::ok)
              .unwrap_or_default()
              .into_iter()
          }
          key=|(id,_)| id.clone()
          children=|(_, item)| view! { <li>{ item }</li> }
        />
      </Transition>
    </ul>
  }
}
