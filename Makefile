NAME		= n-puzzle
CARGO		= cargo
BINARY_PATH	= target/release/$(NAME)

all: $(NAME)

$(NAME):
	@$(CARGO) build --release
	@cp $(BINARY_PATH) .

clean:
	@$(CARGO) clean

fclean: clean
	@rm -f $(NAME)

re: fclean all

test:
	@$(CARGO) test

run:
	@$(CARGO) run $(ARGS)

.PHONY: all clean fclean re test run