use std::collections::HashMap;

pub struct Error;
pub struct Team {
    pub team_name: String,
    pub match_play: i8,
    pub win: i8,
    pub draw: i8,
    pub lose: i8,
    pub point: i8
}

pub struct ScoreBoard {
    pub team: HashMap<String, Team>
}

trait TeamFunction {
    fn update_score(&mut self, result: &str);
}

trait ScoreBoardFunction {
    fn render_score_board(&self) -> String;
}

impl TeamFunction for Team {
    fn update_score(&mut self, result: &str) {
        self.point = match result {
            "win" => {
                self.win += 1;
                self.match_play +=1;
                self.point + 3
            },
            "loss"=> {
                self.lose += 1;
                self.match_play +=1;
                self.point
            },
            "draw"=> {
                self.draw += 1;
                self.match_play +=1;
                self.point + 1
            }
            _ => self.point
        };
        
    }
}

impl ScoreBoardFunction for ScoreBoard {
    fn render_score_board(&self) -> String {
        let mut output = "Team                           | MP |  W |  D |  L |  P".to_string();
        if !self.team.is_empty() {
            let mut team_vec:Vec<&Team> = self.team.values().collect();
            team_vec.sort_by(|this, next| {
                next.point.cmp(&this.point).then_with(|| this.team_name.cmp(&next.team_name))
            });
            
            for team in team_vec {
                output.push_str("\n");
                let team_line = format!("{: <31}|{: >3} |{: >3} |{: >3} |{: >3} |{: >3}", team.team_name, team.match_play, team.win, team.draw, team.lose, team.point);
                output.push_str(&team_line);
            };
        } else {}          
        output
    }
}

pub fn tally(match_results: &str) -> String {
    let lines = match_results.lines();
    let team_list: HashMap<String, Team> = HashMap::new();
    let mut score_board = ScoreBoard {team: team_list};
    for line in lines {
        let temp: Vec<&str> = line.split(";").collect();
        populate_team(temp[0], &mut score_board, temp[2]);
        populate_team(temp[1], &mut score_board, get_opposite_result(temp[2]))
    };

    score_board.render_score_board()
}

fn populate_team(team_name: &str, score_board: &mut ScoreBoard, result: &str) {
    if score_board.team.contains_key(team_name) {
        let team = match score_board.team.get_mut(team_name) {
            Some(team) => team,
            None => panic!("can't find team"),
        };
        team.update_score(result);
    } else {
        score_board.team.insert(team_name.to_string(), Team {team_name: team_name.to_string(), match_play: 0, win: 0, draw: 0, lose: 0, point: 0 });
        let new_team = match score_board.team.get_mut(team_name) {
            Some(team) => team,
            None => panic!("can't find team"),
        };
        new_team.update_score(result);
    }
}

fn get_opposite_result(result: &str) -> &str {
    match result {
        "win"  => "loss",
        "loss" => "win",
        "draw" => "draw",
        _ => panic!("the result is not valid!!!")
    }
}