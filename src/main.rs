fn main() {
    let timeline = duchy::seed_timeline();
    timeline.validate().expect("seed timeline should validate");

    let year = 1050;
    println!("DUCHY seed snapshot for {year}");
    for title in timeline.titles_in_year(year) {
        let holder = timeline.holder_in_year(&title.id, year).unwrap_or("unheld");
        println!("- {:?}: {} ({holder})", title.rank, title.name);
    }

    let path_query = timeline.title_path_query_for_area_in_year("area_bridge_ford", year);
    let answer = path_query
        .answer
        .as_ref()
        .expect("seed area should have a title path");
    let path = answer
        .titles
        .iter()
        .map(|step| step.name.as_str())
        .collect::<Vec<_>>()
        .join(" -> ");
    println!();
    println!(
        "Path for area_bridge_ford in {year} [{:?}/{}]: {path}",
        path_query.status, path_query.trace[0].code
    );

    let transfer_query = timeline.transfers_query_for_area_between(
        "area_old_ford",
        duchy::TitleRank::Duchy,
        1000,
        1100,
    );
    let transfers = transfer_query
        .answer
        .as_ref()
        .expect("seed area should have duchy transfers");
    println!();
    println!(
        "Duchy transfers for area_old_ford [{:?}/{}]:",
        transfer_query.status, transfer_query.trace[0].code
    );
    for transfer in &transfers.transfers {
        println!(
            "- {}: {} -> {}",
            transfer.year, transfer.from_name, transfer.to_name
        );
    }

    let source_catalog = duchy::source_policy_catalog();
    source_catalog
        .validate()
        .expect("source policy catalog should validate");
    println!();
    println!("Source policy records: metadata-only gate validated");

    let fixture_catalog =
        duchy::SourceCatalog::from_metadata_text(include_str!("../fixtures/source-policy.sources"))
            .expect("source policy fixture should parse");
    fixture_catalog
        .validate()
        .expect("source policy fixture should validate");
    println!("Source fixture records: metadata file parsed");

    let blocked_fact = duchy::FactRecord {
        fact_id: "fact-demo-blocked".to_string(),
        subject_id: "c_demo".to_string(),
        claim_kind: duchy::ClaimKind::TitleExists,
        span: None,
        value: "exists".to_string(),
        source_ids: vec!["src-wikidata-licensing".to_string()],
        confidence: duchy::ConfidenceLabel::SingleSource,
        conflict_group: None,
    };
    assert!(source_catalog.validate_fact(&blocked_fact).is_err());
    println!("Source fact gate: metadata-only records cannot import facts");
}
