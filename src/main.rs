use std::path::{Path, PathBuf};

use clap::{command, Parser};
use wallpaper::{DesktopWallpaper, DesktopWallpaperPosition, Monitor};

/// Windows上でデスクトップの壁紙を管理します
#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
enum Args {
    /// 現在の壁紙を取得します
    Get {
        // TODO: `value_parser`を使用します。参照: https://docs.rs/clap/latest/clap/_derive/_tutorial/index.html#validated-values
		// これは、`clap`マクロを使用してコマンドライン引数のデフォルト値を設定しています。`short`と`long`は引数の短い形式と長い形式を指定し、`default_value_t`はデフォルト値を指定しています。
        /// モニターのインデックス（0から開始）
        #[arg(short, long, default_value_t = 0)]
        monitor: usize,
    },

    /// 現在の壁紙を設定します
    #[command(arg_required_else_help = true)] // この記述は、`clap`マクロを使用してコマンドライン引数が必須であることを示しています。
    Set {
        /// 壁紙へのパス
        path: PathBuf,
        /// デスクトップ壁紙の表示方法を指定します
        #[arg(
            short,
            long,
            default_value_t = DesktopWallpaperPosition::Span,
            value_enum
        )]
        scale: DesktopWallpaperPosition,
        /// モニターのインデックス（0から開始）
        #[arg(short, long, default_value_t = 0)]
        monitor: usize,
    },
}

/// 選択されたモニターが範囲内にある場合に処理を実行します
fn if_chosen_monitor_within_range(
    // 選択されたモニターのインデックス
    choice: usize,
    // 利用可能なモニターのリスト
    monitors: &[Monitor],
    // モニターが選択範囲内にある場合に実行する関数
    func: impl FnOnce(&Monitor) -> Result<(), String>,
) -> Result<(), String> {
    match monitors.get(choice) {
        Some(m) => func(m),
        None => Err(format!(
			// "利用可能なモニターは0から{}ですが、{choice}が指定されました"
            "The available monitors are from 0 - {} but {choice} was given",
            monitors.len() - 1,
        )),
    }
}

fn main() -> Result<(), String> {
	// `DesktopWallpaper`の新しいインスタンスを作成し、エラーが発生した場合にエラーメッセージを文字列に変換しています。
    let mut wallpaper = DesktopWallpaper::new().map_err(|e| e.to_string())?;

    let monitors = wallpaper
        .get_monitors()
		// "利用可能なモニターの取得に失敗しました: {error}"
        .map_err(|error| format!("Failed to retrieve available monitors: {error}"))?;

    let args = Args::parse();

    // 引数に応じて処理を分岐
    match args {
        // Getコマンドの場合
        Args::Get { monitor } => if_chosen_monitor_within_range(monitor, &monitors, |monitor| {
            // モニターが選択された場合の処理
            Ok(println!("{}", wallpaper.get_wallpaper(monitor)?.display()))
        })?,
        // Setコマンドの場合
        Args::Set {
            monitor,
            scale,
            path,
        } => if_chosen_monitor_within_range(monitor, &monitors, |monitor| {
            // モニターが選択された場合の処理
            wallpaper
                .set_wallpaper(monitor, Path::new(&path), scale)
				// デスクトップ壁紙の設定に失敗しました: {error}
                .map_err(|error| format!("Failed to set the desktop wallpaper: {error}"))
        })?,
    }

    Ok(())
}
