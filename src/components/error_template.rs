use cfg_if::cfg_if;
use leptos::{error::Errors, prelude::*};
use crate::utils::error::LoginError;

#[cfg(feature = "ssr")]
use leptos_axum::ResponseOptions;

#[component]
pub fn ErrorTemplate (
    #[prop(optional)] outside_errors: Option<Errors>,
    #[prop(optional)] errors: Option<RwSignal<Errors>>
) -> impl IntoView {
    let errors = match outside_errors {
        Some(e) => RwSignal::new(e),
        None => match errors {
            Some(e) => e,
            None => panic!("No Errors Found")
        }
    };

    let errors: Vec<LoginError> = errors
        .get()
        .into_iter()
        .filter_map(|(_, v)| v.downcast_ref::<LoginError>().cloned())
        .collect();

    cfg_if! {
        if #[cfg(feature = "ssr")] {
            let response = use_context::<ResponseOptions>();
            if let Some(response) = response {
                response.set_status(errors[0].status_code());
            }
        }
    }

    view! {
        <h1>"Errors"</h1>
        <For 
            each = move || {errors.clone().into_iter().enumerate()}
            key = |(index, _error)| *index
            children = move |error| {
                let error_string = error.1.to_string();
                let error_code = error.1.status_code();
                view! {
                    <h2>{error_code.to_string()}</h2>
                    <p>"Error: " {error_string}</p>
                }
            }
        />
    }
}