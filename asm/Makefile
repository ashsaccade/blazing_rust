asm:
	docker run --rm -ti \
	    --platform=linux/amd64 \
		-v $(CURDIR):/asm \
	    asm bash

build-docker:
	docker build --platform=linux/amd64 -t asm .

hello:
	clang -nostdlib -no-pie t201_hello_world.s -o hello
	./hello

sum:
	clang -no-pie t202_sum.s -o sum
	./sum ; echo $$?

fib:
	clang -no-pie t203_fibonacci.s -o fib
	./fib ; echo $$?
