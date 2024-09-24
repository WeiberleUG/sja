use leptos::prelude::*;
use leptos_router::components::A;

#[island]
pub fn Header() -> impl IntoView {
    provide_context(RwSignal::new(false));
    let links = RwSignal::new(vec![
        ("".to_string(), "Angebote".to_string()),
        ("outdated".to_string(), "Abgelaufen".to_string()),
        ("new".to_string(), "Neu".to_string()),
    ]);
    let link_view = move || {
        view! {
            <For
                each=move || links.get().clone().into_iter().enumerate()
                key=|(i, _)| *i
                children=move |(_, (href, link)): (usize, (String, String))| {
                    view! {
                        <A href=move || format!("/{}", href.clone()) {..} class="link-open">
                            {link}
                        </A>
                    }
                }
            />
        }
    };

    view! {
        <div class="header-div-1">
            <div class="header-div-2">
                <div class="header-div-3">
                    <div class="header-div-4">
                        <A href="/">
                            <img src="/images/sja_big.png" class="home-image" alt="Home" />
                        </A>
                        // <ShowWhenOpen is=false>
                            <div class="header-div-5">
                                <div class="header-div-6">{link_view}</div>
                            </div>
                        // </ShowWhenOpen>
                        <div>
                            <div class="header-div-7">
                                <A href="https://github.com/Weiberle17/sja">
                                    <img
                                        class="header-img"
                                        src="/images/github-mark.svg"
                                        alt="GitHub"
                                    />
                                </A>
                                <HamburgerMenuToggle />
                            </div>
                        </div>
                    </div>
                    // <ShowWhenOpen is=true>
                    //     <div class="header-div-8">
                    //         <div class="header-div-9">{link_view}</div>
                    //     </div>
                    // </ShowWhenOpen>
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
