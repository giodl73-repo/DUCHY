use std::collections::BTreeMap;
use std::env;
use std::fs;
use std::path::Path;
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
    if let [command, manifest_path, output_path] = args.as_slice() {
        if command == "source-stubs" {
            return source_stubs(manifest_path, output_path);
        }
        if command == "rejected-report" {
            return rejected_report(manifest_path, output_path);
        }
        if command == "active-manifest" {
            return active_manifest(manifest_path, output_path);
        }
        if command == "archive-manifest" {
            return archive_manifest(manifest_path, output_path);
        }
        if command == "manifest-report" {
            return manifest_report(manifest_path, output_path);
        }
        if command == "duplicate-url-report" {
            return duplicate_url_report(manifest_path, output_path);
        }
        if command == "manifest-tsv" {
            return manifest_tsv(manifest_path, output_path);
        }
        if command == "manifest-from-tsv" {
            return manifest_from_tsv(manifest_path, output_path);
        }
        if command == "parentage-gap-report" {
            return parentage_gap_report(manifest_path, output_path);
        }
    }
    if let [command, sources_path, facts_path, output_path] = args.as_slice() {
        if command == "parentage-coverage-report" {
            return parentage_coverage_report(sources_path, facts_path, output_path);
        }
        if command == "parentage-gap-tsv" {
            return parentage_gap_tsv(sources_path, facts_path, output_path);
        }
    }
    if let [command, manifest_path, output_dir, chunk_size] = args.as_slice() {
        if command == "shard-manifest" {
            return shard_manifest(manifest_path, output_dir, chunk_size);
        }
        if command == "parentage-gap-shard" {
            return parentage_gap_shard(manifest_path, output_dir, chunk_size);
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
                "usage: duchy-import [status [sources-file facts-file]] | manifest manifest-file | source-stubs manifest-file output.sources | rejected-report manifest-file output.md | active-manifest manifest-file output.manifest | archive-manifest manifest-file output.manifest | manifest-report manifest-file output.md | duplicate-url-report manifest-file output.md | manifest-tsv manifest-file output.tsv | manifest-from-tsv input.tsv output.manifest | shard-manifest manifest-file output-dir chunk-size | parentage-coverage-report sources-file facts-file output.md | parentage-gap-tsv sources-file facts-file output.tsv | parentage-gap-shard input.tsv output-dir chunk-size | parentage-gap-report input.tsv output.md".to_string(),
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

fn parentage_coverage_report(
    sources_path: &str,
    facts_path: &str,
    output_path: &str,
) -> Result<(), Vec<String>> {
    let source_text = fs::read_to_string(sources_path)
        .map_err(|error| vec![format!("failed to read {sources_path}: {error}")])?;
    let fact_text = fs::read_to_string(facts_path)
        .map_err(|error| vec![format!("failed to read {facts_path}: {error}")])?;

    let catalog = duchy::SourceCatalog::from_metadata_text(&source_text)?;
    let facts = duchy::fact_records_from_text(&fact_text)?;
    duchy::validate_fact_records(&catalog, &facts)?;
    let titles = duchy::source_backed_titles_from_facts(&catalog, &facts)?;
    let timeline = duchy::source_backed_timeline_from_facts(&catalog, &facts)?;
    timeline.validate().map_err(|errors| {
        errors
            .into_iter()
            .map(|error| format!("timeline: {error}"))
            .collect::<Vec<_>>()
    })?;

    let parentage_facts = facts
        .iter()
        .filter(|fact| fact.claim_kind == duchy::ClaimKind::Parentage)
        .collect::<Vec<_>>();
    let mut parentage_by_child: BTreeMap<&str, Vec<&duchy::FactRecord>> = BTreeMap::new();
    for fact in &parentage_facts {
        parentage_by_child
            .entry(fact.subject_id.as_str())
            .or_default()
            .push(fact);
    }

    let mut rank_totals: BTreeMap<&'static str, (usize, usize)> = BTreeMap::new();
    for title in &titles {
        let label = title_rank_label(title.rank);
        let entry = rank_totals.entry(label).or_insert((0, 0));
        entry.0 += 1;
        if parentage_by_child.contains_key(title.id.as_str()) {
            entry.1 += 1;
        }
    }

    let unparented = titles
        .iter()
        .filter(|title| !parentage_by_child.contains_key(title.id.as_str()))
        .collect::<Vec<_>>();
    let multiple_parentage = parentage_by_child
        .iter()
        .filter(|(_, facts)| facts.len() > 1)
        .collect::<Vec<_>>();

    let mut output = String::new();
    output.push_str("# DUCHY Parentage Coverage Report\n\n");
    output.push_str(&format!("sources: {}\n", catalog.record_count()));
    output.push_str(&format!("facts: {}\n", facts.len()));
    output.push_str(&format!("titles: {}\n", titles.len()));
    output.push_str(&format!("parentage_facts: {}\n", parentage_facts.len()));
    output.push_str(&format!(
        "titles_with_parentage: {}\n",
        parentage_by_child.len()
    ));
    output.push_str(&format!(
        "titles_without_parentage: {}\n\n",
        unparented.len()
    ));

    output.push_str("## Coverage By Rank\n\n");
    output.push_str("| Rank | Titles | With Parentage | Without Parentage |\n");
    output.push_str("|---|---:|---:|---:|\n");
    for (rank, (total, with_parentage)) in &rank_totals {
        output.push_str(&format!(
            "| {rank} | {total} | {with_parentage} | {} |\n",
            total - with_parentage
        ));
    }

    output.push_str("\n## Titles Without Parentage\n\n");
    for title in &unparented {
        output.push_str(&format!(
            "- {} | {} | {} | {}\n",
            title.id,
            title.name,
            title_rank_label(title.rank),
            year_span_label(&title.exists)
        ));
    }

    output.push_str("\n## Titles With Multiple Parentage Facts\n\n");
    if multiple_parentage.is_empty() {
        output.push_str("none\n");
    } else {
        for (child_id, child_facts) in multiple_parentage {
            let title_name = titles
                .iter()
                .find(|title| title.id == *child_id)
                .map(|title| title.name.as_str())
                .unwrap_or("unknown title");
            output.push_str(&format!("- {child_id} | {title_name}\n"));
            for fact in child_facts {
                output.push_str(&format!(
                    "  - {} | {} | {}\n",
                    fact.fact_id,
                    fact.value,
                    fact.span
                        .as_ref()
                        .map(year_span_label)
                        .unwrap_or_else(|| "missing-span".to_string())
                ));
            }
        }
    }

    fs::write(output_path, output)
        .map_err(|error| vec![format!("failed to write {output_path}: {error}")])?;

    println!("DUCHY parentage coverage report");
    println!("- titles: {}", titles.len());
    println!("- parentage facts: {}", parentage_facts.len());
    println!("- titles without parentage: {}", unparented.len());
    println!("- output: {output_path}");

    Ok(())
}

fn parentage_gap_tsv(
    sources_path: &str,
    facts_path: &str,
    output_path: &str,
) -> Result<(), Vec<String>> {
    let source_text = fs::read_to_string(sources_path)
        .map_err(|error| vec![format!("failed to read {sources_path}: {error}")])?;
    let fact_text = fs::read_to_string(facts_path)
        .map_err(|error| vec![format!("failed to read {facts_path}: {error}")])?;

    let catalog = duchy::SourceCatalog::from_metadata_text(&source_text)?;
    let facts = duchy::fact_records_from_text(&fact_text)?;
    duchy::validate_fact_records(&catalog, &facts)?;
    let titles = duchy::source_backed_titles_from_facts(&catalog, &facts)?;
    let timeline = duchy::source_backed_timeline_from_facts(&catalog, &facts)?;
    timeline.validate().map_err(|errors| {
        errors
            .into_iter()
            .map(|error| format!("timeline: {error}"))
            .collect::<Vec<_>>()
    })?;

    let mut parentage_by_child: BTreeMap<&str, Vec<&duchy::FactRecord>> = BTreeMap::new();
    for fact in facts
        .iter()
        .filter(|fact| fact.claim_kind == duchy::ClaimKind::Parentage)
    {
        parentage_by_child
            .entry(fact.subject_id.as_str())
            .or_default()
            .push(fact);
    }

    let mut output = String::new();
    output.push_str(parentage_gap_tsv_header());
    output.push('\n');
    let mut gap_count = 0;
    for title in titles
        .iter()
        .filter(|title| !parentage_by_child.contains_key(title.id.as_str()))
    {
        output.push_str(&tsv_escape(&title.id));
        output.push('\t');
        output.push_str(&tsv_escape(&title.name));
        output.push('\t');
        output.push_str(title_rank_label(title.rank));
        output.push('\t');
        output.push_str(&year_span_label(&title.exists));
        output.push('\t');
        output.push('0');
        output.push('\t');
        output.push_str(parentage_gap_priority(title.rank));
        output.push('\t');
        output.push_str(parentage_gap_note(title.rank));
        output.push('\n');
        gap_count += 1;
    }

    fs::write(output_path, output)
        .map_err(|error| vec![format!("failed to write {output_path}: {error}")])?;

    println!("DUCHY parentage gap TSV");
    println!("- titles: {}", titles.len());
    println!("- gap rows: {gap_count}");
    println!("- output: {output_path}");

    Ok(())
}

fn parentage_gap_shard(
    input_path: &str,
    output_dir: &str,
    chunk_size: &str,
) -> Result<(), Vec<String>> {
    let chunk_size = chunk_size
        .parse::<usize>()
        .map_err(|error| vec![format!("invalid chunk size {chunk_size}: {error}")])?;
    if chunk_size == 0 {
        return Err(vec!["chunk size must be greater than zero".to_string()]);
    }

    let input_text = fs::read_to_string(input_path)
        .map_err(|error| vec![format!("failed to read {input_path}: {error}")])?;
    let rows = parentage_gap_rows_from_tsv(&input_text)?;
    if rows.is_empty() {
        return Err(vec![format!("{input_path} has no parentage gap rows")]);
    }

    fs::create_dir_all(output_dir)
        .map_err(|error| vec![format!("failed to create {output_dir}: {error}")])?;

    let mut index = String::new();
    index.push_str("# DUCHY Parentage Gap Shards\n\n");
    index.push_str(&format!("source_tsv: {input_path}\n"));
    index.push_str(&format!("gap_rows: {}\n", rows.len()));
    index.push_str(&format!("chunk_size: {chunk_size}\n\n"));
    index.push_str("| Shard | Rows | High | Medium | Root |\n");
    index.push_str("|---|---:|---:|---:|---:|\n");

    let header = parentage_gap_tsv_header();
    for (index_number, chunk) in rows.chunks(chunk_size).enumerate() {
        let shard_name = format!("batch-{:03}.tsv", index_number + 1);
        let shard_path = Path::new(output_dir).join(&shard_name);
        let shard_text = parentage_gap_rows_to_tsv(chunk, header);
        fs::write(&shard_path, shard_text)
            .map_err(|error| vec![format!("failed to write {}: {error}", shard_path.display())])?;

        let counts = parentage_gap_priority_counts(chunk);
        index.push_str(&format!(
            "| {shard_name} | {} | {} | {} | {} |\n",
            chunk.len(),
            counts.high,
            counts.medium,
            counts.root
        ));
    }

    let index_path = Path::new(output_dir).join("INDEX.md");
    fs::write(&index_path, index)
        .map_err(|error| vec![format!("failed to write {}: {error}", index_path.display())])?;

    println!("DUCHY parentage gap shards");
    println!("- gap rows: {}", rows.len());
    println!("- shards: {}", rows.chunks(chunk_size).len());
    println!("- output: {output_dir}");

    Ok(())
}

fn parentage_gap_report(input_path: &str, output_path: &str) -> Result<(), Vec<String>> {
    let input_text = fs::read_to_string(input_path)
        .map_err(|error| vec![format!("failed to read {input_path}: {error}")])?;
    let rows = parentage_gap_rows_from_tsv(&input_text)?;
    if rows.is_empty() {
        return Err(vec![format!("{input_path} has no parentage gap rows")]);
    }

    let mut by_rank: BTreeMap<&str, usize> = BTreeMap::new();
    let mut by_priority: BTreeMap<&str, usize> = BTreeMap::new();
    for row in &rows {
        *by_rank.entry(row.rank.as_str()).or_default() += 1;
        *by_priority.entry(row.review_priority.as_str()).or_default() += 1;
    }

    let mut output = String::new();
    output.push_str("# DUCHY Parentage Gap Review Report\n\n");
    output.push_str(&format!("source_tsv: {input_path}\n"));
    output.push_str(&format!("gap_rows: {}\n\n", rows.len()));

    output.push_str("## Priority Counts\n\n");
    output.push_str("| Priority | Rows |\n");
    output.push_str("|---|---:|\n");
    for (priority, count) in &by_priority {
        output.push_str(&format!("| {priority} | {count} |\n"));
    }

    output.push_str("\n## Rank Counts\n\n");
    output.push_str("| Rank | Rows |\n");
    output.push_str("|---|---:|\n");
    for (rank, count) in &by_rank {
        output.push_str(&format!("| {rank} | {count} |\n"));
    }

    output.push_str("\n## Review Rows\n\n");
    for row in &rows {
        output.push_str(&format!("### {} | {}\n\n", row.title_id, row.name));
        output.push_str(&format!("- rank: {}\n", row.rank));
        output.push_str(&format!("- exists: {}\n", row.exists));
        output.push_str(&format!("- review_priority: {}\n", row.review_priority));
        output.push_str(&format!("- notes: {}\n\n", row.notes));
    }

    fs::write(output_path, output)
        .map_err(|error| vec![format!("failed to write {output_path}: {error}")])?;

    println!("DUCHY parentage gap report");
    println!("- gap rows: {}", rows.len());
    println!("- output: {output_path}");

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

fn source_stubs(manifest_path: &str, output_path: &str) -> Result<(), Vec<String>> {
    let manifest_text = fs::read_to_string(manifest_path)
        .map_err(|error| vec![format!("failed to read {manifest_path}: {error}")])?;
    let candidates = duchy::candidate_records_from_text(&manifest_text)?;
    duchy::validate_candidate_records(&candidates)?;

    let reviewed = candidates
        .iter()
        .filter(|candidate| candidate.status == duchy::CandidateStatus::Reviewed)
        .collect::<Vec<_>>();
    if reviewed.is_empty() {
        return Err(vec![
            "manifest has no reviewed candidates for source stub generation".to_string(),
        ]);
    }

    let mut output = String::new();
    output.push_str("# Generated source stubs; review before promotion.\n");
    for (index, candidate) in reviewed.iter().enumerate() {
        if index > 0 {
            output.push_str("---\n");
        }
        output.push_str(&format!("source_id: {}\n", candidate.source_id));
        output.push_str("source_kind: other\n");
        output.push_str(&format!("source_url: {}\n", candidate.source_url));
        output.push_str("license: REVIEW_REQUIRED\n");
        output.push_str("retrieved_on: REVIEW_REQUIRED\n");
        output.push_str("allowed_use: blocked\n");
        output.push_str("notes: Generated from reviewed candidate manifest; replace review fields before fact promotion.\n");
        output.push_str("review_decision: blocked_scope\n");
        output.push_str("reviewer: Source Custody Reviewer\n");
        output.push_str("review_note: Generated stub only; source-custody review required before fact extraction.\n");
    }

    duchy::SourceCatalog::from_metadata_text(&output)?;
    fs::write(output_path, output)
        .map_err(|error| vec![format!("failed to write {output_path}: {error}")])?;

    println!("DUCHY source stubs");
    println!("- reviewed candidates: {}", reviewed.len());
    println!("- output: {output_path}");

    Ok(())
}

fn rejected_report(manifest_path: &str, output_path: &str) -> Result<(), Vec<String>> {
    let manifest_text = fs::read_to_string(manifest_path)
        .map_err(|error| vec![format!("failed to read {manifest_path}: {error}")])?;
    let candidates = duchy::candidate_records_from_text(&manifest_text)?;
    duchy::validate_candidate_records(&candidates)?;

    let rejected = candidates
        .iter()
        .filter(|candidate| candidate.status == duchy::CandidateStatus::Rejected)
        .collect::<Vec<_>>();
    if rejected.is_empty() {
        return Err(vec![
            "manifest has no rejected candidates to archive".to_string()
        ]);
    }

    let mut output = String::new();
    output.push_str("# DUCHY Rejected Candidate Report\n\n");
    output.push_str(&format!("rejected_candidates: {}\n\n", rejected.len()));
    for candidate in &rejected {
        output.push_str(&format!("## {}\n\n", candidate.candidate_id));
        output.push_str(&format!("- source_id: {}\n", candidate.source_id));
        output.push_str(&format!("- source_url: {}\n", candidate.source_url));
        if let Some(reason) = candidate.exclusion_reason {
            output.push_str(&format!(
                "- exclusion_reason: {}\n",
                exclusion_reason_label(reason)
            ));
        }
        output.push_str(&format!(
            "- notes: {}\n\n",
            candidate.notes.as_deref().unwrap_or("none")
        ));
    }

    fs::write(output_path, output)
        .map_err(|error| vec![format!("failed to write {output_path}: {error}")])?;

    println!("DUCHY rejected candidate report");
    println!("- rejected candidates: {}", rejected.len());
    println!("- output: {output_path}");

    Ok(())
}

fn active_manifest(manifest_path: &str, output_path: &str) -> Result<(), Vec<String>> {
    let manifest_text = fs::read_to_string(manifest_path)
        .map_err(|error| vec![format!("failed to read {manifest_path}: {error}")])?;
    let candidates = duchy::candidate_records_from_text(&manifest_text)?;
    duchy::validate_candidate_records(&candidates)?;

    let active = candidates
        .iter()
        .filter(|candidate| {
            matches!(
                candidate.status,
                duchy::CandidateStatus::Pending | duchy::CandidateStatus::Reviewed
            )
        })
        .collect::<Vec<_>>();
    if active.is_empty() {
        return Err(vec![
            "manifest has no pending or reviewed candidates to keep active".to_string(),
        ]);
    }

    let mut output = String::new();
    output.push_str("# Active candidate manifest generated from staging queue.\n");
    for (index, candidate) in active.iter().enumerate() {
        if index > 0 {
            output.push_str("---\n");
        }
        output.push_str(&candidate_manifest_block(candidate));
    }

    let parsed_output = duchy::candidate_records_from_text(&output)?;
    duchy::validate_candidate_records(&parsed_output)?;
    fs::write(output_path, output)
        .map_err(|error| vec![format!("failed to write {output_path}: {error}")])?;

    println!("DUCHY active manifest");
    println!("- active candidates: {}", active.len());
    println!("- output: {output_path}");

    Ok(())
}

fn archive_manifest(manifest_path: &str, output_path: &str) -> Result<(), Vec<String>> {
    let manifest_text = fs::read_to_string(manifest_path)
        .map_err(|error| vec![format!("failed to read {manifest_path}: {error}")])?;
    let candidates = duchy::candidate_records_from_text(&manifest_text)?;
    duchy::validate_candidate_records(&candidates)?;

    let archived = candidates
        .iter()
        .filter(|candidate| {
            matches!(
                candidate.status,
                duchy::CandidateStatus::Promoted | duchy::CandidateStatus::Rejected
            )
        })
        .collect::<Vec<_>>();
    if archived.is_empty() {
        return Err(vec![
            "manifest has no promoted or rejected candidates to archive".to_string(),
        ]);
    }

    let mut output = String::new();
    output.push_str("# Archived candidate manifest generated from staging queue.\n");
    for (index, candidate) in archived.iter().enumerate() {
        if index > 0 {
            output.push_str("---\n");
        }
        output.push_str(&candidate_manifest_block(candidate));
    }

    let parsed_output = duchy::candidate_records_from_text(&output)?;
    duchy::validate_candidate_records(&parsed_output)?;
    fs::write(output_path, output)
        .map_err(|error| vec![format!("failed to write {output_path}: {error}")])?;

    println!("DUCHY archive manifest");
    println!("- archived candidates: {}", archived.len());
    println!("- output: {output_path}");

    Ok(())
}

fn shard_manifest(
    manifest_path: &str,
    output_dir: &str,
    chunk_size_text: &str,
) -> Result<(), Vec<String>> {
    let chunk_size = chunk_size_text
        .parse::<usize>()
        .map_err(|error| vec![format!("invalid chunk size {chunk_size_text}: {error}")])?;
    if chunk_size == 0 {
        return Err(vec!["chunk size must be greater than zero".to_string()]);
    }

    let manifest_text = fs::read_to_string(manifest_path)
        .map_err(|error| vec![format!("failed to read {manifest_path}: {error}")])?;
    let candidates = duchy::candidate_records_from_text(&manifest_text)?;
    duchy::validate_candidate_records(&candidates)?;
    if candidates.is_empty() {
        return Err(vec!["manifest has no candidates to shard".to_string()]);
    }

    fs::create_dir_all(output_dir)
        .map_err(|error| vec![format!("failed to create {output_dir}: {error}")])?;

    let shard_count = candidates.len().div_ceil(chunk_size);
    let mut index_output = String::new();
    index_output.push_str("# DUCHY Manifest Shard Index\n\n");
    index_output.push_str(&format!("source_manifest: {manifest_path}\n"));
    index_output.push_str(&format!("candidates: {}\n", candidates.len()));
    index_output.push_str(&format!("chunk_size: {chunk_size}\n"));
    index_output.push_str(&format!("shards: {shard_count}\n\n"));
    index_output.push_str("| Shard | Candidates | Pending | Reviewed | Promoted | Rejected |\n");
    index_output.push_str("|---|---:|---:|---:|---:|---:|\n");

    for (index, chunk) in candidates.chunks(chunk_size).enumerate() {
        let output = candidate_manifest_text(
            "Candidate manifest shard generated from staging queue.",
            chunk,
        )?;
        let shard_name = format!("batch-{:03}.manifest", index + 1);
        let output_path = Path::new(output_dir).join(&shard_name);
        fs::write(&output_path, output).map_err(|error| {
            vec![format!(
                "failed to write {}: {error}",
                output_path.display()
            )]
        })?;

        let counts = candidate_status_counts(chunk);
        index_output.push_str(&format!(
            "| {shard_name} | {} | {} | {} | {} | {} |\n",
            chunk.len(),
            counts.pending,
            counts.reviewed,
            counts.promoted,
            counts.rejected
        ));
    }
    let index_path = Path::new(output_dir).join("INDEX.md");
    fs::write(&index_path, index_output)
        .map_err(|error| vec![format!("failed to write {}: {error}", index_path.display())])?;

    println!("DUCHY manifest shards");
    println!("- candidates: {}", candidates.len());
    println!("- chunk size: {chunk_size}");
    println!("- shards: {shard_count}");
    println!("- output: {output_dir}");
    println!("- index: {}", index_path.display());

    Ok(())
}

fn manifest_report(manifest_path: &str, output_path: &str) -> Result<(), Vec<String>> {
    let manifest_text = fs::read_to_string(manifest_path)
        .map_err(|error| vec![format!("failed to read {manifest_path}: {error}")])?;
    let candidates = duchy::candidate_records_from_text(&manifest_text)?;
    duchy::validate_candidate_records(&candidates)?;
    if candidates.is_empty() {
        return Err(vec!["manifest has no candidates to report".to_string()]);
    }

    let counts = candidate_status_counts(&candidates);
    let mut output = String::new();
    output.push_str("# DUCHY Candidate Manifest Report\n\n");
    output.push_str(&format!("source_manifest: {manifest_path}\n"));
    output.push_str(&format!("candidates: {}\n", candidates.len()));
    output.push_str(&format!("pending: {}\n", counts.pending));
    output.push_str(&format!("reviewed: {}\n", counts.reviewed));
    output.push_str(&format!("promoted: {}\n", counts.promoted));
    output.push_str(&format!("rejected: {}\n\n", counts.rejected));

    push_manifest_report_section(
        &mut output,
        "Pending",
        &candidates,
        duchy::CandidateStatus::Pending,
    );
    push_manifest_report_section(
        &mut output,
        "Reviewed",
        &candidates,
        duchy::CandidateStatus::Reviewed,
    );
    push_manifest_report_section(
        &mut output,
        "Promoted",
        &candidates,
        duchy::CandidateStatus::Promoted,
    );
    push_manifest_report_section(
        &mut output,
        "Rejected",
        &candidates,
        duchy::CandidateStatus::Rejected,
    );

    fs::write(output_path, output)
        .map_err(|error| vec![format!("failed to write {output_path}: {error}")])?;

    println!("DUCHY manifest report");
    println!("- candidates: {}", candidates.len());
    println!("- output: {output_path}");

    Ok(())
}

fn duplicate_url_report(manifest_path: &str, output_path: &str) -> Result<(), Vec<String>> {
    let manifest_text = fs::read_to_string(manifest_path)
        .map_err(|error| vec![format!("failed to read {manifest_path}: {error}")])?;
    let candidates = duchy::candidate_records_from_text(&manifest_text)?;
    duchy::validate_candidate_records(&candidates)?;
    if candidates.is_empty() {
        return Err(vec![
            "manifest has no candidates to scan for duplicate source URLs".to_string(),
        ]);
    }

    let mut by_url = BTreeMap::<&str, Vec<&duchy::CandidateRecord>>::new();
    for candidate in &candidates {
        by_url
            .entry(candidate.source_url.as_str())
            .or_default()
            .push(candidate);
    }

    let duplicate_groups = by_url
        .iter()
        .filter(|(_, matching)| matching.len() > 1)
        .collect::<Vec<_>>();
    let duplicate_group_count = duplicate_groups.len();

    let mut output = String::new();
    output.push_str("# DUCHY Duplicate Source URL Report\n\n");
    output.push_str(&format!("source_manifest: {manifest_path}\n"));
    output.push_str(&format!("candidates: {}\n", candidates.len()));
    output.push_str(&format!(
        "duplicate_source_urls: {duplicate_group_count}\n\n"
    ));

    if duplicate_groups.is_empty() {
        output.push_str("No duplicate source URLs found.\n");
    } else {
        for (source_url, matching) in duplicate_groups {
            output.push_str(&format!("## {source_url}\n\n"));
            for candidate in matching {
                output.push_str(&format!("- candidate_id: {}\n", candidate.candidate_id));
                output.push_str(&format!("  source_id: {}\n", candidate.source_id));
                output.push_str(&format!(
                    "  status: {}\n",
                    candidate_status_label(candidate.status)
                ));
                if let Some(entity_class) = candidate.entity_class {
                    output.push_str(&format!(
                        "  entity_class: {}\n",
                        entity_class_label(entity_class)
                    ));
                }
                if let Some(import_scope) = candidate.import_scope {
                    output.push_str(&format!(
                        "  import_scope: {}\n",
                        import_scope_label(import_scope)
                    ));
                }
                if let Some(notes) = &candidate.notes {
                    output.push_str(&format!("  notes: {notes}\n"));
                }
            }
            output.push('\n');
        }
    }

    fs::write(output_path, output)
        .map_err(|error| vec![format!("failed to write {output_path}: {error}")])?;

    println!("DUCHY duplicate source URL report");
    println!("- candidates: {}", candidates.len());
    println!("- duplicate source URLs: {duplicate_group_count}");
    println!("- output: {output_path}");

    Ok(())
}

fn manifest_tsv(manifest_path: &str, output_path: &str) -> Result<(), Vec<String>> {
    let manifest_text = fs::read_to_string(manifest_path)
        .map_err(|error| vec![format!("failed to read {manifest_path}: {error}")])?;
    let candidates = duchy::candidate_records_from_text(&manifest_text)?;
    duchy::validate_candidate_records(&candidates)?;
    if candidates.is_empty() {
        return Err(vec!["manifest has no candidates to export".to_string()]);
    }

    let mut output = String::new();
    output.push_str("candidate_id\tsource_id\tsource_url\tstatus\treview_batch_id\timport_scope\trank_basis\tentity_class\tsource_claims_used\tconfidence_detail\tparentage_status\tquery_readiness\texclusion_reason\tnotes\n");
    for candidate in &candidates {
        output.push_str(&tsv_cell(&candidate.candidate_id));
        output.push('\t');
        output.push_str(&tsv_cell(&candidate.source_id));
        output.push('\t');
        output.push_str(&tsv_cell(&candidate.source_url));
        output.push('\t');
        output.push_str(candidate_status_label(candidate.status));
        output.push('\t');
        output.push_str(&tsv_cell(
            candidate.review_batch_id.as_deref().unwrap_or(""),
        ));
        output.push('\t');
        output.push_str(candidate.import_scope.map(import_scope_label).unwrap_or(""));
        output.push('\t');
        output.push_str(candidate.rank_basis.map(rank_basis_label).unwrap_or(""));
        output.push('\t');
        output.push_str(candidate.entity_class.map(entity_class_label).unwrap_or(""));
        output.push('\t');
        output.push_str(&tsv_cell(&candidate.source_claims_used.join(",")));
        output.push('\t');
        output.push_str(
            candidate
                .confidence_detail
                .map(confidence_detail_label)
                .unwrap_or(""),
        );
        output.push('\t');
        output.push_str(
            candidate
                .parentage_status
                .map(parentage_status_label)
                .unwrap_or(""),
        );
        output.push('\t');
        output.push_str(
            candidate
                .query_readiness
                .map(query_readiness_label)
                .unwrap_or(""),
        );
        output.push('\t');
        output.push_str(
            candidate
                .exclusion_reason
                .map(exclusion_reason_label)
                .unwrap_or(""),
        );
        output.push('\t');
        output.push_str(&tsv_cell(candidate.notes.as_deref().unwrap_or("")));
        output.push('\n');
    }

    fs::write(output_path, output)
        .map_err(|error| vec![format!("failed to write {output_path}: {error}")])?;

    println!("DUCHY manifest TSV");
    println!("- candidates: {}", candidates.len());
    println!("- output: {output_path}");

    Ok(())
}

fn manifest_from_tsv(input_path: &str, output_path: &str) -> Result<(), Vec<String>> {
    let input_text = fs::read_to_string(input_path)
        .map_err(|error| vec![format!("failed to read {input_path}: {error}")])?;
    let mut lines = input_text.lines();
    let Some(header) = lines.next() else {
        return Err(vec![format!("{input_path} is empty")]);
    };
    let expected_header = "candidate_id\tsource_id\tsource_url\tstatus\treview_batch_id\timport_scope\trank_basis\tentity_class\tsource_claims_used\tconfidence_detail\tparentage_status\tquery_readiness\texclusion_reason\tnotes";
    if header != expected_header {
        return Err(vec![format!(
            "invalid TSV header in {input_path}: expected {expected_header}"
        )]);
    }

    let mut output = String::new();
    output.push_str("# Candidate manifest generated from TSV import.\n");
    let mut candidate_count = 0;
    for (line_index, line) in lines.enumerate() {
        if line.trim().is_empty() {
            continue;
        }
        let fields = line.split('\t').collect::<Vec<_>>();
        let line_number = line_index + 2;
        if fields.len() != 14 {
            return Err(vec![format!(
                "line {line_number}: expected 14 TSV columns, found {}",
                fields.len()
            )]);
        }
        let candidate_id = manifest_tsv_cell(fields[0], line_number)?;
        let source_id = manifest_tsv_cell(fields[1], line_number)?;
        let source_url = manifest_tsv_cell(fields[2], line_number)?;
        let status = manifest_tsv_cell(fields[3], line_number)?;
        let review_batch_id = manifest_tsv_cell(fields[4], line_number)?;
        let import_scope = manifest_tsv_cell(fields[5], line_number)?;
        let rank_basis = manifest_tsv_cell(fields[6], line_number)?;
        let entity_class = manifest_tsv_cell(fields[7], line_number)?;
        let source_claims_used = manifest_tsv_cell(fields[8], line_number)?;
        let confidence_detail = manifest_tsv_cell(fields[9], line_number)?;
        let parentage_status = manifest_tsv_cell(fields[10], line_number)?;
        let query_readiness = manifest_tsv_cell(fields[11], line_number)?;
        let exclusion_reason = manifest_tsv_cell(fields[12], line_number)?;
        let notes = manifest_tsv_cell(fields[13], line_number)?;

        if candidate_count > 0 {
            output.push_str("---\n");
        }
        output.push_str(&format!("candidate_id: {candidate_id}\n"));
        output.push_str(&format!("source_id: {source_id}\n"));
        output.push_str(&format!("source_url: {source_url}\n"));
        output.push_str(&format!("status: {status}\n"));
        push_optional_manifest_line(&mut output, "review_batch_id", &review_batch_id);
        push_optional_manifest_line(&mut output, "import_scope", &import_scope);
        push_optional_manifest_line(&mut output, "rank_basis", &rank_basis);
        push_optional_manifest_line(&mut output, "entity_class", &entity_class);
        push_optional_manifest_line(&mut output, "source_claims_used", &source_claims_used);
        push_optional_manifest_line(&mut output, "confidence_detail", &confidence_detail);
        push_optional_manifest_line(&mut output, "parentage_status", &parentage_status);
        push_optional_manifest_line(&mut output, "query_readiness", &query_readiness);
        push_optional_manifest_line(&mut output, "exclusion_reason", &exclusion_reason);
        if !notes.is_empty() {
            output.push_str(&format!("notes: {notes}\n"));
        }
        candidate_count += 1;
    }
    if candidate_count == 0 {
        return Err(vec![format!("{input_path} has no candidate rows")]);
    }

    let parsed_output = duchy::candidate_records_from_text(&output)?;
    duchy::validate_candidate_records(&parsed_output)?;
    fs::write(output_path, output)
        .map_err(|error| vec![format!("failed to write {output_path}: {error}")])?;

    println!("DUCHY manifest from TSV");
    println!("- candidates: {candidate_count}");
    println!("- output: {output_path}");

    Ok(())
}

fn candidate_manifest_block(candidate: &duchy::CandidateRecord) -> String {
    let mut output = String::new();
    output.push_str(&format!("candidate_id: {}\n", candidate.candidate_id));
    output.push_str(&format!("source_id: {}\n", candidate.source_id));
    output.push_str(&format!("source_url: {}\n", candidate.source_url));
    output.push_str(&format!(
        "status: {}\n",
        candidate_status_label(candidate.status)
    ));
    if let Some(review_batch_id) = &candidate.review_batch_id {
        output.push_str(&format!("review_batch_id: {review_batch_id}\n"));
    }
    if let Some(import_scope) = candidate.import_scope {
        output.push_str(&format!(
            "import_scope: {}\n",
            import_scope_label(import_scope)
        ));
    }
    if let Some(rank_basis) = candidate.rank_basis {
        output.push_str(&format!("rank_basis: {}\n", rank_basis_label(rank_basis)));
    }
    if let Some(entity_class) = candidate.entity_class {
        output.push_str(&format!(
            "entity_class: {}\n",
            entity_class_label(entity_class)
        ));
    }
    if !candidate.source_claims_used.is_empty() {
        output.push_str(&format!(
            "source_claims_used: {}\n",
            candidate.source_claims_used.join(", ")
        ));
    }
    if let Some(confidence_detail) = candidate.confidence_detail {
        output.push_str(&format!(
            "confidence_detail: {}\n",
            confidence_detail_label(confidence_detail)
        ));
    }
    if let Some(parentage_status) = candidate.parentage_status {
        output.push_str(&format!(
            "parentage_status: {}\n",
            parentage_status_label(parentage_status)
        ));
    }
    if let Some(query_readiness) = candidate.query_readiness {
        output.push_str(&format!(
            "query_readiness: {}\n",
            query_readiness_label(query_readiness)
        ));
    }
    if let Some(exclusion_reason) = candidate.exclusion_reason {
        output.push_str(&format!(
            "exclusion_reason: {}\n",
            exclusion_reason_label(exclusion_reason)
        ));
    }
    if let Some(notes) = &candidate.notes {
        output.push_str(&format!("notes: {notes}\n"));
    }
    output
}

fn candidate_manifest_text(
    header: &str,
    candidates: &[duchy::CandidateRecord],
) -> Result<String, Vec<String>> {
    let mut output = String::new();
    output.push_str("# ");
    output.push_str(header);
    output.push('\n');
    for (index, candidate) in candidates.iter().enumerate() {
        if index > 0 {
            output.push_str("---\n");
        }
        output.push_str(&candidate_manifest_block(candidate));
    }

    let parsed_output = duchy::candidate_records_from_text(&output)?;
    duchy::validate_candidate_records(&parsed_output)?;
    Ok(output)
}

fn candidate_status_label(status: duchy::CandidateStatus) -> &'static str {
    match status {
        duchy::CandidateStatus::Pending => "pending",
        duchy::CandidateStatus::Reviewed => "reviewed",
        duchy::CandidateStatus::Promoted => "promoted",
        duchy::CandidateStatus::Rejected => "rejected",
    }
}

#[derive(Clone)]
struct ParentageGapRow {
    title_id: String,
    name: String,
    rank: String,
    exists: String,
    parentage_count: String,
    review_priority: String,
    notes: String,
}

#[derive(Default)]
struct ParentageGapPriorityCounts {
    high: usize,
    medium: usize,
    root: usize,
}

fn parentage_gap_rows_from_tsv(input: &str) -> Result<Vec<ParentageGapRow>, Vec<String>> {
    let mut lines = input.lines();
    let Some(header) = lines.next() else {
        return Err(vec!["parentage gap TSV is empty".to_string()]);
    };
    let expected_header = parentage_gap_tsv_header();
    if header != expected_header {
        return Err(vec![format!(
            "invalid parentage gap TSV header: expected {expected_header}"
        )]);
    }

    let mut rows = Vec::new();
    for (line_index, line) in lines.enumerate() {
        if line.trim().is_empty() {
            continue;
        }
        let line_number = line_index + 2;
        let fields = line.split('\t').collect::<Vec<_>>();
        if fields.len() != 7 {
            return Err(vec![format!(
                "line {line_number}: expected 7 TSV columns, found {}",
                fields.len()
            )]);
        }
        rows.push(ParentageGapRow {
            title_id: fields[0].to_string(),
            name: fields[1].to_string(),
            rank: fields[2].to_string(),
            exists: fields[3].to_string(),
            parentage_count: fields[4].to_string(),
            review_priority: fields[5].to_string(),
            notes: fields[6].to_string(),
        });
    }

    Ok(rows)
}

fn parentage_gap_rows_to_tsv(rows: &[ParentageGapRow], header: &str) -> String {
    let mut output = String::new();
    output.push_str(header);
    output.push('\n');
    for row in rows {
        output.push_str(&tsv_escape(&row.title_id));
        output.push('\t');
        output.push_str(&tsv_escape(&row.name));
        output.push('\t');
        output.push_str(&tsv_escape(&row.rank));
        output.push('\t');
        output.push_str(&tsv_escape(&row.exists));
        output.push('\t');
        output.push_str(&tsv_escape(&row.parentage_count));
        output.push('\t');
        output.push_str(&tsv_escape(&row.review_priority));
        output.push('\t');
        output.push_str(&tsv_escape(&row.notes));
        output.push('\n');
    }
    output
}

fn parentage_gap_priority_counts(rows: &[ParentageGapRow]) -> ParentageGapPriorityCounts {
    let mut counts = ParentageGapPriorityCounts::default();
    for row in rows {
        match row.review_priority.as_str() {
            "high_parentage_review" => counts.high += 1,
            "medium_parentage_review" => counts.medium += 1,
            "root_or_successor_review" => counts.root += 1,
            _ => {}
        }
    }
    counts
}

fn parentage_gap_tsv_header() -> &'static str {
    "title_id\tname\trank\texists\tparentage_count\treview_priority\tnotes"
}

fn title_rank_label(rank: duchy::TitleRank) -> &'static str {
    match rank {
        duchy::TitleRank::County => "County",
        duchy::TitleRank::Duchy => "Duchy",
        duchy::TitleRank::Kingdom => "Kingdom",
        duchy::TitleRank::Crown => "Crown",
        duchy::TitleRank::Empire => "Empire",
    }
}

