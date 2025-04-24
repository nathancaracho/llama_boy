use crate::components::pokemon::PokemonCard;
use yew::prelude::*;

use super::pokemon::Pokemon;

#[derive(Properties, PartialEq)]
pub struct DeckProps {
    pub pokemons: Vec<Pokemon>,
}

#[function_component(DeckView)]
pub fn deck_view(props: &DeckProps) -> Html {
    let hovered = use_state(|| None::<usize>);

    html! {
        <div style="position: relative;height: 220px; width: 100%; display: flex; justify-content: center;">
            {
                for props.pokemons.iter().enumerate().map(|(i, poke)| {
                    let hovered_clone = hovered.clone();

                    let on_mouse_enter = {
                        let hovered = hovered.clone();
                        Callback::from(move |_e: web_sys::MouseEvent| hovered.set(Some(i)))
                    };

                    let on_mouse_leave = {
                        let hovered = hovered.clone();
                        Callback::from(move |_e: web_sys::MouseEvent| hovered.set(None))
                    };

                    let is_hovered = *hovered_clone == Some(i);


                    html! {
                        <div
                            style="margin-left: 10px;cursor:pointer;"
                            onmouseenter={on_mouse_enter}
                            onmouseleave={on_mouse_leave}
                        >
                            <PokemonCard pokemon={poke.clone()} expanded={is_hovered} />
                        </div>
                    }
                })
            }
        </div>
    }
}
