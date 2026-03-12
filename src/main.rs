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
        use kost_management_v2::state::AppState;
        

    }
}