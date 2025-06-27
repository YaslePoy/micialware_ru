use crate::{Footer, Nav};
use yew::{Html, html};

pub fn flagships() -> Html {
    html! {
            <>
            <link href="css/flagship.css" rel="stylesheet"/>
            <header>
            <Nav/>
            <h1>{"Флагманские проекты"}</h1>
            <h2>{"Основные проекты, которым уделяется наибольшее внимание"}</h2>
        </header>

        <main>
            <div class="project-grid">
                <div class="project-card">
                    <img src="img/mroaLogo.png" alt="mROA" />
                    <div class="project-card-text">
                        <h3>{"mROA"}</h3>
                        <p>{"Основной проект, над которым я работаю в настоящее время. Это на данный момент такой типо gRPC с
                        блекджеком и ООП. И тут реально очень сильно развита тема объектно-ориентированного
                        программировани, как она представлена конкретно в C# на данный момент, но по сети. Данная
                        библиотека может использоваться например для очень простого внедрения микросерсной архитектуры в
                        ASP.NET Core например при помощи DI или в других случаях взаимодействия по сети."}</p>
                        <a href="mroa" class="more-link">{"Подробнее →"}</a>
                    </div>

                </div>
                <div class="project-card">
                    <img src="img/mroacgtLogo.png" alt="mROA.CodegenTools" />
                    <div class="project-card-text">
                        <h3>{"mROA.CodegenTools"}</h3>
                        <p>{"Инструментарий для генерации кода, разрабатываемый в рамках проекта mROA. Не сказать, что эта библиотечка делает магию, но может облегчить всевозможную работу с генерацией кода на разных языках."}</p>
                        <a href="mroacgt.html" class="more-link">{"Подробнее →"}</a>
                    </div>
                </div>
            </div>
        </main>
        <Footer/>
    </>
        }
}
