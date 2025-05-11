# 初赛系统调用

### *#*  getcwd 17

功能：将当前工作目录的绝对路径名复制到`buf`指向的数组中，该数组长度为`size`。成功时返回一个指向字符串的指针，该字符串包含当前工作目录的路径名；失败时返回NULL。

声明：`char *getcwd(char *buf, size_t size);`

细节:

* 如果当前工作目录的绝对路径名长度（包括终止的空字符）超过`size`字节，则返回`NULL`，并将`errno`设置为`ERANGE`；应用程序应检查此错误，必要时分配一个更大的缓冲区。

* 作为对 POSIX.1 - 2001 标准的扩展，如果`buf`为`NULL`，glibc 的`getcwd()`会使用`malloc`动态分配缓冲区。在这种情况下，当`size`不为零，分配的缓冲区长度为`size`；当`size`为零时，会分配足够大的缓冲区。调用者应使用`free`释放返回的缓冲区。

### *#* pipe2 59 

功能：创建一个管道，这是一个单向数据通道，可用于进程间通信。成功时，返回 0;出错时，返回 -1。

声明：`int pipe2(int pipefd[2], int flags);`

细节：

* 数组`pipefd`用于返回两个指向管道两端的文件描述符。`pipefd[0]`指向管道的读端，`pipefd[1]`指向管道的写端。写入管道写端的数据会由内核进行缓冲，直到从管道读端读取数据。
* 可以在`flags`中按位或运算以下值来获得不同的行为：
  - O_CLOEXEC：在两个新的文件描述符上设置执行时关闭（FD_CLOEXEC）标志。
  - O_DIRECT（自 Linux 3.4 起）：创建一个以 “数据包” 模式执行 I/O 操作的管道。每次对管道的`write`操作都会被当作一个单独的数据包处理，而`read`操作每次将读取一个数据包。
  - O_NONBLOCK：在新文件描述符所引用的打开文件描述上设置`O_NONBLOCK`文件状态标志。
  - O_NOTIFICATION_PIPE：自 Linux 5.8 起，通用通知机制构建在管道之上，内核会将通知消息拼接到用户空间打开的管道中。管道的所有者必须告知内核要监视哪些事件源，并且还可以应用过滤器来选择哪些子事件应放入管道中。

### *#* dup 23

功能：分配一个新的文件描述符，该描述符与`oldfd`所引用的打开文件描述相同。成功时，返回新的文件描述符；出错时，返回 -1。

声明：`int dup(int oldfd);`

细节：

* 新文件描述符的编号必定是调用进程中未使用的编号最小的文件描述符。
* 成功返回后，旧的和新的文件描述符可以互换使用。由于这两个文件描述符引用同一个打开文件描述，它们共享文件偏移量和文件状态标志。例如，如果通过对其中一个文件描述符使用`lseek`修改了文件偏移量，另一个文件描述符的偏移量也会随之改变 。
* 但这两个文件描述符并不共享文件描述符标志（即执行时关闭标志）。复制得到的描述符的执行时关闭标志（FD_CLOEXEC；）是关闭的。

### *#* dup3 24 

功能：分配一个新的文件描述符，文件描述符`newfd`会被调整，使其与`oldfd`引用同一个打开文件描述。成功时，返回新的文件描述符；出错时，返回 -1。

声明：`int dup3(int oldfd, int newfd, int flags);`

细节：

* 调用者可以通过在`flags`中指定`O_CLOEXEC`，强制为新文件描述符设置执行时关闭标志。

* 如果文件描述符`newfd`之前是打开的，在重新使用它之前会先将其关闭；关闭操作是静默执行的（即关闭过程中出现的任何错误，`dup3()`都不会报告）。
* 如果`oldfd`不是有效的文件描述符，那么调用失败，并且`newfd`不会被关闭；如果`oldfd`是有效的文件描述符，且`oldfd`等于`newfd`，那么`dup3()`会失败，并返回错误`EINVAL`。

### *#* chdir 49

功能：将调用进程的当前工作目录更改为`path`指定的目录。成功时，返回 0；出错时，返回 -1。

声明：`int chdir(const char *path);`

### *#* openat 56  

