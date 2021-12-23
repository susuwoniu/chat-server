use serde::{Deserialize, Serialize};
use structopt::StructOpt;

#[derive(StructOpt, Debug, Clone)]
#[structopt(name = "server")]
/// the stupid content tracker
pub enum CliOpt {
    /// fetch branches from remote repository
    Server(ServerCommand),
    Client(ClientCommand),
    Keygen,
    Admin(AdminCommand),
}

#[derive(Debug, PartialEq, StructOpt, Clone)]
pub enum ServerCommand {
    Start {},
}

#[derive(Debug, PartialEq, StructOpt, Clone)]
pub enum ClientCommand {
    Create,
}
#[derive(Debug, PartialEq, StructOpt, Clone)]
pub struct InitTemplateOpts {
    pub file: String,
}
#[derive(Debug, PartialEq, StructOpt, Clone)]
pub enum AdminCommand {
    // normal subcommand
    Create,
    Set,
    InitTemplates(InitTemplateOpts),
}
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostTemplateItem {
    TitleWithContent(String),
    OnlyTitle([String; 1]),
}
