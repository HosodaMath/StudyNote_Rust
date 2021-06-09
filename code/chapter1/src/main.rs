fn main() {
    // chapter0
    // 変数 
    let value1 = 500;
    let value2: i64 = 500;
    println!("型はi32 = {} 型はi64 = {}", value1, value2);
    
    // 変数宣言における決まりごと
    let first_value = 256.0;
    println!("{}", first_value);
    // エラーにはならないが警告が出る。
    // let secondValue = 512.0;
    // println!("{}", secondValue); 
    let second_value = 512.0;
    println!("{}", second_value); 

    // 変数の変更
    // 問題なく動作する
    let value_data1 = 256;
    println!("{}", value_data1);
    // エラーが出る
    // let value_data2 = 256;
    // value_data2 += value_data1;
    // エラーは出ない
    let mut value_data2 = 256;
    value_data2 += value_data1;
    println!("{}", value_data2);

    // 基本的なデータ型
    
    // bool
    let type_bool = true;
    println!("bool = {}", type_bool);
    
    // 符号無し整数型
    let type_u32: u32 = 32;
    let type_u64: u64 = 64;
    println!("u32 = {}, u64 = {}", type_u32, type_u64); 
    // 符号無し整数型の範囲を求める
    let min_u64: u64 = u64::MIN;
    let max_u64: u64 = u64::MAX;
    println!("min = {}, max = {}", min_u64, max_u64);
    
    // 符号付き整数型
    let type_i32: i32 = 32;
    let type_i64: i64 = 64;
    println!("i32 = {}, i64 = {}", type_i32, type_i64); 
    // 符号付き整数型の範囲を求める
    let min_i64: i64 = i64::MIN;
    let max_i64: i64 = i64::MAX;
    println!("min = {}, max = {}", min_i64, max_i64);

    // 浮動小数点型
    let type_f32: f32 = 32.5;
    let type_f64: f64 = 64.5;
    println!("f32 = {}, f64 = {}", type_f32, type_f64); 
    
    // ポインタサイズ型
    let type_usize: usize = 2048;
    let type_isize: isize = 1024;
    println!("usize = {}, isize = {}", type_usize, type_isize); 
    
    // タプル型
    let type_tuple = (8, 16, 32, 64, 128, 256, 512);
    println!("{}", type_tuple.0); // 8
    println!("{}", type_tuple.6); // 512
    // println!("{}", type_tuple.7); // error😔
    // タプル型は異なる型は混在させられる
    let type_different_tuple = ("Hello", 100, 0.33333);
    println!("{}", type_different_tuple.0); // Hello
    println!("{}", type_different_tuple.2); // 0.33333
    
    // 配列型
    let type_array1 = [10, 20, 30];
    println!("{}", type_array1[0]);
    // 型と大きさを指定する
    let type_array2:[f64; 4] = [1.1, 2.2, 3.3, 4.4];
    println!("{}", type_array2[0]); 
    // 配列の値を変更する
    let mut type_array3:[f64; 4] = [1.1, 2.2, 3.3, 4.4];
    println!("{}", type_array3[0]); 
    type_array3[0] = 10.1;
    println!("{}", type_array3[0]); 

    // 文字列型
    let type_str = "Hello";
    println!("{}", type_str);
}
