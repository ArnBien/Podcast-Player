use std::collections::HashMap;

struct Podcast {
    title: String,
    host: String,
    episodes: Vec<String>,
}

struct PodcastPlayer {
    podcasts: HashMap<String, Podcast>,
    history: Vec<String>,
}

impl PodcastPlayer {
    fn new() -> Self {
        PodcastPlayer {
            podcasts: HashMap::new(),
            history: Vec::new(),
        }
    }
    fn add_podcast(&mut self, title: &str, host: &str, episodes: Vec<&str>) {
        self.podcasts.insert(
            title.to_string(),
            Podcast {
                title: title.to_string(),
                host: host.to_string(),
                episodes: episodes.iter().map(|s| s.to_string()).collect(),
            },
        );
    }
    fn play_episode(&mut self, title: &str, ep_idx: usize) {
        if let Some(pod) = self.podcasts.get(title) {
            if ep_idx < pod.episodes.len() {
                println!("Playing '{}' - {}: {}", pod.title, pod.host, pod.episodes[ep_idx]);
                self.history.push(format!("{}: {}", pod.title, pod.episodes[ep_idx]));
            }
        }
    }
    fn show_history(&self) {
        println!("Play History:");
        for h in &self.history {
            println!("{}", h);
        }
    }
}

fn main() {
    let mut player = PodcastPlayer::new();
    player.add_podcast("TechTalk", "Zarokin", vec!["Ep1: AI", "Ep2: Web"]);
    player.add_podcast("Morning Show", "Alice", vec!["Ep1: News", "Ep2: Weather"]);
    player.play_episode("TechTalk", 0);
    player.play_episode("Morning Show", 1);
    player.show_history();
}