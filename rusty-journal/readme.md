# RUSTY-JOURNAL COMMAND LINE TOOL

use below commands to validate cmd tool

```console
$ cargo run -- -j test-journal.json add "buy milk"

$ cargo run -- -j test-journal.json add "take the dog for a walk"

$ cargo run -- -j test-journal.json add "water the plants"

$ cargo run -- -j test-journal.json list
1: buy milk                                           [2023-10-15 13:12]
2: take the dog for a walk                            [2023-10-15 13:12]
3: water the plants                                   [2023-10-15 13:12]

$ cargo run -- -j test-journal.json done 2

$ cargo run -- -j test-journal.json list
1: buy milk                                           [2023-10-15 13:12]
2: water the plants                                   [2023-10-15 13:12]
```