use crate::error_template::{AppError, ErrorTemplate};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use serde::{de::DeserializeOwned, Deserializer};

// server functions should return Result<T, ServerFnError>
// server function을 사용하기 위해서는 cargo add serde --features=derive 로 serde를 추가해야 한다
// #[server(RootApi, "/test")]
// pub async fn root_api() -> Result<() , ServerFnError> {
//    let result = "you are not login!!!";
//    Ok(result)
// }

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/leptos-ecommerce.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! { <ErrorTemplate outside_errors/> }.into_view()
        }>
            <main>
                <Routes>
                    <Route path="" view=HomePage/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button
    let (count, set_count) = create_signal(0);
    let on_click = move |_| set_count.update(|count| *count += 1);

    view! {
        <h1>"Welcome to Leptos!!!!!222222233355553"</h1>
        <h1>"Welcome to Leptos!!!555344"</h1>
        <button on:click=on_click>"Click Me: " {count}</button>
    }
}

