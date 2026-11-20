use rustydiscord::formatters::{
    TimestampStyle, escape_markdown, time, user_mention,
};
use rustydiscord::model::id::UserId;
use rustydiscord::model::util::Timestamp;

fn main() {
    let when = Timestamp::parse("2026-11-11T12:00:00.000000+00:00").unwrap();
    let relative = time(when).style(TimestampStyle::RelativeTime);
    println!("Happened {relative}");

    let mention = user_mention(UserId::new(1));
    println!("Ping {mention} without pinging anyone by accident");

    let raw = escape_markdown("**not bold**, *not italic*");
    println!("Escaped: {raw}");
}