功能：相对于目录文件描述符`fd`打开文件，`path`参数指向一个命名文件的路径名。成功完成时，函数应打开文件并返回一个表示文件描述符的非负整数；否则，返回 -1 。

声明：`int openat(int fd, const char *path, int oflag, mode_t mode);`

细节：

* 应在`oflag`的值中准确指定以下前五个值（文件访问模式）中的一个：
  - **O_EXEC**：仅以执行模式打开（非目录文件）。如果将此标志应用于目录，结果是未定义的。
  - **O_RDONLY**：仅以只读模式打开。
  - **O_RDWR**：以读写模式打开。如果将此标志应用于 FIFO，结果是未定义的。
  - **O_SEARCH**：仅以搜索模式打开目录。如果将此标志应用于非目录文件，结果是未定义的。
  - **O_WRONLY**：仅以只写模式打开。
* mode：文件的所有权描述。

### *#* close 57

功能：关闭一个文件描述符`fd`，使其不再指向任何文件，并且该文件描述符可被重新使用。成功执行，返回0；失败，返回-1。

声明：`int close(int fd);`

细节：

* 如果`fd`是指向底层打开文件描述的最后一个文件描述符，那么与该打开文件描述相关联的资源将被释放；如果该文件描述符是指向已通过`unlink(2)`删除文件的最后一个引用，那么该文件将被删除。

### *#* getdents64 61

功能：打开文件描述符`fd`所指向目录中的若干个`linux_dirent64`结构体读取到`dirp`指向的缓冲区中。参数`count`指定了该缓冲区的大小。成功执行，返回读取的字节数，当到目录结尾，则返回0；失败，则返回-1。

声明：`ssize_t getdents64(int fd, struct linux_dirent64 *dirp, size_t count);`

涉及结构体：

```
struct linux_dirent64 {
    ino64_t        d_ino;    /* 64位索引节点号 */
    off64_t        d_off;    /* 并非偏移量；详见getdents() */
    unsigned short d_reclen; /* 这个目录项的大小 */
    unsigned char  d_type;   /* 文件类型 */
    char           d_name[]; /* 文件名（以空字符结尾） */
};
```

### *#* read 63

功能：从文件描述符`fd`所指向的文件中读取最多`count`字节的数据，并存入以`buf`起始的缓冲区。成功执行，返回读取的字节数。如为0，表示文件结束。错误，则返回-1。

声明：`ssize_t read(int fd, void *buf, size_t count;);`

细节：

* 对于支持随机访问的文件，读取操作从文件偏移量处开始，并且文件偏移量会按实际读取的字节数递增。若文件偏移量位于文件末尾或之后，将不会读取任何字节，此时`read()`返回 0。
* 若`count`为 0，`read()`可能会检测出下文所述的错误。若没有错误发生，或者`read()`未进行错误检查，那么`count`为 0 的`read()`调用将返回 0，且不会产生其他影响。

### *#* write 64

功能：从以`buf`为首地址的缓冲区中，最多读取`count`字节的数据，并写入到文件描述符`fd`所指向的文件中。成功执行，返回写入的字节数；错误，则返回-1。

声明：`ssize_t write(int fd, const void *buf, size_t count;);`

细节：

* 对于可随机访问的文件（即可以应用`lseek`函数的文件，如普通文件），写入操作从文件偏移量处开始，文件偏移量会按实际写入的字节数递增。如果文件是以`O_APPEND`标志打开，那么在写入前，文件偏移量会先被设置到文件末尾。文件偏移量的调整和写入操作是原子性执行的。
* 如果`count`为 0 且`fd`指向普通文件，那么如果检测到错误，`write()`可能返回失败状态。如果未检测到错误，或者未执行错误检测，则返回 0 且不产生其他任何影响。如果`count`为 0 且`fd`指向普通文件以外的其他文件，其结果未作明确规定。

### *#* linkat 37

功能：创建一个指向现有文件的新链接（也就是硬链接）。成功执行，返回0；失败，返回-1。

声明：`int linkat(int olddirfd, const char *oldpath, int newdirfd, const char *newpath, int flags);`

细节：

