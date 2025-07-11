use crate::cli::cli_data::InitShell;
use std::path::PathBuf;

pub struct ShellOpts {
    pub app_path: String,
    pub data_dir: String,
    pub alias_file_path: String,
}

impl ShellOpts {
    pub fn new() -> Self {
        let app_path = std::env::current_exe()
            .unwrap_or_else(|_| PathBuf::from("alman"))
            .to_string_lossy()
            .to_string();
        
        let data_dir = crate::database::persistence::get_data_directory()
            .unwrap_or_else(|_| PathBuf::from(".").join(".alman"))
            .to_string_lossy()
            .to_string();
        
        let alias_file_path = crate::database::persistence::load_config()
            .and_then(|cfg| cfg.alias_file_paths.first().cloned())
            .unwrap_or_else(|| crate::database::persistence::get_default_alias_file_path());
        
        Self {
            app_path,
            data_dir,
            alias_file_path,
        }
    }
}

pub fn render_shell_init(shell: InitShell, opts: &ShellOpts) -> String {
    match shell {
        InitShell::Bash => render_bash(opts),
        InitShell::Zsh => render_zsh(opts),
        InitShell::Fish => render_fish(opts),
        InitShell::Posix => render_posix(opts),
    }
}

fn render_bash(opts: &ShellOpts) -> String {
    let mut script = String::new();
    script.push_str("# Alman shell integration for bash\n");
    script.push_str("# Add this to your ~/.bashrc\n\n");
    script.push_str(&format!("export ALMAN_DATA_DIR=\"{}\"\n", opts.data_dir));
    script.push_str(&format!("export ALMAN_ALIAS_FILE=\"{}\"\n", opts.alias_file_path));
    script.push_str(&format!("export ALMAN_BIN=\"{}\"\n\n", opts.app_path));
    
    script.push_str("alman_preexec() {\n");
    script.push_str("    if [ -n \"$1\" ]; then\n");
    script.push_str(&format!("        {} custom \"$1\" 2>/dev/null\n", opts.app_path));
    script.push_str("    fi\n");
    script.push_str("}\n\n");
    
    script.push_str("if [ -n \"$BASH_VERSION\" ]; then\n");
    script.push_str("    # For bash, we need to use PROMPT_COMMAND\n");
    script.push_str("    if [ -n \"$PROMPT_COMMAND\" ]; then\n");
    script.push_str("        PROMPT_COMMAND=\"alman_preexec \\$?; $PROMPT_COMMAND\"\n");
    script.push_str("    else\n");
    script.push_str("        PROMPT_COMMAND=\"alman_preexec \\$?\"\n");
    script.push_str("    fi\n");
    script.push_str("fi\n");
    
    script
}

fn render_zsh(opts: &ShellOpts) -> String {
    let mut script = String::new();
    script.push_str("# Alman shell integration for zsh\n");
    script.push_str("# Add this to your ~/.zshrc\n\n");
    script.push_str(&format!("export ALMAN_DATA_DIR=\"{}\"\n", opts.data_dir));
    script.push_str(&format!("export ALMAN_ALIAS_FILE=\"{}\"\n", opts.alias_file_path));
    script.push_str(&format!("export ALMAN_BIN=\"{}\"\n\n", opts.app_path));
    
    script.push_str("alman_preexec() {\n");
    script.push_str("    if [ -n \"$1\" ]; then\n");
    script.push_str(&format!("        {} custom \"$1\" 2>/dev/null\n", opts.app_path));
    script.push_str("    fi\n");
    script.push_str("}\n\n");
    
    script.push_str("autoload -U add-zsh-hook\n");
    script.push_str("add-zsh-hook preexec alman_preexec\n");
    
    script
}

fn render_fish(opts: &ShellOpts) -> String {
    let mut script = String::new();
    script.push_str("# Alman shell integration for fish\n");
    script.push_str("# Add this to your ~/.config/fish/config.fish\n\n");
    script.push_str(&format!("set -gx ALMAN_DATA_DIR \"{}\"\n", opts.data_dir));
    script.push_str(&format!("set -gx ALMAN_ALIAS_FILE \"{}\"\n", opts.alias_file_path));
    script.push_str(&format!("set -gx ALMAN_BIN \"{}\"\n\n", opts.app_path));
    
    script.push_str("function alman_preexec --on-event fish_preexec\n");
    script.push_str("    if test -n \"$argv[1]\"\n");
    script.push_str(&format!("        {} custom \"$argv[1]\" 2>/dev/null\n", opts.app_path));
    script.push_str("    end\n");
    script.push_str("end\n");
    
    script
}

fn render_posix(opts: &ShellOpts) -> String {
    let mut script = String::new();
    script.push_str("# Alman shell integration for POSIX shells (ksh, dash, etc.)\n");
    script.push_str("# Add this to your ~/.profile or ~/.kshrc\n\n");
    script.push_str(&format!("export ALMAN_DATA_DIR=\"{}\"\n", opts.data_dir));
    script.push_str(&format!("export ALMAN_ALIAS_FILE=\"{}\"\n", opts.alias_file_path));
    script.push_str(&format!("export ALMAN_BIN=\"{}\"\n\n", opts.app_path));
    
    script.push_str("alman_preexec() {\n");
    script.push_str("    if [ -n \"$1\" ]; then\n");
    script.push_str(&format!("        {} custom \"$1\" 2>/dev/null\n", opts.app_path));
    script.push_str("    fi\n");
    script.push_str("}\n\n");
    
    script.push_str("# Note: POSIX shells don't have built-in preexec hooks\n");
    script.push_str("# You may need to manually call alman_preexec in your PS1\n");
    script.push_str("# Example: PS1='$(alman_preexec $?) $ '\n");
    
    script
} 