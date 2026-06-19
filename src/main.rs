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

    duchy::validate_first_real_facts().expect("first real facts should pass source custody");
    println!("Reviewed real facts: fixture-backed facts and parentage validated");

    let first_real_fixture_catalog = duchy::first_real_source_catalog_from_fixture()
        .expect("first real source fixture should parse");
    println!("Reviewed real source fixture: metadata file parsed");

    let first_real_fixture_facts = duchy::first_real_fact_records_from_fixture()
        .expect("first real fact fixture should parse");
    for fact in &first_real_fixture_facts {
        first_real_fixture_catalog
            .validate_fact(fact)
            .expect("first real fixture fact should validate");
    }
    println!(
        "Reviewed real fact fixture: {} records parsed",
        first_real_fixture_facts.len()
    );

    let first_real_titles =
        duchy::first_real_titles_from_fixture().expect("first real title should materialize");
    for title in &first_real_titles {
        println!(
            "Reviewed real title: {:?} {} ({}-{})",
            title.rank,
            title.name,
            title.exists.start,
            title
                .exists
                .end
                .map_or_else(|| "present".to_string(), |end| end.to_string())
        );
    }

    let first_real_timeline = duchy::first_real_timeline_from_fixture()
        .expect("first real fixture timeline should materialize");
    if let Some(title) = first_real_titles.first() {
        let first_real_query = first_real_timeline.title_path_query_for_title_in_year(
            &title.id,
            title.exists.start,
            duchy::SourceClass::SourceBacked,
        );
        println!(
            "Reviewed real title query [{:?}/{}]: {}",
            first_real_query.status,
            first_real_query.trace[0].code,
            first_real_query
                .answer
                .as_ref()
                .and_then(|answer| answer.titles.first())
                .map_or("unanswered", |step| step.name.as_str())
        );
    }
    for parentage_fact in first_real_fixture_facts
        .iter()
        .filter(|fact| fact.claim_kind == duchy::ClaimKind::Parentage)
    {
        let Some(span) = &parentage_fact.span else {
            continue;
        };
        let query = first_real_timeline.title_path_query_for_title_in_year(
            &parentage_fact.subject_id,
            span.start,
            duchy::SourceClass::SourceBacked,
        );
        let path = query
            .answer
            .as_ref()
            .map(|answer| {
                answer
                    .titles
                    .iter()
                    .map(|step| step.name.as_str())
                    .collect::<Vec<_>>()
                    .join(" -> ")
            })
            .unwrap_or_else(|| "unanswered".to_string());
        println!(
            "Reviewed real parentage query [{:?}/{}]: {}",
            query.status, query.trace[0].code, path
        );
    }

    let reviewed_source_id = first_real_fixture_facts
        .iter()
        .find_map(|fact| fact.source_ids.first())
        .expect("reviewed fixture should contain at least one source")
        .clone();
    let contested_demo = vec![
        duchy::FactRecord {
            fact_id: "fact-demo-contested-rank-duchy".to_string(),
            subject_id: "title-demo-contested".to_string(),
            claim_kind: duchy::ClaimKind::Rank,
            span: None,
            value: "Duchy".to_string(),
            source_ids: vec![reviewed_source_id.clone()],
            confidence: duchy::ConfidenceLabel::Contested,
            conflict_group: Some("conflict-demo-rank".to_string()),
        },
        duchy::FactRecord {
            fact_id: "fact-demo-contested-rank-kingdom".to_string(),
            subject_id: "title-demo-contested".to_string(),
            claim_kind: duchy::ClaimKind::Rank,
            span: None,
            value: "Kingdom".to_string(),
            source_ids: vec![reviewed_source_id],
            confidence: duchy::ConfidenceLabel::Contested,
            conflict_group: Some("conflict-demo-rank".to_string()),
        },
    ];
    let contested_groups = duchy::contested_fact_groups(&contested_demo);
    println!(
        "Contested fact review: {} group requires resolution before materialization",
        contested_groups.len()
    );
}
