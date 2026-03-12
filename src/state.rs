use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use leptos::prelude::LeptosOptions;
        use sqlx::MySqlPool;
        use axum_macros::FromRef;

        #[derive(FromRef, Debug, Clone)]
        pub struct AppState {
            pub leptos_option: LeptosOptions,
            pub pool: MySqlPool,
        }
    }
}