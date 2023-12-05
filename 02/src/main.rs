#[derive(Debug, Default)]
struct Game(pub Vec<Set>);

#[derive(Debug, Default)]
struct Set {
    blue: u32,
    red: u32,
    green: u32,
}

fn main() {
    let input = std::fs::read_to_string("./input").unwrap();
    let mut vec = Vec::new();

    for i in input.trim().lines() {
        let split = i.split(':').collect::<Vec<&str>>();
        let cubes = split[1]
            .split(';')
            .filter(|f| !f.is_empty())
            .collect::<Vec<&str>>();

        let mut q = Game(Vec::new());
        for sets in cubes {
            let mut set = Set::default();
            for set_str in sets.split(',').collect::<Vec<_>>() {
                println!("{set_str:?}");

                for chunk in set_str
                    .split(' ')
                    .filter(|f| !f.is_empty())
                    .collect::<Vec<_>>()
                    .chunks(2)
                {
                    let name = chunk[1].to_lowercase();

                    let color: u32 = chunk[0].parse().unwrap();

                    match name.as_str() {
                        "blue" => set.blue += color,
                        "green" => set.green += color,
                        "red" => set.red += color,
                        _ => {}
                    }
                }
            }

            q.0.push(set);
        }

        vec.push(q);
    }

    let mut sum = 0;

    for Game(sets) in vec {
        let (mut m_red, mut m_blue, mut m_green) = (0, 0, 0);
        for Set { blue, red, green } in sets {
            m_blue = m_blue.max(blue);
            m_red = m_red.max(red);
            m_green = m_green.max(green);
        }
        println!("{} {} {}", m_blue, m_green, m_red);

        sum += m_blue * m_green * m_red;
    }

    println!("{sum:?}");
}
