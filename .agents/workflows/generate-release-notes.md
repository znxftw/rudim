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

Categorize the commits into the sections below. **Keep entries ultra-concise, punchy, and brief.** Avoid verbose explanations, listing modified files, or adding implementation details. Condense the entries into simple noun phrases or short action statements.

- **Features**: New functionality, search improvements (e.g., LMR, RFP, PV tables).
- **Fixes**: Bug fixes, logic corrections (e.g., UCI reporting fixes, draw checks).
- **Maintenance**: Search & evaluation optimizations, refactors, docs, CI/CD changes.

Format the list using a clean, lowercase style with short bullet points, grouping related commits and listing their short hashes in parentheses. If a feature or fix was completely reverted within the same commit range, omit or combine them to keep the notes clean.

**Example Format:**
- reverse futility pruning (hash1, hash2)
- pv table (hash3)

## 3. Calculate Search Statistics

- Node counts are derived from the `traversal_test_case!` expectations in `tests/search.rs`.
- **Current Version**: Read the node count and depth values from `tests/search.rs` in the current branch.
- **Previous Version**:
  - Check if search depths in `tests/search.rs` have changed between the previous version and the current version.
  - **If depths are the same**: Run `git show <latest_tag>:tests/search.rs` to retrieve the previous expectations.
  - **If depths have changed (Same-Depth Comparison)**:
    1. Temporarily checkout to the previous version tag (`git checkout <latest_tag>`).
    2. Modify the test cases in `tests/search.rs` to run at the new depths used in the current version.
    3. Run the traversal tests (e.g., `cargo test --test search traversal -- --nocapture`) to measure the actual node counts taken by the previous version at those depths.
    4. Revert the temporary changes and checkout back to the original branch (`git checkout -- tests/search.rs && git checkout -`).
- **Calculation**: Calculate the reduction as `Previous Node Count / Current Node Count`.
- **Formatting**: Use 'k' suffix for thousands (e.g., `172.2k`) and bold the reduction ratio (e.g., `**1.48x**`).

## 4. Generate Release Notes Draft

- Read the current network name from the `NETWORK_NAME` constant in `build.rs` and link it to its corresponding release tag page under `https://github.com/znxftw/rudim-networks/releases/tag/<network_name>`.
- Use the following template to generate the release notes:

```markdown
Network: [<network_name>](https://github.com/znxftw/rudim-networks/releases/tag/<network_name>)

## Features
<List features here>

## Fixes
<List fixes here>

## Maintenance
<List maintenance here>

## Statistics

Node Count Reduction (Same-Depth Comparison):
| Position | Depth | <Previous_Version> (Previous) | <Current_Version> (Current) | Reduction |
|----------|-------|-------------------|------------------|-----------|
| Starting | 9     | 000k              | 000k             | **0.0x**  |
| Advanced | 11    | 000k              | 000k             | **0.0x**  |
| Kiwi Pete| 8     | 000k              | 000k             | **0.0x**  |
| Endgame  | 13    | 000k              | 000k             | **0.0x**  |

Tournament vs <Previous_Version>
```
--------------------------------------------------
<Place for tournament results - Leave as placeholder for user to fill>
--------------------------------------------------
```

**Full Changelog**: https://github.com/znxftw/rudim/compare/<Previous_Version>...<Current_Version>

*NB: If your system does not support AVX2, please recompile the binary on your native machine. AVX2 was chosen as a default as [>95% of users](https://store.steampowered.com/hwsurvey) have a compatible machine.*
```


## 5. Final Review

- Ensure all commits are accounted for.
- Verify the GitHub comparison link is correct.
- Present the final markdown to the user.
