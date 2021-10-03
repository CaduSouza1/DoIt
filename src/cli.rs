use std::num::ParseIntError;

pub fn get_list_name(matches: &clap::ArgMatches) -> Result<String, clap::ErrorKind> {
    matches
        .value_of("name")
        .ok_or(clap::ErrorKind::EmptyValue)
        .and_then(|name| Ok(name.to_string()))
}

pub fn get_task_title(matches: &clap::ArgMatches) -> Result<String, clap::ErrorKind> {
    matches
        .value_of("title")
        .ok_or(clap::ErrorKind::EmptyValue)
        .and_then(|title| Ok(title.to_string()))
}

pub fn get_task_description(matches: &clap::ArgMatches) -> Result<String, clap::ErrorKind> {
    matches
        .value_of("description")
        .ok_or(clap::ErrorKind::EmptyValue)
        .and_then(|description| Ok(description.to_string()))
}

pub enum ParseIndexError {
    ParseError(ParseIntError),
    EmptyValue(clap::ErrorKind),
}

impl From<ParseIntError> for ParseIndexError {
    fn from(v: ParseIntError) -> Self {
        Self::ParseError(v)
    }
}

pub fn get_index(matches: &clap::ArgMatches) -> Result<usize, ParseIndexError> {
    matches
        .value_of("index")
        .ok_or(ParseIndexError::EmptyValue(clap::ErrorKind::EmptyValue))
        .and_then(|index| Ok(index.parse()?))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_app() -> clap::App<'static, 'static> {
        clap::App::new("test").args(&[
            clap::Arg::with_name("name").takes_value(true).short("n"),
            clap::Arg::with_name("title").takes_value(true).short("t"),
            clap::Arg::with_name("description")
                .takes_value(true)
                .short("d"),
        ])
    }

    #[test]
    fn list_name_parsing() {
        let expected_result = "List name test";
        let matches = create_test_app().get_matches_from(&["test", "-n", expected_result]);
        let actual_result = get_list_name(&matches).unwrap();

        assert_eq!(expected_result, actual_result)
    }

    #[test]
    fn task_title_parsing() {
        let expected_result = "Task title test";
        let matches = create_test_app().get_matches_from(&["test", "-t", expected_result]);
        let actual_result = get_task_title(&matches).unwrap();

        assert_eq!(expected_result, actual_result);
    }

    #[test]
    fn task_description_parsing() {
        let expected_result = "Task description test";
        let matches = create_test_app().get_matches_from(&["test", "-d", expected_result]);
        let actual_result = get_task_description(&matches).unwrap();

        assert_eq!(expected_result, actual_result)
    }
}
