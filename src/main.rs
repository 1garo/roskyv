use hyper::body::Buf;
use hyper::client::HttpConnector;
use hyper::Client;
use hyper_tls::HttpsConnector;
use serde::Deserialize;

// TODO: add error validation on searches

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;
#[derive(Deserialize, Debug)]
struct BeerStyles {
    name: String,
    original_gravity_min: f32,
    original_gravity_max: f32,
    final_gravity_min: f32,
    final_gravity_max: f32,
    abv_min: f32,
    abv_max: f32,
    ibu_min: i16,
    ibu_max: i16,
    color_srm_min: f32,
    color_srm_max: f32,
    description: String,
}

async fn get_beer_styles(
    https: HttpsConnector<HttpConnector>,
    url: hyper::Uri,
) -> Result<Vec<BeerStyles>> {
    let client = Client::builder().build::<_, hyper::Body>(https);
    let res = client.get(url).await?;

    // asynchronously aggregate the chunks of the body
    let body = hyper::body::aggregate(res).await?;

    // try to parse as json with serde_json
    let users = serde_json::from_reader(body.reader())?;

    Ok(users)
}

#[tokio::main]
async fn main() -> Result<()> {
    // TODO: create server to consume this client
    let https = HttpsConnector::new();

    let uri = hyper::Uri::builder()
        .scheme("https")
        // TODO: replace url from .env
        .authority("rustybeer.herokuapp.com")
        // TODO: find an uriEncoder for this
        .path_and_query("/styles?name=American%20Lager")
        .build()
        .unwrap();

    let beer_styles = get_beer_styles(https, uri).await?;

    println!("beer styles: {:#?}", beer_styles);

    Ok(())
}
