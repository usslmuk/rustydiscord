use divan::{Bencher, black_box};
use rustydiscord::builders::{
    ActionRowBuilder, ButtonBuilder, CheckboxGroupBuilder, ComponentsVersion, ContainerBuilder,
    EmbedBuilder, FileUploadBuilder, LabelBuilder, MediaGalleryBuilder, ModalBuilder,
    RadioGroupBuilder, SectionBuilder, SeparatorBuilder, SlashCommandBuilder,
    SlashCommandIntegerOption, SlashCommandStringOption, SlashCommandSubcommandBuilder,
    SlashCommandSubcommandGroupBuilder, SlashCommandUserOption, StringSelectMenuBuilder,
    TextInputBuilder, ThumbnailBuilder, embed_length, validate_components, validate_embeds,
};
use rustydiscord::model::{component::Component, emoji::PartialEmoji, util::Locale};

#[global_allocator]
static ALLOC: divan::AllocProfiler = divan::AllocProfiler::system();

fn main() {
    divan::main();
}

fn full_embed() -> EmbedBuilder {
    (0..25).fold(
        EmbedBuilder::new()
            .title("Weekly report — rustydiscord")
            .url("https://example.com/report")
            .description("A summary of what happened this week. ".repeat(20))
            .color(0x58_65F2)
            .author("rustydiscord bot")
            .footer("Generated automatically")
            .image("https://example.com/chart.png")
            .thumbnail("attachment://logo.png")
            .timestamp_now(),
        |embed, index| {
            embed.inline_field(
                format!("Metric #{index}"),
                format!("{index} 🔥 events per second, up {index}% from last week"),
            )
        },
    )
}

fn small_embed() -> EmbedBuilder {
    EmbedBuilder::new()
        .title("Pong!")
        .description("Latency: 42 ms")
        .color(0x57_F287)
}

fn large_command() -> SlashCommandBuilder {
    let subcommand = |group: usize, index: usize| {
        let choices = (0..10).fold(
            SlashCommandStringOption::new("kind", "What kind of entry").required(true),
            |option, choice| option.choice(format!("Kind {choice}"), format!("k{choice}")),
        );
        SlashCommandSubcommandBuilder::new(
            format!("action-{group}-{index}"),
            format!("Runs action {index} of group {group}"),
        )
        .description_localization(
            Locale::Polish,
            format!("Wykonuje akcję {index} grupy {group}"),
        )
        .option(choices)
        .option(SlashCommandUserOption::new("target", "Who it is about").required(true))
        .option(
            SlashCommandIntegerOption::new("amount", "How many")
                .min_value(1)
                .max_value(100),
        )
        .option(SlashCommandStringOption::new("note", "An optional note").max_length(200))
    };
    (0..5).fold(
        SlashCommandBuilder::new("admin", "Administration tools")
            .name_localization(Locale::Polish, "admin")
            .description_localization(Locale::Polish, "Narzędzia administracyjne"),
        |command, group| {
            command.subcommand_group((0..5).fold(
                SlashCommandSubcommandGroupBuilder::new(
                    format!("group-{group}"),
                    format!("Group {group}"),
                ),
                |builder, index| builder.subcommand(subcommand(group, index)),
            ))
        },
    )
}

fn small_command() -> SlashCommandBuilder {
    SlashCommandBuilder::new("ping", "Checks the latency")
}

fn v2_message() -> Vec<Component> {
    let card = (0..5)
        .fold(
            ContainerBuilder::new()
                .accent_color(0x58_65F2)
                .text("## Server settings"),
            |card, index| {
                card.component(
                    SectionBuilder::new(
                        ButtonBuilder::secondary(format!("edit-{index}")).label("Edit"),
                    )
                    .text(format!("**Setting {index}** is on")),
                )
            },
        )
        .component(SeparatorBuilder::new())
        .component((0..4).fold(MediaGalleryBuilder::new(), |gallery, index| {
            gallery.item(format!("https://example.com/{index}.png"))
        }))
        .component((0..5).fold(ActionRowBuilder::new(), |row, index| {
            row.component(ButtonBuilder::primary(format!("page-{index}")).label("Page"))
        }));
    let menu = (0..25).fold(StringSelectMenuBuilder::new("pick"), |menu, index| {
        menu.option(format!("Option {index}"), format!("o{index}"))
    });
    vec![
        card.build_unchecked(),
        ActionRowBuilder::new().component(menu).build_unchecked(),
    ]
}

