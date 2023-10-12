use celes::Country;
use worker::*;

#[event(fetch)]
async fn main(req: Request, _env: Env, _ctx: Context) -> Result<Response> {
    if req.path() == "/coordinates" {
        let lat_lang = req.cf().coordinates().unwrap_or_default();
        Response::ok(format!("[{1}, {0}]", lat_lang.0, lat_lang.1))
    } else {
        let city = req.cf().city().unwrap_or_default();
        let country_alpha2 = req.cf().country().unwrap_or_default();
        let country = Country::from_alpha2(country_alpha2).unwrap().long_name;
        Response::ok(format!("\"{}, {}\"", city, country))
    }
}
