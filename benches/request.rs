use divan::{Bencher, black_box};
use rustydiscord::model::{
    id::{ApplicationId, ChannelId, GuildId, MessageId, ScheduledEventId, UserId, WebhookId},
    oauth2::Scope,
    permissions::Permissions,
    rest::CreateMessage,
    util::ImageHash,
};
use rustydiscord::rest::{
    ReactionEmoji, Request, Route,
    cdn::{Cdn, ImageOptions, ImageSize},
    oauth2::AuthorizeUrl,
};

#[global_allocator]
static ALLOC: divan::AllocProfiler = divan::AllocProfiler::system();

fn main() {
    divan::main();
}

mod route_path {
    use super::{
        ChannelId, GuildId, MessageId, ReactionEmoji, Route, ScheduledEventId, WebhookId, black_box,
    };

    #[divan::bench]
    fn one_id() -> String {
        black_box(Route::CreateMessage {
            channel_id: ChannelId::new(1_200_000_000_000_000_000),
        })
        .path()
    }

    #[divan::bench]
    fn two_ids() -> String {
        black_box(Route::GetGuildScheduledEventUsers {
            guild_id: GuildId::new(1_100_000_000_000_000_000),
            event_id: ScheduledEventId::new(1_280_000_000_000_000_001),
        })
        .path()
    }

    #[divan::bench]
    fn unicode_reaction() -> String {
        black_box(Route::CreateReaction {
            channel_id: ChannelId::new(1_200_000_000_000_000_000),
            message_id: MessageId::new(1_300_000_000_000_000_001),
            emoji: ReactionEmoji::Unicode("👍🏽"),
        })
        .path()
    }

    #[divan::bench]
    fn webhook_token() -> String {
        black_box(Route::ExecuteWebhook {
            webhook_id: WebhookId::new(1_400_000_000_000_000_000),
            webhook_token: "webhook-token-for-benchmarks-0123456789-abcdefghijklmnopqrstuvwxyz01",
        })
        .path()
    }
}

mod request {
    use super::{Bencher, ChannelId, CreateMessage, Request, Route, black_box};

    #[divan::bench]
    fn create_message(bencher: Bencher<'_, '_>) {
        let body = CreateMessage::from("Pong!");
        bencher.bench(|| {
            Request::new(Route::CreateMessage {
                channel_id: ChannelId::new(1),
            })
            .json(black_box(&body))
            .unwrap()
            .reason(Some("benchmark"))
        });
    }
}

mod urls {
    use super::{
        ApplicationId, AuthorizeUrl, Bencher, Cdn, ImageHash, ImageOptions, ImageSize, Permissions,
        Scope, UserId, black_box,
    };

    #[divan::bench]
    fn cdn_animated_avatar(bencher: Bencher<'_, '_>) {
        let cdn = Cdn::default();
        let hash: ImageHash = "a_1269e74af4df7417b13759eae50c83dc".parse().unwrap();
        bencher.bench(|| {
            cdn.user_avatar(
                black_box(UserId::new(80_351_110_224_678_912)),
                black_box(hash),
                ImageOptions::new().size(ImageSize::S256),
            )
        });
    }

    #[divan::bench]
    fn authorize_url(bencher: Bencher<'_, '_>) {
        bencher.bench(|| {
            AuthorizeUrl::code(
                black_box(ApplicationId::new(157_730_590_492_196_864)),
                "https://example.com/callback",
            )
            .scopes([Scope::Identify, Scope::Guilds, Scope::Bot])
            .permissions(Permissions::SEND_MESSAGES)
            .state("15773059ghq9183habn")
            .build()
        });
    }
}
