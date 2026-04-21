use rand::{Rng, thread_rng};
use std::io;

fn main() {
    // 生成一个随机数组
    let num_arr: [i32; 5] = std::array::from_fn(|_| thread_rng().gen_range(1..=100));
    println!("随机数组 {:?}", num_arr);
    println!("请输入一个索引值");
    loop {
        let mut index = String::new();
        io::stdin()
            .read_line(&mut index)
            .expect("读取输入失败");

        let index: usize = match index.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };
        if index > num_arr.len() {
            println!("请输入一个正确的索引值，范围：[0, {}]", num_arr.len());
            continue;
        }
        let element = num_arr[index];
        println!("索引的值是：{}", element);
        break;
    }
}
