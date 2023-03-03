use std::env;

/// Representation of a Purple Exporter Configuration
/// # Fields
/// * `sensor_ip` - IP address of the sensor to query
/// * `query_rate` - Rate (in seconds) at which to query Purple API (min: 300)
/// * `port` - The network port to serve the exporter on
/// * `adjust` - Flag, true if measurements should be adjusted for air temperature and pressure
pub struct Config {
  pub sensor_ip: String,
  pub query_rate: u64,
  pub port: String,
  pub adjust: bool
}

/// Creates exporter configuration based on environment variables, falling back
/// to command line arguments if a critical environment variable is not set.
/// # Returns
/// * Config struct parsed from args
pub fn load_config() -> Config{
  let env_var_not_set = env::var("SENSOR_IP").is_err() ;
  if env_var_not_set{
    return config_from_args();
  } else {
    return config_from_env();
  }
}

/// Creates Exporter Configuration based on environment variables set when
/// exporter is started.
/// # Returns
/// * Config struct parsed from environment variables
fn config_from_env() -> Config{
  // Required Environment Variables
  let sensor_ip = env::var("SENSOR_IP").unwrap();
  // Optional Environment Variables
  let query_rate = env::var("REQUEST_RATE").unwrap_or(String::from("300"));
  let port = env::var("PORT").unwrap_or(String::from("9184"));
  let adjust = env::var("ADJUST").unwrap_or(String::from("false"));

  // Convert Query Rate to u64, adjust to bool
  let query_rate_conv = query_rate.parse::<u64>().unwrap_or(300);
  let adjust_conv = adjust.parse::<bool>().unwrap_or(false);

  let config = Config{
    sensor_ip: sensor_ip,
    query_rate: query_rate_conv,
    port: port,
    adjust: adjust_conv
  };

  return config;
}

/// Creates Exporter Configuration based on command line arguments passed at 
/// execution time.
/// # Returns
/// * Config struct parsed from args
fn config_from_args() -> Config{
  let args = clap_app!(myapp => 
    (version: "0.6.0")
    (author: "Wayne Manselle <wayne@viridianforge.tech")
    (about: "Purple Air API Prometheus Exporter")
    (@arg rate: -r --rate +takes_value "How often to query Purple API (seconds, min 300)")
    (@arg sensor: -s --sensor +takes_value "Purple Air Sensor IP Address to get readings from (string)")
    (@arg port: -p --port +takes_value "Port for exporter to listen on (string)")
    (@arg adjust: -a --adjust +takes_value "Adjust humidity and temperature to reflect ambient air")
  ).get_matches();

  // Set up configuration items
  let port_string = args.value_of("port").unwrap_or("9184");
  let adjust_flag = value_t!(args, "adjust", bool).unwrap_or(false);
  let request_rate = value_t!(args, "rate", u64).unwrap_or(300);
  let sensor_ip = args.value_of("sensor").expect("Invalid or missing Sensor Index");

  let config = Config{
    sensor_ip: sensor_ip.to_string(),
    query_rate: request_rate,
    port: port_string.to_string(),
    adjust: adjust_flag
  };

  return config;
}