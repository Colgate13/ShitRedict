#[macro_use] extern crate rocket;

use rocket::response::Redirect;

struct Link {
    url: String,
    id: i32,
}

struct LinkCollection {
    links: Vec<Link>,
}

impl LinkCollection {
    fn new() -> Self {
        LinkCollection { links: Vec::new() }
    }

    fn add(&mut self, url: String, id: i32) {
        let link = Link { url, id };
        self.links.push(link);
    }
}

static mut LINKS: Option<LinkCollection> = None;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/<id>")]
fn redirectRoute(id: i32) -> Result<Redirect, &'static str> {
    unsafe {
        if let Some(links) = &LINKS {
            if let Some(link) = links.links.iter().find(|link| link.id == id) {
                return Ok(Redirect::to(link.url.clone()));
            }
        }
    }

    Err("Link not found")
}

#[launch]
fn rocket() -> _ {
    unsafe {
        LINKS = Some(LinkCollection::new());
        if let Some(links) = &mut LINKS {
            links.add(String::from("https://www.example.com"), 1);
            links.add(String::from("https://www.google.com"), 2);
            links.add(String::from("https://www.openai.com"), 3);
        }
    }

    rocket::build()
        .mount("/", routes![index, redirectRoute])
}