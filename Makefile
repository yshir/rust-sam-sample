STACK_NAME ?= rust-sam-sample-E6073B2D-B5A2-428F-BA6E-4B41386ECCB9
ARCH := aarch64-unknown-linux-gnu

.PHONY: build deploy tests

all: build tests-unit deploy tests-integ
ci: build tests-unit

build:
	cross build --release --target $(ARCH)
	rm -rf ./build
	mkdir -p ./build
	cp -v ./target/$(ARCH)/release/lambda ./build/bootstrap

deploy:
	if [ -f samconfig.toml ]; \
		then sam deploy --stack-name $(STACK_NAME); \
		else sam deploy -g --stack-name $(STACK_NAME); \
	fi

tests-unit:
	cargo test --lib --bins

tests-integ:
	RUST_BACKTRACE=1 API_URL=$$(aws cloudformation describe-stacks --stack-name $(STACK_NAME) \
		--query 'Stacks[0].Outputs[?OutputKey==`ApiUrl`].OutputValue' \
		--output text) cargo test
