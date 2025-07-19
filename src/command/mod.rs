use crate::{
    args::Args,
    command::command_pattern::CommandPattern,
    option::{
        Option, Options,
        option_pattern::{ArgumentedOptPatArg, OptionPattern},
    },
};

pub mod command_pattern;

/// Command generated from an appropriate defined `CommandPattern`. It stores information of the
/// result command based on `Args`, therefore `Command` is a result type, and not a container.
///
/// ### Fields
/// - command_patterns: Original `CommandPattern` from which this `Command` was generated. It is
/// stored in order to call its `callback` field and to determine which command was matched.
/// - arguments: Arguments generated following `CommandPattern`'s `args_count` field.
/// - options: Options generated following `CommandPattern`'s `option_patterns` field.
///
/// ### Example
/// ```rust
/// Command::new(
///     Args::CommandLineArgs,
///     &[
///         CommandPattern::new(
///             "exit",
///             &[], // No arguments are accepted.
///             &[], // No options are accepted.
///             &|_, _| std::process:exit(0),
///         ),
///     ],
/// )
/// ```
pub struct Command<'a> {
    pub command_pattern: CommandPattern<'a>,
    pub arguments: Vec<String>,
    pub options: Options,
}

impl<'a> Command<'a> {
    /// Creates a new `Command` from commandargs's `Args`.
    ///
    /// ### Fields
    /// - args: Source of arguments used, refer to `Args` to know the possible variants that can be
    /// passed here.
    /// - command_patterns: Accepted `CommandPattern`'s used to get a result `Command`.
    ///
    /// ### Example
    /// ```rust
    /// Command::from_args(
    ///     Args::CommandLineArgs,
    ///     &[
    ///         CommandPattern::new(
    ///             "exit",
    ///             &[], // No arguments are accepted.
    ///             &[], // No options are accepted.
    ///             &|_, _| std::process:exit(0),
    ///         ),
    ///     ],
    /// )
    /// ```
    pub fn from_args(args: Args, command_patterns: &[CommandPattern<'a>]) -> Result<Self, String> {
        let args = args.to_vec();
        let mut args = args.iter();
        _ = args.next();

        let Some(command) = args.next() else {
            return Err(format!(
                "Expected any of these commands:
    {:?}",
                command_patterns
                    .iter()
                    .map(|pat| &pat.name)
                    .collect::<Vec<_>>()
            ));
        };

        let Some(command_pattern) = command_patterns
            .iter()
            .find(|pat| pat.name == *command)
            .cloned()
        else {
            return Err(format!("Invalid command: {command}."));
        };

        let mut arguments = Vec::new();
        let mut options = Vec::new();
        let args = args.collect::<Vec<_>>();
        let args_left = args.len();
        let mut index = 0;

        while index < args_left {
            let arg = args.get(index).unwrap().to_string();

            if arg.starts_with('-') {
                let mut option = arg;
                option.remove(0);

                let pattern = command_pattern
                    .option_patterns
                    .iter()
                    .find(|pat| pat.name() == option);

                if pattern.is_none() {
                    return Err(format!(
                        "Invalid option: {option}, expected any of these:
    {}",
                        command_pattern
                            .option_patterns
                            .iter()
                            .map(|pat| format!("{pat}"))
                            .collect::<Vec<_>>()
                            .join(", ")
                    ));
                }

                let option = match pattern.unwrap() {
                    OptionPattern::Standalone(name) => Option::Standalone(name),

                    OptionPattern::Argumented(name, expected_values) => {
                        if index + 1 < args_left
                            && match expected_values {
                                ArgumentedOptPatArg::Specific(values) => {
                                    values.contains(&args.get(index + 1).unwrap().as_str())
                                }
                                ArgumentedOptPatArg::Any => true,
                            }
                        {
                            let arg = args.get(index + 1).unwrap();
                            index += 1;

                            Option::Argumented(name, arg.to_string())
                        } else {
                            return Err(format!("-{name} requires {}.", expected_values,));
                        }
                    }
                };

                options.push(option);
            } else {
                if arguments.len() + 1 > command_pattern.args_count {
                    return Err(format!(
                        "{command} expects {} arguments.",
                        command_pattern.args_count,
                    ));
                }

                arguments.push(arg.to_string());
            }

            index += 1;
        }

        if arguments.len() < command_pattern.args_count {
            return Err(format!(
                "{command} expects {} arguments.",
                command_pattern.args_count,
            ));
        }

        Ok(Self {
            command_pattern,
            arguments,
            options: Options::from(options),
        })
    }

    /// Execute the result command, by calling the `callback` field of the matched
    /// `CommandPattern`.
    pub fn execute(&'a self) {
        (self.command_pattern.callback)(&self.arguments, &self.options);
    }
}
