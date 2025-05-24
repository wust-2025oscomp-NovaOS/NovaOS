###############################################################################
# 全局变量定义部分
###############################################################################

# Building 构建配置
# TARGET: 目标架构为RISC-V 64位通用架构
# MODE: 构建模式(release/debug)
# KERNEL_ELF: 生成的内核ELF文件路径
# KERNEL_BIN: 剥离元数据后的内核二进制文件
# DISASM_TMP: 反汇编临时文件存储路径
# FS_IMG: 用户态文件系统镜像路径
# APPS: 用户程序源码路径通配符
TARGET := riscv64gc-unknown-none-elf
MODE := release
KERNEL_ELF := target/$(TARGET)/$(MODE)/os
KERNEL_BIN := $(KERNEL_ELF).bin
DISASM_TMP := target/$(TARGET)/$(MODE)/asm
FS_IMG_SRC := ../ext4_rs/ex4.img
FS_IMG_BAK := ../easy-fs-fuse/fs.img
FS_IMG_TAR := ../user/target/$(TARGET)/$(MODE)/fs.img
APPS := ../user/src/bin/*

# BOARD 硬件平台配置
# BOARD: 开发板类型(默认为qemu模拟器)
# SBI: 使用的Supervisor二进制接口实现
# BOOTLOADER: 引导加载程序路径
BOARD := qemu
SBI ?= rustsbi
BOOTLOADER := ../bootloader/$(SBI)-$(BOARD).bin

# GUI 图形界面控制
# GUI_OPTION: 根据GUI标志控制QEMU显示输出(off时隐藏GUI)
GUI ?= off
ifeq ($(GUI), off)
	GUI_OPTION := -display none
endif

# Building mode argument 构建模式参数
# MODE_ARG: release模式时添加--release构建参数
ifeq ($(MODE), release)
	MODE_ARG := --release
endif

# KERNEL ENTRY 内核入口地址
KERNEL_ENTRY_PA := 0x80200000

# Binutils 二进制工具配置
# OBJDUMP: 反汇编工具及架构参数
# OBJCOPY: 二进制转换工具及架构参数
OBJDUMP := rust-objdump --arch-name=riscv64
OBJCOPY := rust-objcopy --binary-architecture=riscv64

# Disassembly 反汇编选项配置
DISASM ?= -x


###############################################################################
# 主要构建目标
###############################################################################

# 主构建目标：检查环境 → 构建内核 → 生成文件系统镜像
build: env $(KERNEL_BIN) fs-img 

# 环境检查与依赖安装：
# 1. 检查并安装RISC-V目标工具链
# 2. 安装cargo-binutils工具
# 3. 添加rust源码组件
# 4. 添加LLVM工具预览组件
env:
	(rustup target list | grep "riscv64gc-unknown-none-elf (installed)") || rustup target add $(TARGET)
	cargo install cargo-binutils
	rustup component add rust-src
	rustup component add llvm-tools-preview

# 内核二进制文件生成规则：
# 使用objcopy工具从ELF文件生成纯二进制文件，移除所有符号和重定位信息
$(KERNEL_BIN): kernel
	@$(OBJCOPY) $(KERNEL_ELF) --strip-all -O binary $@

# 文件系统镜像生成规则：
# 1. 构建用户程序
# 2. 删除旧文件系统镜像
# 3. 使用easy-fs-fuse工具打包用户程序生成新镜像
fs-img: $(APPS)
	@cd ../user && make build TEST=$(TEST)
	@rm -f $(FS_IMG_TAR)
	@cp $(FS_IMG_SRC) $(FS_IMG_BAK)
	@cd ../easy-fs-fuse && cargo run --release -- -s ../user/src/bin/ -t ../user/target/riscv64gc-unknown-none-elf/release/
	@cp $(FS_IMG_BAK) $(FS_IMG_TAR)
	@rm -f $(FS_IMG_BAK)

# 内核构建规则：
# 1. 显示当前硬件平台
# 2. 根据平台选择链接脚本
# 3. 执行cargo release构建
# 4. 清理临时链接脚本
kernel:
	@echo Platform: $(BOARD)
	@cp src/linker-$(BOARD).ld src/linker.ld
	@cargo build --release
	@rm src/linker.ld


###############################################################################
# 辅助功能目标
###############################################################################

# 清理构建产物
clean:
	@cargo clean

# 反汇编查看（使用less分页器）
disasm: kernel
	@$(OBJDUMP) $(DISASM) $(KERNEL_ELF) | less

# 反汇编到文件并用vim查看
disasm-vim: kernel
	@$(OBJDUMP) $(DISASM) $(KERNEL_ELF) > $(DISASM_TMP)
	@nvim $(DISASM_TMP)
	@rm $(DISASM_TMP)


###############################################################################
# QEMU运行与调试配置
###############################################################################

# QEMU虚拟机参数配置：
# -machine: 指定虚拟机类型为virt
# -bios: 指定引导加载程序
# -serial: 重定向串口输出到标准IO
# -device: 加载内核二进制到指定内存地址
# -drive: 挂载文件系统镜像
# -netdev: 配置网络端口转发规则
QEMU_ARGS := -machine virt \
			 -bios $(BOOTLOADER) \
			 -serial stdio \
			 $(GUI_OPTION) \
			 -device loader,file=$(KERNEL_BIN),addr=$(KERNEL_ENTRY_PA) \
			 -drive file=$(FS_IMG_TAR),if=none,format=raw,id=x0 \
			 -device virtio-blk-device,drive=x0 \
			 -device virtio-gpu-device \
			 -device virtio-keyboard-device \
			 -device virtio-mouse-device \
			 -device virtio-net-device,netdev=net0 \
			 -netdev user,id=net0,hostfwd=udp::6200-:2000,hostfwd=tcp::6201-:80

# 标准运行目标（实际执行run-inner）
run: run-inner

fdt:
	@qemu-system-riscv64 -M 128m -machine virt,dumpdtb=virt.out
	fdtdump virt.out

QEMU_NAME := qemu-system-riscv64
qemu-version-check:
	@sh scripts/qemu-ver-check.sh $(QEMU_NAME)

# 带版本检查的QEMU运行
run-inner: qemu-version-check build
	@qemu-system-riscv64 $(QEMU_ARGS)

# 调试模式启动：
# 使用tmux分屏，左侧运行QEMU调试服务，右侧连接GDB调试器
# -s: 开启GDB服务器监听1234端口
# -S: 启动时暂停CPU执行
debug: qemu-version-check build
	@tmux new-session -d \
		"qemu-system-riscv64 $(QEMU_ARGS) -s -S" && \
		tmux split-window -h "riscv64-unknown-elf-gdb -ex 'file $(KERNEL_ELF)' -ex 'set arch riscv:rv64' -ex 'target remote localhost:1234'" && \
		tmux -2 attach-session -d

gdbserver: qemu-version-check build
	@qemu-system-riscv64 $(QEMU_ARGS) -s -S

gdbclient:
	@riscv64-unknown-elf-gdb -ex 'file $(KERNEL_ELF)' -ex 'set arch riscv:rv64' -ex 'target remote localhost:1234'

# 伪目标声明（防止与同名文件冲突）
.PHONY: build env kernel clean disasm disasm-vim run-inner fs-img gdbserver gdbclient fdt qemu-version-check