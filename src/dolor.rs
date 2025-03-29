use std::collections::HashSet;

/// Returns a HashSet containing the "dolar corpus" collection of words commonly found in Lorem Ipsum text.
///
/// Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. 
/// Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. 
/// Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. 
/// Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.
pub fn corpus() -> HashSet<&'static str> {
    let words: &[&str] = &[
        // Core Lorem Ipsum words
        "lorem", "ipsum", "dolor", "sit", "amet", "consectetur", "adipiscing", "elit",
        "sed", "do", "eiusmod", "tempor", "incididunt", "ut", "labore", "et", "dolore",
        "magna", "aliqua", "enim", "ad", "minim", "veniam", "quis", "nostrud",
        "exercitation", "ullamco", "laboris", "nisi", "aliquip", "ex", "ea", "commodo",
        "consequat", "duis", "aute", "irure", "in", "reprehenderit", "voluptate", "velit",
        "esse", "cillum", "eu", "fugiat", "nulla", "pariatur", "excepteur", "sint",
        "occaecat", "cupidatat", "non", "proident", "sunt", "culpa", "qui", "officia",
        "deserunt", "mollit", "anim", "id", "est",
        // Expanded corpus: extra Latin-like terms and common filler words
        "phasellus", "rutrum", "nibh", "quis", "ligula", "malesuada", "sollicitudin", "mauris", "risus",
        "lacinia", "vestibulum", "nullam", "leo", "dictum", "massa", "sagittis", "fermentum", "sapien",
        "elementum", "vehicula", "pellentesque", "nunc", "turpis", "tempus", "scelerisque", "mi",
        "ac", "varius", "libero", "facilisis", "dapibus", "nibh", "cursus", "sem", "rhoncus",
        "pulvinar", "orci", "luctus", "quisque", "egestas", "metus", "at", "urna", "condimentum",
        "congue", "euismod", "tortor", "habitant", "morbi", "tristique", "senectus", "netus", "fames",
        "porta", "aliquam", "interdum", "commodo", "scelerisque", "felis", "sodales", "praesent",
        "accumsan", "turpis", "vivamus", "varius", "facilisis", "sapien", "iaculis",
        "dictum", "porta", "aliquet", "integer", "vel", "augue", "venenatis", "dui",
        "rutrum", "elementum", "leo", "curabitur", "consectetur", "vivamus", "ullamcorper",
        "magna", "a", "nunc", "fermentum"
    ];
    words.iter().cloned().collect()
}

