use leptos::prelude::*;
use leptos::*;

#[component]
pub fn AboutPage() -> impl IntoView {
    view! {
        <div class="about-page">
            <h1>"About Turn-Formal"</h1>
            <div class="content">
                <p>"Turn-Formal is a unified visualization tool for formal mathematics, logic, and foundational theories."</p>
                <p>"This tool helps users visualize and explore complex mathematical concepts in an interactive way."</p>
            </div>
        </div>
    }
}
