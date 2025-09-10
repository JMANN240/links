use url::Url;
use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub struct Link {
    name: String,
    url: Url,
}

impl Link {
    pub fn new(name: impl Into<String>, url: Url) -> Self {
        Self {
            name: name.into(),
            url,
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct LinkProps {
    pub link: Link,
}

#[function_component]
pub fn LinkComponent(LinkProps { link }: &LinkProps) -> Html {
    html! {
        <a href={link.url.to_string()} target="_blank" class="font-bold flex border border-white w-64 p-2 justify-center hover:bg-white hover:text-neutral-950 duration-100">
            { link.name.clone() }
        </a>
    }
}

#[function_component]
pub fn App() -> Html {
    let links = vec![
        Link::new(
            "MIRRORS",
            Url::parse("https://mirrors.kent.software").unwrap(),
        ),
        Link::new(
            "Penny Arcade",
            Url::parse("https://arcade.kent.software").unwrap(),
        ),
        Link::new(
            "Dots Not Bots Club",
            Url::parse("https://dotsnotbots.club").unwrap(),
        ),
        Link::new("Keys", Url::parse("https://keys.kent.software").unwrap()),
        Link::new(
            "Whiteboard",
            Url::parse("https://whiteboard.kent.software").unwrap(),
        ),
        Link::new(
            "Sharecard",
            Url::parse("https://sharecard.kent.software").unwrap(),
        ),
        Link::new("Main Site", Url::parse("https://kent.software").unwrap()),
    ];

    html! {
        <main class="flex flex-col items-center p-16">
            <img src="/images/logo.png" class="w-32 mb-4 aspect-square" />
            <h1 class="text-xl font-bold">
                { "Kent Software Collective" }
            </h1>
            <h2 class="text-ksc text-md mb-4">
                { "Tomorrow's Solutions, Today" }
            </h2>
            <section class="flex flex-col gap-4">
                {
                    links.iter().map(|link| html! { <LinkComponent link={link.clone()} /> }).collect::<Html>()
                }
            </section>
        </main>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
