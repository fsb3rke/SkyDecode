use metar::{Metar, WindSpeed, WindDirection};
use tabled::Tabled;

#[derive(Tabled)]
pub struct DecodedMetar {
    pub station: String,
    pub time: String,
    pub wind: String,
    pub visibility: String,
    pub temperature: String,
    pub raw: String,
}

fn wind_speed_to_knots(speed: &WindSpeed) -> u32 {
    match speed {
        WindSpeed::Knot(k) => *k,
        _ => todo!(),
    }
}

fn wind_direction_to_heading(heading: &WindDirection) -> u32 {
    match heading {
        WindDirection::Heading(h) => *h,
        _ => todo!(),
    }
}

pub fn decode_metar(raw: &str) -> Option<DecodedMetar> {
    if let Ok(m) = Metar::parse(raw.to_string()) {
        let time = format!("{} {:02}:{:02}Z", m.time.date, m.time.hour, m.time.minute);
        
        let wind = match m.wind.gusting {
            Some(g) => format!("{:?} {:?}kt (gust {:?}kt)", wind_direction_to_heading(&m.wind.dir), wind_speed_to_knots(&m.wind.speed), g),
            None => format!("{:?} {:?}kt", wind_direction_to_heading(&m.wind.dir), wind_speed_to_knots(&m.wind.speed)),
        };

        let visibility = match m.visibility {
            metar::Visibility::Metres(v) => format!("{} m", v),
            _ => todo!(),
        };


        Some(DecodedMetar {
            station: m.station.clone(),
            time: time,
            wind: wind,
            visibility: visibility,
            temperature: m.temperature.to_string(),
            raw: raw.to_string(),
        })
    } else {
        None
    }
}
