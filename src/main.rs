use state::{ State, Item };
use yew::html::Scope;
use yew::{ html, Component, Context, Html };

mod state;

pub enum Msg {
    Add(f32),
    Remove(usize),
    Reset()
}

pub struct AppViewModel {
    state: State
}

impl Component for AppViewModel {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let state = State {
            total: 0.0,
            average: 0.0,
            items: Vec::new(),
            buttons: (0..10).collect()
        };

        Self { state }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Add(val) => {
                let item = Item::new(val);
                   
                self.state.items.push(item);
                self.state.recount();
            },
            Msg::Remove(id) => {
                self.state.remove(id);
                self.state.recount();
            },
            Msg::Reset() => {
                self.state.reset();
                self.state.recount();
            }
        }
        
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // This gives us a component's "`Scope`" which allows us to send messages, etc to the component.
        let link = ctx.link();
        
        html! {
            <div class="content">
                <div class="panels">
                    <div class="panel-left">
                        <div class="card">
                            <div class="card-header">
                                <div class="row">
                                    <div class="col">
                                        <span>{"Počet:"}</span>
                                        <span class="ms-2" style="font-weight: bold">{self.state.items.len()}</span>
                                        <span class="ms-4">{"Průměr:"}</span>
                                        <span class="ms-2" style="font-weight: bold">{ self.state.average }</span>
                                    </div>
                                </div>
                                <div class="row mt-2">
                                    <div class="col-8">
                                        <h1><span class="badge bg-success">{ self.state.total }</span></h1>
                                    </div>
                                    <div class="col-4 text-end">
                                        <button type="button" class="btn" style="font-size: 1.4em;" 
                                            onclick={link.callback(|_| Msg::Reset())}>
                                            <i class="fas fa-sync"></i>
                                        </button>
                                    </div>
                                </div>
                            </div>
                        </div>
                        <div class="card-body">
                            <div class="mt-2">
                            { for self.state.items.iter().enumerate().map(|e| self.view_vote(e, ctx.link())) }
                            </div>
                        </div>
                    </div>
                    <div class="panel-right">
                        <div class="buttons">
                        { for self.state.buttons.iter().map(|e| self.init_buttons(e, ctx.link())) }
                        </div>
                    </div>
                </div>
            </div>
        }
    }
}

impl AppViewModel {

    fn init_buttons(&self, value: &i32, link: &Scope<Self>) -> Html {
        let left: f32;
        let right: f32;

        match value {
            0 => {
                left = 10.0;
                right = 0.5;
            },
            _ => {
                left = value.clone() as f32;
                right = left + 0.5;
            }
        }
        html! {
            <>
            <button class="btn btn-primary" onclick={link.callback(move |_| Msg::Add(left))}>{ format!("{}", left) }</button>
            <button class="btn btn-secondary" onclick={link.callback(move |_| Msg::Add(right))}>{ format!("{}", right) }</button>
            </>
        }
    }

    fn view_vote(&self, (idx, item): (usize, &Item), link: &Scope<Self>) -> Html {
        html! {
            <div class="row">
                <div class="col-6"><div class="text-end mt-2"><h3>{ format!("{}", item.value) }</h3></div></div>
                // <div class="col-6 text-center>"><h3>{ format!("{}", item.value) }</h3></div>
                <div class="col-6">
                    <button class="btn btn-danger m-1 ms-2" style="width: 60px; height: 50px"
                        onclick={link.callback(move |_| Msg::Remove(idx))}><i class="fa fa-trash" aria-hidden="true"></i></button>
                </div>
            </div>
        }
    }

    // fn view_vote2(&self, (idx, item): (usize, &Item), link: &Scope<Self>) -> Html {
    //     html! {
    //         <li>
    //         <button class="btn btn-secondary m-2" style="width: 60px; height: 50px"
    //             onclick={link.callback(move |_| Msg::Remove(idx))}>{ format!("{}", item.value) }</button>
    //         </li>
    //     }
    // }
}
    

fn main() {
    yew::start_app::<AppViewModel>();
}
