PY_PROJECT_DIR := python/stockstats-polars
PY_MANIFEST := $(PY_PROJECT_DIR)/Cargo.toml
UV := uv
UV_RUN := $(UV) run --group dev
PY_RUSTFLAGS := -C link-arg=-undefined -C link-arg=dynamic_lookup

.PHONY: py-build py-develop py-wheel py-test clean

py-build:
	RUSTFLAGS="$(PY_RUSTFLAGS) $$RUSTFLAGS" cargo build -p stockstats-polars-py

py-develop:
	$(UV_RUN) maturin develop --manifest-path $(PY_MANIFEST)

py-wheel:
	$(UV_RUN) maturin build --manifest-path $(PY_MANIFEST)

py-test:
	$(UV_RUN) pytest tests

clean:
	cargo clean
