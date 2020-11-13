// The Western Suburbs Croquet Club has two categories of membership, Senior and Open. They would like your help with an application form that will tell prospective members which category they will be placed.

// To be a senior, a member must be at least 55 years old and have a handicap greater than 7. In this croquet club, handicaps range from -2 to +26; the better the player the lower the handicap.
// Input

// Input will consist of a list of lists containing two items each. Each list contains information for a single potential member. Information consists of an integer for the person's age and an integer for the person's handicap.

// Note for F#: The input will be of (int list list) which is a List<List>
// Example Input

// vec![(45, 12), (55,21), (19, -2), (104, 20)])

// Output

// Output will consist of a list of string values (in Haskell: Open or Senior) stating whether the respective member is to be placed in the senior or open category.
// Example Output

// vec!["Open", "Senior", "Open", "Senior"]

fn open_or_senior(data: Vec<(i32, i32)>) -> Vec<String> {
    data
        .iter()
        .map(|(age, handicap)| {
            if age >= &55 && handicap > &7 {
                "Senior"
            } else {  
                "Open"
            }.to_string()
        })
        .collect()
}