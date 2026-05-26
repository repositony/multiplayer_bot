use serenity::model::id::{GuildId, RoleId};
use std::env;
use std::sync::LazyLock;

// Use the test server tokens/IDs for debug builds
#[cfg(debug_assertions)]
const ENV_BOT_TOKEN: &str = "TEST_BOT";
#[cfg(debug_assertions)]
const ENV_GUILD_ID: &str = "TEST_GUILD_ID";
#[cfg(debug_assertions)]
const ENV_TRUSTED_ROLE_ID: &str = "TEST_TRUSTED_ROLE";

// Swap to brumders tokens/IDs for release
#[cfg(not(debug_assertions))]
const ENV_BOT_TOKEN: &str = "MUTLIPLAYER_BOT";
#[cfg(not(debug_assertions))]
const ENV_GUILD_ID: &str = "BRUMDERS_GUILD_ID";
#[cfg(not(debug_assertions))]
const ENV_TRUSTED_ROLE_ID: &str = "BRUMDERS_TRUSTED_ROLE";

/// The developer discord token for the bot
pub static BOT_TOKEN: LazyLock<String> =
    LazyLock::new(|| env::var(ENV_BOT_TOKEN).expect("The appropriate BOT_TOKEN was not found"));

/// ID of the discord "guild" (aka server) the bot will be working on
pub static GUILD_ID: LazyLock<GuildId> = LazyLock::new(|| {
    GuildId::new(
        env::var(ENV_GUILD_ID)
            .expect("The appropriate GUILD_ID was not found")
            .parse()
            .expect("GUILD_ID must be an integer"),
    )
});

/// ID of a trusted role to limit bot interactions to certain users
pub static TRUSTED_ROLE_ID: LazyLock<RoleId> = LazyLock::new(|| {
    RoleId::new(
        env::var(ENV_TRUSTED_ROLE_ID)
            .expect("The appropriate TRUSTED_ROLE_ID was not found")
            .parse()
            .expect("TRUSTED_ROLE_ID must be an integer"),
    )
});
