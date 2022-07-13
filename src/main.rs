use minigrep::tasks::*;
use std::time::Instant;
fn main() {
    let search_time = Instant::now();
    let (cfg, txt) = init_work_ui(parse_config());
    let num_of_target = show_dest_lines(cfg.str(), &txt[..], cfg.case_sensitive());
    use thousands::Separable;
    println!(
        "hit {} / {} row(s) totally\nat {} places",
        num_of_target.1.separate_with_commas(),
        num_of_target.0.separate_with_commas(),
        num_of_target.2.separate_with_commas(),
    );
    println!(
        "time consumption: {} ns\n",
        search_time.elapsed().as_nanos().separate_with_commas()
    );
}
