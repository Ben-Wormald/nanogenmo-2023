# Il nome della zebra

A [zebra puzzle](https://en.wikipedia.org/wiki/Zebra_Puzzle) murder mystery set in a Benedictine
Abbey, for [NaNoGenMo 2023](https://github.com/NaNoGenMo/2023/issues/3).

> As we neared the end of Sext a brother let slip to me that the monk who hails from Moudon lives in the cell to the left of the monk whose patron saint is Hildegard of Bingen.


## Data sources

- https://www.englandsimmigrants.com/browse/
- https://en.wikipedia.org/wiki/List_of_saints#Christian_saints_since_AD_300


## Useful commands

```sh
# fix dataset encoding
iconv -f UTF-16LE -t UTF-8 data/sources/englands_immigrants_search_results.csv > data/sources/englands_immigrants_search_results.utf8.csv

# count words in output
cat output.txt | wc -w
```
