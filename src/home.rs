use yew::{html, Html};
use crate::Nav;

pub fn home() -> Html {
    html! {
        <>
        <style>
            {"body { background: linear-gradient(to bottom, #1a1a2e, #16213e); }"}
        </style>
        <header>
            <Nav/>
                <h1>{"Добро пожаловать"}</h1>
                <h2>{"на сайт практически программиста"}</h2>
        </header>
        <main>
        
        </main>
        </>
    }
}