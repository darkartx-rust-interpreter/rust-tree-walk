use std::error;

pub fn parse_number(number_str: &str) -> Result<f64, Box<dyn error::Error>> {
    Ok(number_str.parse::<f64>()?)
}