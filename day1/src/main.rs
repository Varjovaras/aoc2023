use std::fs;

fn main() {
    my_sol();
    blyat_sol();
}

fn blyat_sol() -> i32 {
    pub fn solve() -> (i64, i64) {
        let input: Vec<&str> = include_str!("input.txt").lines().collect();

        let names = vec![
            "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
        ];

        let result1 = input
            .iter()
            .map(|line| get_calibration_value(line))
            .sum::<i64>();
        let result2 = input
            .iter()
            .map(|line| get_real_calibration_value(line, &names))
            .sum::<i64>();

        println!("1\t{result1:<20}\t{result2:<20}");

        (result1, result2)
    }

    fn get_calibration_value(text: &str) -> i64 {
        let digits: Vec<char> = text.chars().filter(|c| c.is_ascii_digit()).collect();

        format!("{}{}", digits.first().unwrap(), digits.last().unwrap())
            .parse::<i64>()
            .unwrap()
    }

    fn get_real_calibration_value(text: &str, names: &[&str]) -> i64 {
        let mut replaced = text.to_owned();

        for (i, name) in names.iter().enumerate() {
            replaced = replaced.replace(name, format!("{}{}{}", name, i, name).as_str());
            println!("{}", replaced);
        }

        get_calibration_value(replaced.as_str())
    }

    let i = solve();
    i.0 as i32
}

fn my_sol() -> i32 {
    const NUMBERS: [&str; 9] = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let content = fs::read_to_string("input.txt").expect("Could not read the file");
    let mut sum = 0;

    for line in content.lines() {
        let mut first_digit = String::new();
        let mut last_digit = String::new();

        let mut num_str = String::new();

        //first number
        for ch in line.chars() {
            if ch.is_ascii_digit() {
                first_digit = ch.to_string();
                break;
            }

            num_str += &ch.to_string();
            if NUMBERS.contains(&num_str.as_str()) {
                first_digit = match num_str.as_str() {
                    "one" => "1".into(),
                    "two" => "2".into(),
                    "three" => "3".into(),
                    "four" => "4".into(),
                    "five" => "5".into(),
                    "six" => "6".into(),
                    "seven" => "7".into(),
                    "eight" => "8".into(),
                    "nine" => "9".into(),
                    _ => panic!("first number should be digit as string but is not"),
                };
                break;
            }

            let mut found = false;
            for &number in NUMBERS.iter() {
                if number.contains(&num_str) {
                    found = true;
                    break;
                }
            }

            if !found {
                num_str = ch.into();
            }
        }

        //last number
        for ch in line.chars().rev() {
            if ch.is_numeric() {
                last_digit = ch.to_string();
                break;
            }
            num_str.insert_str(0, &ch.to_string());
            // println!("{} blyat", num_str);

            if NUMBERS.contains(&num_str.as_str()) {
                last_digit = match num_str.as_str() {
                    "one" => "1".into(),
                    "two" => "2".into(),
                    "three" => "3".into(),
                    "four" => "4".into(),
                    "five" => "5".into(),
                    "six" => "6".into(),
                    "seven" => "7".into(),
                    "eight" => "8".into(),
                    "nine" => "9".into(),
                    _ => panic!("first number should be digit as string but is not"),
                };
                break;
            }

            let mut found = false;
            for &number in NUMBERS.iter() {
                if number.contains(&num_str) {
                    found = true;
                    break;
                }
            }

            if !found {
                num_str = String::from(ch);
            }
        }

        let num = first_digit + &last_digit;
        // println!("{}", num);

        sum += num.parse::<i32>().unwrap();
        // println!("{}", sum);
    }
    // println!("{}", sum);
    sum
}
