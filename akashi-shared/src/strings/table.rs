/*!
    The following code is part of the [Assyst2 repo](https://github.com/Jacherr/assyst2)
    Which is licensed under [MIT LICENSE](https://github.com/Jacherr/assyst2/blob/3ae4f7095a3ea38fd637541c48ca9e31e90e09d6/LICENSE)

    TL;DR: Please Jacherr don't sue me.
*/

/// Returns the longer string of the two given strings
fn get_longer_str<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() > b.len() {
        a
    } else {
        b
    }
}

/// Generates a table given a list of tuples containing strings
pub fn generate_table<T: AsRef<str>>(input: &[(T, T)], reverse: bool) -> String {
    let longest: &str = input
        .iter()
        .fold(input[0].0.as_ref(), |previous, (current, _)| {
            get_longer_str(previous, current.as_ref())
        });

    if reverse {
        input
            .iter()
            .map(|(key, value)| {
                format!(
                    "{}{}: {}\n",
                    key.as_ref(),
                    " ".repeat(longest.len() - key.as_ref().len()),
                    value.as_ref()
                )
            })
            .fold(String::new(), |a, b| a + &b)
    } else {
        input
            .iter()
            .map(|(key, value)| {
                format!(
                    "{}{}: {}\n",
                    " ".repeat(longest.len() - key.as_ref().len()),
                    key.as_ref(),
                    value.as_ref()
                )
            })
            .fold(String::new(), |a, b| a + &b)
    }
}

/// Generates a list given a list of tuples containing strings
pub fn generate_list<K: AsRef<str>, V: AsRef<str>>(
    key_name: &str,
    value_name: &str,
    values: &[(K, V)],
) -> String {
    generate_list_fixed_delim(
        key_name,
        value_name,
        values,
        key_name.len(),
        value_name.len(),
    )
}

/// Generates a list given a list of tuples containing strings
pub fn generate_list_fixed_delim<K: AsRef<str>, V: AsRef<str>>(
    key_name: &str,
    value_name: &str,
    values: &[(K, V)],
    key_delim_len: usize,
    value_delim_len: usize,
) -> String {
    let longest = get_longer_str(
        key_name,
        values
            .iter()
            .fold(values[0].0.as_ref(), |previous, (current, _)| {
                get_longer_str(previous, current.as_ref())
            }),
    );

    let mut output = format!(
        " {4}{}\t{}\n {4}{}\t{}",
        key_name,
        value_name,
        "-".repeat(key_delim_len),
        "-".repeat(value_delim_len),
        " ".repeat(longest.len() - key_name.len()),
    );

    let formatted_values = values
        .iter()
        .map(|(k, v)| {
            format!(
                " {}{}\t{}",
                " ".repeat(longest.len() - k.as_ref().chars().count()),
                k.as_ref(),
                v.as_ref()
            )
        })
        .collect::<Vec<_>>()
        .join("\n");

    output = format!("{output}\n{formatted_values}");

    output
}
