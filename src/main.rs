use dioxus::{
    prelude::*,
    router::{Route, Router},
};

fn main() {
    dioxus::web::launch(app);
}

fn app(cx: Scope) -> Element {
    cx.render(rsx! {
        Router {
            p { "-- Dioxus Blog --" }
            Route { to: "/", self::homepage {}}
            Route { to: "", self::page_not_found {}}
        }
    })
}

fn homepage(cx: Scope) -> Element {
    cx.render(rsx! {
        p { "Welcome to Dioxus Blog!" }
    })
}

fn page_not_found(cx: Scope) -> Element {
    cx.render(rsx! {
        p { "Oops! The page you are looking for doesn't exist!" }
    })
}