fn year_span_label(span: &duchy::YearSpan) -> String {
    match span.end {
        Some(end) => format!("{}..{end}", span.start),
        None => format!("{}..", span.start),
    }
}

fn parentage_gap_priority(rank: duchy::TitleRank) -> &'static str {
    match rank {
        duchy::TitleRank::County => "high_parentage_review",
        duchy::TitleRank::Duchy => "high_parentage_review",
        duchy::TitleRank::Kingdom => "medium_parentage_review",
        duchy::TitleRank::Crown => "medium_parentage_review",
        duchy::TitleRank::Empire => "root_or_successor_review",
    }
}

fn parentage_gap_note(rank: duchy::TitleRank) -> &'static str {
    match rank {
        duchy::TitleRank::County => "Find reviewed duchy, kingdom, or empire parentage source.",
        duchy::TitleRank::Duchy => "Find reviewed kingdom or empire parentage source.",
        duchy::TitleRank::Kingdom => {
            "Find reviewed empire, union, confederation, or successor-context source."
        }
        duchy::TitleRank::Crown => "Find reviewed empire, union, or successor-context source.",
        duchy::TitleRank::Empire => {
            "May be a root title; review only if successor, union, or super-entity claim exists."
        }
    }
}

fn tsv_escape(value: &str) -> String {
    value.replace(['\t', '\r', '\n'], " ").trim().to_string()
}

