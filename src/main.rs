#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use axum::{routing::get, Router}; 
    use leptos::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use {{crate_name}}::app::*;
    use {{crate_name}}::fileserv::{error_handler, assets_service, static_file_service};

    // Setting get_configuration(None) means we'll be using cargo-leptos's env values
    // For deployment these variables are:
    // <https://github.com/leptos-rs/start-axum#executing-a-server-on-a-remote-machine-without-the-toolchain>
    // Alternately a file can be specified such as Some("Cargo.toml")
    // The file would need to be included with the executable when moved to deployment
    let conf = get_configuration(None).await.unwrap();
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;
    let routes = generate_route_list(App);

    // build our application with a route
    let app = Router::new()
        // serve JS/WASM/CSS from `pkg`
        .route("/pkg/*file", get(static_file_service))
        // serve other assets from the `assets` directory
        .route("/assets/*file", get(assets_service))
        // serve the favicon from /favicon.ico
        .route("/favicon.ico", get(static_file_service))
        .leptos_routes(&leptos_options, routes, App)
        .fallback(error_handler)
        .with_state(leptos_options);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    logging::log!("listening on http://{}", &addr);
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for a purely client-side app
    // see lib.rs for hydration function instead
}
