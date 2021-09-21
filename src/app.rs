use log::*;
use yew::prelude::*;

use crate::scene::Scene;

pub struct App {}

pub enum Msg {}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _link: ComponentLink<Self>) -> Self {
        App {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        info!("rendered!");
        html! {
            <div class="index">
                <header class="header">
                    <h1>{ "Ant Challenge" }</h1>
                </header>
                <section class="todoapp">
                    <section class="main">
                        <Scene/>
                    </section>
                </section>
                <footer>
                    <p>{ "Author: Matthias Lochbrunner" }</p>
                </footer>
            </div>
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }
}