* 要是新路径已经存在，它不会被覆盖。
* 要是 oldpath 参数给出的路径名是相对路径，那么它会被当作相对于文件描述符 olddirfd 所指向的目录。
* 要是 oldpath 是相对路径，并且 olddirfd 的值为特殊值 AT_FDCWD，那么 oldpath 会被解释为相对于调用进程的当前工作目录。
* 要是 oldpath 是绝对路径，olddirfd 就会被忽略。
* newpath 的解释方式和 oldpath 相同，不过相对路径名会被解释为相对于文件描述符 newdirfd 所指向的目录。
* 在 Linux 2.6.18 版本之前，flags 参数是没有用的，必须将其指定为 0。

### *#* unlinkat 35

功能：从文件系统里删除一个名称。成功执行，返回0；失败，返回-1。

声明：`int unlinkat(int dirfd, const char *pathname, int flags);`

细节：

* 要是这个名称是某个文件的最后一个链接，而且没有进程打开该文件，那么这个文件就会被删除，它所占用的空间也会被释放出来以供重新使用。
* 要是这个名称是某个文件的最后一个链接，但仍有进程打开着该文件，那么这个文件会一直存在，直到最后一个引用它的文件描述符被关闭。
* 要是这个名称指向的是一个符号链接，那么这个链接会被移除。
* 要是这个名称指向的是一个套接字、FIFO 或者设备，那么它的名称会被移除，不过已经打开这个对象的进程仍然可以继续使用它。
* 要是 pathname 参数给出的路径名是相对路径，那么它会被解释为相对于文件描述符 dirfd 所指向的目录。
* 要是 pathname 是相对路径，并且 dirfd 的值为特殊值 AT_FDCWD，那么 pathname 会被解释为相对于调用进程的当前工作目录。
* 要是 pathname 是绝对路径，dirfd 就会被忽略。
* flags 是一个位掩码，它可以被指定为 0或AT_REMOVEDIR。

### *#* mkdirat 34

功能：创建一个名为 `pathname `的目录。成功执行，返回0；失败，返回-1。

声明：`int mkdirat(int dirfd, const char *pathname, mode_t mode);`

细节：

* 参数 mode 用于设定新目录的权限模式。
* 要是 pathname 参数给出的路径名是相对路径，那么它会被解释为相对于文件描述符 dirfd 所指向的目录。
* 要是 pathname 是相对路径，并且 dirfd 的值为特殊值 AT_FDCWD，那么 pathname 会被解释为相对于调用进程的当前工作目录。
* 要是 pathname 是绝对路径，dirfd 就会被忽略。

### *#* umount2 39

功能：用于卸载目标文件系统，但允许使用额外的标志`flags`来控制操作行为。成功返回0，失败返回-1。

声明：` int umount2(const char *target, int flags);`

### *#* mount 40

功能：将 source 参数所指定的文件系统（它通常是一个指向设备的路径名，但也可以是目录、文件的路径名，或者是一个虚拟字符串）挂载到 target 参数里的路径名所指定的位置（一个目录或文件）。成功返回0，失败返回-1。

声明：`int mount(const char *source, const char *target,const char *filesystemtype, unsigned long mountflags,const void *data);`

细节：

* 挂载文件系统需要有相应的权限。
* 内核所支持的 filesystemtype 参数值会列在 /proc/filesystems  文件中（例如：“btrfs”、“ext4”、“jfs”、“xfs”、“vfat”、“fuse”、“tmpfs”、“cgroup”、“proc”、“mqueue”、“nfs”、“cifs”、“iso9660”）。当加载了相应的模块之后，可能会有更多的文件系统类型可用。
* data 参数会由不同的文件系统进行解析。一般来说，它是一个由逗号分隔的选项字符串，这些选项能被该文件系统所识别。如果没有选项，可以将此参数指定为 NULL。
* 根据 mountflags 参数中设置的位，调用 mount () 函数可以执行多种不同类型的操作。具体执行哪种操作，是通过检查 mountflags 中设置的位来决定的。

### *#* fstat 80

功能：用于获取与文件描述符 fildes 相关联的已打开文件的信息，并将这些信息写入由 buf 指向的区域。成功返回0，失败返回-1。

声明：`int fstat(int fildes, struct stat *buf);`

细节：

* 如果 fildes 引用的是一个共享内存对象，实现应在 buf 参数指向的 stat 结构中更新 st_uid、st_gid、st_size 和 st_mode 字段，并且只有 S_IRUSR、S_IWUSR、S_IRGRP、S_IWGRP、S_IROTH 和 S_IWOTH  文件权限位需要有效。实现可以更新其他字段和标志。
* 如果 fildes 引用的是一个带类型的内存对象，实现应在 buf 参数指向的 stat 结构中更新 st_uid、st_gid、st_size 和 st_mode 字段，并且只有 S_IRUSR、S_IWUSR、S_IRGRP、S_IWGRP、S_IROTH 和 S_IWOTH  文件权限位需要有效。实现可以更新其他字段和标志。
* buf 参数是一个指向 stat 结构的指针，该结构在 <sys/stat.h> 中定义，其中放置了与文件相关的信息。
* 对于 POSIX.1-2017 该卷中定义的所有其他文件类型，结构成员  st_mode、st_ino、st_dev、st_uid、st_gid、st_atim、st_ctim 和 st_mtim  应具有有意义的值，并且 st_nlink 成员的值应设置为指向该文件的链接数。
* 如果实现提供了额外的或替代的文件访问控制机制，在实现定义的条件下，可能会导致 fstat () 函数失败。
* fstat () 函数应在将信息写入 stat 结构之前，更新任何与时间相关的字段。

涉及结构体：

```
struct stat {
	dev_t st_dev;
	ino_t st_ino;
	mode_t st_mode;
	nlink_t st_nlink;
	uid_t st_uid;
	gid_t st_gid;
	dev_t st_rdev;
	unsigned long __pad;
	off_t st_size;
	blksize_t st_blksize;
	int __pad2;
	blkcnt_t st_blocks;
	long st_atime_sec;
	long st_atime_nsec;
	long st_mtime_sec;
	long st_mtime_nsec;
	long st_ctime_sec;
	long st_ctime_nsec;
	unsigned __unused[2];
};
```

### *#* clone 220

功能：创建一个子进程，控制调用进程和子进程之间共享的执行上下文的具体部分。成功时，子进程的线程 ID 会在调用者的执行线程中返回；失败时，在调用者的上下文里会返回 -1，不会创建子进程。

声明：

```
int clone(int flags,
          void *stack,
          pid_t *parent_tid,
          void *tls,
          pid_t *child_tid);
```

细节：

* stack 参数指定子进程使用的栈的位置。由于子进程和调用进程可能共享内存，子进程不可能与调用进程在同一个栈中执行。因此，调用进程必须为子进程的栈设置内存空间，并将指向该空间的指针传递给 clone ()。

* flags: 创建的标志，如SIGCHLD；

  stack: 指定新进程的栈，可为0；

  ptid: 父线程ID；

  tls: TLS线程本地存储描述符；

  ctid: 子线程ID；

### *#* execve 221

功能：执行由 pathname 所引用的程序。成功不返回，失败返回-1。

声明：`int execve(const char *pathname, char *const argv[], char *const envp[]);`

细节：

* 这会导致调用进程当前正在运行的程序被一个新程序所取代，新程序拥有新初始化的栈、堆以及（已初始化和未初始化的）数据段。
* pathname 必须要么是一个二进制可执行文件，要么是一个以如下形式的行开头的脚本：`#!interpreter [optional-arg]`
* argv 是一个指针数组，这些指针指向作为命令行参数传递给新程序的字符串。按照惯例，这些字符串中的第一个（即 argv  [0]）应该包含与正在执行的文件相关联的文件名。argv 数组必须以一个空指针结尾。（因此，在新程序中，argv [argc]  将是一个空指针。）
* envp 是一个指针数组，这些指针指向字符串，其形式通常为 key=value，它们作为新程序的环境变量被传递。envp 数组必须以一个空指针结尾。

### *#* wait4 260

功能：等待调用进程的子进程发生状态变化，并获取有关状态已改变的子进程的信息。成功则返回进程ID；如果指定了WNOHANG，且进程还未改变状态，直接返回0；失败则返回-1。

声明：`pid_t wait4(pid_t pid, int * wstatus, int options);`

细节：

