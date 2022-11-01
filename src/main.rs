use car_checker::regulations::Regulations;
use std::collections::HashMap;
use serenity::async_trait;
use serenity::prelude::*;
use serenity::model::prelude::Ready;
use serenity::model::channel::Message;
use serenity::framework::standard::macros::{command, group, hook};
use serenity::framework::standard::{StandardFramework, CommandResult, DispatchError};

mod leaderboard;
use leaderboard::Leaderboard;

#[group]
#[required_permissions("MANAGE_ROLES")]
#[commands(ping, create_leaderboard_post, add_win, remove_win, add_podium, remove_podium, refresh_leaderboard)]
struct General;

#[group]
#[commands(submit_group_c)]
struct Open;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[hook]
async fn dispatch_error(ctx: &Context, msg: &Message, error: DispatchError, _command_name: &str) {
    if let DispatchError::Ratelimited(info) = error {
        // We notify them only once.
        if info.is_first_try {
            let _ = msg
                .channel_id
                .say(&ctx.http, &format!("Try this again in {} seconds.", info.as_secs()))
                .await;
        }
    } else {
        let _ = msg.channel_id.say(ctx, &format!("Error occured: {:?}", error)).await;
    }
}

#[tokio::main]
async fn main() {
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("!")) // set the bot's prefix to "!"
        .on_dispatch_error(dispatch_error)
        .group(&GENERAL_GROUP)
        .group(&OPEN_GROUP);

    // Login with a bot token from the environment
    let token = include_str!("../token.txt").trim();
    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    // start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Pong!").await?;
    Ok(())
}

#[command]
async fn create_leaderboard_post(ctx: &Context, msg: &Message) -> CommandResult {
    let msg = msg.channel_id.say(ctx, "Leaderboard placeholder message!").await?;
    println!("Loading leaderboard...");
    let mut leaderboard: Leaderboard = serde_json::from_str(&std::fs::read_to_string("leaderboard.json")?)?;
    leaderboard.channel_id = *msg.channel_id.as_u64();
    leaderboard.message_id = *msg.id.as_u64();
    // Shitty hack
    if leaderboard.leaderboard.is_none() {
        leaderboard.leaderboard = Some(HashMap::new());
    }
    println!("Leaderboard updated...");
    std::fs::write("leaderboard.json", serde_json::to_string(&leaderboard)?)?;
    println!("Made leaderboard post!");
    Ok(())
}

#[command]
async fn add_win(ctx: &Context, msg: &Message) -> CommandResult {
    let players_won = &msg.mentions;
    println!("Loading leaderboard...");
    let mut leaderboard: Leaderboard = serde_json::from_str(&std::fs::read_to_string("leaderboard.json")?)?;
    if leaderboard.leaderboard.is_none() {
        leaderboard.leaderboard = Some(HashMap::new());
    }
    if let Some(ref mut lb) = leaderboard.leaderboard {
        for player in players_won {
            if let Some((wins, _podiums)) = lb.get_mut(player.id.as_u64()) {
                *wins += 1;
            } else {
                lb.insert(*player.id.as_u64(), (1, 0));
            }
        }
    }
    println!("Leaderboard updated...");
    std::fs::write("leaderboard.json", serde_json::to_string(&leaderboard)?)?;
    println!("Made leaderboard post!");

    // Update leaderboard post
    let lb_channel_id = leaderboard.channel_id;
    let lb_post_id = leaderboard.message_id;
    let mut lb_msg = ctx.http.get_message(lb_channel_id, lb_post_id).await?;
    let content = leaderboard.get_formatted(&ctx.http).await;
    lb_msg.edit(ctx, |m| m.content(content)).await?;
    println!("Message edited!");

    Ok(())
}

#[command]
async fn remove_win(ctx: &Context, msg: &Message) -> CommandResult {
    let players_corrected = &msg.mentions;
    println!("Loading leaderboard...");
    let mut leaderboard: Leaderboard = serde_json::from_str(&std::fs::read_to_string("leaderboard.json")?)?;
    if leaderboard.leaderboard.is_none() {
        leaderboard.leaderboard = Some(HashMap::new());
    }
    if let Some(ref mut lb) = leaderboard.leaderboard {
        for player in players_corrected {
            if let Some((wins, _podiums)) = lb.get_mut(player.id.as_u64()) {
                if *wins > 0 { *wins -= 1; }
            }
        }
    }
    println!("Leaderboard updated...");
    std::fs::write("leaderboard.json", serde_json::to_string(&leaderboard)?)?;
    println!("Made leaderboard post!");

    // Update leaderboard post
    let lb_channel_id = leaderboard.channel_id;
    let lb_post_id = leaderboard.message_id;
    let mut lb_msg = ctx.http.get_message(lb_channel_id, lb_post_id).await?;
    let content = leaderboard.get_formatted(&ctx.http).await;
    lb_msg.edit(ctx, |m| m.content(content)).await?;
    println!("Message edited!");
    Ok(())
}

