use bindings::exports::arithmetic::overflow::overflowing_add::{Guest, Arguments};

#[allow(warnings)]
mod bindings;

struct Library;

impl Guest for Library {
    /// Returns a tuple of the addition along with a boolean indicating whether 
    /// an arithmetic overflow would occur. If an overflow would have occurred 
    /// then the wrapped value is returned.
    fn overflowing_add(Arguments { x, y }: Arguments) -> (i32, bool) {
        x.overflowing_add(y)
    }
}

bindings::export!(Library with_types_in bindings);
