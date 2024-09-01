#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_logger::tracing;

pub mod models;
pub mod queries;
pub mod schema;

#[derive(Clone, Routable, Debug, PartialEq)]
enum Route {
    #[layout(NavBar)]
    #[route("/")]
    Home {},
    #[route("/admin")]
    Admin {},
    #[end_layout]
    #[route("/:..route")]
    PageNotFound { route: Vec<String> },
}

fn main() {
    // Init logger
    dioxus_logger::init(tracing::Level::INFO).expect("failed to init logger");
    tracing::info!("starting app");

    use std::net::{IpAddr, Ipv4Addr, SocketAddr};
    let cfg = server_only!(dioxus::fullstack::Config::new()
        .addr(SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 8080)));

    LaunchBuilder::fullstack().with_cfg(cfg).launch(App);
}

fn App() -> Element {
    rsx! {
        link { rel: "stylesheet", href: "main.css" }
        Router::<Route> {}
    }
}

#[component]
fn NavBar() -> Element {
    rsx! {
        nav {
            id: "navbar",
                Link { class: "navbar_link", to: Route::Home {}, "Home" }

                Link { class: "navbar_link", to: Route::Admin {}, "Admin" }
        }
        // The Outlet component will render child routes (In this case just the Home component) inside the Outlet component
        Outlet::<Route> {}
    }
}

#[component]
fn PageNotFound(route: Vec<String>) -> Element {
    rsx! {
        h1 { "Page not found" }
        p { "We are terribly sorry, but the page you requested doesn't exist." }
        pre { color: "red", "log:\nattemped to navigate to: {route:?}" }
    }
}

#[component]
fn Home() -> Element {
    rsx!(
        div { "Mini Paint Inventory" }
    )
}

#[component]
fn Admin() -> Element {
    rsx!(
        div { "Administration page" }
    )
}

// #[component]
// fn Home() -> Element {
//     let mut count = use_signal(|| 0);
//     let mut text = use_signal(|| String::from("..."));
//
//     rsx! {
//         div {
//             h1 { "High-Five counter: {count}" }
//             button { onclick: move |_| count += 1, "Up high!" }
//             button { onclick: move |_| count -= 1, "Down low!" }
//             button {
//                 onclick: move |_| async move {
//                     if let Ok(data) = get_server_data().await {
//                         tracing::info!("Client received: {}", data);
//                         text.set(data.clone());
//                         post_server_data(data).await.unwrap();
//                     }
//                 },
//                 "Get Server Data"
//             }
//             p { "Server data: {text}"}
//        }
//    }
//}
//
//#[server(PostServerData)]
//async fn post_server_data(data: String) -> Result<(), ServerFnError> {
//    tracing::info!("Server received: {}", data);
//    Ok(())
//}
//
//#[server(GetServerData)]
//async fn get_server_data() -> Result<String, ServerFnError> {
//    Ok("Hello from the server!".to_string())
//}
