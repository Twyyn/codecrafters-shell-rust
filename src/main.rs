use codecrafters_shell::Shell;

fn main() -> anyhow::Result<()> {
    std::process::exit(Shell::new().run()?)
}
