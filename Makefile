# Project level variables
PROJECT_NAME         = rust-api
PROJECT_DESCRIPTION  = 

# Tools
GIT                  = git

# Vendoring
ifneq ("$(wildcard .vendor)","")
include .vendor/make/prelude.mk
include .vendor/make/help.mk
include .vendor/make/lambda_rust.mk
include .vendor/make/terraform.mk
endif

.PHONY : all check deploy clean 

##all    - Build everything
all: lambda_rust__build terraform__build 

##clean  - Clean up project
clean: lambda_rust__clean terraform__clean 

check-deploy: all terraform__plan


deploy: all terraform__apply


##vendor - Vendor makefiles
vendor:
	@echo Vendoring Makefiles
	@rm -rf .vendor
	@$(GIT) clone https://github.com/richardanaya/makefiles.git .vendor/make
