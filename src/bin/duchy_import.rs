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
    if let [command, manifest_path] = args.as_slice() {
        if command == "manifest" {
            return manifest_status(manifest_path);
        }
    }

    let (sources_path, facts_path) = match args.as_slice() {
        [] => ("fixtures/first-real.sources", "fixtures/first-real.facts"),
        [command] if command == "status" => {
            ("fixtures/first-real.sources", "fixtures/first-real.facts")
        }
        [command, sources, facts] if command == "status" => (sources.as_str(), facts.as_str()),
        _ => {
            return Err(vec![
                "usage: duchy-import [status [sources-file facts-file]] | manifest manifest-file"
                    .to_string(),
            ])
        }
    };

    let source_text = fs::read_to_string(sources_path)
        .map_err(|error| vec![format!("failed to read {sources_path}: {error}")])?;
    let fact_text = fs::read_to_string(facts_path)
        .map_err(|error| vec![format!("failed to read {facts_path}: {error}")])?;

    let catalog = duchy::SourceCatalog::from_metadata_text(&source_text)?;
    let facts = duchy::fact_records_from_text(&fact_text)?;
    duchy::validate_fact_records(&catalog, &facts)?;
    let titles = duchy::source_backed_titles_from_facts(&catalog, &facts)?;
    let timeline = duchy::source_backed_timeline_from_facts(&catalog, &facts)?;

    let parentage_count = facts
        .iter()
        .filter(|fact| fact.claim_kind == duchy::ClaimKind::Parentage)
        .count();

    println!("DUCHY import status");
    println!("- sources: {}", catalog.record_count());
    println!("- reviews: {}", catalog.review_count());
    println!("- facts: {}", facts.len());
    println!("- titles: {}", titles.len());
    println!("- parentage facts: {parentage_count}");
    timeline.validate().map_err(|errors| {
        errors
            .into_iter()
            .map(|error| format!("timeline: {error}"))
            .collect::<Vec<_>>()
    })?;
    println!("- timeline: valid");

    Ok(())
}

fn manifest_status(manifest_path: &str) -> Result<(), Vec<String>> {
    let manifest_text = fs::read_to_string(manifest_path)
        .map_err(|error| vec![format!("failed to read {manifest_path}: {error}")])?;
    let candidates = duchy::candidate_records_from_text(&manifest_text)?;
    duchy::validate_candidate_records(&candidates)?;

    let mut pending = 0;
    let mut reviewed = 0;
    let mut promoted = 0;
    let mut rejected = 0;
    for candidate in &candidates {
        match candidate.status {
            duchy::CandidateStatus::Pending => pending += 1,
            duchy::CandidateStatus::Reviewed => reviewed += 1,
            duchy::CandidateStatus::Promoted => promoted += 1,
            duchy::CandidateStatus::Rejected => rejected += 1,
        }
    }

    println!("DUCHY manifest status");
    println!("- candidates: {}", candidates.len());
    println!("- pending: {pending}");
    println!("- reviewed: {reviewed}");
    println!("- promoted: {promoted}");
    println!("- rejected: {rejected}");

    Ok(())
}
