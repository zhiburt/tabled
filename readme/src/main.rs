use std::{
    collections::{HashMap, HashSet},
    io::{BufRead, BufReader, Error, ErrorKind, Read, Result},
    process::Command,
};

use tabled::{builder::Builder, Style, Table};

fn main() {
    let data = collect_data().unwrap();
    let benches = parse_benches(std::io::Cursor::new(data)).unwrap();

    let table = build_markdown_table(benches).with(Style::markdown());

    println!("{}", table);
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

fn build_markdown_table(mut benches: Vec<Bench>) -> Table {
    benches.sort_by(|a, b| a.name.cmp(&b.name));

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
    for bench in &benches {
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
