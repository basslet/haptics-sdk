# Requirements

This branch tries [tracey](https://github.com/bearcove/tracey) 1.4.0 on the `haptic_dsp` crate. It
is an experiment: nothing here is on `main`, and whether it ever goes there is open.

Requirements go in `docs/requirements/<area>.md`. Each one starts with its ID at column 0, in the
form `r[area.name]`, then states one behaviour with "shall". This file is part of the spec too, so an
ID at column 0 here, outside a code fence, would define a requirement.

Mark the code above the item it covers, in a line comment: `// r[impl <id>]` in the code,
`// r[verify <id>]` in a test. tracey drops any other verb without a warning.

Check everything with `tracey query validate --deny warnings`, and look one requirement up with
`tracey query rule <id>`.
