# Pry
---
**A Filtering and Text Search API**

## Roadmap

### Tier 1

[] - fuzzy logic
	[] - searching algorithms ( and / or )
		[] - `Double Metaphone` phonetic algorithm for sounds
		[] - `Damerau-Levenshtein distance` for spelling
[] - indexing data
[] - encoding data
	[] - cap'n proto
	[] - json as fallback
[] - codebase
	[] - as small as possible
	[] - files
		[] - accomplish a single task, which can then be used in other files

### Tier 2

[] - establish filtering system
	[] - types
		[] - numeric
		[] - string
		[] - boolean
		[] - misc type

### Tier 3

[] - facets
	[] - allow selecting of characteristics (i.e. brand, genre)
[] - filters
	[] - allow defining of filters
	[] - optional filters
	[] - filter scoring
[] - synonyms (i.e. pants = trousers)
	[] - match only relevent synonyms
[] - prefix search
[] - contains search
[] - optional words

## Details of API

### Filtering System

#### Facets
#### Filters

### Displaying Results

### Fuzzy Matching System

#### Typo Tolerance
#### Relevent Synonyms
#### Types of Fuzzy Searching

### Result Ranking
