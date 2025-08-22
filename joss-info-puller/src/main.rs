#[tokio::main]
async fn main() {
    let octo = octocrab::instance();

    let labels = ["published".to_string()];

    let mut page = octo
        .issues("openjournals", "joss-reviews")
        .list()
        .labels(&labels)
        .state(octocrab::params::State::Closed)
        .per_page(100)
        .send()
        .await
        .unwrap();

    let mut k = 0;
    loop {
        for issue in &page {
            let mut subpage = octo
                .issues("openjournals", "joss-reviews")
                .list_comments(issue.number)
                .per_page(100)
                .send()
                .await
                .unwrap();
            'subpage_loop: loop {
                for comment in &subpage {
                    if comment
                        .body
                        .as_ref()
                        .is_some_and(|x| x.contains("Rust") || x.contains("rust"))
                    {
                        println!("{k:4} {}", issue.title);
                        k += 1;
                        break 'subpage_loop;
                    }
                }

                subpage = match octo
                    .get_page::<octocrab::models::issues::Comment>(&subpage.next)
                    .await
                    .unwrap()
                {
                    Some(next_subpage) => next_subpage,
                    None => break,
                }
            }
        }
        page = match octo
            .get_page::<octocrab::models::issues::Issue>(&page.next)
            .await
            .unwrap()
        {
            Some(next_page) => next_page,
            None => break,
        }
    }
}
