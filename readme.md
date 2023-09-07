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
- indexing data
- encoding data
- [ ] codebase
	- [ ] as small as possible
	- [ ] files accomplish a single task, which can then be used in other files
        - [ ] give ability create extensions to suit needs
        - [ ] should only do the abdolute core functionality of searching
                         - [ ] querying
                            - [X] parse search query
                            - [X] pass search to database
                                - [X] build primative query infrastructure around database
                            - [ ] query file metadata
                         - [X] indexing
                            - [X] detect language of file
                            - [X] split file and remove stopwords
                         - [ ] database operations
                            - [X] inserting files
                            - [ ] updating files
                            - [ ] removing files

### Indexing file

identify language
split into lines, then words
remove stopwords

split text files into sentences (split by periods), then group into paragraphs (segment breaks, i.e newlines), then into files

how to do this very fast

### Storage Layout

- use sled crate for db
    - hashmap: file name (path relative to db directory) -> id
    - database: word -> vec of file ids
    - collect files into db trees (directory)
    - single db over all trees
- generic over db ??
- files (individual files), and directories (sled tree)

word -> documents or documents -> word
use fst map for words to documents (roaring bitmap of document ids) (<- search through this maybe)
find returned ids in the sled db and return the file names/documents

### Tagging files ??

put words/files (??) into trees in sled_db
multiple tags -> put word/file into multiple trees ??

## Querying

- take in search (parse though combine)
- optionally specify the directory / db (if more than one are running)

## Metadata

associated with each tree ??
