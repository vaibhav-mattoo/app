// use core::time;
// use std::collections::BTreeSet;
use std::time::{SystemTime, UNIX_EPOCH};
use super::database_structs::{Command, Database, DeletedCommands};

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
    let mult: f64;
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
    pub fn add_command(&mut self, command_str: String, deleted_commands: &DeletedCommands) {
        if !deleted_commands.deleted_commands.contains(&command_str) {
            if let Some(existing_command) = self.reverse_command_map.get(&command_str) {
                let mut updated_command = existing_command.clone();
                self.total_score -= updated_command.score as i64;
                self.total_num_commands -= 1;
                updated_command.add();

                self.command_list.remove(existing_command);
                self.reverse_command_map.remove(&command_str);

                self.command_list.insert(updated_command.clone());
                self.reverse_command_map.insert(command_str.clone(), updated_command);

                self.total_num_commands += 1;
                self.total_score += self.reverse_command_map.get(&command_str).unwrap().score as i64;
            } else {
                let new_command: Command = Command::new(command_str.clone());
                if new_command.length <= 5 && new_command.number_of_words == 1 {
                    return; // Ignore commands that are too short and single-word
                }
                self.command_list.insert(new_command.clone());
                self.reverse_command_map.insert(command_str.clone(), new_command);
                self.total_num_commands += 1;
                self.total_score += self.reverse_command_map.get(&command_str).unwrap().score as i64;
            }
        }
    }

    pub fn remove_command(&mut self, command_str: &String, deleted_commands: &mut DeletedCommands) {
        // first check if already exists in DeletedCommands
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

    pub fn update_db (&mut self) {
        // we should iterate over the reverse_command_map and update each command using its update function
        // maybe also remove commands which have very low score, to be implemented later
        let mut copy = self.reverse_command_map.clone();
        for (key, command) in copy.iter_mut() {

            self.total_score -= command.score as i64;
            command.update();
            self.total_score += command.score as i64;

            self.command_list.remove(&command);
            self.reverse_command_map.remove(key);

            self.command_list.insert(command.clone());
            self.reverse_command_map.insert(key.clone(), command.clone());
        }
    }

    pub fn get_top_commands(&mut self, n: Option<usize>) -> Vec<&Command> {
        let n = n.unwrap_or(5);
        self.command_list.iter().take(n).collect()
    }

    // pub fn get_total_score(&self) -> i64 {
    //     self.total_score
    // }

    // pub fn score_reset(&mut self){
    //     //iterate through the set and reduce the score of each string by 90%
    //     for value in self.reverse_command_map.values_mut() {
    //         value.score = (value.score as f32 * 0.9).round() as i32;
    //     }
    // }

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
    pub fn add(&mut self) {
        // here we update the last_access time, frequency, and score
        let now = SystemTime::now();
        self.last_access_time = now.duration_since(UNIX_EPOCH).unwrap().as_secs() as i64; // placeholder
        self.frequency += 1;
        self.score = get_score(self);
    }
    pub fn update (&mut self) {
        // just update with time and score
        self.score = get_score(self);
    }
}
