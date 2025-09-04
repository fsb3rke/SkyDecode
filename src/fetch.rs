use indicatif::ProgressBar;
use reqwest;
use std::time::Duration;

pub async fn fetch_metar(ids: &str) -> Result<String, reqwest::Error> {
    let url = format!(
        "https://aviationweather.gov/cgi-bin/data/metar.php?datasource=metars&requestType=retrieve&ids={}&hoursBeforeNo==1",
        ids
    );

    let pb = ProgressBar::new_spinner();
    pb.set_message("METAR loading...");
    pb.enable_steady_tick(Duration::from_millis(100));

    let body = reqwest::get(&url).await?.text().await?;

    pb.finish_with_message("OK");
    Ok(body)
}
