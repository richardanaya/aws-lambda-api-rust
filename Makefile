# Project level variables
PROJECT_NAME         = rust-api
PROJECT_DESCRIPTION  = A simple api with Rust and Lambda

# Tools
GIT                  = git
CARGO                = docker run -v $(abspath .vendor/cargo):/home/.cargo -e CARGO_HOME='/home/.cargo' -v `pwd`:/code -w /code richardanaya/aws-lambda-rust:1.28.0 cargo
#TERRAFORM            = docker run -v $(abspath .vendor/terraform):/home/.terraform.d -v $(PWD):/code -w /code/dist/terraform hashicorp/terraform:0.11.8
# Vendoring
ifneq ("$(wildcard .vendor)","")
include .vendor/make/prelude.mk
include .vendor/make/help.mk
include .vendor/make/lambda_rust.mk
include .vendor/make/terraform.mk
endif

.PHONY : all check deploy clean

##all          - Build everything
all: lambda_rust__build terraform__build
	#We need to include this library with our lambda so diesel works
	@zip -9 -j dist/lambdas/rust-api.zip src/libpq.so.5

##clean        - Clean up project
clean: lambda_rust__clean terraform__clean

##check-deploy - Verify next deploy will succeed
check-deploy: all terraform__plan

##deploy       - Deploy infrastructure
deploy: all terraform__apply

##destroy      - Destroy infrastructure
destroy: all terraform__destroy

##vendor       - Vendor makefiles
vendor:
	@echo Vendoring Makefiles
	@rm -rf .vendor
	@$(GIT) clone https://github.com/richardanaya/makefiles.git .vendor/make
	@mkdir -p .vendor/cargo