* 状态变化被视为以下情况：子进程终止；子进程被信号停止；或者子进程被信号恢复。对于已终止的子进程，执行等待操作可使系统释放与该子进程相关的资源；如果不执行等待操作，那么已终止的子进程将处于 “僵尸” 状态。
* 如果子进程已经发生了状态变化，那么这些调用会立即返回。
* pid 的值可以是：
  - pid < -1：表示等待其进程组 ID 等于 pid 绝对值的任何子进程。
  - pid = -1：表示等待任何子进程。
  - pid = 0：表示等待其进程组 ID 等于调用 waitpid () 时调用进程的进程组 ID 的任何子进程。
  - pid > 0：表示等待其进程 ID 等于 pid 值的子进程。
* options 的值是零个或多个以下常量的按位或：
  - **WNOHANG**：如果没有子进程退出，则立即返回。
  - **WUNTRACED**：如果子进程已停止（但未通过 ptrace (2) 进行跟踪），也返回。即使未指定此选项，也会提供已停止的被跟踪子进程的状态。
  - **WCONTINUED（自 Linux 2.6.10 起）**：如果已停止的子进程已通过传递 SIGCONT 恢复，则也返回。
* 如果 wstatus 不为 NULL，会将状态信息存储在它所指向的整数中。

### *#* exit 93

功能：导致进程正常终止，并且状态值的最低有效字节（即 status & 0xFF）会返回给父进程。无返回值。

声明：`void exit(int status);`

细节：

* exit：终止状态值。

### *#* getppid 173

功能：返回调用进程的父进程 ID。成功返回父进程ID。

声明：`pid_t getppid(void);`

细节：

* getppid () 函数总是会成功执行，并且没有保留任何返回值来表示错误。

### *#* getpid 172

功能：返回调用进程的进程 ID。成功返回进程ID。

声明：`pid_t getpid(void);`

细节：

* 函数总是成功执行。

### *#* brk 214

功能：调整程序断点（program break）的位置，该断点定义了进程数据段的末尾（即程序断点是未初始化数据段结束后的第一个位置）。成功返回0，失败返回-1。

声明：`int brk(void *addr);`

细节：

* 增大程序断点会为进程分配内存；减小程序断点则会释放内存。

* 当 `addr` 值合理、系统有足够内存且进程未超出其最大数据段大小时，将数据段末尾设置为 `addr` 指定的值。

### *#* munmap 215

功能：移除进程地址空间中，从 `addr` 起始并延续 `len` 字节范围内的所有页映射。成功返回0，失败返回-1。

声明：`int munmap(void *addr, size_t len);`

细节：

* 实现可能要求 `addr` 是由 `sysconf()` 返回的页大小的整数倍。
* 若待移除的映射为私有映射，则在此地址范围内所做的任何修改都将被丢弃。
* 若从类型化内存对象中移除的映射导致内存池的对应地址范围无法被系统中的任何进程访问（除非通过可分配映射，即使用 `POSIX_TYPED_MEM_MAP_ALLOCATABLE` 标志打开的类型化内存对象的映射），则该内存池范围将被释放，并可能用于满足未来的类型化内存分配请求。

### *#* mmap 222

功能：用于在调用进程的虚拟地址空间中创建一个新的映射。成功返回已映射区域的指针，失败返回-1。

声明：`void *mmap(void *addr, size_t length, int prot, int flags, int fd, off_t offset);`

细节：

* 新映射的起始地址由 `addr` 指定，`length` 参数指定映射的长度（必须大于 0）。

* 关于addr：

  * 若 `addr` 为 `NULL`，内核会自动选择页面对齐的地址创建映射（推荐方式）。

  * 若 `addr` 非 `NULL`，内核将其作为映射位置的建议。

* 文件映射的内容会使用文件描述符 `fd` 所引用的文件（或其他对象）中从偏移量 `offset` 开始的 `length` 个字节来初始化。`offset` 必须是由 `sysconf(_SC_PAGE_SIZE)` 返回的页大小的倍数。

* `prot` 参数描述了对映射所需的内存保护（并且不能与文件的打开模式冲突）。它要么是 `PROT_NONE`，要么是以下一个或多个标志的按位或：

  - `PROT_EXEC`：页面可以被执行。
  - `PROT_READ`：页面可以被读取。
  - `PROT_WRITE`：页面可以被写入。
  - `PROT_NONE`：页面不能被访问。

