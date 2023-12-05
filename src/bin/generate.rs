use std::{
    fmt::Write,
    fs::{self, OpenOptions},
    io::{self, Write as IOWrite},
    path::{Path, PathBuf},
    str::FromStr,
};

use regex::Regex;

fn read_line(request: &str) -> String {
    if !request.is_empty() {
        print!("{}", request);
        std::io::Write::flush(&mut io::stdout()).unwrap();
    }
    let mut res = String::new();
    io::stdin()
        .read_line(&mut res)
        .expect("Failed to read line");

    res.trim().to_owned()
}

fn read_valid<T>(request: &str, default_value: T) -> T
where
    T: FromStr,
    T::Err: std::fmt::Debug,
    T: std::fmt::Display,
    T: Clone,
{
    let numbers =
        std::iter::repeat_with(|| read_line(&format!("{} ({}): ", request, default_value.clone())))
            .map(|value_str| {
                if value_str.is_empty() {
                    Ok(default_value.clone())
                } else {
                    value_str.parse()
                }
            })
            .filter(Result::is_ok)
            .map(|x| x.unwrap());

    numbers.take(1).next().unwrap()
}

fn main() {
    let year: u32 = read_valid("Enter year", 2023);
    let day: u32 = read_valid("Enter day number", 1);
    let tasks: u32 = read_valid("Enter task number", 2);

    generate(year, day, tasks)
}

fn generate_justfile(year: u32, day: u32, tasks: u32) {
    let justfile_path = Path::new("justfile");
    let year_short = format!("y{}", year % 2000);
    let lib = format!("{year_short}d{:02}", day);
    let mut content = String::new();
    writeln!(&mut content, "bench-{lib}:").unwrap();
    writeln!(&mut content, "    cargo bench --bench {lib}").unwrap();
    writeln!(&mut content, "test-{lib}:").unwrap();
    writeln!(
        &mut content,
        "    cargo watch -x 'test --package advent --lib -- {year_short}::{lib}::tests --nocapture'"
    )
    .unwrap();
    for i in 1..=tasks {
        writeln!(&mut content, "test-{lib}-task-{i}:").unwrap();
        writeln!(&mut content, "    cargo watch -x 'test --package advent --lib -- {year_short}::{lib}::tests::test_task{i} --exact --nocapture'").unwrap();
        writeln!(&mut content, "test-{lib}-task-{i}-actual:").unwrap();
        writeln!(&mut content, "    cargo watch -x 'test --package advent --lib -- {year_short}::{lib}::tests::test_task{i}_actual --exact --nocapture'").unwrap();
    }

    let mut justfile = OpenOptions::new().append(true).open(justfile_path).unwrap();

    write!(justfile, "\n{content}").unwrap();
}

