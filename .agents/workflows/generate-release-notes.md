---
description: Generate Release Notes for the next version of Rudim
---

# Generate Release Notes

This workflow guides the agent through generating release notes for the Rudim chess engine by comparing the current `main` branch with the latest released tag.

## 1. Identify Latest Tag and Commits

- Run `git describe --tags --abbrev=0` to find the latest tag (e.g., `v2.0.0`).
- Run `git log <latest_tag>..main --oneline` to get the list of commits since that tag.
- Note the current "Next Version" (e.g., `v2.1.0`) if provided or inferred.

## 2. Categorize and Format Commits

Categorize the commits into the following sections based on the commit message content:

- **Features**: New functionality, search improvements (e.g., LMR, new evaluation terms).
- **Fixes**: Bug fixes, logic corrections (e.g., hashing issues, rule implementation fixes).
- **Maintenance**: Dependency updates, refactoring, documentation, CI/CD changes.

For each entry, group related commits and list their short hashes in parentheses.

**Example Format:**
- Feature description (hash1, hash2)
- Fix description (hash3)

## 3. Calculate Search Statistics

- Node counts are derived from the `traversal_test_case!` expectations in `tests/search.rs`.
- **Current Version**: Read the node count values from `tests/search.rs` in the `main` branch.
- **Previous Version**: Run `git show <latest_tag>:tests/search.rs` to retrieve the previous expectations.
- **Comparison**: Map the test cases (`Starting`, `Advanced`, `Kiwi Pete`, `Endgame`) to the statistics table.
- **Calculation**: Calculate the reduction as `Previous Node Count / Current Node Count`.
- **Formatting**: Use 'k' suffix for thousands (e.g., `172.2k`) and bold the reduction ratio (e.g., `**1.48x**`).

## 4. Generate Release Notes Draft

Use the following template to generate the release notes:

```markdown
## Features
<List features here>

## Fixes
<List fixes here>

## Maintenance
<List maintenance here>

## Statistics

Node Count Reduction : 
| Position | <Previous_Version> | <Current_Version> | Reduction |
|----------|--------------------|-------------------|-----------|
| Starting | 000k               | 000k              | **0.0x**  |
| Custom   | 000k               | 000k              | **0.0x**  |
| Kiwi Pete| 000k               | 000k              | **0.0x**  |

Tournament vs <Previous_Version>
```
--------------------------------------------------
<Place for tournament results - Leave as placeholder for user to fill>
--------------------------------------------------
```

**Full Changelog**: https://github.com/znxftw/rudim/compare/<Previous_Version>...<Current_Version>
```

## 5. Final Review

- Ensure all commits are accounted for.
- Verify the GitHub comparison link is correct.
- Present the final markdown to the user.
