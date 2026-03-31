use std::collections::HashMap;

// A structure to store the goal details of a team.
struct Team {
    goals_scored: u8,
    goals_conceded: u8,
}

fn build_scores_table(results: String) -> HashMap<String, Team> {
    let mut scores: HashMap<String, Team> = HashMap::new();

    for r in results.lines() {
        let v: Vec<&str> = r.split(',').collect();
        let team_1_name = v[0].to_string();
        let team_1_score: u8 = v[2].parse().unwrap();
        let team_2_name = v[1].to_string();
        let team_2_score: u8 = v[3].parse().unwrap();

        if let Some(team) = scores.get_mut(&team_1_name) {
            team.goals_scored += team_1_score;
            team.goals_conceded += team_2_score;
        } else {
            scores.insert(
                team_1_name,
                Team {
                    goals_scored: team_1_score,
                    goals_conceded: team_2_score,
                },
            );
        }

        if let Some(team) = scores.get_mut(&team_2_name) {
            team.goals_scored += team_2_score;
            team.goals_conceded += team_1_score;
        } else {
            scores.insert(
                team_2_name,
                Team {
                    goals_scored: team_2_score,
                    goals_conceded: team_1_score,
                },
            );
        }
    }

    scores
}