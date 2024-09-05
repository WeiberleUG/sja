use leptos::prelude::*;

#[component]
pub fn Header() -> impl IntoView {
    provide_context(RwSignal::new(false));

    view! {
        <div class="header-div-1">
            <div class="header-div-2">
                <div class="header-div-3">
                    <div class="header-div-4">
                        <a href="/">
                            <img src="/images/sja_big.png" class="home-image" alt="Home" />
                        </a>
                        <ShowWhenOpen is=false>
                            <div class="header-div-5">
                                <div class="header-div-6">
                                    <a href="/angebote" class="link-open">
                                        "Angebote"
                                    </a>
                                    <a href="/outdatet" class="link-open">
                                        "Abgelaufen"
                                    </a>
                                </div>
                            </div>
                        </ShowWhenOpen>
                        <div>
                            <div class="header-div-7">
                                <a href="https://github.com/Weiberle17/sja">
                                    <img
                                        class="header-img"
                                        src="/images/github-mark.svg"
                                        alt="GitHub"
                                    />
                                </a>
                                <HamburgerMenuToggle />
                            </div>
                        </div>
                    </div>
                    <ShowWhenOpen is=true>
                        <div class="header-div-8">
                            <div class="header-div-9">
                                <a href="/angebote" class="link-open">
                                    "Angebote"
                                </a>
                                <a href="/outdatet" class="link-open">
                                    "Abgelaufen"
                                </a>
                            </div>
                        </div>
                    </ShowWhenOpen>
                    <h1>"Stiftung Jugendhilfe Aktiv"</h1>
                </div>
            </div>
        </div>
    }
}

#[island]
fn HamburgerMenuToggle() -> impl IntoView {
    let (hamburger_menu_open, set_hamburger_menu_open) = expect_context::<RwSignal<bool>>().split();

    view! {
        <button on:click=move |_| {
            dbg!("Test");
            set_hamburger_menu_open.update(|n| *n = !*n);
        }>
            <img
                class="hamburger-menu"
                src=move || match hamburger_menu_open() {
                    true => "/images/x_close-black.png",
                    false => "/images/mobile_menu-black.png",
                }
                alt="Toggle Menu"
            />
        </button>
    }
}

#[island]
fn ShowWhenOpen(is: bool, children: Children) -> impl IntoView {
    let hamburger_menu_open = expect_context::<RwSignal<bool>>().read_only();

    view! {
        <div style=move || {
            if hamburger_menu_open() == is { "display: contents" } else { "display: none" }
        }>{children()}</div>
    }
}
