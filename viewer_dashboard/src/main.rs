use leptos::*;
use std::fs;
use std::path::PathBuf;

#[component]
fn SearchPage(cx: Scope, query: String) -> Element {
    view! { cx,
        <div>"Search results for: " {query}</div>
    }
}

#[component]
fn NotFound() -> Element {
    view! { cx,
        <div>"Page not found"</div>
    }
}

async fn file_handler(req: Request, dir: &str) -> Result<Response, ()> {
    let path = PathBuf::from(dir).join(req.path());
    if path.exists() {
        let content = fs::read_to_string(path).await.map_err(|_| ())?;
        Response::ok().body(content)
    } else {
        Ok(Response::not_found().body("Page not found"))
    }
}

#[server(Client)]
async fn search(cx: Scope<Client>, query: String) -> Result<(), ServerFnError<Client>> {
    let resp = file_handler(cx.request(), "templates").await.map_err(|_| ())?;
    if let Response::Ok(mut res) = resp {
        res.set_header("Content-Type", "text/html");
        cx.respond_with(res);
    }
    Ok(())
}

#[start]
async fn main() -> ! {
    let app = Router::new();

    app.get("/search", search_page);
    app.get("*", not_found);

    app.serve("0.0.0.0:8080").await.unwrap();

    loop {}
}

fn search_page(cx: Scope<Client>) -> Element {
    let (query, set_query) = create_signal(cx, String::from(""));

    view! { cx,
        <main class="container">
            <h1>"Search"</h1>
            <form on:submit=move |_| {
                search(cx, query.get()).await;
            }>
                <input type="text" value=move || query.get() on:input=move |e| {
                    set_query.set(e.value);
                } />
                <button type="submit">"Search"</button>
            </form>
            <SearchPage query=query.get() />
        </main>
    }
}

fn not_found(_cx: Scope<Client>) -> Element {
    view! { cx,
        <main class="container">
            <NotFound />
        </main>
    }
}