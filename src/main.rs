use yew::prelude::*;

mod text_input;
use crate::text_input::TextInput;

use reqwest::header::AUTHORIZATION;

enum Msg {
    SuperFetch,
    Update(String),
    UpdateContent(String),
}

fn patch_ticket(id: String) {
    let base = "https://ticketguru22.herokuapp.com/tickets?code=".to_string();
    let client = reqwest::ClientBuilder::new().build().unwrap();

    wasm_bindgen_futures::spawn_local(async move {
        match client
            .patch(base + id.as_ref())
            .header(AUTHORIZATION, "Basic YWRtaW46YWRtaW4=")
            .send()
            .await
        {
            Ok(_) => Msg::UpdateContent("Ok!".to_string()),
            Err(_) => Msg::UpdateContent("Error!".to_string()),
        };
    });
}

struct CounterComponent {
    id: String,
    content: String,
}

impl Component for CounterComponent {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            content: String::new(),
            id: String::new(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::SuperFetch => patch_ticket(self.id.clone()),
            Msg::Update(id) => self.id = id,
            Msg::UpdateContent(content) => self.content = content,
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();

        let on_change = link.callback(Msg::Update);
        let onclick = link.callback(|_| Msg::SuperFetch);

        html! {
            <div class="container">
                <p> {&self.content }</p>
                <TextInput {on_change} value={self.id.clone()} />
                <button {onclick}> {"Patch"} </button>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<CounterComponent>();
}