fn import_scope_label(import_scope: duchy::ImportScope) -> &'static str {
    match import_scope {
        duchy::ImportScope::TitleIdentityOnly => "title_identity_only",
        duchy::ImportScope::ParentageReady => "parentage_ready",
        duchy::ImportScope::TerritoryReady => "territory_ready",
        duchy::ImportScope::HolderReady => "holder_ready",
        duchy::ImportScope::ContestedReview => "contested_review",
    }
}

fn rank_basis_label(rank_basis: duchy::RankBasis) -> &'static str {
    match rank_basis {
        duchy::RankBasis::Literal => "literal",
        duchy::RankBasis::Normalized => "normalized",
        duchy::RankBasis::Approximate => "approximate",
        duchy::RankBasis::Unsupported => "unsupported",
    }
}

fn entity_class_label(entity_class: duchy::EntityClass) -> &'static str {
    match entity_class {
        duchy::EntityClass::County => "county",
        duchy::EntityClass::Duchy => "duchy",
        duchy::EntityClass::Kingdom => "kingdom",
        duchy::EntityClass::Principality => "principality",
        duchy::EntityClass::FreeCity => "free_city",
        duchy::EntityClass::TheocraticState => "theocratic_state",
        duchy::EntityClass::Confederation => "confederation",
        duchy::EntityClass::Empire => "empire",
        duchy::EntityClass::AdministrativeRegion => "administrative_region",
        duchy::EntityClass::Other => "other",
    }
}

