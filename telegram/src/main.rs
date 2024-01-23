use std::fs::OpenOptions;
use std::io::{self, Write};
use teloxide::{
    dispatching::{dialogue, dialogue::InMemStorage, UpdateHandler},
    prelude::*,
    types::{MediaKind, File, MediaText, Message, MessageEntityKind, MessageId, MessageKind, MediaPhoto, ChatId, UserId},
    utils::command::BotCommands,
    net::Download,
};
use tokio::fs;

use reqwest::header::*;

extern crate mime;

use scraper::{Html, Selector};
use url::Url;

type MyDialogue = Dialogue<State, InMemStorage<State>>;
type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;

const BRAIN_LOCATION: &str = "/Users/foursixnine/Library/Mobile Documents/iCloud~md~obsidian/Documents/codex.foursixnine.io/Codex/00-Captured.md";

#[derive(Clone, Default)]
pub enum State {
    #[default]
    Start,
}

// commands

#[derive(BotCommands, Clone)]
#[command(
    rename_rule = "lowercase",
    description = "These commands are supported:"
)]
enum Command {
    // #[command(description = "handle a username.")]
    // Username(String),
    // #[command(description = "handle a username and an age.", parse_with = "split")]
    // UsernameAndAge { username: String, age: u8 },
    #[command(description = "displaly Help this text.")]
    Help,
    #[command(description = "start stufftuff")]
    Start,
    #[command(description = "cancel stuff")]
    Cancel,
    #[command(description = "flower stuff")]
    Flower,
}

#[derive(Clone)]
struct ConfigParameters {
    bot_maintainer: UserId,
}

const PARAMETERS: ConfigParameters = ConfigParameters {
    bot_maintainer: UserId(51739298), // Paste your ID to run this bot.
};

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting dialogue bot...");

    let bot = Bot::from_env();

    Dispatcher::builder(bot, update_handler())
        .dependencies(dptree::deps![InMemStorage::<State>::new()])
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}

fn update_handler() -> UpdateHandler<Box<dyn std::error::Error + Send + Sync + 'static>> {
    use dptree::case;

    let command_handler = teloxide::filter_command::<Command, _>()
        .branch(case![Command::Start].endpoint(start))
        .branch(case![Command::Help].endpoint(help))
        .branch(case![Command::Flower].endpoint(flower))
        .branch(case![Command::Cancel].endpoint(cancel));

    let message_handler = Update::filter_message()
        .branch(command_handler)
        .branch(case![State::Start].endpoint(handle_message));

    dialogue::enter::<Update, InMemStorage<State>, State, _>().branch(message_handler)
}

async fn help(bot: Bot, msg: Message) -> HandlerResult {
    let help_message = format!(
        "help has been invoked, your user id is {}",
        msg.from().expect("User should have id").id
    );
    bot.send_message(msg.chat.id, help_message).await?;
    Ok(())
}

async fn cancel(bot: Bot, msg: Message) -> HandlerResult {
    bot.send_message(msg.chat.id, "cancel has been invoked")
        .await?;
    Ok(())
}

async fn flower(bot: Bot, msg: Message) -> HandlerResult {
    bot.send_message(msg.chat.id, "hi").await?;
    Ok(())
}

async fn start(bot: Bot, _dialogue: MyDialogue, msg: Message) -> HandlerResult {
    bot.send_message(msg.chat.id, "This is the start").await?;
    Ok(())
}

async fn handle_message(bot: Bot, _dialogue: MyDialogue, msg: Message) -> HandlerResult {
    bot.send_message(msg.chat.id, "This is the handle_message")
        .reply_to_message_id(msg.id)
        .await?;

    let _is_owner = |msg: &Message| {
        msg.from()
            .map(|user| {
                // let reply = format!("Your id is {}", user.id);
                user.id == PARAMETERS.bot_maintainer
            })
            .unwrap_or_default()
    };

    if !_is_owner(&msg) {
        return Ok(());
    }
    match msg.kind {
        MessageKind::Common(chat) => {
            match chat.media_kind {
                MediaKind::Text(content) => {
                    handle_text_content(bot, msg.chat.id, msg.id, Some(content)).await?;
                }
                MediaKind::Photo(content) => {
                    bot.send_message(msg.chat.id, "Got Photo!")
                        .reply_to_message_id(msg.id)
                        .await?;
                    handle_photo_content(bot, msg.chat.id, msg.id, Some(content.clone())).await?;
                    log::debug!("{:#?}", content);
                    // to download the photo
                }
                _ => {
                    bot.send_message(msg.chat.id, "Media::Kind Type not implemented")
                        .reply_to_message_id(msg.id)
                        .await?;
                    log::debug!("{:#?} not implemented", chat);
                    // log::debug!("{:#?} not implemented", msg);
                } //todo!(), // todo for media_kind
            }
        }
        // _ => todo!(), //todo for msg kind
        _ => {
            bot.send_message(msg.chat.id, "MessageKind not implemented")
                .reply_to_message_id(msg.id)
                .await?;
        } //todo!(), // todo for media_kind
    };
    // }
    Ok(())
}

