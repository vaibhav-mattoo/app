use std::collections::HashSet;
use std::env;

#[derive(Debug, Clone)]
pub struct AliasSuggestion {
    pub alias: String,
    pub command: String,
    pub reason: String,
}

pub struct AliasSuggester {
    existing_aliases: HashSet<String>,
    system_commands: HashSet<String>,
}

impl AliasSuggester {
    pub fn new(alias_file_path: &str) -> Self {
        let existing_aliases = Self::load_existing_aliases(alias_file_path);
        let system_commands = Self::load_system_commands();
        
        Self {
            existing_aliases,
            system_commands,
        }
    }

    fn load_existing_aliases(alias_file_path: &str) -> HashSet<String> {
        use crate::ops::alias_ops::get_aliases;
        
        let aliases = get_aliases(alias_file_path);
        aliases.into_iter().map(|(alias, _)| alias).collect()
    }

    fn load_system_commands() -> HashSet<String> {
        let mut commands = HashSet::new();
        
        // Get PATH from environment
        if let Ok(path) = env::var("PATH") {
            for path_dir in path.split(':') {
                if let Ok(entries) = std::fs::read_dir(path_dir) {
                    for entry in entries.flatten() {
                        if let Ok(metadata) = entry.metadata() {
                            if metadata.is_file() {
                                // Check if file is executable (simplified check)
                                if let Some(name) = entry.file_name().to_str() {
                                    commands.insert(name.to_string());
                                }
                            }
                        }
                    }
                }
            }
        }
        
        commands
    }

    pub fn suggest_aliases(&self, command: &str) -> Vec<AliasSuggestion> {
        let mut suggestions = Vec::new();
        
        // Generate different types of suggestions
        suggestions.extend(self.generate_semantic_aliases(command));
        suggestions.extend(self.generate_abbreviation_aliases(command));
        suggestions.extend(self.generate_acronym_aliases(command));
        suggestions.extend(self.generate_combined_aliases(command));
        
        // Filter out conflicts and sort by priority
        suggestions.retain(|s| !self.has_conflicts(&s.alias));
        suggestions.sort_by(|a, b| self.get_priority(b).cmp(&self.get_priority(a)));
        
        // Limit to top 3 suggestions
        suggestions.into_iter().take(3).collect()
    }

    fn generate_semantic_aliases(&self, command: &str) -> Vec<AliasSuggestion> {
        let mut suggestions = Vec::new();
        let parts: Vec<&str> = command.split_whitespace().collect();
        
        if parts.is_empty() {
            return suggestions;
        }

        let tool = parts[0];
        let args = &parts[1..];

        // Handle relative path commands (e.g., ./target/debug/app)
        if tool.starts_with("./") || tool.starts_with("../") {
            suggestions.extend(self.generate_relative_path_aliases(command, tool, args));
            return suggestions;
        }

        // Generate tool-specific semantic aliases
        if let Some(semantic_alias) = self.generate_tool_specific_alias(tool, args) {
            suggestions.push(semantic_alias);
        }

        // Generate general semantic aliases
        if !args.is_empty() {
            let subcommand = args[0];
            let combined = format!("{}{}", tool.chars().next().unwrap_or('x'), subcommand);
            suggestions.push(AliasSuggestion {
                alias: combined,
                command: command.to_string(),
                reason: format!("{}-{} combination", tool, subcommand),
            });
        }

        suggestions
    }

    fn generate_relative_path_aliases(&self, command: &str, tool: &str, args: &[&str]) -> Vec<AliasSuggestion> {
        let mut suggestions = Vec::new();
        
        // Extract the executable name from the path
        let executable_name = tool.split('/').last().unwrap_or(tool);
        
        // Generate alias based on executable name
        if let Some(name) = executable_name.strip_suffix(".exe") {
            // For Windows executables
            suggestions.push(AliasSuggestion {
                alias: name.to_string(),
                command: command.to_string(),
                reason: "Executable name".to_string(),
            });
        } else {
            // For Unix executables
            suggestions.push(AliasSuggestion {
                alias: executable_name.to_string(),
                command: command.to_string(),
                reason: "Executable name".to_string(),
            });
        }

        // Generate abbreviation based on executable name
        if executable_name.len() > 2 {
            let abbrev = executable_name.chars().take(3).collect::<String>();
            suggestions.push(AliasSuggestion {
                alias: abbrev,
                command: command.to_string(),
                reason: "Executable abbreviation".to_string(),
            });
        }

        // If there are arguments, create a more specific alias
        if !args.is_empty() {
            let first_arg = args[0];
            let combined = format!("{}{}", executable_name.chars().next().unwrap_or('x'), first_arg);
            suggestions.push(AliasSuggestion {
                alias: combined,
                command: command.to_string(),
                reason: format!("{}-{} combination", executable_name, first_arg),
            });
        }

        suggestions
    }

    fn generate_tool_specific_alias(&self, tool: &str, args: &[&str]) -> Option<AliasSuggestion> {
        if args.is_empty() {
            return None;
        }

        let subcommand = args[0];
        let remaining_args = &args[1..];

        match tool {
            "git" => self.generate_git_alias(subcommand, remaining_args),
            "docker" => self.generate_docker_alias(subcommand),
            "npm" => self.generate_npm_alias(subcommand),
            "ssh" => self.generate_ssh_alias(subcommand),
            _ => None,
        }
    }

    fn generate_git_alias(&self, subcommand: &str, remaining_args: &[&str]) -> Option<AliasSuggestion> {
        match subcommand {
            "status" => Some(AliasSuggestion {
                alias: "gs".to_string(),
                command: "git status".to_string(),
                reason: "Git status".to_string(),
            }),
            "add" => {
                if !remaining_args.is_empty() && remaining_args[0] == "." {
                    Some(AliasSuggestion {
                        alias: "gaa".to_string(),
                        command: "git add .".to_string(),
                        reason: "Git add all".to_string(),
                    })
                } else {
                    Some(AliasSuggestion {
                        alias: "ga".to_string(),
                        command: "git add".to_string(),
                        reason: "Git add".to_string(),
                    })
                }
            }
            "commit" => {
                if remaining_args.len() >= 2 && remaining_args[0] == "-m" {
                    Some(AliasSuggestion {
                        alias: "gcm".to_string(),
                        command: format!("git commit -m \"{}\"", remaining_args[1]),
                        reason: "Git commit with message".to_string(),
                    })
                } else {
                    Some(AliasSuggestion {
                        alias: "gc".to_string(),
                        command: "git commit".to_string(),
                        reason: "Git commit".to_string(),
                    })
                }
            }
            "checkout" => {
                if !remaining_args.is_empty() && remaining_args[0] == "-b" {
                    Some(AliasSuggestion {
                        alias: "gcb".to_string(),
                        command: format!("git checkout -b {}", remaining_args[1]),
                        reason: "Git checkout new branch".to_string(),
                    })
                } else {
                    Some(AliasSuggestion {
                        alias: "gco".to_string(),
                        command: "git checkout".to_string(),
                        reason: "Git checkout".to_string(),
                    })
                }
            }
            "push" => Some(AliasSuggestion {
                alias: "gp".to_string(),
                command: "git push".to_string(),
                reason: "Git push".to_string(),
            }),
            "pull" => Some(AliasSuggestion {
                alias: "gl".to_string(),
                command: "git pull".to_string(),
                reason: "Git pull".to_string(),
            }),
            "log" => Some(AliasSuggestion {
                alias: "glg".to_string(),
                command: "git log".to_string(),
                reason: "Git log".to_string(),
            }),
            "branch" => Some(AliasSuggestion {
                alias: "gb".to_string(),
                command: "git branch".to_string(),
                reason: "Git branch".to_string(),
            }),
            _ => None,
        }
    }

    fn generate_docker_alias(&self, subcommand: &str) -> Option<AliasSuggestion> {
        match subcommand {
            "ps" => Some(AliasSuggestion {
                alias: "dps".to_string(),
                command: "docker ps".to_string(),
                reason: "Docker ps".to_string(),
            }),
            "run" => Some(AliasSuggestion {
                alias: "dr".to_string(),
                command: "docker run".to_string(),
                reason: "Docker run".to_string(),
            }),
            "build" => Some(AliasSuggestion {
                alias: "db".to_string(),
                command: "docker build".to_string(),
                reason: "Docker build".to_string(),
            }),
            "exec" => Some(AliasSuggestion {
                alias: "de".to_string(),
                command: "docker exec".to_string(),
                reason: "Docker exec".to_string(),
            }),
            "rm" => Some(AliasSuggestion {
                alias: "drm".to_string(),
                command: "docker rm".to_string(),
                reason: "Docker rm".to_string(),
            }),
            "rmi" => Some(AliasSuggestion {
                alias: "drmi".to_string(),
                command: "docker rmi".to_string(),
                reason: "Docker rmi".to_string(),
            }),
            _ => None,
        }
    }

