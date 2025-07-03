use gloo::console;
use gloo::console::console;
use crate::{Footer, Nav};
use gloo::timers::callback::Interval;
use yew::prelude::*;
use yew::{html, Html};
use log::log;

pub struct DrunkTimer {
    time: i32,
    formatted_time: String,
}

impl DrunkTimer {
    fn format_time(time: i32) -> String {
        let mut hours = (time / 3600).to_string();
        let mut minutes = ((time / 60) % 60).to_string();
        let mut seconds = (time % 60).to_string();
        if hours.len() == 1 {
            hours = format!("0{}", hours);
        }
        if minutes.len() == 1 {
            minutes = format!("0{}", minutes);
        }
        if seconds.len() == 1 {
            seconds = format!("0{}", seconds);
        }

        format!("{}:{}:{}", hours, minutes, seconds)
    }
}

pub enum TimeChange {
    Up,
    Down,
}

impl Component for DrunkTimer {
    type Message = TimeChange;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let link = ctx.link().clone();
        Interval::new(1000, move || {
            let random = getrandom::u32().unwrap() % 1000;
            console::log!(random);
            if random > 400 {
                link.send_message(TimeChange::Up)
            }else {
                link.send_message(TimeChange::Down)
            }
        }).forget();
        Self {
            time: 19 * 60 * 60,
            formatted_time: "19:00:00".to_string()
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            TimeChange::Down => {
                self.time += 1;
            }
            TimeChange::Up => {
                self.time -= 1;
            }
        }
        self.formatted_time = DrunkTimer::format_time(self.time);
        true
    }

    fn changed(&mut self, ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
        <>

        <link href="css/services.css" rel="stylesheet"/>
        <header>
        <Nav/>
        <h1>{"Услуги"}</h1>
        <div class="timer">
            <h2>{"Удивительная скидка 90‰"}</h2>
            <p>{"осталось до окончания акции"}</p>
            <p id="timer-watch" style="font-size: 30px;">{ &self.formatted_time }</p>
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
}

fn drunk_timer() -> Html {
    html! {
    }
}
// 
// #[function_component(ServicesPage)]
// pub fn services() -> Html {
//     html! {
//         <>
// 
//         <link href="css/services.css" rel="stylesheet"/>
//         <header>
//         <Nav/>
//         <h1>{"Услуги"}</h1>
//         <div class="timer">
//             <h2>{"Удивительная скидка 90‰"}</h2>
//             <p>{"осталось до окончания акции"}</p>
//             <DrunkTimer/>
//         </div>
//     </header>
// 
//     <main>
//         <div class="service-card">
//             <img src="img/arma76.png" alt="Raspberry Pi хостинг"/>
//             <div class="content">
//                 <h3>{"Простой хостинг"}</h3>
//                 <p>{"Хостинг на базе Raspberry Pi с предоставлением субдомена. Идеальное решение для небольших проектов и
//                     тестирования."}</p>
//                 <ul>
//                     <li>{"Хостинг на Raspberry Pi"}</li>
//                     <li>{"Предоставление субдомена"}</li>
//                     <li>{"Подходит для небольших проектов"}</li>
//                 </ul>
//                 <a href="terms-basic.html" class="terms-link">{"Условия использования →"}</a>
//             </div>
//         </div>
//         <div class="service-card">
//             <img src="img/7700x.webp" alt="Ryzen сервер"/>
//             <div class="content">
//                 <h3>{"Unlimited-хостинг"}</h3>
//                 <p>{"Мощный хостинг на сервере с процессором Ryzen 7 7700x. Максимальная производительность для ваших
//                     проектов."}</p>
//                 <ul>
//                     <li>{"Хостинг на Ryzen 7 7700x"}</li>
//                     <li>{"Высокая производительность"}</li>
//                     <li>{"Подходит для крупных проектов"}</li>
//                 </ul>
//                 <a href="terms-unlimited.html" class="terms-link">{"Условия использования →"}</a>
//             </div>
//         </div>
//     </main>
//         <Footer/>
//         </>
//     }
// }
