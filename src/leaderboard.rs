use serenity::http::Http;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Leaderboard {
    pub channel_id: u64,
    pub message_id: u64,
    pub leaderboard: Option<HashMap<u64, (u32, u32)>>,
}

impl Leaderboard {
    pub fn get_sorted_results(&self) -> Vec<(u64, (u32, u32))> {
        if let Some(lb) = &self.leaderboard {
            let mut lb_vec: Vec<(u64,(u32, u32))> = lb.iter().map(|(k,v)| (*k,*v)).collect();
            lb_vec.sort_by(|(_ka,va), (_kb,vb)| va.cmp(vb).reverse());
            lb_vec
        } else {
            Vec::new()
        }
    }

    pub async fn get_formatted(&self, http: &Http) -> String {
        let lb_sorted = self.get_sorted_results();
        let mut content = String::from("```md\n === Leaderboard ===\n");
        let mut i = 1;
        for (id, (wins, podiums)) in lb_sorted {
            if i > 50 { break; } // Show a maximum of top 50
            let name = http.get_user(id).await.map(|user| format!("{}#{}", user.name, user.discriminator)).unwrap_or(format!("{}", id));
            content.push_str(&format!("[{}][ {} wins / {} podiums - {} ]\n", i, wins, podiums, name));
            i += 1;
        }
        content.push_str("```");
        content
    }
}