* `flags` 参数决定了对映射的更新是否对映射同一区域的其他进程可见，以及更新是否会传播到底层文件。

### *#* times 153

功能：将当前进程的时间存储在 `buf` 所指向的 `tms` 结构体中。成功返回已经过去的滴答数，失败返回-1。

声明：`clock_t times(struct tms *buf);`

细节：

* `tms_utime` 字段包含调用进程执行指令所花费的 CPU 时间。`tms_stime` 字段包含代表调用进程在内核中执行任务所花费的 CPU 时间。
* `tms_cutime` 字段包含所有已等待的终止子进程的 `tms_utime` 和 `tms_cutime` 值的总和。`tms_cstime` 字段包含所有已等待的终止子进程的 `tms_stime` 和 `tms_cstime` 值的总和。
* 已终止子进程（及其后代进程）的时间是在 `wait` 或 `waitpid` 返回它们的进程 ID 时添加的。特别地，子进程未等待的孙进程的时间是不会被统计的。
* 所报告的所有时间的单位都是时钟滴答数。

涉及结构体：

```
struct tms {
    clock_t tms_utime;  // 用户时间
    clock_t tms_stime;  // 系统时间
    clock_t tms_cutime; // 子进程的用户时间
    clock_t tms_cstime; // 子进程的系统时间
};
```

### *#* uname 160

功能：将系统信息存储在 `buf` 所指向的结构体中。成功返回0，失败返回-1。

声明：`int uname(struct utsname *buf);`

涉及结构体：

```
struct utsname {
    char sysname[];    // 操作系统名称（例如，"Linux"）
    char nodename[];   // 节点所连接的通信网络中的名称（如果有的话）
    char release[];    // 操作系统发行版本（例如，"2.6.28"）
    char version[];    // 操作系统版本
    char machine[];    // 硬件类型标识符
#ifdef _GNU_SOURCE
    char domainname[]; // NIS 或 YP 域名
#endif
};
```

### *#* sched_yield 124

功能：使调用线程放弃 CPU。成功返回0，失败返回-1。

声明：`int sched_yield(void);`

细节：

* 该线程会被移至与其静态优先级对应的队列末尾，然后会有一个新线程开始运行。

### *#* gettimeofday 169

功能：获取时间以及时区。成功返回0，失败返回-1。

声明：`int gettimeofday(struct timeval *tv, NULL);`

细节：

* `tv`给出自纪元时间以来的秒数和微秒数。

涉及结构体：

```
struct timeval {
    time_t      tv_sec;     // 秒
    suseconds_t tv_usec;    // 微秒
};
```

### *#* nanosleep 101

功能：暂停调用线程的执行，直到至少经过 `*duration` 中指定的时间，或者传递了一个信号，该信号会触发调用线程中信号处理程序的执行，或者终止进程。成功返回0，失败返回-1。

声明：`int nanosleep(const struct timespec *duration, struct timespec * rem);`

细节：

* 如果调用被信号处理程序中断，`nanosleep()` 将返回 -1，将 `errno` 设置为 `EINTR`，并且会将剩余时间写入 `rem` 所指向的结构体（除非 `rem` 为 `NULL`）。然后，`*rem` 的值可用于再次调用 `nanosleep()` 并完成指定的暂停操作。
* `timespec` 结构体用于以纳秒精度指定时间间隔。

涉及结构体：

```
struct timespec {
	time_t tv_sec;        /* 秒 */
	long   tv_nsec;       /* 纳秒, 范围在0~999999999 */
};
```



## 调用

```
static inline _u64 internal_syscall(long n, _u64 _a0, _u64 _a1, _u64 _a2, _u64
		_a3, _u64 _a4, _u64 _a5) {
	register _u64 a0 asm("a0") = _a0;
	register _u64 a1 asm("a1") = _a1;
	register _u64 a2 asm("a2") = _a2;
	register _u64 a3 asm("a3") = _a3;
	register _u64 a4 asm("a4") = _a4;
	register _u64 a5 asm("a5") = _a5;
	register long syscall_id asm("a7") = n;
	asm volatile ("ecall" : "+r"(a0) : "r"(a1), "r"(a2), "r"(a3), "r"(a4), "r"
			(a5), "r"(syscall_id));
	return a0;
}
```
