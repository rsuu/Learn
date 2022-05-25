#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    use octocrab::{models, params, Octocrab};

    let token = std::env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN env variable is required");

    let octocrab = Octocrab::builder().personal_token(token).build()?;

    let mut page = octocrab
        .issues("rsuu", "test")
        .list()
        .state(params::State::All)
        .per_page(1)
        .send()
        .await?;

    let mut issues_count: usize;
    'l: loop {
        for issue in &page {
            issues_count = issue
                .html_url
                .as_ref()
                .split('/')
                .collect::<Vec<&str>>()
                .last()
                .unwrap()
                .parse::<usize>()
                .unwrap_or(0);
            if issues_count != 0 {
                println!("{}", issues_count);
                break 'l;
            }
        }
        page = match octocrab
            .get_page::<models::issues::Issue>(&page.next)
            .await?
        {
            Some(next_page) => next_page,
            None => break 'l,
        }
    }
    Ok(())
}