fn generate(year: u32, day: u32, tasks: u32) {
    generate_bench(year, day, tasks);
    generate_justfile(year, day, tasks);

    let year_library_folder = PathBuf::from(format!("src/y{}", year % 2000));
    let year_library_path = year_library_folder.join("./mod.rs");
    let day_lib_path = format!("src/y{}/y{}d{:02}.rs", year % 2000, year % 2000, day);
    let day_lib_path = Path::new(&day_lib_path);

    if !year_library_folder.exists() {
        std::fs::create_dir(&year_library_folder).expect("failed to create year folder");
    }
    if !year_library_path.exists() {
        std::fs::write(&year_library_path, "").expect("failed to create a file for year");
    }

    for task in 1..=tasks {
        let id = format!("y{}d{:02}t{}", year % 2000, day, task);

        let bin_path = format!("src/bin/{id}.rs");
        let bin_path = Path::new(&bin_path);

        if !bin_path.exists() {
            let content = get_bin_content(year % 2000, day, task);
            fs::write(bin_path, content).unwrap();
        }
    }

    {
        let mut modules = get_modules(Path::new("src/lib.rs"));
        let year_module_name = format!("y{}", year % 2000);
        if !modules.contains(&year_module_name) {
            modules.push(year_module_name);
            modules.sort();
            let content = get_mod_content(&modules);
            fs::write("src/lib.rs", content).unwrap();
        } else {
            println!("Module for year already exists");
        }
    }

    {
        let mut modules = get_modules(&year_library_path);
        let module_name = format!("y{}d{:02}", year % 2000, day);
        if !modules.contains(&module_name) {
            modules.push(module_name);
            modules.sort();
            let content = get_mod_content(&modules);
            fs::write(year_library_path, content).unwrap();
        } else {
            println!("Module already exists");
        }
    }

    {
        if !day_lib_path.exists() {
            let content = get_day_lib_content(year, day, tasks);
            fs::write(day_lib_path, content).unwrap();
        } else {
            println!("Day lib already exists")
        }
    }

    {
        // Generate example folder
        let example_dir_path = format!("src/y{}/y{}d{:02}", year % 1000, year % 1000, day);
        match std::fs::create_dir(&example_dir_path) {
            Ok(()) => {}
            Err(e) => match e.raw_os_error() {
                Some(17) => {}
                _ => {
                    println!("Cannot generate folder for example");
                }
            },
        };
        let example_file_path = format!("{example_dir_path}/example.txt");
        std::fs::write(example_file_path, "").unwrap();
    }
}

fn get_day_lib_content(year: u32, day: u32, tasks: u32) -> String {
    let mut res = String::new();
    for task in 1..=tasks {
        let fun = format!(
            "pub fn solve_task{}(file_content: &str) -> impl std::fmt::Display {{
    0
}}",
            task
        );
        res.push_str(&fun);
        res.push('\n');
    }
    res.push_str("#[cfg(test)]\n");
    res.push_str("mod tests {\n");
    res.push_str("    use super::*;\n");
    res.push_str("    const INPUT: &str = include_str!(\"");
    let example_file_path = format!("./y{}d{:02}/example.txt", year % 1000, day);
    res.push_str(&example_file_path);
    res.push_str("\");\n");
    let actual_file_path = format!(
        "../../benches/y{}/y{}d{:02}.txt",
        year % 1000,
        year % 1000,
        day
    );
    res.push_str("    const ACTUAL: &str = include_str!(\"");
    res.push_str(&actual_file_path);
    res.push_str("\");");
    for task in 1..=tasks {
        let mut test = String::new();
        test.push_str("\n    #[test]\n");
        test.push_str("    fn test_task");
        let num = task.to_string();
        test.push_str(&num);
        test.push_str("() {\n");
        test.push_str("        assert_eq!(format!(\"{}\", solve_task");
        test.push_str(&num);
        test.push_str("(INPUT)), \"0\");\n");
        test.push_str("    }\n");

        test.push_str("\n    #[test]\n");
        test.push_str("    fn test_task");
        let num = task.to_string();
        test.push_str(&num);
        test.push_str("_actual() {\n");
        test.push_str("        assert_eq!(format!(\"{}\", solve_task");
        test.push_str(&num);
        test.push_str("(ACTUAL)), \"0\");\n");
        test.push_str("    }\n");
        res.push_str(&test);
    }
    res.push_str("}\n");
    res
}

fn get_mod_content(modules: &[String]) -> String {
    let mut content = String::new();
    for modul in modules {
        let line = format!("pub mod {modul};\n");
        content.push_str(&line);
    }
    content
}
fn get_modules(global_library_path: &Path) -> Vec<String> {
    let content = fs::read_to_string(global_library_path).unwrap();
    let mut res: Vec<String> = Vec::new();
    let modules_regex = Regex::new(r"mod (\w+);").unwrap();
    for line in content.lines() {
        if let Some(captures) = modules_regex.captures(line) {
            res.push(captures.get(1).unwrap().as_str().to_owned());
        }
    }
    res
}

