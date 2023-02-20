use std::{
    collections::{HashMap, HashSet},
    io::{BufRead, BufReader, Error, ErrorKind, Read, Result},
    process::Command,
};

use tabled::{
    builder::Builder,
    papergrid::{
        records::{Records, RecordsMut},
        width::CfgWidthFunction,
    },
    Style, Table, TableOption,
};

fn main() {
    let data = collect_data().unwrap();
    let mut benches = parse_benches(std::io::Cursor::new(data)).unwrap();

    sort_benches(&mut benches);

    let mut table = build_markdown_table(&benches);
    table.with(Style::markdown());
    table.with(HighlightMin);

    println!("{}", table);
}

fn sort_benches(benches: &mut [Bench]) {
    benches.sort_by(|a, b| {
        a.function
            .cmp(&b.function)
            .then_with(|| a.group.cmp(&b.group))
            .then_with(|| {
                let ap = a.param.parse::<usize>().unwrap();
                let bp = b.param.parse::<usize>().unwrap();

                ap.cmp(&bp)
            })
    });
}

fn collect_data() -> Result<String> {
    let output = Command::new("./readme/benchcmp.sh").output()?;
    if !output.status.success() {
        println!(
            "failed to run benchcmp: status={} stdout={:?} stderr={:?}",
            output.status,
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr),
        );
        return Ok(String::new());
    }

    return Ok(String::from_utf8_lossy(&output.stdout).into_owned());
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Bench {
    name: String,
    group: String,
    function: String,
    param: String,
    value: String,
}

impl Bench {
    fn new(name: String, function: String, group: String, param: String, value: String) -> Self {
        Self {
            name,
            group,
            function,
            param,
            value,
        }
    }
}

fn parse_benches<R>(input: R) -> Result<Vec<Bench>>
where
    R: Read,
{
    let mut out = vec![];
    let reader = BufReader::new(input);
    for line in reader.lines() {
        let line = line?;

        let mut parts = line.split_whitespace().take(5);
        let name = get_data_particle(&mut parts).unwrap();
        let function = get_data_particle(&mut parts).unwrap();
        let group = get_data_particle(&mut parts).unwrap();
        let param = get_data_particle(&mut parts).unwrap();
        let value = get_data_particle(&mut parts).unwrap();

        out.push(Bench::new(name, function, group, param, value));
    }

    Ok(out)
}

fn get_data_particle<'a>(parts: &mut impl Iterator<Item = &'a str>) -> Result<String> {
    parts.next().map(|s| s.to_string()).ok_or_else(|| {
        Error::new(
            ErrorKind::Other,
            "unexpected format, was expected to get [group, name, value] being space separated",
        )
    })
}

fn build_markdown_table(benches: &[Bench]) -> Table {
    let (_, group_index) =
        benches
            .iter()
            .map(|b| &b.group)
            .fold((0, HashMap::new()), |(mut i, mut acc), group| {
                if !acc.contains_key(group) {
                    acc.insert(group.to_string(), i);
                    i += 1;
                }

                (i, acc)
            });

    let mut groups = vec![String::new(); group_index.len() + 1];
    for (group, i) in &group_index {
        groups[i + 1] = group.clone();
    }

    let mut funcs: Vec<(&str, &str, Vec<&Bench>)> = vec![];
    let mut seen = HashSet::new();
    for bench in benches {
        let key = (&bench.function, &bench.param);
        if seen.contains(&key) {
            continue;
        }

        seen.insert(key);

        let benches = benches
            .iter()
            .filter(|b| bench.function == b.function && bench.param == b.param)
            .collect::<Vec<_>>();

        funcs.push((bench.function.as_str(), bench.param.as_str(), benches));
    }

    let mut b = Builder::new();
    b.set_columns(groups);

    for (func, param, func_benches) in funcs {
        let mut record = vec![String::new(); group_index.len() + 1];
        record[0] = format!("{}/{}", func, param);

        for bench in func_benches {
            let i = group_index[bench.group.as_str()];
            record[i + 1] = bench.value.to_owned();
        }

        b.add_record(record);
    }

    b.build()
}

// returns ns
fn parse_value(val: &str) -> Option<usize> {
    let (value, diff) = val.split_once('±')?;
    let value = value.parse::<f32>().ok()?;

    let suffix = if diff.ends_with("ns") {
        "ns"
    } else if diff.ends_with("µs") {
        "µs"
    } else if diff.ends_with("ms") {
        "ms"
    } else if diff.ends_with('s') {
        "s"
    } else {
        return None;
    };

    let mult = match suffix {
        "ns" => 1.0,
        "µs" => 1_000.0,
        "ms" => 1_000_000.0,
        "s" => 1_000_000_000.0,
        _ => unreachable!(),
    };

    Some((mult * value) as usize)
}

struct HighlightMin;

impl<R> TableOption<R> for HighlightMin
where
    R: Records + RecordsMut<String>,
{
    fn change(&mut self, table: &mut Table<R>) {
        let (count_rows, count_cols) = table.shape();
        for row in 1..count_rows {
            let values = (1..count_cols)
                .map(|col| table.get_records().get_text((row, col)))
                .map(parse_value)
                .collect::<Option<Vec<_>>>();

            if let Some(values) = values {
                if let Some(min) = values.iter().min() {
                    for (col, value) in values.iter().enumerate() {
                        if value == min {
                            let text = table.get_records().get_text((row, col + 1));
                            let text = format!("**{}**", text);

                            let pos = (row, col + 1);
                            let w = CfgWidthFunction::default();

                            table.get_records_mut().set(pos, text, w);
                        }
                    }
                }
            }
        }
    }
}
