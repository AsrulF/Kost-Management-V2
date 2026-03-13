use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use axum::{
            response::{Response, IntoResponse},
            routing::get,
            extract::{Path, State, RawQuery},
            http::{Request, header::HeaderMap},
            body::Body as AxumBody,
            Router,
        };
        use leptos_axum::{
            generate_route_list, 
            LeptosRoutes, 
            handle_server_fns_with_context
        };
        use leptos::{
            logging::log,
            view,
            prelude::provide_context,
            prelude::get_configuration,  
        };
        use sqlx::{
            MySqlPool,
            mysql::MySqlPoolOptions,
        };
        use axum_session::{
            SessionConfig,
            SessionLayer,
            SessionStore,
        };
        use axum_session_auth::{
            AuthSessionLayer,
            AuthConfig,
            SessionAnyPool
        };
        
        use kost_management_v2::state::AppState;
        use kost_management_v2::utils::fallback::file_and_error_handler;

        

        #[tokio::main]
        async fn main() {

        }
    }
}