build:
	cabal configure
	cabal build
run:
	@./dist/build/scheme/scheme $(filter-out $@,$(MAKECMDGOALS))
%:
	@:
