fn main() {
    for i in 10..=i32::MAX {
        let decimal_str = i.to_string();

        if kaibun(decimal_str) {
            let mut binary_str = format!("{:16b}", i);
            binary_str.retain(|c| c != ' ');

            if kaibun(binary_str) {
                let mut octal_str = format!("{:16o}", i);
                octal_str.retain(|c| c != ' ');

                if kaibun(octal_str) {
                    println!("{}", i);
                    break;
                }
            }
        }
    }
}

fn kaibun(str: String) -> bool {
    str == str.chars().rev().collect::<String>()
}

#[test]
fn test_kaibun() {
    assert_eq!(kaibun(String::from("515")), true);
    assert_eq!(kaibun(String::from("99k")), false);
}
