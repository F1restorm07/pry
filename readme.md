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
- Skim
- Sonic
- many others

## Roadmap

- fuzzy logic
	- searching algorithms ( and / or )
                - [X] fzf syntax
- [X] unicode-aware
- indexing data
        - [X] index into fst
        - [ ] roaring bitmaps
- encoding data
        - [X] fst
- [ ] codebase
	- [ ] as small as possible
	- [ ] files accomplish a single task, which can then be used in other files
        - [ ] give ability create extensions to suit needs
        - [ ] should only do the abdolute core functionality of searching
                         - [ ] querying
                         - [ ] indexing
                         - [ ] other essential searching stuff