#[command]
async fn add_podium(ctx: &Context, msg: &Message) -> CommandResult {
    let players_won = &msg.mentions;
    println!("Loading leaderboard...");
    let mut leaderboard: Leaderboard = serde_json::from_str(&std::fs::read_to_string("leaderboard.json")?)?;
    if leaderboard.leaderboard.is_none() {
        leaderboard.leaderboard = Some(HashMap::new());
    }
    if let Some(ref mut lb) = leaderboard.leaderboard {
        for player in players_won {
            if let Some((_wins, podiums)) = lb.get_mut(player.id.as_u64()) {
                *podiums += 1;
            } else {
                lb.insert(*player.id.as_u64(), (0, 1));
            }
        }
    }
    println!("Leaderboard updated...");
    std::fs::write("leaderboard.json", serde_json::to_string(&leaderboard)?)?;
    println!("Made leaderboard post!");

    // Update leaderboard post
    let lb_channel_id = leaderboard.channel_id;
    let lb_post_id = leaderboard.message_id;
    let mut lb_msg = ctx.http.get_message(lb_channel_id, lb_post_id).await?;
    let content = leaderboard.get_formatted(&ctx.http).await;
    lb_msg.edit(ctx, |m| m.content(content)).await?;
    println!("Message edited!");

    Ok(())
}

#[command]
async fn remove_podium(ctx: &Context, msg: &Message) -> CommandResult {
    let players_corrected = &msg.mentions;
    println!("Loading leaderboard...");
    let mut leaderboard: Leaderboard = serde_json::from_str(&std::fs::read_to_string("leaderboard.json")?)?;
    if leaderboard.leaderboard.is_none() {
        leaderboard.leaderboard = Some(HashMap::new());
    }
    if let Some(ref mut lb) = leaderboard.leaderboard {
        for player in players_corrected {
            if let Some((_wins, podiums)) = lb.get_mut(player.id.as_u64()) {
                if *podiums > 0 { *podiums -= 1; }
            }
        }
    }
    println!("Leaderboard updated...");
    std::fs::write("leaderboard.json", serde_json::to_string(&leaderboard)?)?;
    println!("Made leaderboard post!");

    // Update leaderboard post
    let lb_channel_id = leaderboard.channel_id;
    let lb_post_id = leaderboard.message_id;
    let mut lb_msg = ctx.http.get_message(lb_channel_id, lb_post_id).await?;
    let content = leaderboard.get_formatted(&ctx.http).await;
    lb_msg.edit(ctx, |m| m.content(content)).await?;
    println!("Message edited!");
    Ok(())
}

#[command]
async fn refresh_leaderboard(ctx: &Context, _msg: &Message) -> CommandResult {
    let leaderboard: Leaderboard = serde_json::from_str(&std::fs::read_to_string("leaderboard.json")?)?;
    let lb_channel_id = leaderboard.channel_id;
    let lb_post_id = leaderboard.message_id;
    let mut lb_msg = ctx.http.get_message(lb_channel_id, lb_post_id).await?;
    let content = leaderboard.get_formatted(&ctx.http).await;
    lb_msg.edit(ctx, |m| m.content(content)).await?;
    println!("Message edited!");
    Ok(())
}

#[command]
async fn submit_group_c(ctx: &Context, msg: &Message) -> CommandResult {
    if msg.attachments.len() != 1 {
        msg.reply(ctx, "Attach 1 file! No more, no less. If your zip submission is too big, get in contact with <@183315569745985545> for now.").await?;
        return Ok(());
    }

    if let Ok(zip_bytes) = msg.attachments[0].download().await {
        let mut zip_name = format!("csv/{}", msg.attachments[0].filename.clone());
        zip_name.pop();
        zip_name.pop();
        zip_name.pop();
        zip_name.pop();
        let mut zip_reader = zip::ZipArchive::new(std::io::Cursor::new(zip_bytes.clone()))?;
        zip_reader.extract(&zip_name)?;
        msg.reply(ctx, "Zip file extracted!").await?;

        // Find CSV file
        let mut csv_path = None;
        'search: for entry in std::fs::read_dir(&zip_name)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                // CSV file might be in this folder
                for entry in std::fs::read_dir(path)? {
                    let entry = entry?;
                    let path = entry.path();
                    if path.extension().map(|os_str| os_str.to_str().unwrap_or("")) == Some("csv") {
                        println!("Located csv file!");
                        csv_path = Some(path);
                        break 'search;
                    }
                }
            } else {
                // CSV file is directly in ZIP file
                if path.extension().map(|os_str| os_str.to_str().unwrap_or("WAH")) == Some("csv") {
                    println!("Located csv file!");
                    csv_path = Some(path);
                    break 'search;
                }
            }
        }
        if csv_path.is_none() {
            msg.reply(ctx, "Failed to find CSV file in your zip! Did you send the right zip file?").await?;
        }

        let csv_path = csv_path.unwrap();
        if let Ok(car_data) = car_checker::from_utf16_file(csv_path.to_str().unwrap_or("WAH")) {
            let result = car_checker::regulations::mcs_s1_group_c::MCS_S1_Group_C::default().check(car_data);
            match result {
                Ok(_) => {
                    if let Err(_) = std::fs::write(format!("registered/group_c/{}", msg.attachments[0].filename.clone()), zip_bytes) {
                        msg.reply(ctx, "Seems like your car is good to go! Something went wrong while registering however. Please ping any of the EMs for this series").await?;
                    } else {
                        msg.reply(ctx, "Seems like your car is good to go! Registered it for the next event, feel free to send in new versions whenever you want!").await?;
                    }
                },
                Err(e) => { msg.reply(ctx, format!("Your car seems to break the regulations. For now, I can only show you the first issue encountered, which is `{}`\nThis version has not been saved for the event.", e)).await?; }
            };
        } else {
            msg.reply(ctx, "Something went wrong with loading the file!").await?;
        }

        std::fs::remove_dir_all(&zip_name)?;
    } else {
        msg.reply(ctx, "Error encountered while downloading file!").await?;
    }

    Ok(())
}