fn confidence_detail_label(confidence_detail: duchy::ConfidenceDetail) -> &'static str {
    match confidence_detail {
        duchy::ConfidenceDetail::WikidataStructuredSingle => "wikidata_structured_single",
        duchy::ConfidenceDetail::WikidataPlusTextCrosscheck => "wikidata_plus_text_crosscheck",
        duchy::ConfidenceDetail::MultiSourceAgreement => "multi_source_agreement",
        duchy::ConfidenceDetail::DateConflict => "date_conflict",
        duchy::ConfidenceDetail::Unsupported => "unsupported",
    }
}

fn parentage_status_label(parentage_status: duchy::ParentageStatus) -> &'static str {
    match parentage_status {
        duchy::ParentageStatus::NoneReviewed => "none_reviewed",
        duchy::ParentageStatus::CandidateAvailable => "candidate_available",
        duchy::ParentageStatus::AcceptedPartial => "accepted_partial",
        duchy::ParentageStatus::AcceptedFull => "accepted_full",
        duchy::ParentageStatus::Contested => "contested",
    }
}

fn query_readiness_label(query_readiness: duchy::QueryReadiness) -> &'static str {
    match query_readiness {
        duchy::QueryReadiness::ExistenceOnly => "existence_only",
        duchy::QueryReadiness::TitlePath => "title_path",
        duchy::QueryReadiness::Transfer => "transfer",
        duchy::QueryReadiness::LineageEvent => "lineage_event",
        duchy::QueryReadiness::Unsupported => "unsupported",
    }
}

