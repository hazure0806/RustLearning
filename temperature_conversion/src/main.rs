use std::io;

fn main() {
    println!("温度を入力してください:");

    let mut temperature_str = String::new();
    io::stdin()
        .read_line(&mut temperature_str)
        .expect("正しい値を入力してください");

    let temperature: f64 = temperature_str.trim().parse().expect("数値ではありません");

    println!("変換後の単位を入力してください（FまたはC）:");
    let mut unit_str = String::new();
    io::stdin()
        .read_line(&mut unit_str)
        .expect("正しい値を入力してください");

    let converted_temperature;

    if unit_str.trim() == "C" {
        // 華氏から摂氏へ
        converted_temperature = (temperature - 32.0) * 5.0 / 9.0;
    } else if unit_str.trim() == "F" {
        // 摂氏から華氏へ
        converted_temperature = temperature * 9.0 / 5.0 + 32.0;
    } else {
        // 単位が不正な場合のエラーハンドリング
        eprintln!("入力された単位が正しくありません。FまたはCを入力してください。");
        return;
    }

    println!("変換後の温度: {}", converted_temperature);
}