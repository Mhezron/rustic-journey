//Getting a http request 

use http::{Request, Response, StatusCode};


struct Activities {
    request: Request<()>,
    response: Response<()>,
}

impl Activities {
    fn activities(&self) {
        
    }
}

fn main() {
    let request = Request::builder()
    .uri("https://www.rust-lang.org/")
    .header("User-Agent", "awesome/1.0")
    .body(())
    .unwrap();

    let response = Response::builder()
        .status(StatusCode::MOVED_PERMANENTLY)
        .header("Location", "https://www.rust-lang.org/install.html")
        .body(())
        .unwrap();
        

    println!("The url {:?} and the response {:?}", request, response)
}
