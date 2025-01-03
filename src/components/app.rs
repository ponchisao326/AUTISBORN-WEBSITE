use yew::prelude::*;
use crate::components::post::{Post, fetch_posts};
use crate::components::server_stats::fetch_server_stats;

#[function_component]
pub fn App() -> Html {

    let online_players = use_state(|| 0);
    let total_deaths = use_state(|| 0);
    let total_days = use_state(|| 0);

    // Utilizamos use_effect para ejecutar la solicitud HTTP al montar el componente
    {
        let online_players = online_players.clone();
        let total_deaths = total_deaths.clone();
        let total_days = total_days.clone();

        use_effect_with_deps(
            move |_| {
                wasm_bindgen_futures::spawn_local(async move {
                    match fetch_server_stats().await {
                        Ok(stats) => {
                            online_players.set(stats.onlinePlayers);
                            total_deaths.set(stats.totalDeaths);
                            total_days.set(stats.daysPlayed);
                        }
                        Err(_) => {
                            web_sys::console::log_1(&"Error al obtener las estadísticas del servidor.".into());
                        }
                    }
                });
                || ()
            },
            (),
        );
    }

    let posts = use_state(Vec::<Post>::new);

    {
        let posts = posts.clone();
        use_effect_with_deps(
            move |_| {
                wasm_bindgen_futures::spawn_local(async move {
                    if let Ok(fetched_posts) = fetch_posts().await {
                        posts.set(fetched_posts);
                    }
                });
                || ()
            },
            (),
        );
    }

        html! {
        <main>
            <header>
                <div class="header">
                    <div class="navbar-title">
                        <a href="#">{"AUTISBORN"}</a>
                    </div>
                    <div class="navbar-bar">
                        <ul class="navbar-nav">
                            <li class="nav-item"> <a href="#actualizaciones"> {"Actualizaciones"} </a></li>
                            <li class="nav-item"> <a href="#estadisticas"> {"Estadísticas"} </a></li>
                            <li class="nav-item"> <a href="#"> {"Mapa"} </a></li>
                            <li class="nav-item"> <a href="#"> {"Misiones"} </a></li>
                            <li class="nav-item"> <a href="#"> {"Guías"} </a></li>
                        </ul>
                    </div>
                </div>
            </header>

            <section class="width-section" id="actualizaciones">
                <div class="actualizaciones-panel">
                    <div class="actualizaciones-title">
                        <img src="https://ponchisaohosting.xyz/downloads/cosmere/assets/newspaper.webp" alt="Actualizaciones" />
                        <h1> {"Actualizaciones del Servidor"}</h1>
                    </div>
                    <div class="actualizaciones-posts">
                        {
                            posts.iter().map(|post| {
                                html! {
                                    <div class="actualizaciones-post">
                                        <h1> {&post.title} </h1>
                                        <p> {&post.content} </p>
                                    </div>
                                }
                            }).collect::<Html>()
                        }
                    </div>
                </div>
            </section>

            <section class="width-section" id="estadisticas">
                <div class="actualizaciones-panel">
                    <div class="actualizaciones-title">
                        <img src="https://ponchisaohosting.xyz/downloads/cosmere/assets/trophy.webp" alt="Estadísticas" />
                        <h1> {"Estadísticas Generales"}</h1>
                    </div>
                    <div class="estadisticas-posts">
                        <div class="left">
                            <div class="online">
                                <h1> {"Jugadores en línea"} </h1>
                                <p> {*online_players} </p>
                            </div>
                            <div class="deaths">
                                <h1> {"Muertes Totales"} </h1>
                                <p> {*total_deaths} </p>
                            </div>
                        </div>
                        <div class="right">
                            <div class="days">
                                <h1> {"Dias Jugados de Servidor (INGAME)"} </h1>
                                <p> {*total_days} </p>
                            </div>
                        </div>
                    </div>
                </div>
            </section>
        </main>
    }
}
