extern crate dotenv;

use std::{fs, io::Read};

use dotenv::dotenv;
use image::Luma;
use qrcode::QrCode;
use teloxide::{prelude::*, types::InputFile};
use uuid::Uuid;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let bot = Bot::from_env();

    teloxide::repl(bot, |bot: Bot, msg: Message| async move {
        let txt = msg.text().unwrap();

        let code = QrCode::new(txt).unwrap();

        let image = code.render::<Luma<u8>>().build();
        let name = Uuid::new_v4().to_string() + ".png";
        image.save(name.to_owned()).unwrap();
        bot.send_photo(msg.chat.id, InputFile::file(name.to_owned()))
            .await
            .unwrap();
        fs::remove_file(name).unwrap();

        Ok(())
    })
    .await
}
