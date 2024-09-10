use rouille::{router, Response};

fn main() {

    // The `start_server` starts listening forever on the given address.
    // Rouille example taken from https://github.com/tomaka/rouille/blob/master/examples/hello-world.rs
    rouille::start_server("localhost:8000", move |request| {
        router!(request,
            (GET) (/) => {
                // If the request's URL is `/`, we jump here.
                Response::redirect_302("/hello/world")
            },

            (GET) (/hello/world) => {
                println!("hello world");
                Response::text("hello world from server")
            },

            (GET) (/{id: u32}) => {
                println!("u32 {:?}", id);
                Response::empty_400()
            },

            (GET) (/{id: String}) => {
                println!("String {:?}", id);
                Response::text(format!("hello, {}", id))
            },

            _ => Response::empty_404()
        )
    });
}