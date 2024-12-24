//! # `leptos_3385::layout::top_nav`
//!
//! Top Navigation Layout Component

use leptos::prelude::*;
use leptos_router::components::A;

#[component]
pub fn TopNav() -> impl IntoView {
  view! {
    <header>
      <nav>
        <A href="/">"  Home  "</A>
        <A href="/a">"  Page A  "</A>
      </nav>
    </header>
  }
}