    fn generate_npm_alias(&self, subcommand: &str) -> Option<AliasSuggestion> {
        match subcommand {
            "install" => Some(AliasSuggestion {
                alias: "ni".to_string(),
                command: "npm install".to_string(),
                reason: "NPM install".to_string(),
            }),
            "run" => Some(AliasSuggestion {
                alias: "nr".to_string(),
                command: "npm run".to_string(),
                reason: "NPM run".to_string(),
            }),
            "start" => Some(AliasSuggestion {
                alias: "ns".to_string(),
                command: "npm start".to_string(),
                reason: "NPM start".to_string(),
            }),
            "test" => Some(AliasSuggestion {
                alias: "nt".to_string(),
                command: "npm test".to_string(),
                reason: "NPM test".to_string(),
            }),
            "publish" => Some(AliasSuggestion {
                alias: "np".to_string(),
                command: "npm publish".to_string(),
                reason: "NPM publish".to_string(),
            }),
            _ => None,
        }
    }

    fn generate_ssh_alias(&self, host: &str) -> Option<AliasSuggestion> {
        // Generate alias based on hostname
        if let Some(short_name) = host.split('.').next() {
            if short_name.len() >= 2 {
                return Some(AliasSuggestion {
                    alias: short_name.to_string(),
                    command: format!("ssh {}", host),
                    reason: format!("SSH to {}", host),
                });
            }
        }
        None
    }

    fn generate_abbreviation_aliases(&self, command: &str) -> Vec<AliasSuggestion> {
        let mut suggestions = Vec::new();
        let parts: Vec<&str> = command.split_whitespace().collect();
        
        if parts.len() < 2 {
            return suggestions;
        }

        // Take first letter of each word
        let abbreviation: String = parts.iter()
            .map(|part| part.chars().next().unwrap_or('x'))
            .collect();

        if abbreviation.len() >= 2 && abbreviation.len() <= 4 {
            suggestions.push(AliasSuggestion {
                alias: abbreviation.clone(),
                command: command.to_string(),
                reason: "Abbreviation".to_string(),
            });
        }

        // Generate variations with numbers if needed
        if abbreviation.len() >= 2 {
            for i in 1..=3 {
                let variation = format!("{}{}", abbreviation, i);
                suggestions.push(AliasSuggestion {
                    alias: variation,
                    command: command.to_string(),
                    reason: format!("Abbreviation variation {}", i),
                });
            }
        }

        suggestions
    }

    fn generate_acronym_aliases(&self, command: &str) -> Vec<AliasSuggestion> {
        let mut suggestions = Vec::new();
        let parts: Vec<&str> = command.split_whitespace().collect();
        
        if parts.len() < 2 {
            return suggestions;
        }

        // Create acronym from first letters
        let acronym: String = parts.iter()
            .map(|part| part.chars().next().unwrap_or('x'))
            .collect();

        if acronym.len() >= 2 && acronym.len() <= 4 {
            suggestions.push(AliasSuggestion {
                alias: acronym,
                command: command.to_string(),
                reason: "Acronym".to_string(),
            });
        }

        suggestions
    }

    fn generate_combined_aliases(&self, command: &str) -> Vec<AliasSuggestion> {
        let mut suggestions = Vec::new();
        let parts: Vec<&str> = command.split_whitespace().collect();
        
        if parts.len() < 2 {
            return suggestions;
        }

        let tool = parts[0];
        let args = &parts[1..];

        // Generate tool + first arg combinations
        if !args.is_empty() {
            let first_arg = args[0];
            if first_arg.len() >= 2 {
                let combined = format!("{}{}", tool.chars().next().unwrap_or('x'), first_arg);
                suggestions.push(AliasSuggestion {
                    alias: combined,
                    command: command.to_string(),
                    reason: format!("{}-{} combination", tool, first_arg),
                });
            }
        }

        // Generate tool + second arg combinations
        if args.len() >= 2 {
            let second_arg = args[1];
            if second_arg.len() >= 2 {
                let combined = format!("{}{}", tool.chars().next().unwrap_or('x'), second_arg);
                suggestions.push(AliasSuggestion {
                    alias: combined,
                    command: command.to_string(),
                    reason: format!("{}-{} combination", tool, second_arg),
                });
            }
        }

        suggestions
    }

    fn has_conflicts(&self, alias: &str) -> bool {
        // Check if alias already exists
        if self.existing_aliases.contains(alias) {
            return true;
        }

        // Check if alias conflicts with system command
        if self.system_commands.contains(alias) {
            return true;
        }

        // Check if alias is too short (likely to conflict)
        if alias.len() < 2 {
            return true;
        }

        false
    }

    fn get_priority(&self, suggestion: &AliasSuggestion) -> i32 {
        let mut priority = 0;
        
        // Higher priority for semantic aliases
        if suggestion.reason.contains("Git") || suggestion.reason.contains("Docker") || suggestion.reason.contains("NPM") {
            priority += 10;
        }
        
        // Higher priority for shorter aliases
        priority += 10 - suggestion.alias.len() as i32;
        
        // Higher priority for abbreviations over acronyms
        if suggestion.reason == "Abbreviation" {
            priority += 5;
        }
        
        priority
    }


} 