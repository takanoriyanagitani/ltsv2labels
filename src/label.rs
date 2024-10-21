use std::io;
use std::io::BufRead;
use std::io::BufReader;
use std::io::BufWriter;
use std::io::Read;
use std::io::Write;

use std::collections::BTreeMap;

pub const FIELD_SEPARATOR_DEFAULT: char = '\t';
pub const VALUE_SEPARATOR_DEFAULT: char = ':';

#[derive(serde::Serialize)]
pub struct Stat {
    pub label: String,
    pub count: u64,
}

pub fn map2stats(m: BTreeMap<String, u64>) -> impl Iterator<Item = Stat> {
    m.into_iter().map(|pair| {
        let (label, count) = pair;
        Stat { label, count }
    })
}

pub fn line2map(line: &str, m: &mut BTreeMap<String, u64>, field_sep: char, value_sep: char) {
    let splited = line.split(field_sep);
    for pair in splited {
        let mut sp = pair.splitn(2, value_sep);
        let label: &str = sp.next().unwrap_or_default();
        let empty: bool = label.is_empty();
        let non: bool = !empty;
        if non {
            match m.get_mut(label) {
                Some(prev_cnt) => {
                    *prev_cnt += 1;
                }
                None => {
                    m.insert(label.into(), 1);
                }
            }
        }
    }
}

pub fn lines2map<I>(lines: I, m: &mut BTreeMap<String, u64>, field_sep: char, value_sep: char)
where
    I: Iterator<Item = String>,
{
    for line in lines {
        line2map(&line, m, field_sep, value_sep);
    }
}

pub fn rdr2map2json2wtr<R, W>(
    rdr: R,
    wtr: &mut W,
    field_sep: char,
    value_sep: char,
) -> Result<(), io::Error>
where
    R: Read,
    W: Write,
{
    let br = BufReader::new(rdr);
    let lines = br.lines();
    let noerr = lines.map_while(Result::ok);
    {
        let mut bw = BufWriter::new(wtr.by_ref());
        let mut m: BTreeMap<String, u64> = BTreeMap::new();
        lines2map(noerr, &mut m, field_sep, value_sep);
        let stats = map2stats(m);
        for stat in stats {
            serde_json::to_writer(&mut bw, &stat)?;
            bw.write_all(b"\n")?;
        }
        bw.flush()?;
    }
    wtr.flush()?;
    Ok(())
}

pub fn stdin2stats2stdout(field_sep: char, value_sep: char) -> Result<(), io::Error> {
    let i = io::stdin();
    let il = i.lock();

    let o = io::stdout();
    let mut ol = o.lock();
    rdr2map2json2wtr(il, &mut ol, field_sep, value_sep)?;
    ol.flush()?;
    Ok(())
}

pub fn stdin2stats2stdout_default() -> Result<(), io::Error> {
    stdin2stats2stdout(FIELD_SEPARATOR_DEFAULT, VALUE_SEPARATOR_DEFAULT)
}
