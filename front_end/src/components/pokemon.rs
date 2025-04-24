use serde::Deserialize;
use yew::prelude::*;

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Pokemon {
    pub ability_1: bool,
    pub evs: StatSpread,
    pub exp: u32,
    pub friendship: u8,
    pub hp: u16,
    pub item: u16,
    pub ivs: StatSpread,
    pub level: u8,
    pub max_hp: u16,
    pub moves: Vec<MoveSlot>,
    pub nature: u8,
    pub pokerus: u8,
    pub pp_bonuses: u8,
    pub species: String,
    pub species_id: u16,
    pub stats: StatSpread,
    pub status: u8,
    pub is_battle: bool,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct StatSpread {
    pub atk: u8,
    pub def: u8,
    pub hp: u8,
    pub spa: u8,
    pub spd: u8,
    pub spe: u8,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct MoveSlot {
    pub move_id: u16,
    pub move_item: Option<MoveInfo>,
    pub pp: u8,
    pub pp_used: u8,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct MoveInfo {
    pub accuracy: u8,
    pub category: String,
    pub id: u16,
    #[serde(rename = "move_type")]
    pub move_type: String,
    pub name: String,
    pub power: Option<u16>,
    pub pp: u8,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Move {
    pub name: String,
    pub move_type: String,
    pub category: String,
    pub power: Option<u16>,
    pub accuracy: u8,
    pub pp: u8,
    pub pp_used: u8,
}

#[derive(Properties, PartialEq)]
pub struct PokemonCardProps {
    pub pokemon: Pokemon,
    #[prop_or(false)]
    pub expanded: bool,
}
#[function_component(PokemonCard)]
pub fn pokemon_card(props: &PokemonCardProps) -> Html {
    let moves: Vec<Move> = props
        .pokemon
        .moves
        .iter()
        .filter_map(|slot| {
            slot.move_item.as_ref().map(|item| Move {
                name: item.name.clone(),
                move_type: item.move_type.clone(),
                category: item.category.clone(),
                power: item.power,
                accuracy: item.accuracy,
                pp: item.pp,
                pp_used: slot.pp_used,
            })
        })
        .collect();
    if props.expanded {
        html! {
            <div class="card hoverable" style="
            width: 350px;
            background: linear-gradient(135deg, #f9f9f9, #ddd);
            border-radius: 12px;
            border: 15px solid #ccc;">

                <div class="card-content animate__slideInDown">
                    <span class="card-title" style="font-weight: bold; font-size: 1.1rem; text-align: center; display: block;">
                        { format!("{} Lv.{}", props.pokemon.species, props.pokemon.level) }
                    </span>
                    <div class="row">
                        <div class="col s12">
                            <div class="card">
                                <img
                                src={format!("https://raw.githubusercontent.com/PokeAPI/sprites/master/sprites/pokemon/other/official-artwork/{}.png", props.pokemon.species_id)}
                                style="width: 150px; height: 150px; object-fit: contain; margin: 0.5rem auto; display: block;"
                            />
                            </div>
                        </div>
                     </div>
                    <div class="row">
                        <div class="col s12">                        <div class="progress red lighten-2" style="height: 8px; border-radius: 4px;">
                                <div class="determinate green" style={format!("width: {}%;", props.pokemon.hp * 100 / props.pokemon.max_hp)}></div>
                            </div>
                        </div>
                     <div class="row">
                        <div class="col s12 center">
                            <div class="col s6">
                                <span style="margin: 0;">{ format!("HP: {}/{}", props.pokemon.hp, props.pokemon.max_hp) }</span>
                            </div>
                            <div class="col s6">
                                <span style="margin: 0;">{ format!("EXP: {}", props.pokemon.exp) }</span>
                            </div>
                            </div>
                        </div>
                    </div>

                    <table class="striped" style="font-size: 0.75rem; margin-top: 0.5rem;">
                        <thead>
                            <tr>
                                <th>{"ATK"}</th><th>{"DEF"}</th><th>{"SPA"}</th><th>{"SPD"}</th><th>{"SPE"}</th>
                            </tr>
                        </thead>
                        <tbody>
                            <tr>
                                <td>{ props.pokemon.stats.atk }</td>
                                <td>{ props.pokemon.stats.def }</td>
                                <td>{ props.pokemon.stats.spa }</td>
                                <td>{ props.pokemon.stats.spd }</td>
                                <td>{ props.pokemon.stats.spe }</td>
                            </tr>
                        </tbody>
                    </table>

                    <div style="margin-top: 0.5rem;">
                        <h6 style="font-weight: bold; font-size: 0.9rem;">{"Moves:"}</h6>
                        {
                            for moves.iter().map(|m| html! {
                                <div class="card-panel z-depth-1" style="
                                padding: 0.4rem;
                                margin: 0.2rem 0;
                                ">
                                    <span style="font-weight: bold;">{ &m.name }</span>
                                    <span style="float: right;">{ format!("PP: {}/{}", m.pp - m.pp_used, m.pp) }</span>
                                    <br />
                                    <span style="font-size: 0.7rem;">
                                        { format!("{} | {} | ACC: {}", m.move_type, m.category, m.accuracy) }
                                    </span>
                                    {
                                        if let Some(power) = m.power {
                                            html! { <span style="font-size: 0.7rem;">{ format!(" | PWR: {}", power) }</span> }
                                        } else {
                                            html! {}
                                        }
                                    }
                                </div>
                            })
                        }
                    </div>
                </div>
            </div>
        }
    } else {
        html! {
        <div class="card z-depth-2" style="width: 120px;
            padding: 0.5rem;
            display: flex;
            flex-direction: column;
            align-items: center;
            justify-content: center;
            border: 5px solid #ccc;
            border-radius: 10px;">
            <span  style="font-weight: bold; font-size: 1.1rem; text-align: center; display: block;">
                { format!("{} Lv.{}", props.pokemon.species, props.pokemon.level) }
                <i class="Tiny material-icons" >{
                    if props.pokemon.is_battle {
                    "pan_tool"} else {"not_interested"}
                }</i>
            </span>
            <div class="row" >
                            <img
                                src={format!("https://play.pokemonshowdown.com/sprites/ani/{}.gif
        ", props.pokemon.species.to_lowercase())}
                                style="width: 50px; height: 50px; object-fit: contain;"
                            />
        </div>
         <div class="row">
            <div class="col s12 center">
                <div class="col s6">
                    <span style="margin: 0;">{ format!("HP: {}/{}", props.pokemon.hp, props.pokemon.max_hp) }</span>
                </div>
                <div class="col s6">
                    <span style="margin: 0;">{ format!("EXP: {}", props.pokemon.exp) }</span>
                </div>
                </div>
            </div>
            </div>

                }
    }
}