fn modal() -> ModalBuilder {
    ModalBuilder::new("feedback", "Your feedback")
        .text("Thanks for helping!")
        .label("Topic", TextInputBuilder::short("topic").max_length(80))
        .component(
            LabelBuilder::new(
                "Details",
                TextInputBuilder::paragraph("body").required(false),
            )
            .description("Optional"),
        )
        .label(
            "Rating",
            (0..5).fold(RadioGroupBuilder::new("rating"), |group, index| {
                group.option(format!("{index} stars"), index.to_string())
            }),
        )
        .label("Screenshots", FileUploadBuilder::new("shots").max_values(3))
}

macro_rules! builder_benches {
    ($module:ident, $make:expr) => {
        mod $module {
            use super::*;

            #[divan::bench]
            fn validate(bencher: Bencher<'_, '_>) {
                let builder = $make;
                bencher
                    .with_inputs(|| builder.clone())
                    .bench_values(|builder| builder.build().unwrap());
            }

            #[divan::bench]
            fn unchecked(bencher: Bencher<'_, '_>) {
                let builder = $make;
                bencher
                    .with_inputs(|| builder.clone())
                    .bench_values(|builder| builder.build_unchecked());
            }

            #[divan::bench]
            fn assemble() -> impl Sized {
                black_box($make).build().unwrap()
            }
        }
    };
}

builder_benches!(embed_full, full_embed());
builder_benches!(embed_small, small_embed());
builder_benches!(command_large, large_command());
builder_benches!(command_small, small_command());
builder_benches!(modal_form, modal());
builder_benches!(
    button,
    ButtonBuilder::primary("confirm")
        .label("Confirm")
        .emoji(PartialEmoji::Unicode { name: "✅".into() })
);
builder_benches!(
    section,
    SectionBuilder::new(ThumbnailBuilder::new("attachment://logo.png").description("Logo"))
        .text("## Title")
        .text("Body")
);
builder_benches!(
    string_select,
    (0..25).fold(StringSelectMenuBuilder::new("pick"), |menu, index| {
        menu.option(format!("Option {index}"), format!("option-{index}"))
    })
);
builder_benches!(
    checkbox_group,
    (0..10).fold(CheckboxGroupBuilder::new("topics"), |group, index| {
        group.option(format!("Topic {index}"), index.to_string())
    })
);

mod message {
    use super::{
        ActionRowBuilder, Bencher, ButtonBuilder, ComponentsVersion, black_box, embed_length,
        full_embed, small_embed, v2_message, validate_components, validate_embeds,
    };

    #[divan::bench]
    fn validate_embeds_10_full(bencher: Bencher<'_, '_>) {
        let embeds: Vec<_> = (0..10).map(|_| full_embed().build_unchecked()).collect();
        bencher.bench(|| validate_embeds(black_box(&embeds)).unwrap_err());
    }

    #[divan::bench]
    fn validate_embeds_10_small(bencher: Bencher<'_, '_>) {
        let embeds: Vec<_> = (0..10).map(|_| small_embed().build_unchecked()).collect();
        bencher.bench(|| validate_embeds(black_box(&embeds)).unwrap());
    }

    #[divan::bench]
    fn embed_length_full(bencher: Bencher<'_, '_>) {
        let embed = full_embed().build_unchecked();
        bencher.bench(|| embed_length(black_box(&embed)));
    }

    #[divan::bench]
    fn validate_components_v2(bencher: Bencher<'_, '_>) {
        let components = v2_message();
        bencher
            .bench(|| validate_components(black_box(&components), ComponentsVersion::V2).unwrap());
    }

    #[divan::bench]
    fn validate_components_legacy_25_buttons(bencher: Bencher<'_, '_>) {
        let components: Vec<_> = (0..5)
            .map(|row| {
                (0..5)
                    .fold(ActionRowBuilder::new(), |builder, index| {
                        builder.component(
                            ButtonBuilder::primary(format!("b{row}-{index}")).label("Button"),
                        )
                    })
                    .build_unchecked()
            })
            .collect();
        bencher.bench(|| {
            validate_components(black_box(&components), ComponentsVersion::Legacy).unwrap();
        });
    }

    #[divan::bench]
    fn assemble_v2_message() -> impl Sized {
        let components = v2_message();
        validate_components(black_box(&components), ComponentsVersion::V2).unwrap();
        components
    }
}
