use crate::{Footer, Nav};
use yew::{Html, html};

pub fn home() -> Html {
    html! {
            <>
            <style>
                {"
        body {
            font-family: Arial, sans-serif;
            line-height: 1.6;
            margin: 0;
            padding: 20px;
            max-width: 1200px;
            margin: 0 auto;
            min-height: 100vh;
            background: linear-gradient(to bottom, #2c3e50, #3498db);
            color: white;
        }

        header {
            text-align: center;
            padding-top: 0px;
            padding-bottom: 2rem;
        }

        nav {
            margin-bottom: 32px;
        }

        nav ul {
            list-style: none;
            padding: 0;
            margin: 0;
            display: flex;
            justify-content: center;
            gap: 2rem;
            align-items: center;
        }

        nav a {
            color: white;
            text-decoration: none;
            font-size: 1.1rem;
            padding: 0.5rem 1rem;
            border-radius: 5px;
            transition: background-color 0.3s;
        }

        nav a:hover {
            background-color: rgba(255, 255, 255, 0.1);
        }

        nav a.brand {
            font-family: 'Space Grotesk', monospace;
            font-size: 1.4rem;
            font-weight: 600;
            color: #e94560;
        }

        nav a.brand:hover {
            color: #ff6b81;
        }

        .github-link {
            display: flex;
            align-items: center;
            gap: 0.5rem;
            color: white;
            text-decoration: none;
            font-size: 1.1rem;
            padding: 0.5rem 1rem;
            border-radius: 5px;
            transition: background-color 0.3s;
        }

        .github-link:hover {
            background-color: rgba(255, 255, 255, 0.1);
        }

        .github-link img {
            width: 20px;
            height: 20px;
        }

        .github-stats {
            display: flex;
            flex-wrap: wrap;
            gap: 1rem;
            justify-content: center;
            margin-top: 2rem;
        }

        .github-stats img {
            border-radius: 8px;
            box-shadow: 0 4px 8px rgba(0, 0, 0, 0.2);
            transition: transform 0.3s;
        }

        .github-stats img:hover {
            transform: translateY(-5px);
        }

        .center {
            text-align: center;
        }

        .end-text {
            text-align: end;
        }

        main {
            padding: 2rem 0;
            background: rgba(255, 255, 255, 0.1);
            border-radius: 10px;
            padding: 2rem;
            backdrop-filter: blur(5px);
        }

        footer {
            text-align: center;
            padding: 1rem 0;
            margin-top: 2rem;
            border-top: 1px solid rgba(255, 255, 255, 0.2);
        }

        h1,
        h2 {
            text-shadow: 2px 2px 4px rgba(0, 0, 0, 0.3);
        }

        .no-margin {
            margin: 0;
            font-size: 22px;
            margin-bottom: 7px;
        }

        @media (max-width: 768px) {
            body {
                padding: 10px;
            }

            nav ul {
                flex-direction: column;
                gap: 1rem;
            }

            nav a {
                font-size: 1.2rem;
                padding: 0.8rem 1.5rem;
                width: 100%;
                text-align: center;
            }

            main {
                padding: 1rem;
            }

            h1 {
                font-size: 1.8rem;
            }

            h2 {
                font-size: 1.4rem;
            }
        }"}
            </style>
            <header>
                <Nav/>
                    <h1>{"Добро пожаловать"}</h1>
                    <h2>{"на сайт практически программиста"}</h2>
            </header>
    <main>
            <section>
                <h1 style="font-size: 32px">{"Привет, меня зовут Миша..."}</h1>
                <p class="no-margin">{"Я вообще много кто, но сейчас я программист"}</p>
                <p class="no-margin">{"Пишу всякие разные проектики на C#, Rust, немного на Python и не много занимаюсь хостингом"}</p>
                <p class="no-margin">{"Мои главные проекты которыми я сейчас занимаюсь - "}<a href="flagships">{"Флагманы"}</a>{". В
                них я вкладываю большую часть своих усилинй на данный момент.
                Они могут быть довольно полезными для обычной зазработки, и я даже сам их использую для других своих
                проектов. Очень советую ознакомиться)))"}</p>
                <p class="no-margin">{"Так же тут есть и другие проекты, которые не являются моими основными и я занимаюсь ими
                когда доходят руки после флагманов. Они тоже в целом прикольные, но врядли особо полезные, разве что
                можно поглазесь. Так же тут есть старые проекты, которые уже врядли когда-либо будут развиваться и
                затрагиваться дальше"}</p>

                <p class="no-margin">{"
                    Большинство кода который мной написан лежит в открытом доступе на моем "}<a href="https://github.com/YaslePoy">{"Гитхабе"}</a>
                </p>
                <h2 class="center">{"Немножечко статистики"}</h2>
                <div class="github-stats">
                    <img src="https://github-readme-stats.vercel.app/api?username=YaslePoy&show_icons=true&theme=dark&hide_border=true" alt="GitHub Stats"/>
                    <img src="https://github-readme-streak-stats.herokuapp.com/?user=YaslePoy&theme=dark&hide_border=true" alt="GitHub Streak"/>
                    <img src="https://github-readme-stats.vercel.app/api/top-langs/?username=YaslePoy&layout=compact&theme=dark&hide_border=true" alt="Top Languages"/>
                </div>
                <p class="end-text">
                    {"P.S.: не удивляйтесь, C++ тут очень не заслужанно... это был просто вулкановский код 3д движка для игры, а там один этот код из за того что на вулкане больше чем вся остальная логика игры..."}
                </p>
            </section>
        </main>
            <Footer/>
            </>
        }
}