async fn handle_photo_content(
    bot: Bot,
    chat_id: ChatId,
    message_id: MessageId,
    message_photo: Option<MediaPhoto>,
) -> HandlerResult {
    bot.send_message(chat_id, "Got Photo!")
        .reply_to_message_id(message_id)
        .await?;
    let content = message_photo.unwrap();
    log::info!("photo: {:#?}", content);
    // log::debug!("object: {:#?}", content);
    let photo = content.photo.last().unwrap();
    let file = bot.get_file(photo.file.id.clone()).await?;
    let file_id = file.path.clone();
    let file_name = file_id.split('/').last().unwrap();
    let file_path = format!("./tmp/tg/{}", file_name);
    log::debug!("file_path: {}", file_path);
    fs::create_dir_all("./tmp/tg/").await?;
    let mut file = fs::File::create(file_path.clone()).await?;
    log::debug!("Debugging file:\n{:#?}", file);
    bot.download_file(&file_id, &mut file).await?;
    let markdown = format!("- ![{}]({})\n", &file_path, &file_path);
    // let markdown = format!(format, text_part, text_url, entity.kind);
    log::debug!("will insert:");
    log::debug!("{}", markdown);
    // log::info!("object: {:#?}", full_url);
    match append_to_brain(&markdown) {
        Ok(()) => {
            bot.send_message(chat_id, "Saved!")
                .reply_to_message_id(message_id)
                .await?
        }
        _ => panic!("{:?}", &markdown),
    };
    Ok(())
}

async fn handle_text_content(
    bot: Bot,
    chat_id: ChatId,
    message_id: MessageId,
    message_text: Option<MediaText>,
) -> HandlerResult {
    bot.send_message(chat_id, "Got text")
        .reply_to_message_id(message_id)
        .await?;

    let content = message_text.unwrap();
    log::info!("text: {}", content.text);
    log::debug!("object: {:#?}", content);

    for entity in content.entities.iter().filter(|e| match Some(&e.kind) {
        Some(MessageEntityKind::Url) => true,
        None => {
            log::info!("{} {}", chat_id, "MessageEntityKind Type not implemented");
            false
        }
        _ => {
            log::info!(
                "{} {}: {:#?}",
                chat_id,
                "MessageEntityKind Type not implemented",
                e.kind
            );
            log::debug!("{:#?} not implemented", e.kind);
            log::debug!("{:#?} not implemented", e);
            false
        } // Some(MessageEntityKind::TextLink) => true,
    }) {
        // TODO: thread 'tokio-runtime-worker' panicked at 'failed trying to parse >: https://thght.works/3vZX6<: RelativeUrlWithoutBase', telegram/src/main.rs:219:40
        let text_url = &content.text[entity.offset..entity.offset + entity.length];
        let text_part = &content.text[0..entity.offset];
        let title_url = match get_website_title(text_url).await {
            Ok(title) => title.to_string(),
            Err(e) => {
                log::debug!("{:?}\n error invoked from {}", e, line!());
                "error in url".to_string()
            }
        };

        let markdown;
        let real_url = &text_url.to_string();
        let real_tex = &content.text.to_string();
        if real_url != real_tex {
            markdown = format!("- {} [{}]({})\n", text_part, title_url.trim(), text_url);
        } else {
            markdown = format!("- [{}]({})\n", title_url.trim(), text_url);
        };
        // let markdown = format!(format, text_part, text_url, entity.kind);
        log::debug!("will insert:");
        log::debug!("{}", markdown);
        // log::info!("object: {:#?}", full_url);
        match append_to_brain(&markdown) {
            Ok(()) => {
                bot.send_message(chat_id, "Saved!")
                    .reply_to_message_id(message_id)
                    .await?
            }
            _ => panic!("{:?}", &markdown),
        };
    }

    log::debug!("found {}", content.entities.len());
    Ok(())
}

fn append_to_brain(text: &str) -> io::Result<()> {
    let mut file = OpenOptions::new()
        .append(true)
        .open(BRAIN_LOCATION)
        .unwrap();
    file.write_all(text.as_bytes())
        .expect("failed to write message");
    Ok(())
}

