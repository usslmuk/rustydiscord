use divan::{Bencher, black_box};
use futures_util::FutureExt;
use rustydiscord::gateway::{
    Config, IdentifyQueue, LocalIdentifyQueue, MessageSender, Shard, create_shards,
};
use rustydiscord::model::{
    gateway::{
        Activity, Intents, ShardId, Status,
        payload::{RequestGuildMembers, UpdatePresence},
    },
    id::GuildId,
};

#[global_allocator]
static ALLOC: divan::AllocProfiler = divan::AllocProfiler::system();

fn main() {
    divan::main();
}

fn config() -> Config {
    Config::builder(
        "not-a-real-token-only-for-benchmarking-0123456789",
        Intents::GUILDS,
    )
    .build()
}

fn shard_and_sender(config: &Config) -> (Shard, MessageSender) {
    let shard = Shard::with_config(ShardId::ONE, config.clone());
    let sender = shard.sender();
    (shard, sender)
}

fn presence() -> UpdatePresence {
    UpdatePresence::new(Status::Online, [Activity::watching("the gateway")])
}

mod commands {
    use super::{
        Bencher, GuildId, RequestGuildMembers, black_box, config, presence, shard_and_sender,
    };

    #[divan::bench]
    fn update_presence_command(bencher: Bencher<'_, '_>) {
        let config = config();
        let presence = presence();
        bencher
            .with_inputs(|| shard_and_sender(&config))
            .bench_local_refs(|(_, sender)| sender.command(black_box(&presence)).unwrap());
    }

    #[divan::bench]
    fn update_presence_kept(bencher: Bencher<'_, '_>) {
        let config = config();
        let presence = presence();
        bencher
            .with_inputs(|| shard_and_sender(&config))
            .bench_local_refs(|(_, sender)| sender.update_presence(black_box(&presence)).unwrap());
    }

    #[divan::bench]
    fn request_guild_members(bencher: Bencher<'_, '_>) {
        let config = config();
        let request =
            RequestGuildMembers::query(GuildId::new(1_100_000_000_000_000_000), "mix", 10);
        bencher
            .with_inputs(|| shard_and_sender(&config))
            .bench_local_refs(|(_, sender)| sender.command(black_box(&request)).unwrap());
    }
}

mod shards {
    use super::{Bencher, config, create_shards};

    #[divan::bench(args = [1, 100, 1_000])]
    fn create(bencher: Bencher<'_, '_>, count: u32) {
        let config = config();
        bencher.bench_local(|| create_shards(0..count, count, config.clone()).collect::<Vec<_>>());
    }
}

mod identify_queue {
    use super::{Bencher, FutureExt, IdentifyQueue, LocalIdentifyQueue, ShardId, black_box};

    #[divan::bench]
    fn free_turn(bencher: Bencher<'_, '_>) {
        bencher
            .with_inputs(|| LocalIdentifyQueue::new(16))
            .bench_local_refs(|queue| {
                queue
                    .wait_turn(black_box(ShardId::new(3, 16)))
                    .now_or_never()
                    .expect("a free bucket never waits");
            });
    }
}
