use yew::prelude::*;

#[function_component]
pub fn App() -> Html {

    let jsonTitle1: &str = "hola";

    html! {
        <main>
            <header>
                <div class="header">
                    <div class="navbar-title">
                        <a href="#">{"Servidor Mistborn Minecraft"}</a>
                    </div>
                    <div class="navbar-bar">
                        <ul class="navbar-nav">
                            <li class="nav-item"> <a href="#"> {"Actualizaciones"} </a></li>
                            <li class="nav-item"> <a href="#"> {"Estadísticas"} </a></li>
                            <li class="nav-item"> <a href="#"> {"Mapa"} </a></li>
                            <li class="nav-item"> <a href="#"> {"Misiones"} </a></li>
                            <li class="nav-item"> <a href="#"> {"Guías"} </a></li>
                        </ul>
                    </div>
                </div>
            </header>

            <section class="section" id="actualizaciones">
                <div class="actualiciones-panel">
                    <div class="actualizaciones-title">
                        <img src="https://ponchisaohosting.xyz/downloads/cosmere/assets/newspaper.webp" alt="Actualizaciones" />
                        <h1> {"Actualizaciones del Servidor"}</h1>
                    </div>
                    <div class="actualizaciones-posts">
                        <div class="actualizaciones-post-1">
                            <h1> {jsonTitle1} </h1>
                        </div>
                    </div>
                </div>
            </section>
        </main>
    }
}
