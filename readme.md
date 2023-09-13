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
	- searching algorithms 
                - [X] fzf syntax
                - [] filters
- [X] unicode-aware
- encoding data
- codebase
	- [ ] modular
        - [ ] query
        - [ ] index
         - [ ] file insertion, deletion, updating
         - [ ] <insert other default modules here?
         - [ ] user-defined modules
    - should only do the abdolute core functionality of searching
                     - [ ] querying
                        - [X] parse search query
                        - [X] pass search to database
                            - [X] build primative query infrastructure around database
                        - [ ] filter queries
                        - [ ] query file metadata
                     - [X] indexing
                        - [X] detect language of file
                        - [X] split file and remove stopwords
                     - [ ] database operations
                        - [X] inserting files
                        - [ ] updating files
                        - [ ] removing files

## Index

maybe create index trait with necessary functions ??
    - access (directory/file/tag)
    - create index
    - insert to index
    - update to index
    - remove from index

maybe use tower crate in order to apply actions to the index
    - request: Operation (or something similar, maybe use trait ??)
    - response: success -> ?? (return nothing maybe, only confirmation of success ??), failure -> error

### Indexing file

identify language
split into lines, then words
remove stopwords

split text files into sentences (split by periods), then group into paragraphs (segment breaks, i.e newlines), then into files

how to do this very fast

### Storage Layout

- generic over external db (i.e. does not store any files, only references to them)
- store relative file path

word -> documents or documents -> word
find returned ids in the sled db and return the file names/documents

database -> tags -> files/directories (tags or user-defined ways to organize the files and directories)
multiple tags -> put pointer to word/file into multiple entries

file_to_words -> how to store variable length words ??

## Querying

- take in search (parse though combine)
- optionally specify the directory / db (if more than one are running)

## Metadata
associated with each tag(or directory)(or file) ??
how to store the metadata with the associated object

## Index watchers

watching a specific path
run specified script on a specified event

### Events

what to put for events ??
possibly executor actions
could name events for easy watching

## Operation

ways to interface with the engine index -> query, update, insert, delete
special batch operation -> contains multiple executors to execute all at once

index needs to be running

requirements (tower service, how to accomplish):
    - [X] (mutably) access index
        - not needed
    - [X] perform actions on index
        - operation struct
    - [X] inject into index (thus, no need for insert_*, update_*, etc. functions in the index itself)
        - event + subscriber system (inspired by tracing)
