use std::io;

use std::io::Read;
use std::io::Seek;

use std::fs::File;

use postgres::GenericClient;

use zip::ZipArchive;
use zip::read::ZipFile;

pub fn rdr2pgcopy2table<R, C>(
    rdr: &mut R,
    client: &mut C,
    trusted_table_name: &str,
) -> Result<(), io::Error>
where
    R: Read,
    C: GenericClient,
{
    let query: String = format!("COPY {trusted_table_name} FROM STDIN WITH BINARY");
    let mut wtr = client.copy_in(&query).map_err(io::Error::other)?;
    io::copy(rdr, &mut wtr)?;
    wtr.finish().map_err(io::Error::other)?;
    Ok(())
}

pub fn zip2files2pgcopy2tables<R, C, N>(
    zipfile: &mut ZipArchive<R>,
    client: &mut C,
    name2tabname: &N,
) -> Result<(), io::Error>
where
    R: Read + Seek,
    C: GenericClient,
    N: Fn(&str) -> String,
{
    let file_count: usize = zipfile.len();
    for i in 0..file_count {
        let mut zitem: ZipFile<_> = zipfile.by_index(i)?;
        let filename: &str = zitem.name();
        let trusted_table_name: String = name2tabname(filename);
        rdr2pgcopy2table(&mut zitem, client, &trusted_table_name)?;
    }
    Ok(())
}

pub fn basename2tablename_default(bname: &str) -> String {
    let mut splited = bname.split(".");
    let tabname: &str = splited.next().unwrap_or_default();
    tabname.into()
}

pub fn zipfilename2files2pgcopy2tables<C, N>(
    zipfilename: &str,
    client: &mut C,
    name2tabname: &N,
) -> Result<(), io::Error>
where
    C: GenericClient,
    N: Fn(&str) -> String,
{
    let file: File = File::open(zipfilename)?;
    let mut zfile: ZipArchive<_> = ZipArchive::new(file)?;
    zip2files2pgcopy2tables(&mut zfile, client, name2tabname)
}

pub fn zipfilename2files2pgcopy2tables_default<C>(
    zipfilename: &str,
    client: &mut C,
) -> Result<(), io::Error>
where
    C: GenericClient,
{
    zipfilename2files2pgcopy2tables(zipfilename, client, &basename2tablename_default)
}
