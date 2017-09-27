extern crate iron;
extern crate router;

use iron::prelude::*;
use iron::status;
use router::Router;


fn main() {
    let mut router = Router::new();

    router.get("/", hello_world, "index");
    router.get("/greet/:name", greet, "greet");

    fn hello_world(_: &mut Request) -> IronResult<Response> {
        Ok(Response::with((status::Ok, "Hello World!")))
    };

    fn greet(req: &mut Request) -> IronResult<Response> {
        let ref query = req.extensions.get::<Router>().unwrap().find("name").unwrap_or("Greet me?");
        Ok(Response::with((status::Ok, "hello ".to_string() + *query)))
    };

    let _server = Iron::new(router).http("localhost:3000").unwrap();
    println!("on 3000");
}