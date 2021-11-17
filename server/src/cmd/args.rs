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
pub enum AdminCommand {
    // normal subcommand
    Create,
}
