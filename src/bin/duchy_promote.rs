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
    let (mode, report_path, accepted_sources, accepted_facts, candidate_sources, candidate_facts) =
        parse_args(&args)?;
    let apply = match mode.as_str() {
        "--dry-run" => false,
        "--apply" => true,
        _ => {
            return Err(vec![
                "duchy-promote mode must be --dry-run or --apply".to_string()
            ]);
        }
    };

    let accepted_source_text = read_text(&accepted_sources)?;
    let accepted_fact_text = read_text(&accepted_facts)?;
    let candidate_source_text = read_text(&candidate_sources)?;
    let candidate_fact_text = read_text(&candidate_facts)?;
    let merged_source_text = merge_fixture_text(&accepted_source_text, &candidate_source_text);
    let merged_fact_text = merge_fixture_text(&accepted_fact_text, &candidate_fact_text);

    let candidate_catalog = duchy::SourceCatalog::from_metadata_text(&candidate_source_text)?;
    let candidate_facts = duchy::fact_records_from_text(&candidate_fact_text)?;
    duchy::validate_fact_records(&candidate_catalog, &candidate_facts)?;

    let merged_catalog = duchy::SourceCatalog::from_metadata_text(&merged_source_text)?;
    let merged_facts = duchy::fact_records_from_text(&merged_fact_text)?;
    duchy::validate_fact_records(&merged_catalog, &merged_facts)?;
    let merged_timeline = duchy::source_backed_timeline_from_facts(&merged_catalog, &merged_facts)?;
    let candidate_titles =
        duchy::source_backed_titles_from_facts(&merged_catalog, &candidate_facts)?;

    if apply {
        fs::write(&accepted_sources, &merged_source_text)
            .map_err(|error| vec![format!("failed to write {accepted_sources}: {error}")])?;
        fs::write(&accepted_facts, &merged_fact_text)
            .map_err(|error| vec![format!("failed to write {accepted_facts}: {error}")])?;
    }

    if let Some(report_path) = report_path {
        let report = promotion_report(
            apply,
            &candidate_catalog,
            &candidate_facts,
            &candidate_titles,
            &merged_catalog,
            &merged_facts,
        );
        fs::write(&report_path, report)
            .map_err(|error| vec![format!("failed to write {report_path}: {error}")])?;
    }

    println!(
        "DUCHY promotion {}",
        if apply { "apply" } else { "dry run" }
    );
    println!("- candidate sources: {}", candidate_catalog.record_count());
    println!("- candidate facts: {}", candidate_facts.len());
    println!("- merged sources: {}", merged_catalog.record_count());
    println!("- merged facts: {}", merged_facts.len());
    merged_timeline.validate().map_err(|errors| {
        errors
            .into_iter()
            .map(|error| format!("timeline: {error}"))
            .collect::<Vec<_>>()
    })?;
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

fn parse_args(
    args: &[String],
) -> Result<(String, Option<String>, String, String, String, String), Vec<String>> {
    let usage = "usage: duchy-promote (--dry-run|--apply) [--report path] accepted.sources accepted.facts candidate.sources candidate.facts";
    let Some(mode) = args.first() else {
        return Err(vec![usage.to_string()]);
    };

    let mut index = 1;
    let mut report_path = None;
    if args.get(index).map(String::as_str) == Some("--report") {
        let Some(path) = args.get(index + 1) else {
            return Err(vec![usage.to_string()]);
        };
        report_path = Some(path.clone());
        index += 2;
    }

    let remaining = &args[index..];
    let [accepted_sources, accepted_facts, candidate_sources, candidate_facts] = remaining else {
        return Err(vec![usage.to_string()]);
    };

    Ok((
        mode.clone(),
        report_path,
        accepted_sources.clone(),
        accepted_facts.clone(),
        candidate_sources.clone(),
        candidate_facts.clone(),
    ))
}

fn promotion_report(
    apply: bool,
    candidate_catalog: &duchy::SourceCatalog,
    candidate_facts: &[duchy::FactRecord],
    candidate_titles: &[duchy::Title],
    merged_catalog: &duchy::SourceCatalog,
    merged_facts: &[duchy::FactRecord],
) -> String {
    let mut report = String::new();
    report.push_str("# DUCHY Promotion Report\n\n");
    report.push_str(&format!(
        "mode: {}\n",
        if apply { "apply" } else { "dry-run" }
    ));
    report.push_str(&format!(
        "candidate_sources: {}\n",
        candidate_catalog.record_count()
    ));
    report.push_str(&format!("candidate_facts: {}\n", candidate_facts.len()));
    report.push_str(&format!("candidate_titles: {}\n", candidate_titles.len()));
    report.push_str(&format!(
        "merged_sources: {}\n",
        merged_catalog.record_count()
    ));
    report.push_str(&format!("merged_facts: {}\n\n", merged_facts.len()));

    report.push_str("## Candidate Titles\n\n");
    for title in candidate_titles {
        report.push_str(&format!(
            "- {} | {:?} | {}..{}\n",
            title.id,
            title.rank,
            title.exists.start,
            title
                .exists
                .end
                .map_or_else(|| "present".to_string(), |end| end.to_string())
        ));
    }

    report.push_str("\n## Candidate Parentage\n\n");
    for fact in candidate_facts
        .iter()
        .filter(|fact| fact.claim_kind == duchy::ClaimKind::Parentage)
    {
        let span = fact
            .span
            .as_ref()
            .map(|span| {
                format!(
                    "{}..{}",
                    span.start,
                    span.end
                        .map_or_else(|| "present".to_string(), |end| end.to_string())
                )
            })
            .unwrap_or_else(|| "missing-span".to_string());
        report.push_str(&format!(
            "- {} | {} -> {} | {}\n",
            fact.fact_id, fact.subject_id, fact.value, span
        ));
    }

    report.push_str("\n## Candidate Fact IDs\n\n");
    for fact in candidate_facts {
        report.push_str(&format!("- {}\n", fact.fact_id));
    }

    report
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
