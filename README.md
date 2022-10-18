# Photo Rating

Jednoduchá webassembly ([yew](https://yew.rs/)) aplikace pro společné hodnocení fotografií ve fotoklubu.

Vznikla pro mojí potřebu seznámení se s Rustem.

## Install

```bash
rustup target add wasm32-unknown-unknown
```

```bash
cargo install trunk
```

## Running

Run a debug version of this application:

```bash
trunk serve
```

Run a release version of this application:

```bash
trunk serve --release
```

## tmp

impl Component for AppViewModel {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self { 
            total: 0.0,
            average: 0.0,
            arr: Vec::new(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Plus(val) => {
                self.total += val;

                let id = Uuid::new_v4().to_string();
                self.arr.push(Item::new(id, val));

                let _avg = ((self.total / self.arr.len() as f32) * 100.0).round() / 100.0;
                self.average = _avg;

                true
            },
            Msg::Minus(id) => {
                // let item = self.arr.drain_filter(filter)
                //self.total -= item.value;
                // self.arr.remove(index)
                println!("{}", id);
                self.arr.pop();

                let _avg = ((self.total / self.arr.len() as f32) * 100.0).round() / 100.0;
                self.average = _avg;

                true
            },
            Msg::Reset => {
                self.total = 0.0;
                self.average = 0.0;
                self.arr.clear();
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // This gives us a component's "`Scope`" which allows us to send messages, etc to the component.
        let link = ctx.link();
        
        html! {
            <div class="main">
    <div class="content">
        <div class="panels">
            <div class="panel-left">
                <div class="card">
                    <div class="card-header">
                        <div class="row">
                            <div class="col-6"><span>{"Počet:"}{self.arr.len()}</span></div>
                            <div class="col-6"><span style="font-weight: bold">{ self.average }</span>
                            </div>
                        </div>
                        <div class="row">
                            <div class="col-8">
                                <h2><span class="badge badge-success">{ self.total }</span></h2>
                            </div>
                            <div class="col-4 text-right">
                                <button type="button" class="btn" style="font-size: 1.4em;" onclick={link.callback(|_|
                                    Msg::Reset)}>
                                    <i class="fas fa-sync"></i>
                                </button>
                            </div>
                        </div>
                    </div>
                </div>
                <div class="card-body">
                {
                    self.arr.iter().map(|item| {
                        html!{
                            <div class="row">
                            <div class="col-2">
                            <button class="btn btn-primary" onclick={link.callback(|_| Msg::Minus(String::from("item.id")))}>{ format!("{}!", item.value) }</button>
                            </div>
                            </div>
                        }
                    }).collect::<Html>()
                }
                </div>
            </div>
            <div class="panel-right">
                [1..9].iter().map(|i| {
                    html!{<div class="">}
                }).collect::<Html>()

                <div class="buttons">
                    <button class="btn btn-primary" onclick={link.callback(|_| Msg::Plus(10.0))}>{ "10" }</button>
                    <button class="btn btn-secondary" onclick={link.callback(|_| Msg::Plus(0.5))}>{ "0,5" }</button>
                    <button class="btn btn-primary" onclick={link.callback(|_| Msg::Plus(1.0))}>{ "1" }</button>
                    <button class="btn btn-secondary" onclick={link.callback(|_| Msg::Plus(1.5))}>{ "1,5" }</button>
                </div>
            </div>
        </div>
    </div>
</div>
        }
    }
}

fn main() {
    yew::start_app::<AppViewModel>();
}
