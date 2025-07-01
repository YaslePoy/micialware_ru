use crate::{Footer, Nav};
use gloo::console;
use gloo::timers::callback::Interval;
use js_sys::Date;
use yew::prelude::*;
use yew::{Html, html, use_state};

fn get_current_time() -> String {
    let date = Date::new_0();
    String::from(date.to_locale_time_string("ru-RU"))
}

#[function_component(DrunkTimer)]
fn drunk_timer() -> Html {
    let time = use_state(|| 0);
    {
        let time = time.clone();
        let downgrade = ||{
            time.set(*time + 1);
        };
        console::log!(format!("First time: {}", *time));
        use_effect_with((), move |_| {
            let time = time.clone();
            time.set(*time + 1);
            Interval::new(1_000, ||{
                downgrade();
            } )
                .forget();
        });
    }

    html! {
            <p id="timer-watch" style="font-size: 30px;">{ *time }</p>
    }
}

#[function_component(ServicesPage)]
pub fn services() -> Html {
    html! {
        <>

        <link href="css/services.css" rel="stylesheet"/>
        <header>
        <Nav/>
        <h1>{"Услуги"}</h1>
        <div class="timer">
            <h2>{"Удивительная скидка 90‰"}</h2>
            <p>{"осталось до окончания акции"}</p>
            <DrunkTimer/>
        </div>
    </header>

    <main>
        <div class="service-card">
            <img src="img/arma76.png" alt="Raspberry Pi хостинг"/>
            <div class="content">
                <h3>{"Простой хостинг"}</h3>
                <p>{"Хостинг на базе Raspberry Pi с предоставлением субдомена. Идеальное решение для небольших проектов и
                    тестирования."}</p>
                <ul>
                    <li>{"Хостинг на Raspberry Pi"}</li>
                    <li>{"Предоставление субдомена"}</li>
                    <li>{"Подходит для небольших проектов"}</li>
                </ul>
                <a href="terms-basic.html" class="terms-link">{"Условия использования →"}</a>
            </div>
        </div>
        <div class="service-card">
            <img src="img/7700x.webp" alt="Ryzen сервер"/>
            <div class="content">
                <h3>{"Unlimited-хостинг"}</h3>
                <p>{"Мощный хостинг на сервере с процессором Ryzen 7 7700x. Максимальная производительность для ваших
                    проектов."}</p>
                <ul>
                    <li>{"Хостинг на Ryzen 7 7700x"}</li>
                    <li>{"Высокая производительность"}</li>
                    <li>{"Подходит для крупных проектов"}</li>
                </ul>
                <a href="terms-unlimited.html" class="terms-link">{"Условия использования →"}</a>
            </div>
        </div>
    </main>
        <Footer/>
        </>
    }
}
