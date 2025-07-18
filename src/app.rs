use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use log::info;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"], js_name = invoke)]
    async fn invoke_without_args(cmd: &str) -> JsValue;
}

fn printit(string: &str) {
    let arg = JsValue::from_str(string);
    // let _ = invoke("printit", arg);
}

// #[derive(Serialize, Deserialize)]
// struct GreetArgs<'a> {
//     name: &'a str,
// }

enum Msg {
    content(String)
}

struct DataField {
    content: String,
}

impl Component for DataField {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_future(async {
            Msg::content(invoke_without_args("decrypt").await.as_string().unwrap())
        });
        Self {
            content: String::new(),
        }
    }
    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::content(content) => {
                self.content = content;
                true
            }
        }
    }
    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div>
                // <textarea id="content" readonly=true>{ &self.content }</textarea>
                <textarea id="content" readonly=true value={ self.content.to_owned() }></textarea>
                // <p>{ &self.content }</p>
            </div>
        }
    }
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <main class="container">
            <DataField />
        </main>
    }
}
