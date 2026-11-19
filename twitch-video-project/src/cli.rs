#[derive(Debug)]
pub struct Options {
    pub project_name: String,
    pub login: Option<String>,
    pub client_id: Option<String>,
    pub client_secret: Option<String>,
}

pub fn parse(args: Vec<String>) -> Result<Options, String> {
    if args.iter().any(|arg| arg == "-h" || arg == "--help") {
        print_help();
        std::process::exit(0);
    }
    let mut project_name = None;
    let (mut login, mut client_id, mut client_secret) = (None, None, None);
    let mut index = 0;
    while index < args.len() {
        let arg = &args[index];
        if let Some((option, value)) = arg.split_once('=') {
            match option {
                "--login" => login = Some(value.to_owned()),
                "--client-id" => client_id = Some(value.to_owned()),
                "--client-secret" => client_secret = Some(value.to_owned()),
                _ => {
                    return Err(format!(
                        "unknown option '{option}'. Run `tvd -h` for usage."
                    ));
                }
            }
        } else if ["--login", "--client-id", "--client-secret"].contains(&arg.as_str()) {
            index += 1;
            let value = args
                .get(index)
                .ok_or_else(|| format!("{arg} needs a value"))?
                .clone();
            match arg.as_str() {
                "--login" => login = Some(value),
                "--client-id" => client_id = Some(value),
                "--client-secret" => client_secret = Some(value),
                _ => unreachable!(),
            }
        } else if arg.starts_with('-') {
            return Err(format!("unknown option '{arg}'. Run `tvd -h` for usage."));
        } else if project_name.is_some() {
            return Err("only one project name is allowed".into());
        } else {
            project_name = Some(arg.clone());
        }
        index += 1;
    }
    let name = project_name.ok_or("missing project name")?;
    let name = name.trim();
    if name.is_empty() || super::project::slug(name).is_empty() {
        return Err("project name must contain at least one letter or number".into());
    }
    Ok(Options {
        project_name: name.to_owned(),
        login,
        client_id,
        client_secret,
    })
}

fn print_help() {
    println!(
        r#"Usage: tvd <project-name> [options]

Select a Twitch VOD and download it into a new video project.

Options:
  --login <name>             Twitch channel login (or TWITCH_LOGIN; prompted if unset)
  --client-id <id>           Twitch client ID (or TWITCH_CLIENT_ID)
  --client-secret <secret>   Twitch client secret (or TWITCH_CLIENT_SECRET)
  -h, --help                 Show help

Requires `cvp` and `yt-dlp` on PATH. Saves video as 02_source/<project-name>.mp4.

Example:
  tvd "stream edit" --login themidpackstruggle
  tvd "stream edit" --client-id ID --client-secret SECRET"#
    );
}

#[cfg(test)]
mod tests {
    use super::parse;

    #[test]
    fn accepts_required_project_name() {
        let options = parse(vec!["stream edit".into()]).unwrap();
        assert_eq!(options.project_name, "stream edit");
        assert!(options.login.is_none());
    }

    #[test]
    fn parses_all_connection_options() {
        let options = parse(vec![
            "project".into(),
            "--login".into(),
            "channel".into(),
            "--client-id=abc".into(),
            "--client-secret".into(),
            "secret".into(),
        ])
        .unwrap();

        assert_eq!(options.login.as_deref(), Some("channel"));
        assert_eq!(options.client_id.as_deref(), Some("abc"));
        assert_eq!(options.client_secret.as_deref(), Some("secret"));
    }

    #[test]
    fn rejects_missing_project_name() {
        assert_eq!(parse(vec![]).unwrap_err(), "missing project name");
    }

    #[test]
    fn rejects_missing_option_value() {
        assert_eq!(
            parse(vec!["project".into(), "--client-id".into()]).unwrap_err(),
            "--client-id needs a value"
        );
    }

    #[test]
    fn rejects_multiple_project_names() {
        assert_eq!(
            parse(vec!["one".into(), "two".into()]).unwrap_err(),
            "only one project name is allowed"
        );
    }
}
