use yew::prelude::*;
use crate::components::players_data::{fetch_player_effects, fetch_player_powers, PlayerPowers, Power};
use crate::components::post::{Post, fetch_posts};
use crate::components::server_stats::fetch_server_stats;
use gloo_timers::callback::Interval;
use js_sys::Date;
use web_sys::console;

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

    // Estados para los poderes y efectos de múltiples jugadores
    let player_powers = use_state(|| Vec::<PlayerPowers>::new());
    let player_effects = use_state(|| Vec::<i32>::new()); // Guardamos un vector de efectos por jugador

    // Fetch de los poderes de múltiples jugadores
    {
        let player_powers = player_powers.clone();
        use_effect_with_deps(
            move |_| {
                wasm_bindgen_futures::spawn_local(async move {
                    let players_urls = vec![
                        "https://ponchisaohosting.xyz/downloads/cosmere/post/uploads/power_Dev.json", // Jugador 1
                        "https://ponchisaohosting.xyz/downloads/cosmere/post/uploads/power_Dev2.json", // Jugador 2
                    ];

                    let mut powers_data = Vec::new();
                    for url in players_urls {
                        if let Ok(fetched_powers) = fetch_player_powers(url).await {
                            powers_data.push(fetched_powers);
                        }
                    }

                    player_powers.set(powers_data);
                });
                || ()
            },
            (),
        );
    }

    // Fetch de los efectos de múltiples jugadores
    {
        let player_effects = player_effects.clone();
        use_effect_with_deps(
            move |_| {
                wasm_bindgen_futures::spawn_local(async move {
                    let players_urls = vec![
                        "https://ponchisaohosting.xyz/downloads/cosmere/post/uploads/effect_Dev.json",
                        "https://ponchisaohosting.xyz/downloads/cosmere/post/uploads/effect_Dev2.json",
                    ];

                    let mut effects_data = Vec::new();
                    for url in players_urls {
                        if let Ok(fetched_effects) = fetch_player_effects(url).await {
                            effects_data.push(fetched_effects.effectsInfo);
                        }
                    }

                    player_effects.set(effects_data);
                });
                || ()
            },
            (),
        );
    }

    // Dentro del componente App, añade este nuevo estado:
    let time_left = use_state(|| (0, 0, 0, 0)); // (días, horas, minutos, segundos)

    // Añade este efecto para el temporizador:
    {
        let time_left = time_left.clone();
        use_effect_with_deps(
            move |_| {
                let handle = Interval::new(1000, move || {
                    let target_date = Date::new_with_year_month_day_hr_min_sec(
                        2025, 2, 20, // 25 de diciembre 2024 (meses 0-based)
                        0, 0, 0
                    );
                    let now = Date::new_0();

                    let difference = target_date.get_time() - now.get_time();

                    if difference > 0.0 {
                        let days = (difference / (1000.0 * 60.0 * 60.0 * 24.0)) as i64;
                        let hours = ((difference % (1000.0 * 60.0 * 60.0 * 24.0)) / (1000.0 * 60.0 * 60.0)) as i64;
                        let minutes = ((difference % (1000.0 * 60.0 * 60.0)) / (1000.0 * 60.0)) as i64;
                        let seconds = ((difference % (1000.0 * 60.0)) / 1000.0) as i64;

                        time_left.set((days, hours, minutes, seconds));
                    } else {
                        console::log_1(&"¡Tiempo cumplido!".into());
                    }
                });

                move || {
                    drop(handle);
                }
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
                            <li class="nav-item"> <a href="#poderes"> {"Poderes"} </a></li>
                            <li class="nav-item"> <a href="#misiones"> {"Misiones"} </a></li>
                        </ul>
                    </div>
                </div>
            </header>

            <section class="width-section" id="actualizaciones">
        <div class="actualizaciones-panel">
            <div class="actualizaciones-title">
                <h1>{"Próxima Actualización del Servidor"}</h1>
            </div>
            <div class="countdown-timer">
                <div class="timer-display">
                    <div class="time-unit">
                        <span class="number">{time_left.0}</span>
                        <span class="label">{"Días"}</span>
                    </div>
                    <div class="time-unit">
                        <span class="number">{time_left.1}</span>
                        <span class="label">{"Horas"}</span>
                    </div>
                    <div class="time-unit">
                        <span class="number">{time_left.2}</span>
                        <span class="label">{"Minutos"}</span>
                    </div>
                    <div class="time-unit">
                        <span class="number">{time_left.3}</span>
                        <span class="label">{"Segundos"}</span>
                    </div>
                </div>
                <div class="target-date">
                    {"Próxima actualización: 20 de Febrero 2025"}
                </div>
            </div>
        </div>
    </section>

            <section class="width-section" id="actualizaciones">
                <div class="actualizaciones-panel">
                    <div class="actualizaciones-title">
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

            <section class="width-section" id="poderes">
                <div class="actualizaciones-panel">
                    <div class="actualizaciones-title">
                        <h1> {"Poderes de los Jugadores"}</h1>
                    </div>
                    <div class="poderes-posts">
                        {
                            player_powers.iter().enumerate().map(|(index, player)| {
                                html! {
                                    <div class="poderes-player">
                                        <h2>{format!("{}", &player.player)}</h2>
                                        <p>{format!("Efectos Activos: {}", player_effects.get(index).unwrap_or(&0))}</p>
                                        <div class="poderes">
                                            {
                                                player.powers.iter().map(|power| {
                                                    html! {
                                                        <div class="poder">
                                                            <h4>{&power.name}</h4>
                                                        </div>
                                                    }
                                                }).collect::<Html>()
                                            }
                                        </div>
                                    </div>
                                }
                            }).collect::<Html>()
                        }
                    </div>
                </div>
            </section>
            <section class="width-section" id="misiones">
                <div class="actualizaciones-panel">
                    <div class="actualizaciones-title">
                        <h1> {"Misiones y Recompensas"}</h1>
                    </div>
                    <div class="misiones-posts">
                        <div class="misiones-panel">
                            <h1>{"PROXIMAMENTE"}</h1>
                            <p class="subtitle">{"PROXIMAMENTE"}</p>
                            <p class="text">{"PROXIMAMENTE"}</p>
                        </div>
                        <div class="misiones-panel">
                            <h1>{"PROXIMAMENTE"}</h1>
                            <p class="subtitle">{"PROXIMAMENTE"}</p>
                            <p class="text">{"PROXIMAMENTE"}</p>
                        </div>
                    </div>
                </div>
            </section>
        </main>
    }
}
