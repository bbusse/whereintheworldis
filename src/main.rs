// SPDX-License-Identifier: BSD-3-Clause
// Author: Björn Busse <bj.rn@baerlin.eu>
//
// Geolocation CLI tool: Query country, region, city, and coordinates for an IP address using ip-api.com
//
// use minreq for HTTP requests
use serde::Deserialize;


#[derive(Deserialize, Debug)]
struct GeoResponse {
	country: Option<String>,
	#[serde(rename = "regionName")]
	region_name: Option<String>,
	city: Option<String>,
	lat: Option<f64>,
	lon: Option<f64>,
	query: Option<String>,
	status: Option<String>,
	message: Option<String>,
}

impl GeoResponse {
	fn latitude(&self) -> Option<String> {
		self.lat.map(|v| v.to_string())
	}
	fn longitude(&self) -> Option<String> {
		self.lon.map(|v| v.to_string())
	}
}

fn main() {
	let mut ip_address: Option<String> = None;
	let mut output: Option<String> = None;
	let args: Vec<String> = std::env::args().collect();
	let mut i = 1;
	while i < args.len() {
		match args[i].as_str() {
			"--ip-address" => {
				if i + 1 < args.len() {
					ip_address = Some(args[i + 1].clone());
					i += 1;
				}
			}
			"--output" => {
				if i + 1 < args.len() {
					output = Some(args[i + 1].clone());
					i += 1;
				}
			}
			_ => {}
		}
		i += 1;
	}
	let ip = ip_address.unwrap_or_else(|| "".to_string());
	let url = if ip.is_empty() {
		"http://ip-api.com/json/".to_string()
	} else {
		format!("http://ip-api.com/json/{}", ip)
	};

	let resp = minreq::get(&url).send();
	let geo: GeoResponse = match resp {
		Ok(response) => {
			if response.status_code != 200 {
				eprintln!("Request failed: HTTP {}", response.status_code);
				return;
			}
			let body = match response.as_str() {
				Ok(b) => b,
				Err(e) => {
					eprintln!("Failed to read response body: {}", e);
					return;
				}
			};
			match serde_json::from_str::<GeoResponse>(body) {
				Ok(data) => data,
				Err(e) => {
					eprintln!("Failed to parse response: {}", e);
					return;
				}
			}
		}
		Err(e) => {
			eprintln!("Request failed: {}", e);
			return;
		}
	};
	if geo.status.as_deref() == Some("success") {
		match output.as_deref() {
			Some("ip") => println!("{}", geo.query.unwrap_or_default()),
			Some("country") => println!("{}", geo.country.unwrap_or_default()),
			Some("region") => println!("{}", geo.region_name.unwrap_or_default()),
			Some("city") => println!("{}", geo.city.unwrap_or_default()),
			Some("latitude") => println!("{}", geo.lat.map_or("N/A".to_string(), |v| v.to_string())),
			Some("longitude") => println!("{}", geo.lon.map_or("N/A".to_string(), |v| v.to_string())),
			Some("coordinates") => println!(
				"{} {}",
				geo.longitude().unwrap_or("N/A".to_string()),
				geo.latitude().unwrap_or("N/A".to_string())
			),
			None => {
				println!("IP: {}", geo.query.unwrap_or_default());
				println!("Country: {}", geo.country.unwrap_or_default());
				println!("Region: {}", geo.region_name.unwrap_or_default());
				println!("City: {}", geo.city.unwrap_or_default());
				println!("Latitude: {}", geo.lat.map_or("N/A".to_string(), |v| v.to_string()));
				println!("Longitude: {}", geo.lon.map_or("N/A".to_string(), |v| v.to_string()));
			}
			Some(other) => {
				eprintln!("Unknown output field: {}", other);
			}
		}
	} else {
		eprintln!("Error: {}", geo.message.unwrap_or("Unknown error".to_string()));
	}
}
