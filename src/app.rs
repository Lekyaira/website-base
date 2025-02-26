use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};
use leptos::logging;
use crate::models::prelude::*;
use crate::mock::prelude::*;
use crate::db::prelude::*;

pub fn shell(options: LeptosOptions) -> impl IntoView {
    // Set up the app's shell html.
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone() />
                <HydrationScripts options/>
                <MetaTags/>
            </head>
            <body>
                <App/>
                <script src="https://cdn.jsdelivr.net/npm/bootstrap@5.3.3/dist/js/bootstrap.bundle.min.js" integrity="sha384-YvpcrYf0tY3lHB60NNkmXc5s9fDVZLESaAA55NDzOxhy9GkcIdslK1eN7N6jIeHz" crossorigin="anonymous"></script>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/leptos-ssr.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes fallback=|| "Page not found.".into_view()>
                    <Route path=StaticSegment("") view=HomePage/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button
    let count = RwSignal::new(0);
    let on_click = move |_| *count.write() += 1;

    let (posts_backing, set_posts_backing) = signal(Vec::<Post>::new());
    let posts = Resource::new(posts_backing, |posts_backing| async move { 
        get_posts().await.unwrap_or_else(|e| {
            logging::log!("Error: {}", e.to_string());
            vec![]
        })
    });

    view! {
        <div class="container mx-auto">
            <h1 class="text-center">"Welcome to Leptos, with Bootstrap!"</h1>
            <div class="d-flex">
                <button 
                    class="btn btn-primary mx-auto"
                    on:click=on_click
                >
                    "Click Me: " {count}
                </button>
            </div>
        </div>
        <hr/>
        <div class="container mx-auto">
            <Suspense 
                fallback=move || view! { <p>"Loading..."</p> }
            >
                <For 
                    each=move || posts.get().unwrap_or_else(|| vec![])
                    key=|state| state.id.clone()
                    let:post
                >
                    <h2>{post.title}</h2>
                    <p>{post.body}</p>
                </For>
            </Suspense>
        </div>
        //<button on:click=move |_| {
        //    let result = leptos::task::spawn_local(async {
        //        match get_posts().await {
        //            Err(e) => logging::log!("Error: {}", e.to_string()),
        //            Ok(r) => logging::log!("{:?}", r),
        //        }
        //    });
        //}>
        //        "Get Posts"
        //</button>
        //<Suspense
        //    fallback=move || view! { <p>"Loading..."</p> }
        //>
        //    <div class="container mx-auto">
        //        <For
        //            each=move || posts.get().unwrap()
        //            key=|state| state.id.clone()
        //            let:post
        //        >
        //            <h2>{post.title}</h2>
        //            <p>{post.body}</p>
        //        </For>
        //    </div>
        //</Suspense>
    }
}
