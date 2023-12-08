
fn main() -> std::io::Result<()> {
    let input = include_str!("../input1.txt");

    let calories = input.lines().collect::<Vec<_>>();

    let mut total = calories.as_slice().split(|x| x == &"")
                                    .map(|arr| arr.iter().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>())
                                    .map(|arr| arr.iter().sum::<i32>())
                                    .collect::<Vec<i32>>();
    total.sort();
    total.reverse();
    total.resize(3, 0);

    println!("Max = {}", total[0]);
    println!("Top 3 : {:?}", total);
    println!("Top 3 total : {:?}", total.iter().sum::<i32>());

    Ok(())
}
