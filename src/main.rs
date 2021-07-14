use std::collections::HashMap;
fn main() {
    // 整数のリストが与えられ、ベクタを使ってmean(平均値)、
    // median(ソートされた時に真ん中に来る値)、
    // mode(最も頻繁に出現する値; ハッシュマップがここでは有効活用できるでしょう)
    // を返してください。
    let list = vec![1, 30, 13, 20, 21, 1];

    println!("List: {:?}", list);
    println!("Mean: {}", get_mean(&list));
    println!("Median: {}", get_median(&list));
    println!("Mode: {:?}", get_mode(&list));
}

fn get_mean(list: &Vec<i32>) -> i32 {
    // listをforで回して、sumを求めて、個数で割る
    let mut sum = 0;
    for i in list {
        sum += i;
    }
    sum / list.len() as i32
}

fn get_median(list: &Vec<i32>) -> i32 {
    let mut sort_list = list.clone();
    sort_list.sort();
    // println!("sorted_list: {:?}", sort_list);

    let len = list.len() as i32;
    // println!("len: {}", len);

    if let 0 = len % 2 {
        (sort_list[(len as usize / 2) - 1] + sort_list[len as usize / 2]) / 2 // Even
    } else {
        sort_list[len as usize / 2] // Odd
    }
}

fn get_mode(list: &Vec<i32>) -> Vec<i32> {
    let mut map = HashMap::new();
    for i in list {
        let count = map.entry(i).or_insert(0);
        *count += 1;
    }
    // println!("{:?}", map);

    let mut max_numbers: Vec<i32> = Vec::new();
    let mut max_count = 0;
    for (number, count) in &map {
        if max_count < *count {
            // 最大値の更新
            max_count = *count;
            max_numbers.clear();
            max_numbers.push(**number);
        } else if max_count == *count {
            // 同数があった場合
            max_numbers.push(**number);
        }
    }
    max_numbers
}
