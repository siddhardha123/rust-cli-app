use structopt::StructOpt;
use exitfailure::{Exitfailure};

#[derive(StructOpt)]
struct Cli {
    city: String,
    country_code: String,
}

struct Forecast {
    coord : Coord,
    weather : Weather,
    base : String
}

struct Coord{
    lon : f64,
    lat : f64,
}

struct Weather{
  details : Details
}

struct Details{
    temp : f64,
    feels_like : f64,
}


impl Forecast {
    async fn get(city : String, country_code : String) -> Result<self,Exitfailure>{
       
    }
}

fn main() {
    let args = Cli::from_args();
    
    println!("City: {}", args.city);
    println!("Country Code: {}", args.country_code);

}


//https://api.openweathermap.org/data/2.5/weather?q={city name}&appid={7677c38ed15abedec9188c1741c5264e}