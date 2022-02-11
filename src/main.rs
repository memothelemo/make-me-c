use std::{env, fs, process::Command};

static HELLO_DOT_C: &str = "#include <stdio.h>

int main()
{
	printf(\"Hello world!\");
	return 0;
}
";

static MAKEFILE_CONTENTS: &str = "CC		= gcc
EXT		= .c

CFLAGS	= -Wall -g -fPIC -pthread

SRC		= $(shell find src -name \"*$(EXT)\")
OBJ		= $(SRC:$(EXT)=.o)

EXEC	= ./build
RM 		= rm -rf

ifeq ($(OS), Windows_NT)
	EXEC	:= $(EXEC).exe
else
	EXEC	:= $(EXEC).out
endif

default: all
all: clean exec

%.o: %$(EXT)
	$(CC) -o $@ -c $< $(CFLAGS)

run: all
	$(EXEC)

exec: $(OBJ)
	$(CC) -o $(EXEC) $^

clean:
	$(RM) $(EXEC) $(OBJ)
";

fn main() {
	let pwd = env::current_dir().unwrap();
	
	// spawn a makefile
	println!("Generating `Makefile`");
	fs::write(pwd.join("Makefile"), MAKEFILE_CONTENTS).unwrap();

	println!("Generating `src/main.c`");
	fs::create_dir(pwd.join("src")).unwrap();
	fs::write(pwd.join("src").join("main.c"), HELLO_DOT_C).unwrap();

	// compile
	println!("Compiling");
	Command::new("make").output().unwrap();
}