fn exclusion_reason_label(exclusion_reason: duchy::ExclusionReason) -> &'static str {
    match exclusion_reason {
        duchy::ExclusionReason::UnsupportedRank => "unsupported_rank",
        duchy::ExclusionReason::NonTitlePolity => "non_title_polity",
        duchy::ExclusionReason::AmbiguousEntity => "ambiguous_entity",
        duchy::ExclusionReason::DateConflict => "date_conflict",
        duchy::ExclusionReason::SuccessorPredecessorIssue => "successor_predecessor_issue",
        duchy::ExclusionReason::RightsBlocked => "rights_blocked",
        duchy::ExclusionReason::QualityBlocked => "quality_blocked",
        duchy::ExclusionReason::ScopeDeferred => "scope_deferred",
    }
}

fn push_optional_manifest_line(output: &mut String, key: &str, value: &str) {
    if !value.is_empty() {
        output.push_str(&format!("{key}: {value}\n"));
    }
}

struct CandidateStatusCounts {
    pending: usize,
    reviewed: usize,
    promoted: usize,
    rejected: usize,
}

fn candidate_status_counts(candidates: &[duchy::CandidateRecord]) -> CandidateStatusCounts {
    let mut counts = CandidateStatusCounts {
        pending: 0,
        reviewed: 0,
        promoted: 0,
        rejected: 0,
    };
    for candidate in candidates {
        match candidate.status {
            duchy::CandidateStatus::Pending => counts.pending += 1,
            duchy::CandidateStatus::Reviewed => counts.reviewed += 1,
            duchy::CandidateStatus::Promoted => counts.promoted += 1,
            duchy::CandidateStatus::Rejected => counts.rejected += 1,
        }
    }
    counts
}

