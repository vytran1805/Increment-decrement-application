// imports the necessary modules from the Yew framework's prelude, 
// which provides commonly used Yew types and macros.
use yew::prelude::*;

// defines a simple struct called Model that represents the application state. 
// It has a single field value of type i64.
struct Model {
    value: i64
}

// function component named app, which is used to define resusable UI components
#[function_component(App)]
fn app()-> Html {
    let state = use_state (|| Model{    //uses the use_state hook provided by Yew to create a mutable state variable called state
        value: 0
    });
    
    let onclick_increment = {
        // clone state to use within the closure
        let state = state.clone();
        
        // one this is called, the value increases by 1
        Callback::from(move |_|{
            state.set(Model{
                value:state.value + 1
            })
        })
    };
    let onclick_decrement = {
        let state = state.clone();
        

        Callback::from(move |_|{
            state.set(Model{
                value:state.value - 1
            })
        })
    };
    html! {
        <div>
            <button onclick={onclick_increment}>{"Increse by 1"}</button>
            <button onclick={onclick_decrement}>{"Decrese by 1"}</button>
            <p> {state.value}</p>
        </div>

    }
}

// root component of the application
fn main(){
    yew::start_app::<App>();
}