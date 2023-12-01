fn contains(string: &str) -> Option<(&str, u32)> {
    let names = vec![
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    for (n, name) in names.iter().enumerate() {
        if string.contains(name) {
            return Some((name, n as u32 + 1));
        }
    }

    return None;
}

fn main() {
    let input: String = std::fs::read_to_string("input").unwrap();
    let lines = input.lines().collect::<Vec<&str>>().clone();
    let mut lines_parsed = Vec::new();

    for line in lines {
        if line.is_empty() {
            continue;
        }

        let mut read = String::new();
        for i in line.chars() {
            read.push(i);
            if let Some((name, n)) = contains(&read) {
                let mut a = read.replace(name, &n.to_string());
                a.push(name.chars().last().unwrap());
                read = a;
            }
        }

        lines_parsed.push(read);
    }

    let mut a: Vec<u32> = Vec::new();
    for i in lines_parsed {
        let mut values = (None, None);

        for i in i.chars() {
            if ('0'..='9').contains(&i) {
                if values.0.is_none() {
                    values.0 = i.to_digit(10);
                } else {
                    values.1 = i.to_digit(10);
                }
            }
        }

        println!("{:?} => {:?}", i, values);
        match values {
            (Some(dec), Some(unit)) => a.push(dec * 10 + unit),
            (Some(digit), None) => a.push(digit * 10 + digit),
            _ => panic!("We are fucked : {:?} ", i),
        }
    }

    println!("{}", a.iter().sum::<u32>());
}
