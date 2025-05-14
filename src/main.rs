use std::process::ExitCode;

use std::io;

use postgres::{Config, NoTls};

use rs_zip2pgcopy2tables::zipfilename2files2pgcopy2tables_default;

fn env_val_by_key(key: &'static str) -> impl FnMut() -> Result<String, io::Error> {
    move || std::env::var(key).map_err(|e| io::Error::other(format!("env var {key} missing: {e}")))
}

fn env2zfilename() -> Result<String, io::Error> {
    env_val_by_key("ENV_ZIP_FILENAME")()
}

fn sub() -> Result<(), io::Error> {
    let user: String = env_val_by_key("PGUSER")()?;
    let host: String = env_val_by_key("PGHOST")()?;
    let pass: String = env_val_by_key("PGPASSWORD")()?;
    let dbnm: String = env_val_by_key("PGDATABASE")()?;

    let mut cfg = Config::new();

    cfg.user(&user);
    cfg.host(&host);
    cfg.password(&pass);
    cfg.dbname(&dbnm);

    let mut client = cfg.connect(NoTls).map_err(io::Error::other)?;
    let zfilename: String = env2zfilename()?;
    zipfilename2files2pgcopy2tables_default(&zfilename, &mut client)
}

fn main() -> ExitCode {
    sub().map(|_| ExitCode::SUCCESS).unwrap_or_else(|e| {
        eprintln!("{e}");
        ExitCode::FAILURE
    })
}
