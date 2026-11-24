# A simple graph library

`graph` is a simple graph library which provides a basic trait for directed,
unweighted graphs and two default implementations:

- One for a graph with general nodes, which are stored as equatable copies
- One for an **index graph**, where all nodes are indices in a vector of a
  certain length. This can speed things up in certain situations.
