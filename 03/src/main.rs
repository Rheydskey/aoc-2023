fn extract(b: &Vec<char>, b_num: &[usize]) -> usize {
    let mut last = '.';
    let mut sum = 0;
    for i in 0..3 {
        if last == '.' || "!\"#$%&'()*+,-/:;<=>?@[\\]^_`{|}~".contains(last) {
            sum += b_num[i];
        }

        last = b[i];
    }

    return sum;
}

fn main() {
    let input = std::fs::read_to_string("./input").unwrap();
    let lines = input.lines().collect::<Vec<_>>();
    let symbols = "!\"#$%&'()*+,-/:;<=>?@[\\]^_`{|}~";
    let mut numbers = Vec::new();
    let mut sum = 0;

    for line in &lines {
        let (mut number_str, mut i) = (String::new(), 0);
        let mut line_number = Vec::new();
        let chars = line.chars().collect::<Vec<_>>();
        for n in 0..chars.len() {
            let c = chars[n];
            if number_str.is_empty() && c.is_digit(10) {
                i = n;
            }

            if c.is_digit(10) {
                number_str.push(c);
            } else {
                if !number_str.is_empty() {
                    let a = number_str.parse::<usize>().unwrap();

                    for _ in i..i + number_str.len() {
                        line_number.push(a);
                    }

                    number_str = String::new();
                }

                line_number.push(0);
            }
        }

        if !number_str.is_empty() {
            let a = number_str.parse::<usize>().unwrap();

            for _ in i..i + number_str.len() {
                line_number.push(a);
            }
        }

        assert!(line_number.len() == line.len());
        numbers.push(line_number);
    }

    for line in 3..=lines.len() {
        let old = sum;
        let switch = lines[line - 2].chars().collect::<Vec<_>>();

        for (n, c) in switch.iter().enumerate() {
            if symbols.contains(*c) {
                let t = lines[line - 3][n - 1..=n + 1].chars().collect::<Vec<_>>();
                let t_num = &numbers[line - 3][n - 1..=n + 1];
                sum += extract(&t, t_num);

                let m = lines[line - 2][n - 1..=n + 1].chars().collect::<Vec<_>>();
                let m_num = &numbers[line - 2][n - 1..=n + 1];
                sum += extract(&m, m_num);

                let b = lines[line - 1][n - 1..=n + 1].chars().collect::<Vec<_>>();
                let b_num = &numbers[line - 1][n - 1..=n + 1];
                sum += extract(&b, b_num);
            }
        }

        println!("Added {}", sum - old);
    }

    println!("{sum:?}");
}
