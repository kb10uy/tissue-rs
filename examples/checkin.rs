//! チェックインのサンプル
//! tissue-rs は非同期ランタイムとして async-std を採用している。

use async_std::task;
use chrono::Local;
use chrono_tz::Asia::Tokyo;
use serde_json::to_string_pretty;
use tissue_rs::{CheckinBuilder, CheckinResponse, IncomingEndpoint};

const WEBHOOK_ID: &'static str = "Tissue Webhook ID";

async fn run() {
    let client = IncomingEndpoint::new(WEBHOOK_ID);

    // `chrono::Local` はタイムゾーンを正しく取得できないことがあるので、
    // `chtono_td` で直接指定する
    let now = Local::now().with_timezone(&Tokyo);
    let mut checkin_builder = CheckinBuilder::with_datetime(now.into());
    checkin_builder.link("https://shibafu528.info").unwrap();
    checkin_builder.note("cyan.png").unwrap();
    checkin_builder.is_private(true);
    checkin_builder.is_too_sensitive(true);

    let checkin = checkin_builder.build();
    println!("Checkin JSON:");
    println!("{}", to_string_pretty(&checkin).unwrap());

    // Ok はあくまで Tissue にリクエストが到達してレスポンスを受け取ったということなので、
    // チェックインが実際に成功したかどうかは `CheckinResponse::Success` を確認しなければならない
    let response = client.send_checkin(&checkin).await;
    match response {
        // チェックイン成功
        Ok(CheckinResponse::Success(received)) => {
            println!("Checkin sent successfully!");
            println!("{:?}", received);
        }

        // バリデーションエラー(主に時刻被り)
        Ok(CheckinResponse::ValidationError(violations)) => {
            eprintln!("Checkin failed for validations!");
            for violation in violations {
                eprintln!("* {}", violation);
            }
        }

        // その他のエラー
        Ok(CheckinResponse::OtherError(e)) => {
            eprintln!("Checkin failed for some errors!");
            eprintln!("* {}", e);
        }

        // Tissue 外のエラー
        Err(e) => eprintln!("Unexpected error: {}", e),
    }
}

fn main() {
    task::block_on(run());
}
