use ip2location::{DB, Record};
use std::{borrow::Cow, net::IpAddr};

const IPV6BIN: &str = "db/ip2location.BIN";

#[derive(Debug, Default)]
pub struct CountryDetails {
    pub name: Option<String>,
    pub code: Option<String>,
    pub region: Option<String>,
}

pub fn get_country_details(ip: IpAddr) -> Option<CountryDetails> {
    let db = DB::from_file(IPV6BIN).ok()?;

    let record = db.ip_lookup(ip).ok()?;
    let rec = match record {
        Record::LocationDb(rec) => rec,
        _ => return None,
    };

    let country = rec.country?;

    // This means that we didn't find a country.
    if country.long_name == "-" {
        return None;
    }

    Some(CountryDetails {
        name: Some(country.long_name.to_string()),
        code: Some(country.short_name.to_string()),
        region: rec.region.map(Cow::into_owned),
    })
}
