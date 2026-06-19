use std::env;
use std::fs;
use std::process;

fn main() {
    if let Err(errors) = run() {
        for error in errors {
            eprintln!("error: {error}");
        }
        process::exit(1);
    }
}

fn run() -> Result<(), Vec<String>> {
    let args = env::args().skip(1).collect::<Vec<_>>();
    let [mode, accepted_sources, accepted_facts, candidate_sources, candidate_facts] =
        args.as_slice()
    else {
        return Err(vec![
            "usage: duchy-promote (--dry-run|--apply) accepted.sources accepted.facts candidate.sources candidate.facts"
                .to_string(),
        ]);
    };
    let apply = match mode.as_str() {
        "--dry-run" => false,
        "--apply" => true,
        _ => {
            return Err(vec![
                "duchy-promote mode must be --dry-run or --apply".to_string()
            ]);
        }
    };

    let accepted_source_text = read_text(accepted_sources)?;
    let accepted_fact_text = read_text(accepted_facts)?;
    let candidate_source_text = read_text(candidate_sources)?;
    let candidate_fact_text = read_text(candidate_facts)?;
    let merged_source_text = merge_fixture_text(&accepted_source_text, &candidate_source_text);
    let merged_fact_text = merge_fixture_text(&accepted_fact_text, &candidate_fact_text);

    let candidate_catalog = duchy::SourceCatalog::from_metadata_text(&candidate_source_text)?;
    let candidate_facts = duchy::fact_records_from_text(&candidate_fact_text)?;
    duchy::validate_fact_records(&candidate_catalog, &candidate_facts)?;

    let merged_catalog = duchy::SourceCatalog::from_metadata_text(&merged_source_text)?;
    let merged_facts = duchy::fact_records_from_text(&merged_fact_text)?;
    duchy::validate_fact_records(&merged_catalog, &merged_facts)?;
    duchy::source_backed_timeline_from_facts(&merged_catalog, &merged_facts)?;

    if apply {
        fs::write(accepted_sources, &merged_source_text)
            .map_err(|error| vec![format!("failed to write {accepted_sources}: {error}")])?;
        fs::write(accepted_facts, &merged_fact_text)
            .map_err(|error| vec![format!("failed to write {accepted_facts}: {error}")])?;
    }

    println!(
        "DUCHY promotion {}",
        if apply { "apply" } else { "dry run" }
    );
    println!("- candidate sources: {}", candidate_catalog.record_count());
    println!("- candidate facts: {}", candidate_facts.len());
    println!("- merged sources: {}", merged_catalog.record_count());
    println!("- merged facts: {}", merged_facts.len());
    println!(
        "- result: valid; {}",
        if apply {
            "accepted fixture files updated"
        } else {
            "no files changed"
        }
    );

    Ok(())
}

fn read_text(path: &str) -> Result<String, Vec<String>> {
    fs::read_to_string(path).map_err(|error| vec![format!("failed to read {path}: {error}")])
}

fn merge_fixture_text(left: &str, right: &str) -> String {
    match (left.trim(), right.trim()) {
        ("", "") => String::new(),
        ("", right) => format!("{right}\n"),
        (left, "") => format!("{left}\n"),
        (left, right) => format!("{left}\n---\n{right}\n"),
    }
}
