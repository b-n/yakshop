# Challenge Notes

Some small notes with regards to some decisions that were made

## My background and/or some context thereof

In recent years, I've mostly been working on:

- Existing Ruby apps built in Ruby on Rails
- Working with Rust for open source projects
- Working as a platform engineer recently, which (in my current job) means a
  more YAML engineering, and less coding time.

With that:

- This is my first time using `warp` and working on Rust web servers in general.
  I'm quite happy/surprised how easy building a very very simple web server in
  this regard.
- I haven't had to deal much with floating point math (Ruby hides this from
  developers), but I enjoyed the challenge and I think handling everything as
  ints and later converting to displayed float values is sensible (although it
  of course adds some complexity to the code)

## The next iteration

The following are the things I would focus on next (in no particular order):

- Observability (e.g. logs, metrics, etc) - Although I would likely look in the
  direction of https://github.com/cloudflare/foundations
- Add some tests for performance.
  - The algorithm for "Next shave date" is fairly naive at present, even though
    it is safe. The next shave date could be specified at time of shave which
    would reduce some math on each day iteration.
  - The algorithm for Milk production is also fairly naive. I am fairly sure
    milk production for a given time period can be achieved in a single math
    call instead of iterating.
- Build environment isolation (e.g. Dockerfile etc)
- Further validation on the XML input document - it currently accepts any XML
  tags, so long as they have the required attributes.
- The mutation endpoint would be implemented
  - with a database instead of loading state from a herd.xml file.
  - with a lot of tests for fringe cases.

## Specific design decisions

- Milk/Wool production was explicitly denormalised. The requirements are
  calling for a specific set of metrics, and normalising this data would lead
  to some performance penalties for an animal which can produce a finite set of
  products.
- Serializing web responses is done from specific response objects. There are a
  number of ways to do this (e.g. putting serde serialization statements on the
  underlying objects), but this is "a way" and is consistent with not adding
  to the shared library that are not used by all consumers.
