# fishextract

A rust native implementation of a `mermaid.js`-interpretable graphs in.

## Foundation

I don't want chromium _and_ puppeteer as 2nd order dependencies when
writing a `mdbook-*` plugin (specifically: [mdbook-fishextract](https://github.com/drahnr/fancybook/tree/master/mdbook-fishextract) ).
Things break with all the complexity in that stack.

## Status

Nothing is implemented, to me state and sequence diagrams are at the top of the list, everything else being
a nice to have addendum:

* [ ] sequence diagrams
* [ ] state diagrams
* [ ] flow chart

* [ ] user journey
* [ ] requirement diagrams
* [ ] entity relationship diagrams
* [ ] pie chart
* [ ] gannt chart
* [ ] quadrant chart
* [ ] class diagrams