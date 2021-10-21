use structopt::StructOpt;

#[derive(StructOpt, Debug, Clone)]
#[structopt(name = "server")]
/// the stupid content tracker
pub enum Opt {
    /// fetch branches from remote repository
    Server {},
    #[structopt(help = "init pairs key")]
    Init {},
    Client {},
    Keygen {},
}
