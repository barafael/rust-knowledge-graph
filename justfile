graph:
    cargo depgraph --workspace-only > graph.dot

image: graph
    dot -Tpng -Gdpi=400 graph.dot > image.png

view: graph
    xdot graph.dot &

clean:
    rm image.png graph.dot

all: clean image

docs:
    cargo doc --workspace --no-deps

docs-open:
    cargo doc --workspace --no-deps --open

