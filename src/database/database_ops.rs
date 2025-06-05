// use core::time;
use std::collections::BTreeSet;
use std::time::{SystemTime, UNIX_EPOCH};
use super::database_structs::{Command, Database, Deleted_Commands};

fn get_score(command: &Command) -> i32 {
    // Make a simple scoring func for now, just return the length
    // let x:f64=(command.length).powf(3.0/5.0);
    let now = SystemTime::now();

    // match now.duration_since(UNIX_EPOCH) {
    //     // Ok(duration) => {},
    //     Ok(duration) => println!("UNIX timestamp: {}", duration.as_secs()),
    //     Err(e) => eprintln!("Time error: {:?}", e),
    // }
    let current_time: i64 = now.duration_since(UNIX_EPOCH).unwrap().as_secs() as i64;
    let time_difference: i64 = current_time - command.last_access_time;
    let mut mult: f64 =1.0;
    if time_difference<=3600{
        mult=4.0;
    }
    else if time_difference<=86400{
        mult=2.0;
    }
    else if time_difference<=604800{
        mult=0.5;
    }
    else{
        mult=0.25;
    }
    let length = command.length as f64;
    let frequency = command.frequency as f64;

    (mult * length.powf(3.0 / 5.0) * frequency) as i32
    
    
}

impl Database {
    pub fn add_command(&mut self, command_str: String, deleted_commands: &Deleted_Commands) {
        if !deleted_commands.deleted_commands.contains(&command_str) {
            if let Some(existing_command) = self.reverse_command_map.get(&command_str) {
                let mut updated_command = existing_command.clone();
                self.total_score -= updated_command.score as i64;
                self.total_num_commands -= 1;
                updated_command.update();

                self.command_list.remove(existing_command);
                self.reverse_command_map.remove(&command_str);

                self.command_list.insert(updated_command.clone());
                self.reverse_command_map.insert(command_str, updated_command);
            } else {
                let new_command: Command = Command::new(command_str.clone());
                self.command_list.insert(new_command.clone());
                self.reverse_command_map.insert(command_str.clone(), new_command);
                self.total_num_commands += 1;
                self.total_score += self.reverse_command_map.get(&command_str).unwrap().score as i64;
            }
        }
    }

    pub fn remove_command(&mut self, command_str: &String, deleted_commands: &mut Deleted_Commands) {
        // first check if already exists in Deleted_Commands
        if !deleted_commands.deleted_commands.contains(command_str) {
            deleted_commands
                .deleted_commands
                .insert(command_str.clone());
            // then remove from command list, first get command from reverse map
            let command_to_remove = self.reverse_command_map.get(command_str);
            // if not null, remove it fromm both
            if let Some(cmd) = command_to_remove {
                self.command_list.remove(cmd);
                self.total_num_commands -= 1;
                self.total_score -= cmd.score as i64;
                self.reverse_command_map.remove(command_str);
            }
        }
    }

    pub fn get_top_commands(&mut self, n: Option<usize>) -> Vec<&Command> {
        // will be sorted by score, get top n
        // let n = n.unwrap_or(10);
        // println!("working");
        let n = n.unwrap_or(5);
        self.command_list.iter().take(n).collect()
    }

    pub fn get_total_score(&self) -> i64 {
        self.total_score
    }

    pub fn score_reset(&mut self){
        //iterate through the set and reduce the score of each string by 90%
        for value in self.reverse_command_map.values_mut() {
            value.frequency = (value.frequency as f32 * 0.1).round() as i32;
            get_score(&value);
        }
        let old_set = std::mem::take(&mut self.command_list);

        for mut cmd in old_set {
            cmd.frequency = (cmd.frequency as f32 * 0.1).round() as i32;
            get_score(&cmd); 
            cmd.update();
            cmd.frequency-=1;
            self.command_list.insert(cmd);
        }
        
        
    }

}

impl Command {
    pub fn new(command_text: String) -> Self {
        let length: i16 = command_text.split_whitespace().map(|s| s.len()).sum::<usize>() as i16;
        //map transforms the strings into their lengths
        let number_of_words: i8 = command_text.split_whitespace().count() as i8;
        let frequency: i32 = 1;
        let now = SystemTime::now();
        let last_access_time: i64 = now.duration_since(UNIX_EPOCH).unwrap().as_secs() as i64; // or set as needed

        // Create a temporary Command to calculate the score
        let temp_command = Command {
            last_access_time,
            frequency,
            length,
            score: 0, // placeholder
            command_text: command_text.clone(),
            number_of_words,
        };

        let score = get_score(&temp_command);

        Command {
            last_access_time,
            frequency,
            length,
            score,
            command_text,
            number_of_words,
        }
    }
    pub fn update(&mut self) {
        // here we update the last_access time, frequency, and score
        let now = SystemTime::now();
        self.last_access_time = now.duration_since(UNIX_EPOCH).unwrap().as_secs() as i64; // placeholder
        self.frequency += 1;
        self.score = get_score(self);
    }
}
