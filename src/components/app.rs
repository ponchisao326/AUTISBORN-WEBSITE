use yew::prelude::*;
use crate::components::post::{Post, fetch_posts};

#[function_component]
pub fn App() -> Html {

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
            (), // Dependencies (empty because it only runs once)
        );
    }

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
        </main>
    }
}
