#[derive(Debug, Clone, Eq, PartialEq)]
pub struct GameSession {
    pub id: u32,
    pub p1: (String, u32),
    pub p2: (String, u32),
    pub nb_games: u32,
}

impl GameSession {
    pub fn new(id: u32, p1_name: String, p2_name: String, nb_games: u32) -> GameSession {
        GameSession {
            id,
            p1: (p1_name, 0),
            p2: (p2_name, 0),
            nb_games,
        }
    }

    pub fn read_winner(&self) -> Option<&(String, u32)> {
        if self.p1.1 > self.p2.1 {
            Some(&self.p1)
        } else if self.p2.1 > self.p1.1 {
            Some(&self.p2)
        } else {
            None
        }
    }

    pub fn update_score(&mut self, user_name: &str) {
        let win_score = self.nb_games / 2 + 1;
        if self.p1.1 >= win_score || self.p2.1 >= win_score {
            return;
        }
        for player in [&mut self.p1, &mut self.p2] {
            if player.0 == user_name {
                player.1 += 1;
            }
        }
    }

    pub fn delete(self) -> String {
        format!("game deleted: id -> {}", self.id)
    }
}
