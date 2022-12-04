use std::fs;

fn main() {
    let contents = fs::read_to_string("./src/input.txt")
        .expect("Should be able to read input");
    let split: Vec<String> = contents.split('\n').map(|s| s.to_string()).collect();

    let mut cals: Vec<i32> = Vec::new();
    let mut curr_cal = 0;
    for s in &split {
        if s.is_empty() {
            cals.push(curr_cal);
            curr_cal = 0;
        } else {
            let cal = s.parse::<i32>().unwrap();
            curr_cal += cal;
        }
    }

    cals.sort();
    cals.reverse();

    let mut top_3 = 0;
    (0..3).for_each(|i| {
        top_3 += cals[i];
    });

    println!("{}", top_3);
}
