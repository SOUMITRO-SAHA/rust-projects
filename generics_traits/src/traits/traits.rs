// Define the Summary trait
pub trait Summary {
    fn summarize(&self) -> String;
}

// Define the Article struct
#[derive(Debug)]
pub struct Article {
    pub headline: String,
    pub content: String,
}

// Implement the Summary trait for Article
impl Summary for Article {
    fn summarize(&self) -> String {
        format!("{}: {}", self.headline, self.content)
    }
}

// Define the Tweet struct
#[derive(Debug)]
pub struct Tweet {
    pub username: String,
    pub content: String,
}

// Implement the Summary trait for Tweet
impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

// Test function for demonstrating traits
pub fn run_traits_test() {
    // Create an instance of Article
    let article = Article {
        headline: String::from("Breaking News!"),
        content: String::from("This is the content of the article."),
    };

    // Create an instance of Tweet
    let tweet = Tweet {
        username: String::from("user123"),
        content: String::from("This is a tweet."),
    };

    // Print the purpose of the following code
    println!("--- Running Traits Test ---");
    println!("This test demonstrates how the Summary trait is implemented for different types (Article and Tweet).");

    // Print before summarizing
    println!("\nBefore summarizing:");
    println!("Article: {:?}", article);
    println!("Tweet: {:?}", tweet);

    // Print the summaries
    println!("\nSummarizing the content using the Summary trait:");
    println!("Article Summary: {}", article.summarize());
    println!("Tweet Summary: {}", tweet.summarize());

    // Print after summarizing
    println!("\nAfter summarizing:");
    println!("Article summary method called: {}", article.summarize());
    println!("Tweet summary method called: {}", tweet.summarize());
}
