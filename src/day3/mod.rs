

pub(crate) fn day3(input: &Vec<&str>) -> (u64, u64) {
    (input.iter().map(|s| (max_two_num_in_line(s))).sum(), 
    input.iter().map(|s| (max_twelve_num_in_line(s))).sum())
}

fn max_two_num_in_line(line: &str) -> u64 {
    let mut nums = line.chars().map(|c| c.to_digit(10).unwrap() as u64).rev();
    
    let (mut d2, mut d1): (u64, u64) = (nums.next().unwrap(), nums.next().unwrap());

    for i in nums {
        if i >= d1 {
            if d1 >= d2{
                d2 = d1;
            }
            d1 = i; 
        }
    };
    println!("d1: {d1}, d2: {d2}");
    d1 * 10 + d2
}

fn max_twelve_num_in_line(line: &str) -> u64 {
    let mut nums = line.chars().map(|c| c.to_digit(10).unwrap() as u64).rev();
    
    let mut digits = [0 ; 12];
    for i in (0..12).rev() {
        digits[i] = nums.next().unwrap()
    }

    for i in nums {
        if i >= digits[0] {
            let mut temp = digits[0];
            digits[0] = i;
            for j in 1..digits.len() {
                if temp >= digits[j] {
                    let temp2 = digits[j];
                    digits[j] = temp;
                    temp = temp2;
                } else {
                    break;
                }
            }
        }
    };

    digits.iter().enumerate().map(|(i,d)| d * 10u64.pow((digits.len()  - 1 - i) as u32)).sum()
}