use reqwest::blocking::Client;

use super::consts::HACKER_NEWS_API;
use super::posts::Post;

#[derive(Default)]
pub struct Query {
    client: Client,
    ids: Vec<u128>,
    posts: Vec<Post>,
    last_filter: Filter,
    last_fetched: usize,
}

#[derive(Default, PartialEq)]
pub enum Filter {
    #[default]
    Top,
    New,
    Best,
}

impl Query {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn update(&mut self, filter: Filter) -> Result<(), reqwest::Error> {
        let filter_str = match filter {
            Filter::Top => "topstories",
            Filter::New => "newstories",
            Filter::Best => "beststories",
        };

        if self.last_filter != filter {
            self.last_fetched = 0;
            self.posts.clear();
        }

        self.ids = self.stories(filter_str)?;
        self.last_filter = filter;

        let _ = self.fetch_items(5)?;
        Ok(())
    }

    pub fn posts(&self) -> &Vec<Post> {
        &self.posts
    }

    fn fetch_items(&mut self, pagination: usize) -> Result<(), reqwest::Error> {
        for id in &self.ids[self.last_fetched..self.last_fetched + pagination] {
            let url = format!("{HACKER_NEWS_API}/item/{}.json", id);
            println!("Fetching: {}", url);
            let post: Post = self.client.get(&url).send()?.json()?;
            if post.post_type == Some("story".to_string()) && post.url.is_some() {
                self.posts.push(post);
            }
        }
        self.last_fetched += pagination;

        Ok(())
    }

    fn stories(&self, filter: &str) -> Result<Vec<u128>, reqwest::Error> {
        let url = format!("{HACKER_NEWS_API}/{filter}.json");
        let ids: Vec<u128> = self.client.get(&url).send()?.json::<Vec<u128>>()?;
        Ok(ids)
    }
}
