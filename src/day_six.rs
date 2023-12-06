const RACES: [(i32, i32); 4] = [
    (59, 597),
    (79, 1234),
    (65, 1032),
    (75, 1328)
];
const RACES_PART2: (u64, u64) = (59796575, 597123410321328);

pub fn part1() {
    let mut ways_to_win = Vec::new();

    for race in RACES {
        let total_time = race.0;
        let distance_to_beat = race.1;
        let mut winning_hold_times = Vec::new();

        for hold_time in 0..total_time {
            let remaining_time = total_time - hold_time;
            if (remaining_time * hold_time > distance_to_beat) {
                winning_hold_times.push(hold_time);
            }
        }
        ways_to_win.push(winning_hold_times.len())
    }
    let mut total = 1;
    for num in ways_to_win {
        total *= num;
    }
    print!("{}", total);
}

pub fn part2() {
    let total_time = RACES_PART2.0;
    let distance_to_beat = RACES_PART2.1;
    let mut ways_to_win = 0;

    for hold_time in 0..total_time {
        let remaining_time = total_time - hold_time;
        if (remaining_time * hold_time > distance_to_beat) {
            ways_to_win +=1 ;
        }
    }
    print!("{}", ways_to_win);
}