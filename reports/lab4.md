#### linkat
我理解linkat创建硬链接的过程等于是复制一个已经打开的的文件inode。

当使用 link 系统调用（或 linkat）时，不会创建新的文件数据，而是 在文件系统中新增一个目录项（dentry），指向 同一个 inode。

1. 首先根据old_name找到原始的inode: old_name -> inode

2. 然后复制inode_id，创建一个新的dir_entry：(new_name, inode)
dir_entry其实就是写inode_block

3. 需要增加inode的nlink
在diskinode中添加nlink字段，

需要考虑，如果直接增加该字段的话，这会导致diskinode的大小发生变化，
而原始diskinode的大小是128字节，和块大小512字节对齐，inodes_per_block=4
加了之后，导致inodes_per_block=3，且存在块内碎片

所以我们的策略是，增加nlink字段之后，把将direct直接索引数减一，这样还是保持diskinode大小为128字节，只是最大文件大小减小了一点，影响不大。

4. 修复了一个bug，bug表现出来时：open_file时create的inode的inode id，和link查找到的inode id不同，导致访问到的block不正确，nlink始终显示为0
   
原因：`inode::inode_id()`实现有问题，没有考虑到inode id的起始block偏移（inode_area_start_block），导致link查找inode的block时，到了错误的block，从而导致数据错误。

修复方案：详见commit，（改了两次，第一次改了还是有问题，第二次是gpt生成的，完全没问题，蚌）


#### unlink

##### questions
1. inode 的释放问题

#### fstat
##### questions
    1. 如何根据fd文件描述符获取其对应的inode？
   
    fd_table中存的是`dyn File`的trait对象，目前其实际对象包括：stdin、stdout和OSInode。只有OSInode存在inode，stdin和stdout都没有inode，所以应该需要获取到该trait对象的原始对象。当原始对象不是OSInode时，返回-1。

    那么问题来了，如何将`trait object`转换成原始类型呢？貌似直接导入`core::any::Any`，然后就可以强制转换成`&dyn Any`就可以了。但是根据https://stackoverflow.com/questions/33687447/how-to-get-a-reference-to-a-concrete-type-from-a-trait-object。我们需要需要给File实现Any trait，同时还需要实现一个自定义的as_any方法。
    
    折腾到最后才发现，过于复杂了，既然反正都要修改File trait的定义了，为啥不直接在File trait里添加能否直接返回state的接口呢？仔细想了想，这才是正确的设计思路。因为，state是File的属性，就应该是在File里面存在公共的访问接口，然后各个具体的类型给出自己实现。不应该还需要使用者自己去downcasting到具体的类型才能获取到state。

    2. 但是我们只能通过OSInode获取到inode的内容，却无法获取到inode number，如何解决？

    我觉得最直接的方式是实现`get_diskinode_pos`的逆过程
