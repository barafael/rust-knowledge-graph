graph:
    cargo depgraph --workspace-only > graph.dot

image: graph
    dot -Tpng -Gdpi=400 graph.dot > image.png

view: graph
    xdot graph.dot &