fn get_bin_content(year: u32, day: u32, task: u32) -> String {
    format!(
        "use advent::y{year}::y{year}d{day:02}::solve_task{task};
use std::{{env::args, fs::read_to_string}};

fn main() {{
    let path_to_input = args().nth(1).unwrap();
    let file_content = read_to_string(path_to_input).unwrap();
    let answer = solve_task{task}(&file_content);

    println!(\"Answer: {{answer}}\")
}}
"
    )
}

fn generate_bench(year: u32, day: u32, tasks: u32) {
    if !PathBuf::from(format!("benches/y{}", year % 2000)).exists() {
        fs::create_dir(format!("benches/y{}", year % 2000))
            .expect("failed to create a folder for the year benchmark");
    }
    let bench_name = format!("y{}d{:02}", year % 2000, day);
    let bench_path = format!("benches/y{}/{}.rs", year % 2000, bench_name);
    add_bench_to_toml(&bench_name, &bench_path);
    let input_file_name = format!("benches/y{}/{bench_name}.txt", year % 2000);
    fs::write(input_file_name, "").unwrap();
    let rs_file_content = get_bench_code(year, day, tasks);
    let rs_file_path = format!("benches/y{}/{bench_name}.rs", year % 2000);
    fs::write(rs_file_path, rs_file_content).unwrap();
}

fn add_bench_to_toml(bench_name: &str, bench_path: &str) {
    let mut content = fs::read_to_string("./Cargo.toml").unwrap();
    let new_bench =
        format!("\n\n[[bench]]\nname = \"{bench_name}\"\npath = \"{bench_path}\"\nharness = false");
    content.push_str(&new_bench);
    fs::write("Cargo.toml", content).unwrap();
}

fn get_bench_code(year: u32, day: u32, tasks: u32) -> String {
    let mut res = String::new();
    writeln!(&mut res, "use std::fs;").unwrap();
    let lib = format!("y{}d{:02}", year % 2000, day);
    write!(&mut res, "use advent::y{}::{lib}::{{", year % 2000).unwrap();
    for task in 1..=tasks {
        if task > 1 {
            write!(&mut res, ", ").unwrap();
        }
        write!(&mut res, "solve_task{task}").unwrap();
    }
    writeln!(&mut res, "}};").unwrap();
    writeln!(
        &mut res,
        "use criterion::{{black_box, criterion_group, criterion_main, Criterion}};\n"
    )
    .unwrap();
    writeln!(&mut res, "pub fn criterion_benchmark(c: &mut Criterion) {{").unwrap();
    writeln!(
        &mut res,
        "    let content = fs::read_to_string(\"benches/y{}/{lib}.txt\").unwrap();",
        year % 2000
    )
    .unwrap();
    for task in 1..=tasks {
        writeln!(&mut res,
        "    c.bench_function(\"{lib}: part {task}\", |b| b.iter(|| solve_task{task}(black_box(&content))));"
    )
        .unwrap();
    }
    writeln!(&mut res, "}}\n").unwrap();
    writeln!(&mut res, "criterion_group!(benches, criterion_benchmark);").unwrap();
    writeln!(&mut res, "criterion_main!(benches);").unwrap();
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[ignore]
    #[test]
    fn test_get_gench_code() {
        let res = get_bench_code(2022, 26, 2);
        assert_eq!("use std::fs;\nuse advent::y22::y22d26::{solve_task1, solve_task2};\nuse criterion::{black_box, criterion_group, criterion_main, Criterion};\n\npub fn criterion_benchmark(c: &mut Criterion) {\n    let content = fs::read_to_string(\"benches/y22/y22d26.txt\").unwrap();\n    c.bench_function(\"y22d26: part 1\", |b| b.iter(|| solve_task1(black_box(&content))));\n    c.bench_function(\"y22d26: part 2\", |b| b.iter(|| solve_task2(black_box(&content))));\n}\n\ncriterion_group!(benches, criterion_benchmark);\ncriterion_main!(benches);\n", res.as_str())
    }

    #[test]
    fn test_name_creation() {
        let x = 5;
        assert_eq!(format!("{x:02}"), "05")
    }
}
