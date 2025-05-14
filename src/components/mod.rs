//! The components module contains all shared components for our app. Components are the building blocks of dioxus apps.
//! They can be used to defined common UI elements like buttons, forms, and modals. In this template, we define a Hero
//! component  to be used in our app.

mod hero;
pub use hero::Hero;

mod compose;
pub use compose::ComposeBlog;

mod loader;
pub use loader::Loader;

mod terminal_view;
pub use terminal_view::Terminal;

mod markdown_vr;
pub use markdown_vr::MarkdownViewer_Response;