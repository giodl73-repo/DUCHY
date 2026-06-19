fn main() {
    let timeline = duchy::seed_timeline();
    timeline.validate().expect("seed timeline should validate");

    let year = 1050;
    println!("DUCHY seed snapshot for {year}");
    for title in timeline.titles_in_year(year) {
        let holder = timeline.holder_in_year(&title.id, year).unwrap_or("unheld");
        println!("- {:?}: {} ({holder})", title.rank, title.name);
    }

    let answer = timeline
        .title_path_for_area_in_year("area_bridge_ford", year)
        .expect("seed area should have a title path");
    let path = answer
        .titles
        .iter()
        .map(|step| step.name.as_str())
        .collect::<Vec<_>>()
        .join(" -> ");
    println!();
    println!("Path for area_bridge_ford in {year}: {path}");
}
