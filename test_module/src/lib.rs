pub fn tournament(n: u32) -> Vec<Vec<(u32, u32)>> {
    if n == 2 {
        return vec![vec![(1, 2)]];
    }

    let rounds = n - 1;
    let mut matches = vec![vec![(0, 0); (n / 2) as usize]; rounds as usize];

    let mut teams: Vec<u32> = (1..=n).collect();
    let mut teams_b: Vec<u32> = teams.drain(n as usize / 2..).collect();

    for round in 0..rounds {
        for (i, team_a) in teams.iter().enumerate() {
            let team_b = teams_b[i];
            matches[round as usize][i] = (*team_a, team_b);
        }

        let (last_team_a, first_team_b) = (teams.pop().unwrap(), teams_b.remove(0));
        teams_b.push(last_team_a);
        teams.insert(1, first_team_b);
    }

    matches
}

#[cfg(test)]
mod tests {
    use super::tournament;
    use std::collections::HashSet;

    mod random_tests {
        use super::*;
        use rand::{thread_rng, Rng};

        #[test]
        fn small_teams() {
            let mut rng = thread_rng();
            let sizes = (0..5).map(|_| rng.gen_range(1..=10));
            for s in sizes {
                let n = s * 2;
                test_tournament(n);
            }
        }

        #[test]
        fn large_teams() {
            let mut rng = thread_rng();
            let sizes = (0..10).map(|_| rng.gen_range(1..=25));
            for s in sizes {
                let n = s * 2;
                test_tournament(n);
            }
        }
    }

    mod fixed_tests {
        use super::*;

        #[test]
        fn with_2_teams() {
            test_tournament(2);
        }

        #[test]
        fn with_4_teams() {
            test_tournament(4);
        }

        #[test]
        fn with_8_teams() {
            test_tournament(8);
        }

        #[test]
        fn with_20_teams() {
            test_tournament(20);
        }
    }

    fn round_robin(n: u32) -> Vec<Vec<(u32, u32)>> {
        let mut p = 1;
        (0..n - 1)
            .map(|_| {
                p = if p + 1 > n / 2 {
                    p + 1 - n / 2
                } else {
                    p + n / 2
                };
                std::iter::once((p, n))
                    .chain((1..n / 2).map(|i| {
                        let l = if p + i > n - 1 { p + i - n + 1 } else { p + i };
                        let r = if p > i { p - i } else { p + n - 1 - i };
                        (l, r)
                    }))
                    .collect()
            })
            .collect()
    }

    fn test_tournament(n: u32) {
        let actual = tournament(n);
        let mut expected_matches: HashSet<(u32, u32)> =
            HashSet::from_iter(round_robin(n).iter().flatten().map(|&(a, b)| {
                if a > b {
                    (b, a)
                } else {
                    (a, b)
                }
            }));
        let expected_teams: Vec<_> = (1..n + 1).collect();
        let mut seen = HashSet::new();
        let half = n as usize / 2;
        assert!(
            actual.len() == n as usize - 1,
            "The match table should have {} rounds, yours had {}\n",
            n as usize - 1,
            actual.len()
        );
        for (mut i, round) in actual.iter().enumerate() {
            i += 1;
            assert!(
                round.len() == half,
                "Round {i}: each round should have {half} matches, yours had {}:\n{round:?}\n",
                round.len()
            );
            let mut participants = Vec::new();
            for (mut a, mut b) in round {
                if a > b {
                    (a, b) = (b, a);
                }
                assert!(
                    !seen.contains(&(a, b)),
                    "Round {i}: Match ({a} vs {b}) has already been played!\n"
                );
                seen.insert((a, b));
                assert!(
                    expected_matches.contains(&(a, b)),
                    "Round {i}: ({a} vs {b}) is not a valid match!\n"
                );
                participants.extend_from_slice(&[a, b]);
                expected_matches.remove(&(a, b));
            }
            participants.sort();
            assert!(participants == expected_teams, "Round {i}: Every team must participate in a round.\n{participants:?} should equal {expected_teams:?}\n")
        }
        assert!(
            expected_matches.is_empty(),
            "{} matches were not played:\n{expected_matches:?}\n",
            expected_matches.len()
        );
    }
}
