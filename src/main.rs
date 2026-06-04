use std::env;
use teloxide::{prelude::*, types::InputFile};

#[tokio::main]
async fn main() {
    let bot = teloxide::Bot::from_env();

    let id = env::var("OWNER_CHAT_ID")
        .unwrap_or_default()
        .parse::<i64>()
        .unwrap_or_default();

    teloxide::repl(bot.clone(), move |bot: Bot, msg: Message| async move {
        if msg.chat.id == ChatId(id) {
            match msg.text() {
                Some(text) => {
                    let _ = bot
                        .send_message(msg.chat.id, format!("Kamu kirim pesan text! {}", text))
                        .await;
                }
                _ => {}
            }

            bot.send_dice(msg.chat.id).await?;
        } else {
            let audio_file = InputFile::file("assets/voices/Belum_Terdaftar.mp3");
            let _ = bot
                .send_voice(msg.chat.id, audio_file)
                .caption("Pesan dari Tarzan!")
                .await?;
        }

        Ok(())
    })
    .await;
}
