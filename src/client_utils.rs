use leptos::*;

// Copy/pasted utils from my other codebases

pub fn get_msg<U: 'static, T: 'static + Clone>(res: Action<U, Result<T, ServerFnError>>) -> String {
    match res.value()() {
        Some(Ok(_)) | None => "".to_string(),
        Some(Err(err)) => match err {
            ServerFnError::ServerError(msg) => msg,
            ServerFnError::Registration(msg) => msg,
            ServerFnError::Request(msg) => msg,
            ServerFnError::Deserialization(msg) => msg,
            ServerFnError::Serialization(msg) => msg,
            ServerFnError::Args(msg) => msg,
            ServerFnError::MissingArg(msg) => msg,
            ServerFnError::Response(msg) => msg,
            ServerFnError::WrappedServerError(nce) => nce.to_string(),
        },
    }
}

#[component]
pub fn ErrorDisplay<U: 'static, T: 'static + Clone>(
    res: Action<U, Result<T, ServerFnError>>,
) -> impl IntoView {
    view! {
        <div hidden=move || !res.value().with(|v| v.as_ref().map(|v| v.is_err()).unwrap_or_default())>
            <p>
                {move || get_msg(res)}
            </p>
        </div>
    }
}
