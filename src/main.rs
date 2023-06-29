use clap::Parser;
use matrix_sdk_appservice::{
    ruma::{api::appservice::{Namespace, Namespaces, Registration, RegistrationInit}, events::room::member::SyncRoomMemberEvent}, AppService, AppServiceRegistration,
};
use serde_yaml;
use tokio::{fs::File, io::AsyncWriteExt};
use uuid::Uuid;

use matrix_appservice_discord_rs::Result;


// TODO: write better help strings
/// Matrix Discord bridge written in Rust
#[derive(Debug, Parser)]
#[command(author, version, about, long_about=None)]
struct Args {
    /// Matrix homeserver URL
    #[arg(short='u', long)]
    homeserver: String,

    /// Matrix server domain name
    #[arg(short, long)]
    domain: String,

    /// Path to registration file
    #[arg(short, long)]
    registration: String,

    /// Listen address
    #[arg(short, long)]
    address: String,

    /// Listen port
    #[arg(short, long)]
    port: u16,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    let mut namespaces = Namespaces::new();
    namespaces.users.push(Namespace::new(true, format!("#_discord_.+:{}", args.domain)));
    namespaces.aliases.push(Namespace::new(true, format!("@_discord_.+:{}", args.domain)));
    // TODO: validate the listen address
    let registration: Registration = RegistrationInit {
        id: format!("PIPO Discord Bridge"),
        url: format!("http://{}:{}", args.address, args.port),
        as_token: Uuid::new_v4().to_string(),
        hs_token: Uuid::new_v4().to_string(),
        sender_localpart: format!("_pipo_discord"),
        namespaces,
        rate_limited: Some(false),
        protocols: Some(vec![format!("discord")])
    }.into();
    let mut f = File::open(args.registration).await?;

    f.write_all(serde_yaml::to_string(&registration)?.as_bytes());

    let mut appservice = AppService::builder(
        args.homeserver.as_str().try_into()?,
        args.domain.try_into()?,
        registration.into(),
    )
    .build()
    .await?;
    appservice
        .user(None)
        .await?
        .add_event_handler(|_ev: SyncRoomMemberEvent| async {
            // do stuff
        });

    let (host, port) = appservice.registration().get_host_and_port()?;
    appservice.run(host, port).await?;

    Ok(())
}
