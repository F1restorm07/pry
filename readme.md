# Pry
---
**Full-text search**

only focusing on indexing documents, querying the indexes, and returning results

*will expand later once the base is finished*

## Inspiration
- Telescope.nvim
- Meilisearch
- Tantivy
- Tinysearch
- many others

## Roadmap

- fuzzy logic
	- searching algorithms ( and / or )
                - [ ] Damerau-Levenshtein distance
                - [X] fzf syntax
- [ ] unicode-aware
- indexing data
        - [X] index into fst
        - [ ] roaring bitmaps
- encoding data
        - [ ] fst
- [ ] codebase
	- [ ] as small as possible
	- [ ] files accomplish a single task, which can then be used in other files
