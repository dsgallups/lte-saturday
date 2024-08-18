use crate::{
    error_template::{AppError, ErrorTemplate},
    poll::Poll,
};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {


        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/lte-saturday.css"/>

        // sets the document title
        <Title text="LTE Saturday"/>

        // content for this welcome page
        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! {
                <ErrorTemplate outside_errors/>
            }
            .into_view()
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
        <h1>"Welcome to Leptos!"</h1>
        <button on:click=on_click>"Click Me: " {count}</button>
    }
}

#[server(PostAnswer)]
async fn post_answer(selection: String) -> Result<(), ServerFnError> {
    Ok(())
}

async fn get_current_poll() -> Poll {
    Poll {
        question: "What is your favorite color?".to_string(),
        answer_choices: vec!["Red".to_string(), "Green".to_string(), "Blue".to_string()],
    }
}

/// Says I'm online
#[server(PollOnline)]
async fn poll_online() -> Result<(), ServerFnError> {
    Ok(())
}
