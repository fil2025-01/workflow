[Prev](./page22.md) | [Next](./page24.md)

# Architecture: Serving the Frontend

This document explains the technical flow of how the application serves the frontend when a user visits the root URL (`/`).

## 1. Route Registration (Backend)
In `src/main.rs`, the application uses the `leptos_axum` integration to register routes automatically based on the Leptos application definition.

```rust
// src/main.rs

// 1. Generates a list of paths defined in your Leptos App (including "/")
let routes = generate_route_list(App);

// ...

let app = Router::new()
    // ... other API routes ...

    // 2. Registers the handler for all Leptos routes
    .leptos_routes_with_context(&state, routes, move || provide_context(pool.clone()), App)
```

## 2. Server-Side Rendering (SSR)
When a request hits `/`:
1.  **Matching**: The `leptos_routes_with_context` middleware identifies `/` as a valid route defined in the `App`.
2.  **Rendering**: It executes the `App` component on the server (Rust).
3.  **Response**: It generates the initial HTML string (including `<head>`, styles, and the body content) and sends it to the browser.

## 3. The View Definition
The actual content served comes from `src/app.rs`. The `App` component defines the layout and routing logic:

```rust
// src/app.rs

#[component]
pub fn App() -> impl IntoView {
  // ...
  view! {
    // ...
    <Router>
      <div class="app-layout h-screen">
        <Sidebar/>
        <main class="main-content">
          <Routes>
            // This matches the root path ""
            <Route path="" view=HomePage/>
            <Route path="/*any" view=NotFound/>
          </Routes>
        </main>
      </div>
    </Router>
  }
}
```

Because `<Route path="" view=HomePage/>` is defined, the server renders the `HomePage` component and sends that HTML to the user.

## 4. Hydration (Client-Side)
Once the browser receives the HTML:
1.  It loads the CSS (`/pkg/workflow.css`).
2.  It loads the WASM bundle (via scripts injected by Leptos).
3.  The `hydrate` function in `src/lib.rs` runs, taking over the static HTML and turning it into an interactive Single Page Application (SPA).

Prev | Next