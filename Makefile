#   -------------------------------------------------------------
#   Alkane :: Build
#   - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#   Project:        Nasqueron
#   License:        BSD-2-Clause
#   -------------------------------------------------------------

CARGO=cargo
MKDIRHIER=mkdir -p
WGET=wget
RM=rm -rf

#   -------------------------------------------------------------
#   Main targets
#   - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

all: data build

clean: clean-data clean-build

#   -------------------------------------------------------------
#   Build targets
#   - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

data: src/data/public_suffix_list.dat

build: target/release/alkane

src/data/public_suffix_list.dat:
	${MKDIRHIER} src/data
	${WGET} -O src/data/public_suffix_list.dat https://publicsuffix.org/list/public_suffix_list.dat

target/release/alkane:
	cd src && ${CARGO} build --release

#   -------------------------------------------------------------
#   Clean targets
#   - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

clean-data:
	${RM} src/data/public_suffix_list.dat

clean-build:
	${RM} target/release
