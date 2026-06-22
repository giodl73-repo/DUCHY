use std::collections::{BTreeMap, BTreeSet};
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
        if command == "parentage-change-report" {
            return parentage_change_report(sources_path, facts_path, output_path);
        }
        if command == "parentage-graph-report" {
            return parentage_graph_report(sources_path, facts_path, output_path);
        }
        if command == "parentage-gap-tsv" {
            return parentage_gap_tsv(sources_path, facts_path, output_path, None);
        }
    }
    if let [command, sources_path, facts_path, output_path, blockers_path] = args.as_slice() {
        if command == "parentage-gap-tsv" {
            return parentage_gap_tsv(sources_path, facts_path, output_path, Some(blockers_path));
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
                "usage: duchy-import [status [sources-file facts-file]] | manifest manifest-file | source-stubs manifest-file output.sources | rejected-report manifest-file output.md | active-manifest manifest-file output.manifest | archive-manifest manifest-file output.manifest | manifest-report manifest-file output.md | duplicate-url-report manifest-file output.md | manifest-tsv manifest-file output.tsv | manifest-from-tsv input.tsv output.manifest | shard-manifest manifest-file output-dir chunk-size | parentage-coverage-report sources-file facts-file output.md | parentage-change-report sources-file facts-file output.md | parentage-graph-report sources-file facts-file output.md | parentage-gap-tsv sources-file facts-file output.tsv [blockers.tsv] | parentage-gap-shard input.tsv output-dir chunk-size | parentage-gap-report input.tsv output.md".to_string(),
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

fn parentage_graph_report(
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

    let title_by_id = titles
        .iter()
        .map(|title| (title.id.as_str(), title))
        .collect::<BTreeMap<_, _>>();
    let parentage_facts = facts
        .iter()
        .filter(|fact| fact.claim_kind == duchy::ClaimKind::Parentage)
        .collect::<Vec<_>>();
    let parentage_by_child = parentage_by_child(&parentage_facts);
    let conflicts = parentage_conflict_rows(&parentage_facts, &title_by_id);
    let rank_skip_rows = parentage_rank_skip_rows(&parentage_facts, &title_by_id);
    let span_coverage = parentage_span_coverage_rows(&titles, &parentage_by_child);
    let snapshot_years = parentage_graph_snapshot_years(&titles, &parentage_facts);
    let snapshots = snapshot_years
        .iter()
        .map(|year| parentage_graph_snapshot(*year, &titles, &parentage_facts, &title_by_id))
        .collect::<Vec<_>>();

    let total_parentable_titles = titles
        .iter()
        .filter(|title| title_is_parentable(title))
        .count();
    let title_edges_possible = total_parentable_titles;
    let titles_with_parentage = parentage_by_child.len();
    let title_edge_fill_percent = percent(titles_with_parentage, title_edges_possible);
    let rank_skip_percent = percent(rank_skip_rows.len(), parentage_facts.len());
    let weighted_coverage_percent = weighted_parentage_coverage_percent(&span_coverage);
    let conflict_count = conflicts.len();
    let cycle_snapshots = snapshots
        .iter()
        .filter(|snapshot| !snapshot.cycles.is_empty())
        .count();

    let mut output = String::new();
    output.push_str("# DUCHY Parentage Graph Report\n\n");
    output.push_str(&format!("sources: {}\n", catalog.record_count()));
    output.push_str(&format!("facts: {}\n", facts.len()));
    output.push_str(&format!("titles: {}\n", titles.len()));
    output.push_str(&format!("parentage_facts: {}\n", parentage_facts.len()));
    output.push_str(&format!("parentable_titles: {total_parentable_titles}\n"));
    output.push_str(&format!("titles_with_parentage: {titles_with_parentage}\n"));
    output.push_str(&format!(
        "title_edge_fill_percent: {:.2}\n",
        title_edge_fill_percent
    ));
    output.push_str(&format!(
        "weighted_span_coverage_percent: {:.2}\n",
        weighted_coverage_percent
    ));
    output.push_str(&format!("rank_skip_facts: {}\n", rank_skip_rows.len()));
    output.push_str(&format!("rank_skip_percent: {:.2}\n", rank_skip_percent));
    output.push_str(&format!("temporal_parent_conflicts: {conflict_count}\n"));
    output.push_str(&format!("snapshot_years: {}\n", snapshots.len()));
    output.push_str(&format!(
        "snapshot_years_with_cycles: {cycle_snapshots}\n\n"
    ));

    output.push_str("## Interpretation\n\n");
    output.push_str("- DUCHY parentage is a temporal forest, not one timeless duchy tree.\n");
    output.push_str("- `title_edge_fill_percent` measures title-level parentage coverage for parentable ranks.\n");
    output.push_str("- `weighted_span_coverage_percent` measures how much of parentable title existence time is covered by parentage spans.\n");
    output.push_str("- `density_percent` in snapshots is active parent edges divided by active parentable titles for that year.\n");
    output.push_str("- `rank_skip_facts` are valid but indicate missing intermediate hierarchy such as duchy or crown layers.\n\n");

    output.push_str("## Snapshot Health\n\n");
    output.push_str("| Year | Active Titles | Parentable | Edges | Density % | Roots | Orphans | Conflicts | Cycles | Max Depth | Avg Depth |\n");
    output.push_str("|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|\n");
    for snapshot in &snapshots {
        output.push_str(&format!(
            "| {} | {} | {} | {} | {:.2} | {} | {} | {} | {} | {} | {:.2} |\n",
            snapshot.year,
            snapshot.active_titles,
            snapshot.active_parentable_titles,
            snapshot.active_edges,
            snapshot.density_percent,
            snapshot.roots,
            snapshot.orphans,
            snapshot.conflicts,
            snapshot.cycles.len(),
            snapshot.max_depth,
            snapshot.avg_depth
        ));
    }

    output.push_str("\n## Current Snapshot By Rank\n\n");
    if let Some(current_snapshot) = snapshots.iter().find(|snapshot| snapshot.year == 2026) {
        output.push_str("| Rank | Active | Parentable | With Parent | Orphans | Roots |\n");
        output.push_str("|---|---:|---:|---:|---:|---:|\n");
        for (rank, totals) in &current_snapshot.by_rank {
            output.push_str(&format!(
                "| {} | {} | {} | {} | {} | {} |\n",
                title_rank_label(*rank),
                totals.active,
                totals.parentable,
                totals.with_parent,
                totals.orphans,
                totals.roots
            ));
        }
    } else {
        output.push_str("No 2026 snapshot was generated.\n");
    }

    output.push_str("\n## Span Coverage By Rank\n\n");
    output.push_str("| Rank | Titles | Covered Years | Expected Years | Coverage % |\n");
    output.push_str("|---|---:|---:|---:|---:|\n");
    for (rank, totals) in parentage_span_coverage_by_rank(&span_coverage) {
        output.push_str(&format!(
            "| {} | {} | {} | {} | {:.2} |\n",
            title_rank_label(rank),
            totals.titles,
            totals.covered_years,
            totals.expected_years,
            totals.coverage_percent()
        ));
    }

    output.push_str("\n## Depth Distribution\n\n");
    output.push_str("| Year | Depth | Active Titles |\n");
    output.push_str("|---:|---:|---:|\n");
    for snapshot in &snapshots {
        for (depth, count) in &snapshot.depth_counts {
            output.push_str(&format!("| {} | {} | {} |\n", snapshot.year, depth, count));
        }
    }

    output.push_str("\n## Rank Skip Facts\n\n");
    if rank_skip_rows.is_empty() {
        output.push_str("none\n");
    } else {
        output.push_str("| Fact | Child | Child Rank | Parent | Parent Rank | Span |\n");
        output.push_str("|---|---|---|---|---|---|\n");
        for row in rank_skip_rows.iter().take(50) {
            output.push_str(&format!(
                "| {} | {} | {} | {} | {} | {} |\n",
                row.fact_id,
                markdown_escape(&row.child),
                title_rank_label(row.child_rank),
                markdown_escape(&row.parent),
                title_rank_label(row.parent_rank),
                row.span
            ));
        }
    }

    output.push_str("\n## Temporal Parent Conflicts\n\n");
    if conflicts.is_empty() {
        output.push_str("none\n");
    } else {
        output.push_str("| Child | Overlap | Parent Facts |\n");
        output.push_str("|---|---|---|\n");
        for conflict in &conflicts {
            output.push_str(&format!(
                "| {} | {} | {} |\n",
                markdown_escape(&conflict.child),
                conflict.overlap,
                markdown_escape(&conflict.parents.join("; "))
            ));
        }
    }

    output.push_str("\n## Snapshot Cycles\n\n");
    let cycle_rows = snapshots
        .iter()
        .flat_map(|snapshot| {
            snapshot
                .cycles
                .iter()
                .map(|cycle| (snapshot.year, cycle.clone()))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    if cycle_rows.is_empty() {
        output.push_str("none\n");
    } else {
        output.push_str("| Year | Cycle |\n");
        output.push_str("|---:|---|\n");
        for (year, cycle) in cycle_rows {
            output.push_str(&format!(
                "| {year} | {} |\n",
                markdown_escape(&cycle.join(" -> "))
            ));
        }
    }

    fs::write(output_path, output)
        .map_err(|error| vec![format!("failed to write {output_path}: {error}")])?;

    println!("DUCHY parentage graph report");
    println!("- titles: {}", titles.len());
    println!("- parentage facts: {}", parentage_facts.len());
    println!("- title edge fill: {:.2}%", title_edge_fill_percent);
    println!(
        "- weighted span coverage: {:.2}%",
        weighted_coverage_percent
    );
    println!("- temporal parent conflicts: {conflict_count}");
    println!("- snapshot years: {}", snapshots.len());
    println!("- output: {output_path}");

    Ok(())
}

fn parentage_change_report(
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

    let rows = parentage_change_rows(&facts, &titles);
    let changed_rows = rows
        .iter()
        .filter(|row| row.distinct_parent_count > 1)
        .collect::<Vec<_>>();
    let total_changes = rows.iter().map(|row| row.change_count).sum::<usize>();
    let county_rows = rows
        .iter()
        .filter(|row| row.child_rank == duchy::TitleRank::County)
        .collect::<Vec<_>>();
    let county_changes = county_rows
        .iter()
        .map(|row| row.change_count)
        .sum::<usize>();
    let county_changed_titles = county_rows
        .iter()
        .filter(|row| row.distinct_parent_count > 1)
        .count();

    let mut output = String::new();
    output.push_str("# DUCHY Parentage Change Report\n\n");
    output.push_str(&format!("sources: {}\n", catalog.record_count()));
    output.push_str(&format!("facts: {}\n", facts.len()));
    output.push_str(&format!("titles: {}\n", titles.len()));
    output.push_str(&format!("parentage_titles: {}\n", rows.len()));
    output.push_str(&format!(
        "titles_with_parent_changes: {}\n",
        changed_rows.len()
    ));
    output.push_str(&format!("parent_changes: {total_changes}\n"));
    output.push_str(&format!("county_parentage_titles: {}\n", county_rows.len()));
    output.push_str(&format!(
        "county_titles_with_parent_changes: {county_changed_titles}\n"
    ));
    output.push_str(&format!("county_parent_changes: {county_changes}\n\n"));

    output.push_str("## Changes By Child Rank\n\n");
    output.push_str("| Child Rank | Titles | Changed Titles | Parent Changes |\n");
    output.push_str("|---|---:|---:|---:|\n");
    for (rank, totals) in parentage_change_rank_totals(&rows) {
        output.push_str(&format!(
            "| {} | {} | {} | {} |\n",
            title_rank_label(rank),
            totals.titles,
            totals.changed_titles,
            totals.parent_changes
        ));
    }

    output.push_str("\n## County Parentage By Parent Rank\n\n");
    output.push_str("| Parent Rank | Facts | Changed Titles | Parent Changes |\n");
    output.push_str("|---|---:|---:|---:|\n");
    for (rank, totals) in parentage_change_parent_rank_totals(&rows, duchy::TitleRank::County) {
        output.push_str(&format!(
            "| {} | {} | {} | {} |\n",
            title_rank_label(rank),
            totals.facts,
            totals.changed_titles,
            totals.parent_changes
        ));
    }

    output.push_str("\n## Top Parent Changes\n\n");
    output.push_str("| Changes | Facts | Child | Rank | Parent Spans |\n");
    output.push_str("|---:|---:|---|---|---|\n");
    for row in rows.iter().take(25) {
        output.push_str(&format!(
            "| {} | {} | {} | {} | {} |\n",
            row.change_count,
            row.fact_count,
            markdown_escape(&format!("{} ({})", row.child_name, row.child_id)),
            title_rank_label(row.child_rank),
            markdown_escape(&row.parent_spans.join("; "))
        ));
    }

    fs::write(output_path, output)
        .map_err(|error| vec![format!("failed to write {output_path}: {error}")])
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
    blockers_path: Option<&String>,
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
    let blockers = match blockers_path {
        Some(path) => parentage_gap_blockers_from_file(path)?,
        None => BTreeMap::new(),
    };

    let mut output = String::new();
    output.push_str(parentage_gap_tsv_header());
    output.push('\n');
    let mut gap_count = 0;
    for title in titles
        .iter()
        .filter(|title| !parentage_by_child.contains_key(title.id.as_str()))
    {
        let blocker = blockers.get(title.id.as_str());
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
        output.push_str(
            blocker
                .map(|blocker| blocker.review_priority.as_str())
                .unwrap_or_else(|| parentage_gap_priority(title.rank)),
        );
        output.push('\t');
        output.push_str(
            &blocker
                .map(|blocker| blocker.notes.as_str())
                .unwrap_or_else(|| parentage_gap_note(title.rank)),
        );
        output.push('\n');
        gap_count += 1;
    }

    fs::write(output_path, output)
        .map_err(|error| vec![format!("failed to write {output_path}: {error}")])?;

    println!("DUCHY parentage gap TSV");
    println!("- titles: {}", titles.len());
    println!("- gap rows: {gap_count}");
    if let Some(path) = blockers_path {
        println!("- blocker review: {path}");
    }
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
    index.push_str("| Shard | Rows | High | Medium | Root | Blocked |\n");
    index.push_str("|---|---:|---:|---:|---:|---:|\n");

    let header = parentage_gap_tsv_header();
    for (index_number, chunk) in rows.chunks(chunk_size).enumerate() {
        let shard_name = format!("batch-{:03}.tsv", index_number + 1);
        let shard_path = Path::new(output_dir).join(&shard_name);
        let shard_text = parentage_gap_rows_to_tsv(chunk, header);
        fs::write(&shard_path, shard_text)
            .map_err(|error| vec![format!("failed to write {}: {error}", shard_path.display())])?;

        let counts = parentage_gap_priority_counts(chunk);
        index.push_str(&format!(
            "| {shard_name} | {} | {} | {} | {} | {} |\n",
            chunk.len(),
            counts.high,
            counts.medium,
            counts.root,
            counts.blocked
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

struct ParentageGapBlocker {
    review_priority: String,
    notes: String,
}

#[derive(Default)]
struct ParentageGapPriorityCounts {
    high: usize,
    medium: usize,
    root: usize,
    blocked: usize,
}

fn parentage_gap_blocker_tsv_header() -> &'static str {
    "title_id\treview_priority\tnotes"
}

#[derive(Debug, Clone)]
struct ParentageChangeRow {
    child_id: String,
    child_name: String,
    child_rank: duchy::TitleRank,
    fact_count: usize,
    distinct_parent_count: usize,
    change_count: usize,
    parent_spans: Vec<String>,
    parent_ranks: Vec<duchy::TitleRank>,
}

#[derive(Default)]
struct ParentageChangeRankTotals {
    titles: usize,
    changed_titles: usize,
    parent_changes: usize,
}

#[derive(Default)]
struct ParentageChangeParentRankTotals {
    facts: usize,
    changed_titles: usize,
    parent_changes: usize,
}

#[derive(Debug)]
struct ParentageConflictRow {
    child: String,
    overlap: String,
    parents: Vec<String>,
}

#[derive(Debug)]
struct ParentageRankSkipRow {
    fact_id: String,
    child: String,
    child_rank: duchy::TitleRank,
    parent: String,
    parent_rank: duchy::TitleRank,
    span: String,
}

#[derive(Debug)]
struct ParentageSpanCoverageRow {
    rank: duchy::TitleRank,
    expected_years: usize,
    covered_years: usize,
}

#[derive(Default)]
struct ParentageSpanCoverageTotals {
    titles: usize,
    expected_years: usize,
    covered_years: usize,
}

impl ParentageSpanCoverageTotals {
    fn coverage_percent(&self) -> f64 {
        percent(self.covered_years, self.expected_years)
    }
}

#[derive(Default)]
struct ParentageSnapshotRankTotals {
    active: usize,
    parentable: usize,
    with_parent: usize,
    orphans: usize,
    roots: usize,
}

struct ParentageGraphSnapshot {
    year: i32,
    active_titles: usize,
    active_parentable_titles: usize,
    active_edges: usize,
    density_percent: f64,
    roots: usize,
    orphans: usize,
    conflicts: usize,
    cycles: Vec<Vec<String>>,
    max_depth: usize,
    avg_depth: f64,
    by_rank: BTreeMap<duchy::TitleRank, ParentageSnapshotRankTotals>,
    depth_counts: BTreeMap<usize, usize>,
}

fn parentage_by_child<'a>(
    facts: &'a [&'a duchy::FactRecord],
) -> BTreeMap<String, Vec<&'a duchy::FactRecord>> {
    let mut by_child: BTreeMap<String, Vec<&duchy::FactRecord>> = BTreeMap::new();
    for fact in facts {
        by_child
            .entry(fact.subject_id.clone())
            .or_default()
            .push(*fact);
    }
    by_child
}

fn title_is_parentable(title: &duchy::Title) -> bool {
    title.rank.parent_rank().is_some()
}

fn percent(numerator: usize, denominator: usize) -> f64 {
    if denominator == 0 {
        0.0
    } else {
        (numerator as f64 / denominator as f64) * 100.0
    }
}

fn parentage_conflict_rows(
    facts: &[&duchy::FactRecord],
    title_by_id: &BTreeMap<&str, &duchy::Title>,
) -> Vec<ParentageConflictRow> {
    let by_child = parentage_by_child(facts);
    let mut rows = Vec::new();
    for (child_id, child_facts) in by_child {
        for left_index in 0..child_facts.len() {
            for right_index in (left_index + 1)..child_facts.len() {
                let left = child_facts[left_index];
                let right = child_facts[right_index];
                let (Some(left_span), Some(right_span)) = (&left.span, &right.span) else {
                    continue;
                };
                let Some(overlap) = span_overlap_label(left_span, right_span) else {
                    continue;
                };
                rows.push(ParentageConflictRow {
                    child: title_display(&child_id, title_by_id),
                    overlap,
                    parents: vec![
                        parentage_fact_display(left, title_by_id),
                        parentage_fact_display(right, title_by_id),
                    ],
                });
            }
        }
    }
    rows.sort_by(|left, right| {
        left.child
            .cmp(&right.child)
            .then_with(|| left.overlap.cmp(&right.overlap))
    });
    rows
}

fn parentage_rank_skip_rows(
    facts: &[&duchy::FactRecord],
    title_by_id: &BTreeMap<&str, &duchy::Title>,
) -> Vec<ParentageRankSkipRow> {
    let mut rows = Vec::new();
    for fact in facts {
        let (Some(child), Some(parent)) = (
            title_by_id.get(fact.subject_id.as_str()),
            title_by_id.get(fact.value.as_str()),
        ) else {
            continue;
        };
        let Some(expected_parent_rank) = child.rank.parent_rank() else {
            continue;
        };
        if parent.rank == expected_parent_rank {
            continue;
        }
        rows.push(ParentageRankSkipRow {
            fact_id: fact.fact_id.clone(),
            child: title_display(child.id.as_str(), title_by_id),
            child_rank: child.rank,
            parent: title_display(parent.id.as_str(), title_by_id),
            parent_rank: parent.rank,
            span: fact
                .span
                .as_ref()
                .map(year_span_label)
                .unwrap_or_else(|| "unspecified".to_string()),
        });
    }
    rows.sort_by(|left, right| {
        left.child
            .cmp(&right.child)
            .then_with(|| left.fact_id.cmp(&right.fact_id))
    });
    rows
}

fn parentage_span_coverage_rows(
    titles: &[duchy::Title],
    parentage_by_child: &BTreeMap<String, Vec<&duchy::FactRecord>>,
) -> Vec<ParentageSpanCoverageRow> {
    let audit_end_year = current_audit_year();
    let mut rows = Vec::new();
    for title in titles.iter().filter(|title| title_is_parentable(title)) {
        let expected_years = span_duration_clamped(&title.exists, audit_end_year);
        let intervals = parentage_by_child
            .get(&title.id)
            .map(|facts| {
                facts
                    .iter()
                    .filter_map(|fact| {
                        fact.span
                            .as_ref()
                            .and_then(|span| clipped_interval(span, &title.exists, audit_end_year))
                    })
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();
        rows.push(ParentageSpanCoverageRow {
            rank: title.rank,
            expected_years,
            covered_years: merged_year_count(intervals),
        });
    }
    rows
}

fn weighted_parentage_coverage_percent(rows: &[ParentageSpanCoverageRow]) -> f64 {
    let covered = rows.iter().map(|row| row.covered_years).sum::<usize>();
    let expected = rows.iter().map(|row| row.expected_years).sum::<usize>();
    percent(covered, expected)
}

fn parentage_span_coverage_by_rank(
    rows: &[ParentageSpanCoverageRow],
) -> BTreeMap<duchy::TitleRank, ParentageSpanCoverageTotals> {
    let mut totals: BTreeMap<duchy::TitleRank, ParentageSpanCoverageTotals> = BTreeMap::new();
    for row in rows {
        let entry = totals.entry(row.rank).or_default();
        entry.titles += 1;
        entry.expected_years += row.expected_years;
        entry.covered_years += row.covered_years;
    }
    totals
}

fn parentage_graph_snapshot_years(
    titles: &[duchy::Title],
    parentage_facts: &[&duchy::FactRecord],
) -> Vec<i32> {
    let mut years = BTreeSet::new();
    years.insert(current_audit_year());
    for title in titles {
        years.insert(title.exists.start);
        if let Some(end) = title.exists.end {
            years.insert(end);
            years.insert(end.saturating_add(1));
        }
    }
    for fact in parentage_facts {
        if let Some(span) = &fact.span {
            years.insert(span.start);
            if let Some(end) = span.end {
                years.insert(end);
                years.insert(end.saturating_add(1));
            }
        }
    }
    years
        .into_iter()
        .filter(|year| titles.iter().any(|title| title.exists.contains(*year)))
        .collect()
}

fn parentage_graph_snapshot(
    year: i32,
    titles: &[duchy::Title],
    parentage_facts: &[&duchy::FactRecord],
    title_by_id: &BTreeMap<&str, &duchy::Title>,
) -> ParentageGraphSnapshot {
    let active_titles = titles
        .iter()
        .filter(|title| title.exists.contains(year))
        .collect::<Vec<_>>();
    let active_title_ids = active_titles
        .iter()
        .map(|title| title.id.as_str())
        .collect::<BTreeSet<_>>();
    let mut active_by_child: BTreeMap<&str, Vec<&duchy::FactRecord>> = BTreeMap::new();
    for fact in parentage_facts {
        let Some(span) = &fact.span else {
            continue;
        };
        if !span.contains(year) {
            continue;
        }
        if !active_title_ids.contains(fact.subject_id.as_str())
            || !active_title_ids.contains(fact.value.as_str())
        {
            continue;
        }
        active_by_child
            .entry(fact.subject_id.as_str())
            .or_default()
            .push(*fact);
    }

    for facts in active_by_child.values_mut() {
        facts.sort_by(|left, right| {
            left.fact_id
                .cmp(&right.fact_id)
                .then_with(|| left.value.cmp(&right.value))
        });
    }

    let mut active_parent: BTreeMap<&str, &str> = BTreeMap::new();
    for (child_id, facts) in &active_by_child {
        if let Some(fact) = facts.first() {
            active_parent.insert(*child_id, fact.value.as_str());
        }
    }

    let active_edges = active_by_child.values().map(Vec::len).sum::<usize>();
    let conflicts = active_by_child
        .values()
        .filter(|facts| facts.len() > 1)
        .count();
    let active_parentable_titles = active_titles
        .iter()
        .filter(|title| title_is_parentable(title))
        .count();
    let orphans = active_titles
        .iter()
        .filter(|title| {
            title_is_parentable(title) && !active_by_child.contains_key(title.id.as_str())
        })
        .count();
    let roots = active_titles
        .iter()
        .filter(|title| {
            !title_is_parentable(title) || !active_by_child.contains_key(title.id.as_str())
        })
        .count();

    let cycles = parentage_snapshot_cycles(&active_parent, title_by_id);
    let mut depths = Vec::new();
    let mut depth_counts = BTreeMap::new();
    for title in &active_titles {
        let depth = parentage_snapshot_depth(title.id.as_str(), &active_parent);
        *depth_counts.entry(depth).or_default() += 1;
        depths.push(depth);
    }
    let max_depth = depths.iter().copied().max().unwrap_or_default();
    let avg_depth = if depths.is_empty() {
        0.0
    } else {
        depths.iter().sum::<usize>() as f64 / depths.len() as f64
    };

    let mut by_rank: BTreeMap<duchy::TitleRank, ParentageSnapshotRankTotals> = BTreeMap::new();
    for title in &active_titles {
        let entry = by_rank.entry(title.rank).or_default();
        entry.active += 1;
        if title_is_parentable(title) {
            entry.parentable += 1;
            if active_by_child.contains_key(title.id.as_str()) {
                entry.with_parent += 1;
            } else {
                entry.orphans += 1;
            }
        }
        if !title_is_parentable(title) || !active_by_child.contains_key(title.id.as_str()) {
            entry.roots += 1;
        }
    }

    ParentageGraphSnapshot {
        year,
        active_titles: active_titles.len(),
        active_parentable_titles,
        active_edges,
        density_percent: percent(active_edges, active_parentable_titles),
        roots,
        orphans,
        conflicts,
        cycles,
        max_depth,
        avg_depth,
        by_rank,
        depth_counts,
    }
}

fn parentage_snapshot_cycles(
    active_parent: &BTreeMap<&str, &str>,
    title_by_id: &BTreeMap<&str, &duchy::Title>,
) -> Vec<Vec<String>> {
    let mut cycles = Vec::new();
    let mut seen_keys = BTreeSet::new();
    for start in active_parent.keys() {
        let mut path = Vec::new();
        let mut positions = BTreeMap::new();
        let mut current = *start;
        while let Some(parent) = active_parent.get(current) {
            if let Some(position) = positions.get(current) {
                let cycle_ids = path[*position..].to_vec();
                let key = canonical_cycle_key(&cycle_ids);
                if seen_keys.insert(key) {
                    cycles.push(
                        cycle_ids
                            .iter()
                            .map(|title_id| title_display(title_id, title_by_id))
                            .collect(),
                    );
                }
                break;
            }
            positions.insert(current, path.len());
            path.push(current);
            current = parent;
        }
    }
    cycles
}

fn parentage_snapshot_depth(start: &str, active_parent: &BTreeMap<&str, &str>) -> usize {
    let mut depth = 0;
    let mut seen = BTreeSet::new();
    let mut current = start;
    while let Some(parent) = active_parent.get(current) {
        if !seen.insert(current) {
            break;
        }
        depth += 1;
        current = parent;
    }
    depth
}

fn canonical_cycle_key(cycle: &[&str]) -> String {
    let mut values = cycle.iter().copied().collect::<Vec<_>>();
    values.sort_unstable();
    values.join("|")
}

fn current_audit_year() -> i32 {
    2026
}

fn span_duration_clamped(span: &duchy::YearSpan, audit_end_year: i32) -> usize {
    let end = span.end.unwrap_or(audit_end_year).min(audit_end_year);
    if span.start > end {
        0
    } else {
        (end - span.start + 1) as usize
    }
}

fn clipped_interval(
    span: &duchy::YearSpan,
    bounds: &duchy::YearSpan,
    audit_end_year: i32,
) -> Option<(i32, i32)> {
    let start = span.start.max(bounds.start);
    let end = span
        .end
        .unwrap_or(audit_end_year)
        .min(bounds.end.unwrap_or(audit_end_year))
        .min(audit_end_year);
    if start <= end {
        Some((start, end))
    } else {
        None
    }
}

fn merged_year_count(mut intervals: Vec<(i32, i32)>) -> usize {
    if intervals.is_empty() {
        return 0;
    }
    intervals.sort_unstable();
    let mut total = 0;
    let mut current = intervals[0];
    for interval in intervals.into_iter().skip(1) {
        if interval.0 <= current.1.saturating_add(1) {
            current.1 = current.1.max(interval.1);
        } else {
            total += (current.1 - current.0 + 1) as usize;
            current = interval;
        }
    }
    total + (current.1 - current.0 + 1) as usize
}

fn span_overlap_label(left: &duchy::YearSpan, right: &duchy::YearSpan) -> Option<String> {
    let start = left.start.max(right.start);
    let left_end = left.end.unwrap_or(i32::MAX);
    let right_end = right.end.unwrap_or(i32::MAX);
    let end = left_end.min(right_end);
    if start > end {
        return None;
    }
    if end == i32::MAX {
        Some(format!("{start}.."))
    } else {
        Some(format!("{start}..{end}"))
    }
}

fn parentage_fact_display(
    fact: &duchy::FactRecord,
    title_by_id: &BTreeMap<&str, &duchy::Title>,
) -> String {
    let parent = title_display(fact.value.as_str(), title_by_id);
    let span = fact
        .span
        .as_ref()
        .map(year_span_label)
        .unwrap_or_else(|| "unspecified".to_string());
    format!("{} -> {parent} [{span}]", fact.fact_id)
}

fn title_display(title_id: &str, title_by_id: &BTreeMap<&str, &duchy::Title>) -> String {
    title_by_id
        .get(title_id)
        .map(|title| format!("{} ({})", title.name, title.id))
        .unwrap_or_else(|| title_id.to_string())
}

fn parentage_change_rows(
    facts: &[duchy::FactRecord],
    titles: &[duchy::Title],
) -> Vec<ParentageChangeRow> {
    let title_by_id = titles
        .iter()
        .map(|title| (title.id.as_str(), title))
        .collect::<BTreeMap<_, _>>();
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

    let mut rows = Vec::new();
    for (child_id, mut child_facts) in parentage_by_child {
        let Some(child) = title_by_id.get(child_id) else {
            continue;
        };
        child_facts.sort_by_key(|fact| {
            fact.span
                .as_ref()
                .map(|span| span.start)
                .unwrap_or(i32::MIN)
        });

        let mut distinct_parents = Vec::<&str>::new();
        let mut parent_spans = Vec::new();
        let mut parent_ranks = Vec::new();
        for fact in &child_facts {
            let Some(parent) = title_by_id.get(fact.value.as_str()) else {
                continue;
            };
            if !distinct_parents
                .iter()
                .any(|parent_id| *parent_id == parent.id.as_str())
            {
                distinct_parents.push(parent.id.as_str());
            }
            parent_ranks.push(parent.rank);
            parent_spans.push(format!(
                "{}: {} [{}]",
                fact.span
                    .as_ref()
                    .map(year_span_label)
                    .unwrap_or_else(|| "unspanned".to_string()),
                parent.name,
                title_rank_label(parent.rank)
            ));
        }
        let distinct_parent_count = distinct_parents.len();
        rows.push(ParentageChangeRow {
            child_id: child.id.clone(),
            child_name: child.name.clone(),
            child_rank: child.rank,
            fact_count: child_facts.len(),
            distinct_parent_count,
            change_count: distinct_parent_count.saturating_sub(1),
            parent_spans,
            parent_ranks,
        });
    }

    rows.sort_by(|left, right| {
        right
            .change_count
            .cmp(&left.change_count)
            .then_with(|| right.fact_count.cmp(&left.fact_count))
            .then_with(|| left.child_name.cmp(&right.child_name))
    });
    rows
}

fn parentage_change_rank_totals(
    rows: &[ParentageChangeRow],
) -> BTreeMap<duchy::TitleRank, ParentageChangeRankTotals> {
    let mut totals: BTreeMap<duchy::TitleRank, ParentageChangeRankTotals> = BTreeMap::new();
    for row in rows {
        let entry = totals.entry(row.child_rank).or_default();
        entry.titles += 1;
        if row.distinct_parent_count > 1 {
            entry.changed_titles += 1;
        }
        entry.parent_changes += row.change_count;
    }
    totals
}

fn parentage_change_parent_rank_totals(
    rows: &[ParentageChangeRow],
    child_rank: duchy::TitleRank,
) -> BTreeMap<duchy::TitleRank, ParentageChangeParentRankTotals> {
    let mut totals: BTreeMap<duchy::TitleRank, ParentageChangeParentRankTotals> = BTreeMap::new();
    for row in rows.iter().filter(|row| row.child_rank == child_rank) {
        for parent_rank in &row.parent_ranks {
            totals.entry(*parent_rank).or_default().facts += 1;
        }
        if row.distinct_parent_count > 1 {
            for parent_rank in row
                .parent_ranks
                .iter()
                .copied()
                .collect::<std::collections::BTreeSet<_>>()
            {
                let entry = totals.entry(parent_rank).or_default();
                entry.changed_titles += 1;
                entry.parent_changes += row.change_count;
            }
        }
    }
    totals
}

fn parentage_gap_blockers_from_file(
    blockers_path: &str,
) -> Result<BTreeMap<String, ParentageGapBlocker>, Vec<String>> {
    let blocker_text = fs::read_to_string(blockers_path)
        .map_err(|error| vec![format!("failed to read {blockers_path}: {error}")])?;
    parentage_gap_blockers_from_tsv(&blocker_text)
}

fn parentage_gap_blockers_from_tsv(
    input: &str,
) -> Result<BTreeMap<String, ParentageGapBlocker>, Vec<String>> {
    let mut lines = input.lines();
    let Some(header) = lines.next() else {
        return Err(vec!["parentage gap blocker TSV is empty".to_string()]);
    };
    let expected_header = parentage_gap_blocker_tsv_header();
    if header != expected_header {
        return Err(vec![format!(
            "invalid parentage gap blocker TSV header: expected {expected_header}"
        )]);
    }

    let mut blockers = BTreeMap::new();
    let mut errors = Vec::new();
    for (line_index, line) in lines.enumerate() {
        if line.trim().is_empty() {
            continue;
        }
        let line_number = line_index + 2;
        let fields = line.split('\t').collect::<Vec<_>>();
        if fields.len() != 3 {
            errors.push(format!(
                "line {line_number}: expected 3 TSV columns, found {}",
                fields.len()
            ));
            continue;
        }
        let title_id = fields[0].trim();
        let review_priority = fields[1].trim();
        let notes = fields[2].trim();
        let mut line_errors = Vec::new();
        if title_id.is_empty() {
            line_errors.push(format!("line {line_number}: title_id is required"));
        }
        if review_priority != "blocked_parentage_review" {
            line_errors.push(format!(
                "line {line_number}: unsupported blocker review_priority {review_priority}"
            ));
        }
        if notes.is_empty() {
            line_errors.push(format!("line {line_number}: notes are required"));
        }
        if !line_errors.is_empty() {
            errors.extend(line_errors);
            continue;
        }
        if blockers
            .insert(
                title_id.to_string(),
                ParentageGapBlocker {
                    review_priority: review_priority.to_string(),
                    notes: notes.to_string(),
                },
            )
            .is_some()
        {
            errors.push(format!("line {line_number}: duplicate blocker {title_id}"));
        }
    }
    if errors.is_empty() {
        Ok(blockers)
    } else {
        Err(errors)
    }
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
            "blocked_parentage_review" => counts.blocked += 1,
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
        duchy::TitleRank::Province => "Province",
        duchy::TitleRank::FreeCity => "FreeCity",
        duchy::TitleRank::Kingdom => "Kingdom",
        duchy::TitleRank::Crown => "Crown",
        duchy::TitleRank::TheocraticState => "TheocraticState",
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
        duchy::TitleRank::Province => "high_parentage_review",
        duchy::TitleRank::FreeCity => "medium_parentage_review",
        duchy::TitleRank::Kingdom => "medium_parentage_review",
        duchy::TitleRank::Crown => "medium_parentage_review",
        duchy::TitleRank::TheocraticState => "medium_parentage_review",
        duchy::TitleRank::Empire => "root_or_successor_review",
    }
}

fn parentage_gap_note(rank: duchy::TitleRank) -> &'static str {
    match rank {
        duchy::TitleRank::County => "Find reviewed duchy, kingdom, or empire parentage source.",
        duchy::TitleRank::Duchy => "Find reviewed kingdom or empire parentage source.",
        duchy::TitleRank::Province => "Find reviewed kingdom or empire parentage source.",
        duchy::TitleRank::FreeCity => "Find reviewed empire, union, or successor-context source.",
        duchy::TitleRank::Kingdom => {
            "Find reviewed empire, union, confederation, or successor-context source."
        }
        duchy::TitleRank::Crown => "Find reviewed empire, union, or successor-context source.",
        duchy::TitleRank::TheocraticState => {
            "Find reviewed empire, union, or successor-context source."
        }
        duchy::TitleRank::Empire => {
            "May be a root title; review only if successor, union, or super-entity claim exists."
        }
    }
}

fn tsv_escape(value: &str) -> String {
    value.replace(['\t', '\r', '\n'], " ").trim().to_string()
}

fn markdown_escape(value: &str) -> String {
    value
        .replace('\\', "\\\\")
        .replace('|', "\\|")
        .replace('\r', " ")
        .replace('\n', " ")
        .trim()
        .to_string()
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_parentage_gap_blockers() {
        let input = concat!(
            "title_id\treview_priority\tnotes\n",
            "title-demo-a\tblocked_parentage_review\tNeeds relation-status modeling.\n",
            "title-demo-b\tblocked_parentage_review\tNeeds split-control modeling.\n",
        );

        let blockers = parentage_gap_blockers_from_tsv(input).expect("blockers should parse");

        assert_eq!(blockers.len(), 2);
        assert_eq!(
            blockers
                .get("title-demo-a")
                .expect("first blocker")
                .review_priority,
            "blocked_parentage_review"
        );
    }

    #[test]
    fn rejects_invalid_parentage_gap_blocker_priority() {
        let input = concat!(
            "title_id\treview_priority\tnotes\n",
            "title-demo-a\thigh_parentage_review\tNeeds relation-status modeling.\n",
        );

        let errors = match parentage_gap_blockers_from_tsv(input) {
            Ok(_) => panic!("priority should fail"),
            Err(errors) => errors,
        };

        assert!(errors
            .iter()
            .any(|error| error.contains("unsupported blocker review_priority")));
    }

    #[test]
    fn counts_blocked_parentage_gap_rows() {
        let rows = vec![
            ParentageGapRow {
                title_id: "title-demo-a".to_string(),
                name: "Demo A".to_string(),
                rank: "Duchy".to_string(),
                exists: "1..2".to_string(),
                parentage_count: "0".to_string(),
                review_priority: "blocked_parentage_review".to_string(),
                notes: "Needs relation-status modeling.".to_string(),
            },
            ParentageGapRow {
                title_id: "title-demo-b".to_string(),
                name: "Demo B".to_string(),
                rank: "Duchy".to_string(),
                exists: "1..2".to_string(),
                parentage_count: "0".to_string(),
                review_priority: "high_parentage_review".to_string(),
                notes: "Find reviewed parentage source.".to_string(),
            },
        ];

        let counts = parentage_gap_priority_counts(&rows);

        assert_eq!(counts.blocked, 1);
        assert_eq!(counts.high, 1);
    }

    #[test]
    fn counts_parentage_changes_by_distinct_parent() {
        let titles = vec![
            title("title-demo-child", "Demo Child", duchy::TitleRank::County),
            title("title-demo-a", "Demo Parent A", duchy::TitleRank::Duchy),
            title("title-demo-b", "Demo Parent B", duchy::TitleRank::Duchy),
        ];
        let facts = vec![
            parentage_fact("fact-demo-1", "title-demo-child", "title-demo-a", 1, 10),
            parentage_fact("fact-demo-2", "title-demo-child", "title-demo-b", 11, 20),
        ];

        let rows = parentage_change_rows(&facts, &titles);

        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].child_rank, duchy::TitleRank::County);
        assert_eq!(rows[0].fact_count, 2);
        assert_eq!(rows[0].distinct_parent_count, 2);
        assert_eq!(rows[0].change_count, 1);
    }

    #[test]
    fn parent_rank_totals_are_limited_to_requested_child_rank() {
        let titles = vec![
            title("title-demo-county", "Demo County", duchy::TitleRank::County),
            title("title-demo-duchy", "Demo Duchy", duchy::TitleRank::Duchy),
            title(
                "title-demo-kingdom",
                "Demo Kingdom",
                duchy::TitleRank::Kingdom,
            ),
            title("title-demo-peer", "Demo Peer", duchy::TitleRank::Duchy),
        ];
        let facts = vec![
            parentage_fact(
                "fact-demo-1",
                "title-demo-county",
                "title-demo-duchy",
                1,
                10,
            ),
            parentage_fact(
                "fact-demo-2",
                "title-demo-county",
                "title-demo-kingdom",
                11,
                20,
            ),
            parentage_fact(
                "fact-demo-3",
                "title-demo-peer",
                "title-demo-kingdom",
                1,
                20,
            ),
        ];

        let rows = parentage_change_rows(&facts, &titles);
        let totals = parentage_change_parent_rank_totals(&rows, duchy::TitleRank::County);

        assert_eq!(
            totals
                .get(&duchy::TitleRank::Duchy)
                .expect("duchy parent total")
                .facts,
            1
        );
        assert_eq!(
            totals
                .get(&duchy::TitleRank::Kingdom)
                .expect("kingdom parent total")
                .facts,
            1
        );
    }

    fn title(id: &str, name: &str, rank: duchy::TitleRank) -> duchy::Title {
        duchy::Title {
            id: id.to_string(),
            name: name.to_string(),
            rank,
            exists: duchy::YearSpan::new(1, Some(20)),
            de_jure_parent: None,
        }
    }

    fn parentage_fact(
        fact_id: &str,
        child_id: &str,
        parent_id: &str,
        start: duchy::Year,
        end: duchy::Year,
    ) -> duchy::FactRecord {
        duchy::FactRecord {
            fact_id: fact_id.to_string(),
            subject_id: child_id.to_string(),
            claim_kind: duchy::ClaimKind::Parentage,
            span: Some(duchy::YearSpan::new(start, Some(end))),
            value: parent_id.to_string(),
            source_ids: vec!["src-demo".to_string()],
            confidence: duchy::ConfidenceLabel::SingleSource,
            conflict_group: None,
        }
    }
}
