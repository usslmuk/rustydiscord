use rustydiscord::builders::{
    ButtonBuilder, EmbedBuilder, ModalBuilder, SlashCommandBuilder, TextInputBuilder,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let command = SlashCommandBuilder::new("ping", "Replies with Pong!").build()?;
    println!("Command: {command:?}");

    let embed = EmbedBuilder::new()
        .title("Report")
        .description("All systems normal.")
        .inline_field("Uptime", "99.9%")
        .footer("Checked just now")
        .build()?;
    println!("Embed: {embed:?}");

    let button = ButtonBuilder::primary("poke").label("Poke").build()?;
    println!("Button: {button:?}");

    let modal = ModalBuilder::new("feedback", "Send feedback")
        .component(TextInputBuilder::short("text").required(true))
        .build()?;
    println!("Modal: {modal:?}");
    Ok(())
}
