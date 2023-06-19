.PHONY: build docs

all: build docs

build:
	cd extension && maturin build && env pip install --force-reinstall target/wheels/*.whl

docs:
	cd docs && make clean html