// mod error;
mod subdomains;

use std::time::Duration;

use crate::subdomains::SubdomainWrapper;

fn main() -> Result<(), anyhow::Error> {

    // build a custom client
    let client = reqwest::blocking::Client::builder()
        .timeout(Duration::from_secs(90))
        .build()?;

    let resp = client.get("https://crt.sh/?q=%25.tesla.com&output=json").send()?;
    
    let ret = resp.json::<subdomains::SubdomainList>();

    let data = match ret {
        Ok(data) => data,
        Err(e) => {
            dbg!("{}", e);
            Vec::new()
        },
    };

    if data.len() == 0 {
        println!("GET failed ...");
        return Ok(());
    }
    
    println!("Common names:");

    // for subd in data.get_unique_common_names() {
    //     println!("{}", subd);
    // }

    for subd in data.get_strict_subdomains("tesla.com".to_string()) {
        println!("{}", subd);
    }

    Ok(())
}
