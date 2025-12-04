#![recursion_limit = "256"]
#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use leptos::logging::log;
    use std::env;

    use axum::{routing::post, Router};
    use leptos::prelude::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use sja::app::*;
    use sqlx::migrate::MigrateDatabase;
    use sqlx::Sqlite;

    dotenvy::dotenv().expect("Error loading dotenv");
    simple_logger::init_with_level(log::Level::Debug).expect("couldn't initialize logging");

    let database_url: String = env::var("DATABASE_URL").expect("DATABASE_URL should be set");

    if !Sqlite::database_exists(&database_url)
        .await
        .unwrap_or(false)
    {
        use sqlx::Sqlite;

        println!("Creating database {}", &database_url);
        match Sqlite::create_database(&database_url).await {
            Ok(_) => println!("Create db success"),
            Err(error) => panic!("error: {}", error),
        }
    } else {
        println!("Database already exists");
    }

    let db = sqlx::sqlite::SqlitePool::connect(&database_url)
        .await
        .map_err(|e| panic!("{}", e.to_string()));

    let pool = db.unwrap();

    let _ = sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .map_err(|e| panic!("{}", e.to_string()));

    let conf = get_configuration(None).unwrap();
    let addr = conf.leptos_options.site_addr;
    let leptos_options = conf.leptos_options;
    let routes = generate_route_list(App);

    let app = Router::new()
        .route("/api/{*fn_name}", post(leptos_axum::handle_server_fns))
        .leptos_routes(&leptos_options, routes, {
            let leptos_options = leptos_options.clone();
            move || shell(leptos_options.clone())
        })
        .fallback(leptos_axum::file_and_error_handler(shell))
        .with_state(leptos_options);

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    log!("listening on http://{}", &addr);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for pure client-side testing
    // see lib.rs for hydration function instead
}
