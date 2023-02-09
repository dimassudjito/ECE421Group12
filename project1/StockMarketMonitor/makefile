src = src/group12/core/
bin = bin/group12/core/
pkg = group12/core/
objs_imp = $(bin)APIFinance.class $(bin)Shares.class $(bin)ShareInfo.class $(bin)ShareUtil.class  $(bin)PickShareImperative.class
objs_func = $(bin)APIFinance.class $(bin)Shares.class $(bin)ShareInfo.class $(bin)ShareUtil.class  $(bin)PickShareFunctional.class

# build targets: 
# 	Build either the functional version 
# 	or the imperative version
func: clean bin/module-info.class $(objs_func)
	cd bin && clear && java $(pkg)PickShareFunctional
imp: clean bin/module-info.class $(objs_imp)
	cd bin && clear && java $(pkg)PickShareImperative

# compile targets
$(bin)PickShareImperative.class: $(src)PickShareImperative.java
	cd src/ && javac $(pkg)PickShareImperative.java -d ../bin
$(bin)PickShareFunctional.class: $(src)PickShareFunctional.java
	cd src/ && javac $(pkg)PickShareFunctional.java -d ../bin
$(bin)ShareInfo.class: $(src)ShareInfo.java
	cd src/ && javac $(pkg)ShareInfo.java -d ../bin
$(bin)Shares.class: $(src)Shares.java
	cd src/ && javac $(pkg)Shares.java -d ../bin
$(bin)ShareUtil.class: $(src)ShareUtil.java
	cd src/ && javac $(pkg)ShareUtil.java -d ../bin
$(bin)APIFinance.class: $(src)APIFinance.java
	cd src/ && javac $(pkg)APIFinance.java -d ../bin
bin/module-info.class: src/module-info.java
	cd src/ && javac module-info.java -d ../bin

# clean bin directory and restore package structure
clean:
	rm -rf bin/group12 || true
	mkdir bin/group12 bin/group12/core