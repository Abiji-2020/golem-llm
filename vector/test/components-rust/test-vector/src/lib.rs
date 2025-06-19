#[allow(static_mut_refs)]
mod bindings;

use crate::bindings::exports::test::vector_exports::test_vector_api::Guest;
use crate::bindings::golem::vector::connection;
use std::cell::RefCell;

/// This is one of any number of data types that our application
/// uses. Golem will take care to persist all application state,
/// whether that state is local to a function being executed or
/// global across the entire program.
struct State {
    total: u64,
}

thread_local! {
    /// This holds the state of our application.
    static STATE: RefCell<State> = RefCell::new(State {
        total: 0,
    });
}

struct Component;

impl Guest for Component {
    /// Updates the component's state by adding the given value to the total.
    fn add(value: u64) {
        let config  = connection::OauthConfig{
            client_id: "ClientID".to_string(),
            client_secret: Some("ClientSecret".to_string()),
            token_url: "https://example.com/token".to_string(),
            scope: Some("read write".to_string()),            
        };
        connection::Credentials::Oauth(config);
        let status = connection::get_connection_status();
        match status {
            Ok(status) => {
                println!("Connection status: {:?}", status);
            }
            Err(e) => {
                println!("Failed to get connection status: {:?}", e);
            }
        }
        STATE.with_borrow_mut(|state| state.total += value);
    }

    /// Returns the current total.
    fn get() -> u64 {
        STATE.with_borrow(|state| state.total)
    }
}

bindings::export!(Component with_types_in bindings);