fn push_manifest_report_section(
    output: &mut String,
    heading: &str,
    candidates: &[duchy::CandidateRecord],
    status: duchy::CandidateStatus,
) {
    output.push_str(&format!("## {heading}\n\n"));
    let matching = candidates
        .iter()
        .filter(|candidate| candidate.status == status)
        .collect::<Vec<_>>();
    if matching.is_empty() {
        output.push_str("none\n\n");
        return;
    }

    for candidate in matching {
        output.push_str(&format!("- candidate_id: {}\n", candidate.candidate_id));
        output.push_str(&format!("  source_id: {}\n", candidate.source_id));
        output.push_str(&format!("  source_url: {}\n", candidate.source_url));
        if let Some(review_batch_id) = &candidate.review_batch_id {
            output.push_str(&format!("  review_batch_id: {review_batch_id}\n"));
        }
        if let Some(import_scope) = candidate.import_scope {
            output.push_str(&format!(
                "  import_scope: {}\n",
                import_scope_label(import_scope)
            ));
        }
        if let Some(rank_basis) = candidate.rank_basis {
            output.push_str(&format!("  rank_basis: {}\n", rank_basis_label(rank_basis)));
        }
        if let Some(entity_class) = candidate.entity_class {
            output.push_str(&format!(
                "  entity_class: {}\n",
                entity_class_label(entity_class)
            ));
        }
        if !candidate.source_claims_used.is_empty() {
            output.push_str(&format!(
                "  source_claims_used: {}\n",
                candidate.source_claims_used.join(", ")
            ));
        }
        if let Some(confidence_detail) = candidate.confidence_detail {
            output.push_str(&format!(
                "  confidence_detail: {}\n",
                confidence_detail_label(confidence_detail)
            ));
        }
        if let Some(parentage_status) = candidate.parentage_status {
            output.push_str(&format!(
                "  parentage_status: {}\n",
                parentage_status_label(parentage_status)
            ));
        }
        if let Some(query_readiness) = candidate.query_readiness {
            output.push_str(&format!(
                "  query_readiness: {}\n",
                query_readiness_label(query_readiness)
            ));
        }
        if let Some(exclusion_reason) = candidate.exclusion_reason {
            output.push_str(&format!(
                "  exclusion_reason: {}\n",
                exclusion_reason_label(exclusion_reason)
            ));
        }
        if let Some(notes) = &candidate.notes {
            output.push_str(&format!("  notes: {notes}\n"));
        }
    }
    output.push('\n');
}

fn tsv_cell(value: &str) -> String {
    value
        .replace('\\', "\\\\")
        .replace('\t', "\\t")
        .replace('\r', "\\r")
        .replace('\n', "\\n")
}

fn manifest_tsv_cell(value: &str, line_number: usize) -> Result<String, Vec<String>> {
    let mut output = String::new();
    let mut chars = value.chars();
    while let Some(character) = chars.next() {
        if character != '\\' {
            output.push(character);
            continue;
        }
        let Some(escaped) = chars.next() else {
            return Err(vec![format!("line {line_number}: trailing TSV escape")]);
        };
        match escaped {
            '\\' => output.push('\\'),
            't' => output.push('\t'),
            'r' | 'n' => {
                return Err(vec![format!(
                    "line {line_number}: escaped line breaks are not supported by manifest import"
                )])
            }
            other => {
                return Err(vec![format!(
                    "line {line_number}: unsupported TSV escape \\{other}"
                )])
            }
        }
    }
    Ok(output)
}
