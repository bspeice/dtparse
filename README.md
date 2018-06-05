# dtparse
A [dateutil](https://github.com/dateutil/dateutil)-compatible timestamp parser for Rust

## Where it stands

Currently, most of the non-timezone functionality is complete (absent a couple of failing test cases that use fractional minutes).

Timezone support in Rust right now is a [bit limited](https://github.com/chronotope/chrono-tz), but should be enough that
a compatible parser can be built.
