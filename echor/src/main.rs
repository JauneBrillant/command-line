use clap::Parser;

// #[]の記法は属性マクロと言われる

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Echor {
    // `-n`オプションを定義（例: `-n`と指定すると、このフィールドは`true`になる）
    #[clap(short)]
    n: bool,

    // 残りのコマンドライン引数を受け取る（例: `echo Hello World` ならば、`ehcor.arguments`は`["Hello", "World"]`になる）
    #[clap()]
    arguments: Vec<String>,
}

fn main() {
    let echor = Echor::parse(); // コマンドライン引数を解析して、Echor構造体にマッピング

    let output = echor.arguments.join(" ");
    let endline = if echor.n { "" } else { "\n" };
    print!("{}{}", output, endline)
}
