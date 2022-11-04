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

## Societal or Educational Value

- step by step process of querying document
- find out the behind-the-scenes when typing a search
- when typing a letter, bubbles pop out with other letters
- *online planner*
        - use search engine in calendar
        - tuned for schoolwork/education
        - for teachers
                - add homework/assignments
                - set due dates
                - set daily plans
                - canvas, but free
        - calendar
        - click on days to reveal daily planner
        - use pry to search planner
        - set filters or timeslots
