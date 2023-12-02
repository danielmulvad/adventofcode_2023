/**
--- Part Two ---
The Elf says they've stopped producing snow because they aren't getting any water! He isn't sure why the water stopped; however, he can show you how to get to the water source to check it out for yourself. It's just up ahead!

As you continue your walk, the Elf poses a second question: in each game you played, what is the fewest number of cubes of each color that could have been in the bag to make the game possible?

Again consider the example games from earlier:

Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
In game 1, the game could have been played with as few as 4 red, 2 green, and 6 blue cubes. If any color had even one fewer cube, the game would have been impossible.
Game 2 could have been played with a minimum of 1 red, 3 green, and 4 blue cubes.
Game 3 must have been played with at least 20 red, 13 green, and 6 blue cubes.
Game 4 required at least 14 red, 3 green, and 15 blue cubes.
Game 5 needed no fewer than 6 red, 3 green, and 2 blue cubes in the bag.
The power of a set of cubes is equal to the numbers of red, green, and blue cubes multiplied together. The power of the minimum set of cubes in game 1 is 48. In games 2-5 it was 12, 1560, 630, and 36, respectively. Adding up these five powers produces the sum 2286.

For each game, find the minimum set of cubes that must have been present. What is the sum of the power of these sets?
*/
use std::collections::HashMap;

pub struct PartTwo<'a> {
    input_values: &'a [String],
}

impl<'a> PartTwo<'a> {
    pub fn new(input_values: &'a [String]) -> Self {
        Self { input_values }
    }

    pub fn calculate_power_of_games(&self) -> Result<u32, &'static str> {
        let mut total_power = 0;

        for game in self.input_values {
            let (_, game_data) = game.split_once(": ").ok_or("Invalid game format")?;

            let mut max_cubes: HashMap<&str, u32> = HashMap::new();

            for subset in game_data.split("; ") {
                for color_count in subset.split(", ") {
                    let mut parts = color_count.trim().split_whitespace();
                    let count_str = parts.next().ok_or("Invalid color count format")?;
                    let color = parts.next().ok_or("Invalid color count format")?;

                    let count = count_str
                        .parse::<u32>()
                        .map_err(|_| "Invalid count format")?;

                    // If the color is not in the map, insert it with the count.
                    // If the color is in the map, update the count if the new count is greater.
                    *max_cubes.entry(color).or_insert(0) =
                        max_cubes.get(color).map_or(count, |&c| c.max(count));
                }
            }

            let power = max_cubes.values().product::<u32>();
            total_power += power;
        }

        Ok(total_power)
    }

    pub fn run(&self) -> u32 {
        self.calculate_power_of_games().unwrap_or(0)
    }
}