async fn get_website_title(url: &str) -> Result<String, reqwest::Error> {
    let this_url = match Url::parse(url) {
        Ok(result) => result,
        Err(..) => match Url::parse("foursixnine.io") {
            Ok(result) => result,
            Err(e) => panic!("Can't recover '{:#?}'\nurl:{}", e, url),
        },
    };

    let title;
    let host = this_url.host_str();
    match host {
        Some("onlyfans.com") => {
            title = format!(
                "OF of {}",
                this_url
                    .path_segments()
                    .expect("broken")
                    .collect::<Vec<_>>()[0]
            )
        }
        Some("fansly.com") => {
            title = format!(
                "FSL of {}",
                this_url
                    .path_segments()
                    .expect("broken")
                    .collect::<Vec<_>>()[0]
            )
        }
        Some("instagram.com") => {
            title = format!(
                "OF of {}",
                this_url
                    .path_segments()
                    .expect("broken")
                    .collect::<Vec<_>>()[0]
            )
        }
        Some("twitter.com") | Some("x.com") => {
            title = format!(
                "Tweet from {}",
                this_url
                    .path_segments()
                    .expect("broken")
                    .collect::<Vec<_>>()[0]
            )
        }
        Some("facebook.com") => {
            title = format!(
                "Facebook link {}",
                this_url
                    .path_segments()
                    .expect("broken")
                    .collect::<Vec<_>>()[0]
            )
        }
        _ => {
            // // Convert the response body to a string
            // let body_str = String::from_utf8(body.to_vec())?;
            // Parse the title from the HTML
            // Send GET request to the specified URL
            let response = reqwest::get(url).await?;

            // response.headers();

            // Read the response body as bytes
            let body_str = response.text().await?;
            title = parse_website_title(&body_str);
        }
    }

    Ok(title)
}

// trait From<T> :Sized {
//     fn from(&self) -> Self;
// }

fn mimetype_has_title(_content_type: HeaderValue) -> bool {
    // let mime_type: Mime = content_type;

    // match mime_type {
    //     Some(mime::TEXT_HTML) => true,
    //     _ => false,
    //     None => panic!("No type found for: {:#?}", content_type),
    // }
    todo!("Not implemented");
}

fn parse_website_title(html: &str) -> String {
    // let document = Html::parse_document(&html);
    // let selector = Selector::parse("title");
    // Extract the title from the HTML using simple string manipulation
    let document = Html::parse_document(html);
    let title_selector = Selector::parse("title").unwrap();
    let title_text = document
        .select(&title_selector)
        .next()
        .map(|x| x.inner_html());

    let binding = title_text.expect("No title found in HTML document");
    let title_string = binding.trim();
    log::debug!(
        "Looks like a title was found, but site had to be parsed: {}",
        line!()
    );
    title_string.to_string()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[tokio::test]
    pub async fn test_parse_website_title() {
        // Test case 1: HTML with a valid title tag
        let html1 = "<html><title> Test Title   </title></html>";
        let title1 = parse_website_title(html1);
        assert_eq!(title1, "Test Title");
    }

    #[tokio::test]
    #[should_panic]
    pub async fn test_parse_website_title_errors() {
        // Test case 2: HTML without a title tag
        let html2 = "<html><h1>Test Heading</h1></html>";
        let title2 = parse_website_title(html2);
        assert_eq!(title2, "");

        // Test case 3: Empty HTML
        let html3 = "";
        let title3 = parse_website_title(html3);
        assert_eq!(title3, "");
    }

    #[tokio::test]
    pub async fn test_get_website_title() {
        let mut _url = "https://onlyFans.com/pepe";
        let mut the_response = get_website_title(_url).await.unwrap();
        let mut expected = "OF of pepe";
        assert_eq!(expected, the_response);


        _url = "https//fansly.com/happyhooha/posts";
        the_response = get_website_title(_url).await.unwrap();
        expected = "FSL of happyhooha";

        _url = "https://twitter.com/foursixnine/status/1074685619618623490?s=20";
        the_response = get_website_title(_url).await.unwrap();
        expected = "Tweet from foursixnine";
        assert_eq!(expected, the_response);
    }

    #[tokio::test]
    #[should_panic]
    pub async fn test_get_website_title_invalid_url() {
        let _url = "g https://twitter.com/aakashg0/status/1666962728055889920?s=52&t=scqAqSz4d-mfoKQNQq-fv";
        let the_response = get_website_title(_url).await.unwrap();
        let expected = "Broken link";
        assert_eq!(expected, the_response);
    }

    // #[test]
    // fn test_process_header_mimetype() {
    //     // let mut map = HeaderMap::new();
    //     let current_mime = HeaderValue::from_static(&mime::TEXT_HTML.to_string());
    //     // map.append(CONTENT_TYPE, current_mime);
    //     // assert_eq!(map.get(CONTENT_TYPE).unwrap(), mime::TEXT_HTML.to_string())
    //     let result = mimetype_has_title(current_mime);
    //     //assert!(result, true);
    // }

    //
    // * “One good test is worth a thousand expert opinions.” \n
    // * – Wernher von Braun @ twitter https://test.com
    // *
    // * thght.works/3ghJZ9t => problem
    // thread 'tokio-runtime-worker' panicked at 'failed trying to parse >: https://thght.works/3vZX6<: RelativeUrlWithoutBase', telegram/src/main.rs:219:40
    //
    // // This has to be converted to a json object
	// DEBUG telegram                          > object: MediaText {
	//    text: "Santiago Zarate, [Jul 8, 2023 at 20:32]\nhttps://www.reddit.com/user/Remarkable-Goat-973/",
	//    entities: [
	//        MessageEntity {
	//            kind: Url,
	//            offset: 40,
	//            length: 48,
	//        },
	//    ],
	//}
    // * */
}
