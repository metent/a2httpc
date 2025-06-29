use std::env;

use a2httpc::Result;

fn main() -> Result {
    env_logger::init();

    let url: String = env::args().collect::<Vec<_>>().into_iter().nth(1).expect("missing url");

    let resp = a2httpc::get(url).send()?;
    println!("Status: {:?}", resp.status());
    println!("Headers:\n{:#?}", resp.headers());
    println!("Body:\n{}", resp.text()?);

    Ok(())
}
