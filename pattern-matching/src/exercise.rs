enum CitySize {
    Town=1_000,       // approximate residents: 1_000
    City=10_000,       // approximate residents: 10_000
    Metropolis=1_000_000, // approximate residents: 1_000_000
}

struct City {
    description: String,
    residents: u64,
    is_coastal: bool,
}

impl City {
    fn new(city_size: CitySize, is_coastal: bool) -> City {
        let (description, residents) = match city_size {
            CitySize::Town => {
                let residents = 1_000;

                (
                    format!("a *town* of approximately {} residents", residents),
                    residents,
                )
            }
            // 👉 TODO Handle the other CitySize variants individually,
            //    in a similar way to how *town* is handled here
            CitySize::City => {
                let residents = 10_000;

                (
                    format!("a *city* of approximately {} residents", residents),
                    residents,
                )
            }
            CitySize::Metropolis => {
                let residents = 1_000_000;

                (
                    format!("a *metropolis* of approximately {} residents", residents),
                    residents,
                )
            }
            
        };

        City {
            description,
            residents,
            is_coastal,
        }
    }
}

fn main() {
    // 👉 TODO Use City::new() to create a Metropolis-sized city here
    let rustville = City::new(CitySize::Metropolis, true);

    println!("This city is {}", rustville.description);

    if rustville.residents > 100_000 {
        println!("Wow!");
    }
}
