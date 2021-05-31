extern crate pretty_env_logger;
#[macro_use] extern crate log;

use reqwest::header;
use reqwest::blocking::Client;
use url::Url;

mod purple_exporter;
use purple_exporter::{start_exporter, update_metrics};

mod purple_reading;
use purple_reading::get_reading;

#[macro_use]
extern crate clap;

fn main() {
    // Use Clap to setup App configuration
    let args = clap_app!(myapp => 
        (version: "0.5.0")
        (author: "Wayne Manselle <wayne@viridianforge.tech")
        (about: "Purple Air API Prometheus Exporter")
        (@arg rate: -r --rate +takes_value "How often to query Purple API (seconds, min 300)")
        (@arg sensor: -s --sensor +takes_value "Purple Air Sensor to get readings from (string)")
        (@arg readkey: -x --readkey +takes_value "API Read Key for Purple Air API (string)")
        (@arg port: -p --port +takes_value "Port for exporter to listen on (string)")
        (@arg adjust: -a --adjust +takes_value "Adjust humidity and temperature to reflect ambient air")
    ).get_matches();

    // TODO -- enable starting server using Environment Variables

    // Set up configuration items
    let port_string = args.value_of("port").unwrap_or("9184");
    //let adjust_string = args.value_of("adjust").unwrap_or("false");
    let adjust_flag = value_t!(args, "adjust", bool).unwrap_or(false);
    //let rate_string = args.value_of("rate").unwrap_or("300");
    let mut request_rate = value_t!(args, "rate", u64).unwrap_or(300);
    let sensor_index = args.value_of("sensor").expect("Invalid or missing Sensor Index");
    let purple_read_key = args.value_of("readkey").expect("Invalid or missing API Read Key");

    // First Parse Request Pacing Safely
    //let mut request_rate:u64 = rate_string.parse().expect("Invalid request rate.");

    // Set Ambient Adjustment Flag
    //let adjust_flag:bool = adjust_string.parse().unwrap_or(false);

    // Purple asks that API requests be limited to at least once every five minutes
    if request_rate < 300 {
        print!("Invalid API Request Rate set, defaulting to 300 seconds.");
        request_rate = 300;
    }

    let update_frequency = std::time::Duration::from_secs(request_rate);

    pretty_env_logger::init();

    // Build Headers
    let mut headers = header::HeaderMap::new();
    headers.insert("X-API-Key", header::HeaderValue::from_str(&purple_read_key).unwrap());

    // Build Client
    let client = Client::builder()
        .default_headers(headers)
        .build().unwrap();

    // Start exporter
    let purple_exporter = start_exporter(port_string);

    // Build URL used in GETs
    let url_string = "https://api.purpleair.com/v1/sensors/".to_owned() + &sensor_index;

    loop{
        info!("Requesting update for Sensor.");

        // Create URL from URL string - consumed by loop
        let sensor_url = Url::parse(&url_string).unwrap();

        // Request Reading from PurpleAPI server       
        let raw_resp = client.get(sensor_url).send()
            .expect("Response from PurpleAPI could not be retrieved.")
            .text()
            .expect("Unable to unwrap PurpleAPI response");

        //TODO -- Add ability to recognize and adapt to API server not responding

        info!("{}", raw_resp);

        update_metrics(get_reading(raw_resp, adjust_flag));

        let _gaurd = purple_exporter.wait_duration(update_frequency);
    }
}