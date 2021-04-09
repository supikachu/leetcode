fn main() {
    println!("{}", multiply("82978".to_string(), "75".to_string()));
}
fn multiply(num1: String, num2: String) -> String {
    let (lhs, rhs) = if num1.len() > num2.len() {
        (
            num1.chars()
                .map(|ch| ch.to_digit(10).unwrap())
                .rev()
                .collect(),
            num2.chars()
                .map(|ch| ch.to_digit(10).unwrap())
                .rev()
                .collect::<Vec<_>>(),
        )
    } else {
        (
            num2.chars()
                .map(|ch| ch.to_digit(10).unwrap())
                .rev()
                .collect::<Vec<_>>(),
            num1.chars()
                .map(|ch| ch.to_digit(10).unwrap())
                .rev()
                .collect::<Vec<_>>(),
        )
    };

    let (mut carry, mut multiply, mut remainder, mut sum) = (0, 0, 0, 0);
    let mut index = 0usize;
    let mut ans = Vec::new();
    for _ in 0..(num1.len() + num2.len()) {
        ans.push(0);
    }

    rhs.iter().enumerate().for_each(|(i, &r)| {
        index = i;
        lhs.iter().for_each(|&l| {
            multiply = r * l;
            carry = multiply / 10;
            remainder = multiply % 10;
            sum = ans[index] + remainder;
            if sum >= 10 {
                ans[index + 1] += sum / 10;
                ans[index] = sum % 10;
                // ans[index + 1] += 1;
                // ans[index] = sum - 10;
            } else {
                ans[index] += remainder;
            }
            // ans[i] += remainder;
            index += 1;
            ans[index] += carry;
        });
    });
    if ans[ans.len() - 1] == 0 {
        ans.pop();
    }
    println!("{:?}", ans);
    // let mut ch;
    let mut strs = String::new();
    ans.iter().rev().for_each(|&dig| {
        strs.push(std::char::from_u32(dig + 48).unwrap());
    });
    strs
}

//          456
//          123
//             1368
//             912
//            10488
