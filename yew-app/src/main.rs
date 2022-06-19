use yew::prelude::*;
use serde::Deserialize;
use reqwasm::http::Request;
#[derive(Clone, PartialEq, Deserialize)]
struct Origin {
    origin: String
}

#[derive(Clone, Properties, PartialEq)]
struct OriginProperties {
    origins: Vec<Origin>,
    on_click: Callback<Origin>
}

#[derive(Clone, Properties, PartialEq)]
struct OriginDetailsProps{
    origin: Origin,
}

#[function_component(App)]
fn app() -> Html {

    let origins = use_state(|| vec![]);

    {

    let origins = origins.clone();

    use_effect_with_deps(move |_| {
    let origins = origins.clone();
    wasm_bindgen_futures::spawn_local(async move {
    let fetched_origins: Origin =
    
        Request::get("http://httpbin.org/ip").send().await.unwrap().json().await.unwrap();
    

    origins.set(vec![fetched_origins])


    });
    || ()
    }, ());

    }

    #[function_component(OriginsList)]
    fn origins_list(OriginProperties { origins, on_click }: &OriginProperties  ) -> Html {

       let on_click = on_click.clone();
       origins.iter().map(|o|{
        let on_origin_select = {
        let on_click = on_click.clone();
        let o = o.clone();
        Callback::from(move |_| {
            on_click.emit(o.clone())
        })

        };

        html! {

            <p onclick = {on_origin_select}>{format!("{}", o.origin)}</p>
        }


       }).collect()
    }
    
    #[function_component(OriginDetails)]
    fn origin_details(OriginDetailsProps {origin}: &OriginDetailsProps) -> Html {

        html! {
            <div>
                <h3>{origin.origin.clone()} </h3>
            </div>
    
        }
    }

    let selected_origin = use_state(|| None);

    let on_origin_select = {
        let selected_origin = selected_origin.clone();
        Callback::from(move | origin: Origin |{
        selected_origin.set(Some(origin))

        })
    };

    let details = selected_origin.as_ref().map(|origin| html!{
    <OriginDetails origin = {origin.clone()} />

    });

    html! {

    <>
        <h1> {"Origins explorer"} </h1>
        <div>
        <h3> {"Origins"} </h3>
        <OriginsList origins = {(*origins).clone()} on_click={on_origin_select.clone()} />
        </div>
        {for details }
    </>
    }


}

fn main() {
    yew::start_app::<App>();
}
