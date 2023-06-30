use std::io::SeekFrom;

use bytes::BytesMut;
use clap::{Args, Parser};
use matrix_sdk_appservice::{
    ruma::{api::appservice::{Namespace, Namespaces, Registration, RegistrationInit}, events::room::member::SyncRoomMemberEvent}, AppService, AppServiceRegistration,
};
use serde_yaml;
use tokio::{fs::File, io::{AsyncReadExt, AsyncSeekExt, AsyncWrite, AsyncWriteExt}};
use uuid::Uuid;

use matrix_appservice_discord_rs::Result;


/// Generate a registration in YAML format
#[derive(Args, Debug)]
struct Register {
    
}

// TODO: write better help strings
/// Matrix Discord bridge written in Rust
#[derive(Debug, Parser)]
#[command(author, version, about, long_about=None)]
struct Cli {
    /// Matrix homeserver URL
    #[arg(short='u', long, required_unless_present="register")]
    homeserver: Option<String>,

    /// Matrix server domain name
    #[arg(short, long)]
    domain: String,

    /// Listen address
    #[arg(short, long, requires="register")]
    address: Option<String>,

    /// Listen port
    #[arg(short, long, requires="register")]
    port: Option<u16>,

    /// Generate a registration file
    #[arg(short, long)]
    register: bool,

    /// Path of registration file
    #[arg(short='f', long, required_unless_present="register")]
    registration: Option<String>,
}

async fn write_registration(registration: Registration, mut writer: impl AsyncWrite + Unpin) -> Result<()> {
    Ok(writer.write_all(serde_yaml::to_string(&registration)?.as_bytes()).await?)
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Cli::parse();

    if args.register {
        let mut namespaces = Namespaces::new();
        namespaces.users.push(Namespace::new(true, format!("#_discord_.+:{}", args.domain)));
        namespaces.aliases.push(Namespace::new(true, format!("@_discord_.+:{}", args.domain)));
        // TODO: validate the listen address
        let registration: Registration = RegistrationInit {
            id: format!("PIPO Discord Bridge"),
            url: format!("http://{}:{}", args.address.unwrap(), args.port.unwrap()),
            as_token: Uuid::new_v4().to_string(),
            hs_token: Uuid::new_v4().to_string(),
            sender_localpart: format!("_pipo_discord"),
            namespaces,
            rate_limited: Some(false),
            protocols: Some(vec![format!("discord")])
        }.into();

        match args.registration {
            Some(file) => {
                let mut f = File::create(file).await?;
                write_registration(registration, f).await?;
            },
            None => {
                let mut stdout = tokio::io::stdout();
                write_registration(registration, stdout).await?;
            }
        }
        
        return Ok(())
    }

    let mut f = File::open(args.registration.unwrap()).await?;
    let end = f.metadata().await?.len();
    let mut buf = BytesMut::with_capacity(end as usize);
    while f.read_buf(&mut buf).await? != 0 {}
    let registration: Registration = serde_yaml::from_slice(&buf)?;

    let mut appservice = AppService::builder(
        args.homeserver.unwrap().as_str().try_into()?,
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
