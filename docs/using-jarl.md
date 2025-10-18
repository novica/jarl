# Using Jarl

## Linting and fixing

`jarl check` is the command required to diagnoze one or several files.
It takes a path as its first argument, such as `jarl check .` to check all files starting from the current directory.
This command will return a list of diagnostics, one per rule violation.

This is already useful information, but it can be tedious to fix those violations one by one.
To help addressing this issue, Jarl can apply automatic fixes to some of those diagnostics.
This is done simply by passing the argument `--fix`, such as `jarl check . --fix`.

For some rules, an automatic fix cannot be inferred simply based on static code analysis.
For example, the rule `for_loop_index` reports cases such as `for (x in foo(x))`, which is problematic because `x` is both in the index and in the sequence component of the loop.
It is recommended to rename `x` to disambiguate its use, but this requires manual interventation.

::: {.callout-warning}
## Automatic fixes and version control

Using `--fix` may modify several files at once depending on the path you specified.
It can be hard to inspect the changes or to revert a large number of changes, so Jarl provides two safeguards:

1. if the file isn't tracked by a Version Control System (VCS, such as Git), then fixes are not applied and you need to specify `--allow-no-vcs` to apply them;
2. if the file is tracked by a VCS but the status isn't clean (meaning that some files aren't committed), then fixes are not applied and you need to specify `--allow-dirty` to apply them. This is to prevent cases where fixes would be mixed together with other unrelated changes and therefore hard to inspect.
:::

Automatic fixes are distinguished between "safe" and "unsafe".

**Safe fixes** do not change the behavior of the code when it runs, but improve its readability or performance, for instance by using more appropriate functions (see [`any_is_na`](rules/any_is_na.md)).

**Unsafe fixes** may change the behavior of the code when it runs.
For example, [`all_equal`](rules/all_equal.md) reports cases such as `!all.equal(x, y)`.
This code is likely a mistake because `all.equal()` returns a character vector and not `FALSE` when `x != y`.
Jarl could fix this to be `!isTRUE(all.equal(x, y))` instead, but this would change the behavior of the code, so it is marked "unsafe".

By default, only safe fixes are applied.
To apply the unsafe fixes, use `--unsafe-fixes`, e.g. `jarl check . --fix --unsafe-fixes`.

## Selecting rules

## Ignoring pieces of code

##
