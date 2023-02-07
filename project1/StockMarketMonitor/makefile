src = src/group12/core/
bin = bin/group12/core/
pkg = group12/core/
objs = $(bin)APIFinance.class $(bin)Shares.class $(bin)ShareInfo.class $(bin)ShareUtil.class  $(bin)PickShareImperative.class

all: bin/module-info.class $(objs)
	cd bin && \
		java $(pkg)PickShareImperative

$(bin)PickShareImperative.class: $(src)PickShareImperative.java
	cd src/ && \
		javac $(pkg)PickShareImperative.java -d ../bin

$(bin)ShareInfo.class: $(src)ShareInfo.java
	cd src/ && \
		javac $(pkg)ShareInfo.java -d ../bin

$(bin)Shares.class: $(src)Shares.java
	cd src/ && \
		javac $(pkg)Shares.java -d ../bin

$(bin)ShareUtil.class: $(src)ShareUtil.java
	cd src/ && \
		javac $(pkg)ShareUtil.java -d ../bin

$(bin)APIFinance.class: $(src)APIFinance.java
	cd src/ && \
		javac $(pkg)APIFinance.java -d ../bin

bin/module-info.class: src/module-info.java
	cd src/ && \
		javac module-info.java -d ../bin