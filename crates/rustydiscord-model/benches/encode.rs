use divan::{Bencher, black_box, counter::BytesCount};
use rustydiscord_model::{
    id::{ChannelId, UserId},
    message::Embed,
    rest::{
        CreateGuildScheduledEvent, CreateMessage, FileUpload, ModifyGuildScheduledEvent,
        RecurrenceRuleData,
    },
    util::{ImageData, Patch, SoundData, Timestamp},
};

#[global_allocator]
static ALLOC: divan::AllocProfiler = divan::AllocProfiler::system();

fn main() {
    divan::main();
}

fn png(len: usize) -> Vec<u8> {
    let mut data = b"\x89PNG\r\n\x1a\n".to_vec();
    data.resize(len, 0x5a);
    data
}

fn ogg(len: usize) -> Vec<u8> {
    let mut data = b"OggS\0\x02".to_vec();
    data.resize(len, 0x5a);
    data
}

mod data_uri {
    use super::{Bencher, BytesCount, ImageData, SoundData, ogg, png};

    #[divan::bench]
    fn image_256kib_to_json(bencher: Bencher<'_, '_>) {
        let image = ImageData::new(png(256 * 1024)).unwrap();
        bencher
            .counter(BytesCount::new(256 * 1024_usize))
            .bench(|| serde_json::to_vec(&image).unwrap());
    }

    #[divan::bench(sample_count = 20)]
    fn image_10mib_to_json(bencher: Bencher<'_, '_>) {
        let image = ImageData::new(png(10 * 1024 * 1024)).unwrap();
        bencher
            .counter(BytesCount::new(10 * 1024 * 1024_usize))
            .bench(|| serde_json::to_vec(&image).unwrap());
    }

    #[divan::bench]
    fn sound_512kib_to_json(bencher: Bencher<'_, '_>) {
        let sound = SoundData::new(ogg(512 * 1024)).unwrap();
        bencher
            .counter(BytesCount::new(512 * 1024_usize))
            .bench(|| serde_json::to_vec(&sound).unwrap());
    }
}

mod bodies {
    use super::{
        ChannelId, CreateGuildScheduledEvent, CreateMessage, Embed, ModifyGuildScheduledEvent,
        Patch, RecurrenceRuleData, Timestamp, black_box,
    };

    #[divan::bench]
    fn create_message_text_to_json() -> Vec<u8> {
        serde_json::to_vec(black_box(&CreateMessage::from("Pong!"))).unwrap()
    }

    #[divan::bench]
    fn create_message_with_embed_to_json(bencher: divan::Bencher<'_, '_>) {
        let mut embed = Embed::default();
        embed.title = Some("Release notes".into());
        embed.description = Some(
            "A longer description of what changed. "
                .repeat(20)
                .as_str()
                .into(),
        );
        embed.timestamp = Some(Timestamp::from_unix_micros(1_794_339_727_123_000));
        let mut body = CreateMessage::from("New version!");
        body.embeds = vec![embed; 3];
        bencher.bench(|| serde_json::to_vec(black_box(&body)).unwrap());
    }

    #[divan::bench]
    fn create_scheduled_event_to_json(bencher: divan::Bencher<'_, '_>) {
        let start = Timestamp::from_unix_micros(1_794_339_727_123_000);
        let mut body = CreateGuildScheduledEvent::stage(ChannelId::new(1), "Office hours", start);
        body.recurrence_rule = Some(RecurrenceRuleData::weekly(
            start,
            rustydiscord_model::guild_scheduled_event::RecurrenceWeekday::Thursday,
        ));
        bencher.bench(|| serde_json::to_vec(black_box(&body)).unwrap());
    }

    #[divan::bench]
    fn modify_with_patches_to_json(bencher: divan::Bencher<'_, '_>) {
        let mut body = ModifyGuildScheduledEvent::default();
        body.description = Patch::Clear;
        body.name = Some("Renamed".into());
        body.scheduled_start_time = Some(Timestamp::from_unix_micros(1_794_339_727_123_000));
        bencher.bench(|| serde_json::to_vec(black_box(&body)).unwrap());
    }
}

mod files {
    use super::{Bencher, FileUpload, UserId, black_box};

    #[divan::bench]
    fn target_users_10k(bencher: Bencher<'_, '_>) {
        let ids: Vec<UserId> = (0..10_000_u64)
            .map(|i| UserId::new(1_000_000_000_000_000_000 + i))
            .collect();
        bencher.bench(|| FileUpload::target_users(black_box(&ids).iter().copied()));
    }
}
