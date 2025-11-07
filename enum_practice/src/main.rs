// // 問題1.信号機の色を表すenumと、その色に応じて動作を指示する関数を作成してください。
// // 1.Red、Green、Yellowの3つのバリアント（状態）を持つTrafficLightというenumを定義してください。
// enum TrafficLight {
//     Red,
//     Green,
//     Yellow,
// }

// // 2. TrafficLightの参照を引数に取り、その色に応じた動作（文字列スライス &str）を返すget_actionという関数をmatch文を使って定義してください。
// // Redなら "Stop!"
// // Greenなら "Go!"
// // Yellowなら "Caution!"
// fn get_action(traffic_light: &TrafficLight) -> &str {
//     match traffic_light {
//         TrafficLight::Red => "Stop!",
//         TrafficLight::Green => "Go!",
//         TrafficLight::Yellow => "Caution!",
//     }
// }

// fn main() {
//     let red = TrafficLight::Red;
//     let green = TrafficLight::Green;
//     let yellow = TrafficLight::Yellow;

//     println!("Red light: {}", get_action(&red));
//     println!("Green light: {}", get_action(&green));
//     println!("Yellow light: {}", get_action(&yellow));
// }

// // 問題2.Option<i32>を受け取り、値がある場合だけ中身を1増やす関数を作成してください。
// // 1.Option<i32>型の値を引数xとして受け取るplus_oneという関数を定義してください。
// // 2.この関数はOption<i32>型を返します。
// // 3.match文を使い、引数xがSome(i)（iは中の数値）であればSome(i + 1)を返します。
// // 4.引数xがNoneであればNoneをそのまま返します。

// fn plus_one(x: Option<i32>) -> Option<i32> {
//     match x {
//         None => None,
//         Some(i) => Some(i + 1),
//     }
// }

// fn main() {
//     let five = Some(5);
//     let six = plus_one(five);
//     let none = plus_one(None);

//     println!("five becomes: {:?}", six);
//     println!("none becomes: {:?}", none);
// }

// 問題3.ユーザーのステータスを表すenumから、特定のステータスの場合だけメッセージを表示してください。
// 1. UserStatusというenumを定義してください。
// 2. 以下のバリアントを持たせてください。
// Active: データなし
// Inactive: データなし
// Banned(String): String型で「BANされた理由」を保持する
// main関数の中で、UserStatus::Banned("Spamming".to_string())というインスタンスを作成してください。
// if let構文を使い、インスタンスがBannedバリアントの場合にのみ、"User banned! Reason: [理由]"とコンソールに出力してください。（ActiveやInactiveの場合は何も表示しません）

// 1-2. UserStatus enumをここに定義
#[derive(Debug)]
enum UserStatus {
    Active,
    Inactive,
    Banned(String),
}

fn main() {
    // 3. Bannedインスタンスを作成
    let status = UserStatus::Banned("Spamming".to_string());

    // 4. if let を使ってBannedの場合だけ理由を出力
    if let UserStatus::Banned(reason) = status {
        println!("User banned! Reason: {:?}", reason);
    } else {
        println!("User is not banned.");
    }
}
