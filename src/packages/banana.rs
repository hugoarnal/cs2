pub fn get_binary_locations() -> Vec<&'static str> {
    vec![
        "/usr/bin/banana-check-repo",
        "/usr/local/bin/banana-check-repo",
    ]
}

pub fn get_plugin_locations() -> Vec<&'static str> {
    vec![
        "/usr/lib/epiclang/plugins/epitech-plugin-banana.so",
        "/usr/lib/epiclang/plugins/epiclang-plugin-banana.so",
        "/usr/local/lib/epiclang/plugins/epiclang-plugin-banana.so",
    ]
}
